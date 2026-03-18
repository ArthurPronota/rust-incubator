use anyhow::Result ;

use std::path::Path ;

use std::net::IpAddr ;

use dotenv ;


/// Переменная окружения порт http сервера
const HTTP_PORT: &str = "HTTP_PORT" ;

/// Переменная окружения хост http сервера
const HTTP_HOST: &str = "HTTP_HOST" ;

/// Обозначение localhost
const LOCALHOST: &str = "localhost" ;

/// переменная окружения с путём соединения с DB
const DB_PATH_CONNECT: &str = "DB_PATH_CONNECT" ;    


/// Получить все переменные env
pub fn get_all_env_cars() ->Result<
                                //(u32, String, String)
                                ()
                            > {
    // Считывание содержимого из .env файла с установкой переменных 
    // окружения если таковые не определены
    if Path::new(".env")    // путь к .env файлу 
            .exists() { // файл .env существует 
        // загрузка переменных окружения из .env файла                
        dotenv::dotenv()
            .map_err(|err|
                anyhow::anyhow!(err)
            )? ;
    }

    // Установить порт http сервера
    let http_port = match 
            std::env::var(HTTP_PORT)
                .map_err(|err|
                    anyhow::anyhow!(err)
                )?
                .trim()
                .parse::<u32>()
                .map_err(|err|
                    anyhow::anyhow!(err)
                )?
        {
            v if v == 0 => return Err(anyhow::anyhow!("Invalid value: {} of http_host", v)),
            v => v,
        } ;


    Ok(())
}