/*
Что можно настроить через RUST_LOG
Синтаксис RUST_LOG довольно гибкий и позволяет точно управлять фильтрацией.

Глобальный уровень: Просто укажите уровень, например, RUST_LOG=info. Тогда будут выводиться все сообщения уровня info и выше (warn, error) из всех модулей проекта .

bash
RUST_LOG=info ./my_app
Поуровнево для модулей: Можно задать разные уровни для разных модулей (крейтов). Формат такой: path::to::module=level. Уровень по умолчанию указывается первым .

bash
RUST_LOG="warn,my_crate::module1=info,my_crate::module2=debug" ./my_app
Эта команда установит:

По умолчанию уровень warn для всех.

Для my_crate::module1 — уровень info.

Для my_crate::module2 — уровень debug.

Отключение логирования: Используйте псевдо-уровень off, чтобы полностью отключить вывод для всего приложения или конкретного модуля .

bash
set RUST_LOG=debug
cargo run
Доступные уровни логирования (от наиболее до наименее подробного): error, warn, info, debug, trace . Регистр букв в названиях уровней не имеет значения .    

// --------------------------------------------

Запуск:
cargo run -- -i "https://avatars.mds.yandex.net/i?id=4964ba82da9ed35f073d39b81a0b98f2c913fc4c-5400140-images-thumbs&n=13 tmp_contents\rust.png" -f tmp_contents\imgs_file.txt --config-file tmp_contents\config.toml -r 1
Или так:
more tmp_contents\from_stdin.txt|cargo run -- -i "https://avatars.mds.yandex.net/i?id=4964ba82da9ed35f073d39b81a0b98f2c913fc4c-5400140-images-thumbs&n=13 tmp_contents\rust.png" -f tmp_contents\imgs_file.txt --config-file tmp_contents\config.toml --stdin -r 1
Или так:
cargo run -- -i "https://avatars.mds.yandex.net/i?id=4964ba82da9ed35f073d39b81a0b98f2c913fc4c-5400140-images-thumbs&n=13 tmp_contents\rust.png" -f tmp_contents\imgs_file.txt --stdin --config-file tmp_contents\config.toml -r 1 < tmp_contents\from_stdin.txt

C:\Users\user\work\MyWorks\Rust\rust-incubator\3_ecosystem\src\main.rs



// Структура проекта
3_ecosystem/
├── Cargo.toml              <- конфигурация программы
├── src/
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