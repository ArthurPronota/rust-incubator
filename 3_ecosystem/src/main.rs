/*
    Contact: https://artaudiochats.t.me/

    1. Структура проекта:

3_ecosystem/
├── Cargo.toml              <- конфигурация программы
├── src/                    <- директорий для хранения исходныъ кодов
│   ├── main.rs             <- точка входа в программу
│   ├── conf_load.rs        <- модуль конфигурации
│   ├── args.rs             <- модуль обработки агрементов CLI
│   └── downloader.rs       <- модуль загрузки изображений
├── tmp_contents/           <- директорий для хранения некоторыз бащовых данных
│   ├── config.toml         <- конфигурационный файл формата toml
│   ├── from_stdin.txt      <- файл со списоком img для stdin
│   ├── imgs_file.txt       <- файл со списоком img
│   ├── images.jpeg         <- файл img
│   ├── images2.png         <- файл img
│   └── rust.png            <- файл img
├── img_output/             <- директорий для хранения загруженных и обработанных img
└── README.md               <- файл с документацией

    2. Настройка уровней логирования:

Уровни логирования настраиваются через переменную окружения RUST_LOG
    a) Глобальный уровень: set RUST_LOG=info
        Для всех модулей выводятся сообщения от уровня info и выше
    b) Уровень по  умолчанию + уровни для отдельных модулей
        set RUST_LOG=info,oxipng=error
    c) Отключене логирования
        set RUST_LOG=off


    3. Помощи при запуске программы:   cargo run -- -h

Usage: step_3.exe [OPTIONS]

Options:
  -i, --images <Img>...           List of images as URLs or files: https://o.i/img1.jpg c:\img2.png
  -f, --images-file <FILE>        Path to the file containing the list of images (Urls,files).
  -c, --img-concurrency <NUMBER>  Number of images processed concurrency (1..10_000) [env: IMG_CONCURRENCY=]
  -o, --output-dir <DIR>          Output directory for storing processed images. [env: IMG_OUTPUT_DIR=]
  -q, --quality-img <NUMBER>      Output quality of processed images (1..100). [env: IMG_QUALITY=]
  -r, --rlim <NUMBER>             Rate limit for image downloads (KiB). If 0 then no restrictions [env: IMG_RATE_LIMIT=]
  -t, --timeout <NUMBER>          Timeout for loading one image in seconds (1..255) [env: IMG_TIMEOUT=]
      --config-file <FILE>        Path to the configuration file
      --stdin                     Read a list of images from STDIN
  -h, --help                      Print help (see more with '--help')

В полсказке указаны переменные окружения начинающиеся с MG_ для установки различных опций.
Если опцмя не указана а соответствующая переменная окружения установлена то опция получает значение от переменной окружения.


    4. Запуск Unit Tests: cargo test

running 6 tests
test args::tests::check_empty_list_img ... ok
test downloader::tests::check_file ... ok
test downloader::tests::check_http_url ... ok
test downloader::tests::check_https_url ... ok
test args::tests::check_not_empty_list_img ... ok
test downloader::check_download_img ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.95s

    5. Запуск E2E (End To End) тестов:

    5.1 С низкой скоростью загрузки:
cargo run -- -i "https://avatars.mds.yandex.net/i?id=4964ba82da9ed35f073d39b81a0b98f2c913fc4c-5400140-images-thumbs&n=13 tmp_contents\rust.png" -f tmp_contents\imgs_file.txt --config-file tmp_contents\config.toml -r 1

    5.2 С более ывсокой скоростью загрузки:
more tmp_contents\from_stdin.txt|cargo run -- -i "https://avatars.mds.yandex.net/i?id=4964ba82da9ed35f073d39b81a0b98f2c913fc4c-5400140-images-thumbs&n=13 tmp_contents\rust.png" -f tmp_contents\imgs_file.txt --config-file tmp_contents\config.toml --stdin -r 10

    5.3 Без ограничения скорости загрузки:
cargo run -- -i "https://avatars.mds.yandex.net/i?id=4964ba82da9ed35f073d39b81a0b98f2c913fc4c-5400140-images-thumbs&n=13 tmp_contents\rust.png" -f tmp_contents\imgs_file.txt --stdin --config-file tmp_contents\config.toml -r 0 < tmp_contents\from_stdin.txt

*/


mod args ;
mod conf_load;
mod downloader ;

// это тип, который представляет момент времени (точку на временной шкале).
// Он используется для измерения промежутков времени и производительности.
use std::time::Instant ;

// единый API для логирования
use log::{
        // Логирование с уровнем INFO 
        error, 
        // Логирование с уровнем ERROR
        info, 
};

// Асинхронные потоки.
use futures::stream::{
        self,
        // Расширение для класса Streams, предоставляющее множество удобных 
        // функций комбинаторов.
        StreamExt,
} ;

// это импорт модуля task из крейта tokio, который предоставляет 
// инструменты для работы с асинхронными задачами. 
use tokio::task ;

// Модуль для работы с процессами.
use std::process ;

/// Создание специальной среды выполнения (runtime), которая управляет 
/// асинхронными задачами. Tokio предоставляет такой runtime.
#[tokio::main]
async fn main() {
    // Возвращает момент времени, соответствующий текущему состоянию.
    let start_time = Instant::now();

    // Инициализирует глобальный логгер с помощью env logger.
    // Поддержка переменной окружения RUST_LOG
    env_logger::init();

    // Получить аргументы командной строки
    let args = args::get_args() ;

    // сформировать текущую конфигурацию
    let conf_now = match conf_load::load_config(&args) {
        Ok(v) => v,
        Err(err) => {
            error!("{}", err) ;
            process::exit(1) ;
        }
    } ;

    // Софрмировать список изображений для загрузки
    let list_images = match args::get_list_all_images(&args) {
        Ok(v) => v,
        Err(err) => {
            error!("{}", err) ;
            process::exit(1) ;
        }
    } ;

    // Проверка отсутствия картинок
    if list_images.is_empty() {
        error!("No images to process") ;
        process::exit(1) ;
    }

    let res_loaded = 
            // Преобразует обычный синхронный итератор (наш вектор путей к img)
            // в асинхронный поток (Stream)
            // Теперь каждый элемент можно обрабатывать асинхронно
            // Поток будет выдавать пути к img по одному, но в асинхронном контексте
            stream::iter(&list_images)
                // отобразить путь к img в новую асинхронную задачу
                .map(|u_f_in| {
                    // Создать строковую копию пути к img
                    let u_f = u_f_in.to_string() ;
                    // Клинировать текущую конфигурацию
                    let conf_clone = conf_now.clone() ;
                    // Порождает новую асинхронную задачу, возвращая для неё объект JoinHandle.
                    task::spawn(async move {
                        // Загрузить img в асинхронном режиме
                        downloader::download_img(
                                        &u_f,
                                        &conf_clone
                                    )
                                    // Приостановить выполнение до тех пор, пока результат выполнения 
                                    // Future не будет готов.                                    
                                    .await
                    })
                 }
                ) 
                // Принимает поток задач (JoinHandle)
                // Запускает до conf_now.img_concurrency задач одновременно
                // Возвращает результаты по мере завершения, не сохраняя порядок
                // Буферизует не больше conf_now.img_concurrency задач в памяти
                // начала запустятся первые conf_now.img_concurrency задач
                // Как только одна завершится, сразу запустится следующая
                // В памяти всегда не более conf_now.img_concurrency одновременно 
                // выполняющихся задач
                .buffer_unordered(conf_now.img_concurrency as usize)
                // Собирает все результаты потока в вектор
                // Тип элементов будет Result<Result<(String, String), Error>, JoinError>
                // Внешний Result — от task::spawn (ошибка выполнения задачи)
                // Внутренний Result — от download_page (ошибка загрузки)                            
                .collect::<Vec<_>>()
                // Приостановить выполнение до тех пор, пока результат выполнения 
                // Future не будет готов.
                .await
                ;

    // Цикл начинает выполняется после звершения всех асинхронных задач
    for result in res_loaded {
        match result {
            // Успешная асмнхронная загрузка img
            Ok(Ok(_)) => {
                //info!("Img loaded.") ;
            },
            // Вывод ошибки загрузки img в лог
            Ok(Err(err)) => {
                error!("Error load Img: {}", err) ;
            },
            // Вывод ошибки выполнения асинхронной задачи в лог
            Err(err) => {
                error!("Execution error: {}", err) ;
            },
        }
    }

    // Запись итогового времени выполнения в логи.
    let elapsed = start_time.elapsed();
    info!("Total processing time: {:?}", elapsed);
}