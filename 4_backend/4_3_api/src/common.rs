use anyhow::Result ;
use serde::{Deserialize, Serialize};

use std::path::Path ;

use std::net::IpAddr ;

use dotenv ;

use utoipa::ToSchema;

/// Переменная окружения порт http сервера
const HTTP_PORT: &str = "HTTP_PORT" ;

/// Переменная окружения хост http сервера
const HTTP_HOST: &str = "HTTP_HOST" ;

/// Обозначение localhost
const LOCALHOST: &str = "localhost" ;

/// переменная окружения с путём соединения с DB
const DB_PATH_CONNECT: &str = "DB_PATH_CONNECT" ;    

/// базовый uri путь
pub const BASE_URI_PATH: &str = "/api/" ;

/// init-db часть uri
pub const INIT_DB_PART: &str = "initdb" ;

/// http протокол
const HTTP_PROTOCOL: &str = "http" ;

/// create-user часть uri
pub const CREATE_USER_PART: &str = "create_user" ;

// Ответы сервера
#[derive(
    Serialize, 
    Deserialize,
    //Debug,
    ToSchema,
 )
]
pub enum Responce {
    // Успех
    Success(String),    // serialization -> {"Success":"Operation completed"}
    // Ошибка
    Error(String),      // serialization -> {"Error":"Database connection failed"}
}


/// Получить все переменные env
pub fn get_all_env_cars() ->Result<(u32, String, String)> {
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
                    anyhow::anyhow!(
                        "The environment variable: {} does not exist, error: {}",
                        HTTP_PORT,
                        err
                    )
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

    // Установить хост http сервера
    let http_host = match
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
            host if host.is_empty() => return Err(anyhow::anyhow!("host is empty.")),
            host if host == LOCALHOST => host,
            host => host.parse::<IpAddr>()
                                .map_err(|err|
                                    anyhow::anyhow!(
                                        "Invalid ip: {}, error: {}",
                                        host,
                                        err,
                                    )                                    
                                )?
                                .to_string()
        } ;

    let db_path = match
            std::env::var(DB_PATH_CONNECT)
                .map_err(|err|
                    anyhow::anyhow!(
                        "The environment variable: {} does not exist, error: {}",
                        DB_PATH_CONNECT,
                        err
                    )                
                )?
                .trim()
        {
            v if v.is_empty() => return Err(anyhow::anyhow!("db_path_conn is empty.")),
            v => v.to_owned(),
        } ;

    Ok((http_port, http_host, db_path))
}

/// получить базовый uri путь
pub fn get_base_uri_path() ->&'static str {
    BASE_URI_PATH
}

/// получить init-db uri
pub fn get_initdb_uri() ->String {
    format!("{}{}", get_base_uri_path(), INIT_DB_PART)
}

/// получить init-db URL
pub fn get_initdb_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_initdb_uri())
}

/// получить create-user uri
pub fn get_create_user_uri() ->String {
    format!("{}{}", HTTP_PROTOCOL, CREATE_USER_PART)
}

/// получить url создания пользователя
pub fn get_create_user_url(host: &str, port: u32) ->String {
    format!("{}:://{}:{}{}", HTTP_PROTOCOL, host, port, get_create_user_uri())
}