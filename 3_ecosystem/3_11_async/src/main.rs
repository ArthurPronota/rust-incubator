use clap::Parser ;
use num_cpus ;
use std::path::PathBuf ;

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

fn main() {
    let args = Args::parse();

    //println!("{}", num_cpus::get_physical()) ;

    println!("args: {:?}", args) ;
}

/*
use clap::Parser;
use futures::stream::{self, StreamExt};
use reqwest::Client;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::task;

/// Асинхронный загрузчик веб-страниц
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Максимальное количество одновременных загрузок
    #[clap(long, value_name = "NUMBER", default_value_t = num_cpus::get())]
    max_threads: usize,

    /// Путь к файлу со списком URL
    #[clap(value_name = "FILE")]
    file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Читаем URL из файла
    let content = fs::read_to_string(&args.file)?;
    let urls: Vec<String> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect();
    
    println!("Загружаю {} URL с максимальной параллельностью {}", 
             urls.len(), args.max_threads);
    
    // Создаём HTTP клиент с таймаутами
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    // Создаём директорию для загрузок если её нет
    fs::create_dir_all("downloads")?;
    
    // Загружаем страницы параллельно с ограничением
    download_pages(&client, &urls, args.max_threads).await?;
    
    Ok(())
}

async fn download_pages(
    client: &Client,
    urls: &[String],
    max_concurrent: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    // Используем поток с ограничением параллелизма
    let results: Vec<_> = stream::iter(urls)
        .map(|url| {
            let client = client.clone();
            let url = url.clone();
            
            // Каждая загрузка выполняется в своей задаче
            task::spawn(async move {
                download_single_page(&client, &url).await
            })
        })
        .buffer_unordered(max_concurrent)
        .collect()
        .await;
    
    // Обрабатываем результаты
    for result in results {
        match result {
            Ok(Ok(filename)) => println!("✓ Загружено: {}", filename),
            Ok(Err(e)) => eprintln!("✗ Ошибка загрузки: {}", e),
            Err(e) => eprintln!("✗ Ошибка задачи: {}", e),
        }
    }
    
    Ok(())
}

async fn download_single_page(client: &Client, url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Создаём безопасное имя файла из URL
    let filename = sanitize_filename(url);
    let filepath = format!("downloads/{}.html", filename);
    
    // Загружаем страницу
    let response = client.get(url).send().await?;
    let status = response.status();
    
    if !status.is_success() {
        return Err(format!("HTTP ошибка: {} для {}", status, url).into());
    }
    
    let content = response.text().await?;
    
    // Сохраняем в файл
    tokio::fs::write(&filepath, content).await?;
    
    Ok(filepath)
}

fn sanitize_filename(url: &str) -> String {
    url
        .replace("https://", "")
        .replace("http://", "")
        .replace(|c: char| !c.is_alphanumeric() && c != '.' && c != '-', "_")
        .chars()
        .take(100) // Ограничиваем длину
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(
            sanitize_filename("https://example.com/page.html"),
            "example.com_page.html"
        );
        assert_eq!(
            sanitize_filename("http://site.ru/путь/на/русском"),
            "site.ru__________"
        );
    }
}
*/