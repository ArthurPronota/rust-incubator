use crate::conf_load::ConfigLoad ;

use std::{
        time::{
            // Тип представляющий промежуток времени
            Duration, 
            // Тип для представления момента времени
            Instant
        }
    };

// крейт для гибкой обработки ошибок в Rust
use anyhow::{
        // Тип возвращаемого значения
        Result,
    } ;

// Расширение функциональной асинхронных потоков (streams)
use futures::StreamExt;

// крейт image
use image::{
        self,
        // Перечисление форматов изображений
        ImageFormat,
    };

// Крейт для работы с JPEG
use mozjpeg::{
        Compress,   // Структура для сжатия JPEG
        ColorSpace  // Цветовые пространства JPEG
    };

// Крейт для работы с PNG
use oxipng::{
    self,
    optimize_from_memory, // Функция оптимизации PNG в памяти
    Options     // Настройки оптимизации PNG
};

// Крейт для криптографического хеширования
use sha2::{
        Sha256,     // Алгоритм хеширования SHA-256
        Digest      // Трейт для работы с хешами
    };

// Макрос для логирования уровня info
use log::info ;

// Крейт для работы с метаданными изображений
use web_image_meta::{
            jpeg,   // Модуль для работы с JPEG метаданными
            png,    // Модуль для работы с PNG метаданными
        } ;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Ubuntu Chromium/37.0.2062.94 Chrome/37.0.2062.94 Safari/537.36" ;

/// Является ли path_to_img url
fn path_to_img_is_url(path_to_img: &str) ->bool {
    if path_to_img.starts_with("https://") || path_to_img.starts_with("http://") {
        true
    } else {
        false
    }
}

/// Загрузка изображения
pub async fn download_img(
            path_to_img: &str,          // путь к изображению
            conf_now: &ConfigLoad       // текущая конфигурация
         ) ->Result<()> {
    // Возвращает момент времени, соответствующий текущему состоянию.
    let start = Instant::now();

    let mut img_bytes = if path_to_img_is_url(path_to_img) { // Это url

        // Создание асинхронного HTTP клиента
        let http_client = match reqwest::Client::builder() 
                                // Установите тайм-аут только для этапа подключения клиента.
                                .connect_timeout(std::time::Duration::from_secs(conf_now.time_out as u64))
                                // Включает тайм-аут запроса.
                                .timeout(std::time::Duration::from_secs(conf_now.time_out as u64))
                                // Задает заголовок User-Agent, который будет использоваться данным клиентом.
                                .user_agent(USER_AGENT)
                                // Возвращает объект Client, использующий данную конфигурацию ClientBuilder.
                                .build() {
            Ok(v) => v,
            Err(err) => return Err(err.into())
        } ;

        // Выполнить запрос созданным http_client
        let resp = http_client
                    // Удобный способ выполнения GET-запроса к URL-адресу.
                    .get(path_to_img)
                    // Формирует запрос и отправляет его на целевой URL, возвращая в ответ полученный результат.
                    .send()
                    // Приостановить выполнение до тех пор, пока результат выполнения 
                    // Future не будет готов.
                    .await
                    // преобразует текущую ошибку в anyhow формат
                    .map_err(|err|
                        anyhow::anyhow!("{} from: {}", err, path_to_img)
                    )? ;

        // Проверьте, что статус не находится в диапазоне 200-299.
        if ! resp.status().is_success() {
            return Err(anyhow::anyhow!("Error loading URL: {}, code: {}", path_to_img, resp.status()));
        }

        if conf_now.rate_limit == 0 { // нет ограничений по скорости загрузки
            // получение тела ответа
            resp
                // Получите полное тело ответа в виде байтов.
                .bytes()
                // Приостановить выполнение до тех пор, пока результат выполнения 
                // Future не будет готов.                
                .await
                // преобразует текущую ошибку в anyhow формат
                .map_err(|err|
                    anyhow::anyhow!("{} from: {}", err, path_to_img)
                )?
                // Конвертировать байты в вектор
                .to_vec()
        } else {    // выполнить загрузку с ограничением скорости загрузки
            // Преобразовать ответ из тела запроса в поток байтов.
            let mut stream = resp.bytes_stream() ;
            // количество загруженных байтов
            let mut downloaded = 0 ;
            // Возвращает момент времени, соответствующий текущему состоянию.
            let start = Instant::now() ;
            // вектор полученных байтов
            let mut all_data = Vec::new();

            while let Some(chunk) = 
                        stream
                            // получение порции байтов из асинхронного потока
                            .next()
                            // Приостановить выполнение до тех пор, пока результат выполнения 
                            // Future не будет готов.
                            .await 
            {
                let chunk = 
                        // Обработка ошибки в полученной порции байтов
                        chunk
                            // преобразует текущую ошибку в anyhow формат
                            .map_err(|err|
                                anyhow::anyhow!("{} from: {}", err, path_to_img)
                            )? ;
                // Обновляем итоговое количество полученных байт
                downloaded += chunk.len() ;

                // Добавляем полеченные байты  в массив
                all_data.extend_from_slice(&chunk);

                // определяем итоговое время загрузки изобрадения как f64
                let elapsed = start.elapsed().as_secs_f64();

                // вычисляем ожидаемое количество байт которые должно быть загружено за текщее итоговое время 
                // на основании установленного ограничения скорости загрузки
                let expected_bytes = (elapsed * (conf_now.rate_limit * 1024) as f64) as usize ;

                // загружено больше чем ожидалось, необходима пауза
                if downloaded > expected_bytes {
                    // выполнить sleep в асинхронном режиме
                    tokio::time::sleep(
                        Duration::from_secs_f64(
                            // расчёт времени ожидания в sec
                            (downloaded - expected_bytes) as f64 / (conf_now.rate_limit * 1024) as f64
                        )
                    )
                    // Приостановить выполнение до тех пор, пока результат выполнения 
                    // Future не будет готов.
                    .await ;
                }
            }

            all_data    // возврат полученных данных
        }
    } else {  // Это файл
        // асинхронное чтение img файла
        tokio::fs::read(path_to_img)
            // Приостановить выполнение до тех пор, пока результат выполнения 
            // Future не будет готов.
            .await
            // преобразует текущую ошибку в anyhow формат
            .map_err(|err|
                anyhow::anyhow!("{} from: {}", err, path_to_img)
            )?
    } ;

    // Определить формат изображения по блоку памяти img
    let img_format = match image::guess_format(&img_bytes) {
        // Это Jpeg или Png
        Ok(imf) if imf == ImageFormat::Jpeg || imf == ImageFormat::Png=> imf,
        // Это иной формат img
        Ok(imf_other) => {
            return Err(anyhow::anyhow!("Unsupported image format: {:?} from {}", imf_other, path_to_img)) ;
        },
        // Возникла ошибка при определении формата img
        Err(err) => {
            return Err(anyhow::anyhow!("{} from: {}", err, path_to_img));
        }
    } ;

    // удаление метаданных из файла изображения
    img_bytes = match img_format {
        ImageFormat::Jpeg => {
            // очистка метаданных в Jpeg
            jpeg::clean_metadata(&img_bytes)
                // преобразует текущую ошибку в anyhow формат
                .map_err(|err|
                    anyhow::anyhow!("{} from: {}", err, path_to_img)
                )?
        },
        ImageFormat::Png => {
            // очистка метаданных в Png
            png::clean_chunks(&img_bytes)
                // преобразует текущую ошибку в anyhow формат
                .map_err(|err|
                    anyhow::anyhow!("{} from: {}", err, path_to_img)
                )?
        },
        _ => {  // недопустимый тип изображения
            return Err(anyhow::anyhow!("Unsupported image format: {:?} from {}", img_format, path_to_img)) ;            
        },
    } ;

    // изменение качества изображения
    if conf_now.img_quality != 100 {    // требуемое качество изображении != 100
        img_bytes = match img_format {
            ImageFormat::Jpeg => {  // изменение качество изображения для Jpeg
                let img = 
                            // Создать новое изображение из байтового среза.
                            image::load_from_memory(&img_bytes)
                                // преобразует текущую ошибку в anyhow формат
                                .map_err(|err|
                                    anyhow::anyhow!("{} from: {}", err, path_to_img)
                                )? ;

                // Возвращает копию этого изображения в формате RGB.
                let rgb_img = img.to_rgb8() ;
                // Получить ширину и высоту этого изображения.
                let (width, height) = rgb_img.dimensions() ;
                // Возвращает базовый необработанный буфер
                let pixels = rgb_img.as_raw() ;

                // создание компесора изображения
                let mut comp = Compress::new(ColorSpace::JCS_RGB) ;
                // установить размер сжимаемого изображения
                comp.set_size(width as usize, height as usize);
                // установить качество сжимаемого изображения
                comp.set_quality(conf_now.img_quality as f32) ;
            
                let mut comp_started = 
                            comp
                                // старт сжатия изображения
                                .start_compress(Vec::new())
                                // преобразует текущую ошибку в anyhow формат
                                .map_err(|err|
                                    anyhow::anyhow!("{} from: {}", err, path_to_img)
                                )? ;
            
                comp_started
                    // Возвращает Ok(()), если все строки в image_src были записаны.
                    .write_scanlines(pixels)
                    // преобразует текущую ошибку в anyhow формат
                    .map_err(|err|
                        anyhow::anyhow!("{} from: {}", err, path_to_img)
                    )? ;

                comp_started
                    // Завершить сжатие
                    .finish()
                    // преобразует текущую ошибку в anyhow формат
                    .map_err(|err| 
                        anyhow::anyhow!("{} from: {}", err, path_to_img)
                    )?
            },
            ImageFormat::Png => { // изменение качество изображения для Png
                // определить степень сжатия для Png на основании текущей конфигурации программы
                let mut options_img = Options::from_preset((conf_now.img_quality as u64 * 7 / 100) as u8) ;

                // текущий тип чересстрочной развертки не изменится.
                options_img.interlace = None ;

                // изменяет качество Png
                optimize_from_memory(&img_bytes, &options_img)
                    // преобразует текущую ошибку в anyhow формат
                    .map_err(|err|
                        anyhow::anyhow!("{} from: {}", err, path_to_img)  
                    )?
            },
            _ => {  // недопустимый тип изображения
                return Err(anyhow::anyhow!("Unsupported image format: {:?} from {}", img_format, path_to_img)) ;            
            },
        } ;
    }

    // создатём имя выходного файла как хеш на основе Url/Path изображения
    let mut hasher = Sha256::new() ;

    // Обработка данных, обновление внутреннего состояния.
    hasher.update(path_to_img);

    // Получить результат и использовать экземпляр хеширования.
    let result_hasher = hasher.finalize() ;

    // сформировать путь к файлу изображения
    let out_file_path = conf_now
                                    .img_output_dir
                                    .join(
                                        format!("{}.{}",
                                            hex::encode(result_hasher), // Кодирует данные в виде шестнадцатеричной строки, используя символы нижнего регистра.
                                            img_format
                                                // возвращает первое строковое расширение для данного формата
                                                .extensions_str()[0]
                                                // приведение его к нижнему регистру
                                                .to_lowercase()
                                        )
                                    ) ;

    let mut out_file = 
                // Открывает файл в режиме только для записи в асинхронном режиме
                tokio::fs::File::create(&out_file_path)
                    // Приостановить выполнение до тех пор, пока результат выполнения 
                    // Future не будет готов.
                    .await
                    // преобразует текущую ошибку в anyhow формат
                    .map_err(|err|
                        anyhow::anyhow!("{}, cannot create file: {:?}", err, out_file_path)
                    )?;

    // Асинхронно копирует всё содержимое в созданный файл
    tokio::io::copy(
                &mut &img_bytes[..],
                &mut out_file
            )
            // Приостановить выполнение до тех пор, пока результат выполнения 
            // Future не будет готов.
            .await
            // преобразует текущую ошибку в anyhow формат
            .map_err(|err|
                anyhow::anyhow!("{}, cannot copy data to file: {:?}", err, out_file_path)
            )?;

    // Запись итогового времени выполнения в логи.
    let elapsed = start.elapsed();
    info!("Processed {} in {:?}", path_to_img, elapsed);

    Ok(())
}

// Тесты
#[cfg(test)]
mod tests {
    use super::* ;

    /// Проверка https протокола в url img
    #[test]
    fn check_https_url() {
        assert_eq!(path_to_img_is_url("https://site.org/"), true) ;
    }

    /// Проверка http протокола в url img
    #[test]
    fn check_http_url() {
        assert_eq!(path_to_img_is_url("http://site.org/"), true) ;
    }

    /// Проверка что это path а не url
    #[test]
    fn check_file() {
        assert_eq!(path_to_img_is_url(r"c:\o.txt"), false) ;        
    }
}

/// Проверка загрузки изображения в асинхронном режиме
#[tokio::test]
async fn check_download_img() {

    let conf = ConfigLoad::default() ;

    let v = download_img(
        "https://rust-lang.org/static/images/rust-social-wide.jpg", 
        &conf
    )
    .await
    .unwrap() ;
    
    assert_eq!(v, ())
}