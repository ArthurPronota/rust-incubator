use clap::Parser ;
use num_cpus ;
use std::{
        char,
        path::{
            Path,
            PathBuf
        }
    } ;

use std::fs ;
use tokio::task ;
use reqwest::Client ;
use futures::stream::{
                self,
                StreamExt,
            } ;

const PAGES_SUB_DIR: &str = "content_pages" ;

/// Структура содержащая разобранные аргументы CLI
#[derive(
    // Автоматическая реализация трейт Parser от крейта clap для 
    // структуры Args. Благодаря этому, вызов Args::parse() сможет 
    // прочитать аргументы командной строки и заполнить поля структуры.
    Parser,
    // Отлка для println!(Args)
    Debug
)]
struct Args {
    #[clap(
        // Устанавливает только полную версию аргумента без предшествующего 
        // символа --, аргумент именованный: max-threads.
        // Это опционный (именованный аргумент)
        long="max-threads",

        // Заполнитель для значения аргумента в справочном сообщении/примере 
        // использования.
        value_name = "NUMBER",

        // Значение аргумента, если он отсутствует.
        // значение по умолчанию будет того же типа, что и 
        // поле num_cpus::get (usize).
        // default_value_t принимает значение непосредственно того типа, 
        // который нужен
        default_value_t = num_cpus::get()
    )]
    max_threads:    usize,

    // Аргумент file является позиционным, а не именованным.
    // т.к. здесь нет атрибута long (и нет short).
    // Аргумент обязательный
    #[clap(
        // Заполнитель для значения аргумента в справочном сообщении/примере 
        // использования.        
        value_name = "FILE",
    )]
    file:   PathBuf,
}

// загрузка страницы
async fn download_page(
            client: &Client,
            url: String
        )   ->Result<
                (
                    String, // url
                    String  // file
                ),
                Box<
                    dyn std::error::Error
                        + std::marker::Send
                        + std::marker::Sync
                >
            > 
{
    // формирование имени файла на основе данного url
    let file_name = url
                                .replace("https://", "")
                                .replace("http://", "")
                                .replace(
                                    // для 'с' обязательно задать тип
                                    |c: char| {
                                        !c.is_alphanumeric()
                                        && c != '.'
                                        && c != '-'
                                    },
                                    "_"
                                ) ;
    
    // путь к файлу с контентом страницы
    let file_path = format!("{}/{}.html", PAGES_SUB_DIR, file_name) ;

    let response = 
                client
                    // Удобный способ выполнения GET-запроса к URL-адресу.
                    .get(&url)
                    // Формирует запрос и отправляет его на целевой URL, 
                    // возвращая в ответ полученный результат.
                    .send()
                    // Приостановить выполнение до тех пор, пока результат выполнения 
                    // Future не будет готов.
                    .await? ;
    
    // проверка статуса ответа для клиента
    if !response
          .status()
          .is_success() {
        return Err(
                format!("HTTP code: {} for url: {}", response.status(), url)
                    .into()
                );
    }

    // контент страницы
    let content_page = response
                                // Получить полный текст ответа.
                                .text()
                                .await? ;

    
    // Создаёт объект Future, который откроет файл для записи и 
    // запишет в него всё его содержимое.
    // Это асинхронный эквивалент std::fs::write.    
    tokio::fs::write(&file_path, content_page) 
        .await? ;

    // Возвразаем url и путь к файлу с содежимым этого url
    Ok((url, file_path))
}


// загрузка страниц
async fn download_pages(
            client: &Client,
            //url: &[String],
            urls: &[&str],
            max_concurrents: usize
        ) ->Result<(), Box<dyn std::error::Error>> {

    let results = 
        // Преобразует итератор в асинхронный поток, который всегда готов
        // выдать следующее значение.    
        stream::iter(urls) 
            .map(|url| {
                // возвразает клон клиента
                let client = client.clone() ;
                // создать строку url из &str
                let url = url.to_string() ;
                // Запускает новую асинхронную задачу, возвращая для неё 
                // объект JoinHandle.
                task::spawn(async move {
                    download_page(
                        &client,
                        url
                    ).await
                })
            })
            // Адаптер для создания буферизованного списка ожидающих завершения 
            // фьючерсов 
            // Если элемент этого потока может быть преобразован в фьючерс, то 
            // адаптер буферизует до n фьючерсов, а затем возвращает выходные 
            // данные в порядке их завершения. В любой момент времени будет 
            // буферизовано не более n фьючерсов, а в зависимости от состояния 
            // каждого фьючерса может быть буферизовано и меньше n.
            // Возвращаемый поток будет представлять собой поток выходных 
            // данных каждого фьючерса.
            .buffer_unordered(max_concurrents)
            .collect::<Vec<_>>()
            .await
            ;

    for result in results {
        match result {
            // успешное завершение страницы
            Ok(Ok((url, file))) => println!("Download url: {} to file: {}", url, file),
            // ошибка при загрузке страницы
            Ok(Err(err)) => eprintln!("Error download: {}", err),
            // ошибка при выполнении задачи
            Err(err) => eprintln!("Error: {}", err),
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Args::parse();

    //println!("args: {:?}", args) ;

    // Проверка наличия файла со ссылками
    if ! args.file.exists() {
        return Err(format!("Not found file: {:?}", args.file).into()) ;
    }

    // Преобразует строковый фрагмент непосредственно в фрагмент Path.
    let dir = Path::new(PAGES_SUB_DIR) ;

    // создание директория для контента страниц
    if ! (dir.exists() && dir.is_dir()) {
        fs::create_dir_all(dir)? ;
    }

    // получаем контент файла файла с urls
    let contents = fs::read_to_string(args.file)? ;

    // заполняем вектор url из содержимого файла
    let urls = contents
                        .lines()
                        .filter(|line| !line.trim().is_empty())
                        .map(|line| line.trim())
                        .filter(|line| line.starts_with("https://") || line.starts_with("http://"))
                        .collect::<Vec<_>>()
                        ;
    if urls.is_empty() {
        return Err(format!("There are no URLs to load pages.").into()) ;
    }

    // Асинхронный клиент для отправки запросов.
    let client = Client::builder()
                    .timeout(std::time::Duration::from_secs(30))
                    .build()? ;

    download_pages(&client, &urls, args.max_threads)
        .await? ;

    // Нормальное завершение работы
    Ok(())
}