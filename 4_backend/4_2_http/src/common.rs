use serde::{    //  крейт serde
        Deserialize,    // Импорт трейтов Deserialize
        Serialize       // Импорт трейтов Serialize
    } ;
use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок
use std::path::Path ;   // Импорт типа Path из стандартной библиотеки для работы с путями файловой системы
use std::net::IpAddr ;  // Импорт типа IpAddr из стандартной библиотеки для работы с IP-адресами (IPv4 и IPv6)

use crate::{
        roles,  // Модуль для работы с ролями 
        users,  // Модуль для работы с пользователями
    } ;

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
    Serialize,      // Сериализация для enum
    Deserialize,    // Десериализация для enum
  )
]
pub enum Response {
    // Успех
    Success(String),    // serialization -> {"Success":"Operation completed"}
    // Ошибка
    Error(String),      // serialization -> {"Error":"Database connection failed"}
    // Роли для пользователя
    UserRole(users::UserWithRole),
    // Перечень ролей для пользователей
    ListUsersRoles(Vec<users::UserWithRole>),
    // Роль
    Role(roles::Role),
    // Список ролей
    ListRoles(Vec<roles::Role>),
}

/// Получить бащовый путь для url
pub fn get_base_path_for_url() ->&'static str {
    "/anycommand"
}

/// получить базовый url
#[allow(dead_code)]
pub fn get_base_url(host: &str, port: u32) ->String {
    format!("http://{}:{}{}", host, port, get_base_path_for_url())
}


/// Получить все переменные env
pub fn get_all_env_vars() ->Result<(u32, String, String)> {

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
                host if host.is_empty() => return Err(anyhow::anyhow!("host is empty.")),
                // Это localhost
                host if host == LOCALHOST => host,
                // Это не localhost
                host =>
                    host.parse::<IpAddr>()
                        .map_err(|err|
                            anyhow::anyhow!(
                                "Invalid ip: {}, error: {}",
                                host,
                                err,
                            )
                        )?
                        .to_string()
            } ;

    // Установить path для DB
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