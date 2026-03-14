use serde::{Deserialize, Serialize} ;
use anyhow::Result ;
use std::f64::consts::E;
use std::path::Path ;
use std::net::IpAddr ;

/// Переменная окружения порт http сервера
const HTTP_PORT: &str = "HTTP_PORT" ;

/// Переменная окружения хост http сервера
const HTTP_HOST: &str = "HTTP_HOST" ;

/// Обозначение localhost
const LOCALHOST: &str = "localhost" ;

/// переменная окружения с путём соединения с DB
const DB_PATH_CONNECT: &str = "DB_PATH_CONNECT" ;    

/// Ответы сервера
#[derive(
    Serialize,
    Deserialize,
    Debug,
  )
]
pub enum Response {
    Success(String),
    Error(String),
}

/// получить базовый url
pub fn get_base_url(host: &str, port: u32) ->String {
    format!("http:://{}:{}/anycommand", host, port)
    //"https://mk.ru/".to_owned()
}

/// Получить все переменные env
pub fn get_all_env_vars() ->Result<(u32, String, String)> {

    if Path::new(".env")    // путь к .env файлу
        .exists() { // файл .env существует
            // загрузка переменных окружения из .env файла
            dotenv::dotenv()
                .map_err(|err|
                    anyhow::anyhow!(err)
                )? ;
        }

    // Установить хост http сервера
    let http_port = 
            // получить переменную окружения из HTTP_PORT
            std::env::var(HTTP_PORT)
                .map_err(|err|
                    anyhow::anyhow!(
                        "The environment variable: {} does not exist, error: {}",
                        HTTP_PORT,
                        err
                    )
                )?
                .trim()
                .parse::<u32>()
                .map_err(|err|
                    anyhow::anyhow!(
                        "Invalid http port: {}, error: {}",
                        HTTP_PORT,
                        err
                    )
                )? ;        

    // Установить хост http сервера
    let http_host = 
            match
                // получить переменную окружения из HTTP_HOST
                std::env::var(HTTP_HOST)
                    .map_err(|err|
                        anyhow::anyhow!(
                            "The environment variable: {} does not exist, error: {}",
                            HTTP_HOST,
                            err
                        )
                    )?
                    .trim()
                    .to_owned()
            {
                // Это localhost
                host if host == LOCALHOST => host,
                // Это не localhost
                host =>
                    host.parse::<IpAddr>()
                        .map_err(|err|
                            anyhow::anyhow!(
                                "Invalid ip: {}, error: {}",
                                HTTP_HOST,
                                err
                            )
                        )?
                        .to_string()
            } ;

    let db_path_conn = match
            std::env::var(DB_PATH_CONNECT)
            .map_err(|err|
                anyhow::anyhow!("{}", err)
            )?
            .trim() {
        v if v.is_empty() => return Err(anyhow::anyhow!("db_path_conn is empty.")),
        v => v.to_owned()
    } ;

    Ok((http_port, http_host, db_path_conn))
}