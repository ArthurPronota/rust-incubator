use anyhow::Result ;    // импортирует тип Result из крейта anyhow для простой обработки ошибок с расширенным контекстом.

// импортирует трейты для сериализации и десериализации данных из крейта serde.
use serde::{
        Deserialize, 
        Serialize
    };

use std::path::Path ;   // импортирует тип Path из стандартной библиотеки для работы с путями файловой системы

use std::net::IpAddr ;  // импортирует перечисление IpAddr из стандартной библиотеки для представления IPv4 и IPv6 адресов.

use dotenv ;    // импортирует крейт dotenv целиком для загрузки переменных окружения из файла .env.

use utoipa::ToSchema;   // импортирует трейт ToSchema из utoipa для генерации OpenAPI схем.

use urlencoding ;   // импортирует крейт urlencoding целиком для кодирования и декодирования URL-строк

use crate::roles;   // импортирует модуль roles из текущего крейта 

use crate::users::UserWithRole ;    // импортирует структуру UserWithRole из модуля users текущего крейта

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
#[allow(dead_code)]
const HTTP_PROTOCOL: &str = "http" ;

/// create-user часть uri
pub const CREATE_USER_PART: &str = "create_user" ;

/// delete-user часть uri
pub const DELETE_USER_PART: &str = "delete_user" ;

/// update_username часть uri 
pub const UPDATE_USERNAME_PART: &str = "update_username" ;

/// update_useremail часть uri 
pub const UPDATE_USEREMAIL_PART: &str = "update_useremail" ;

/// show_users часть uri 
pub const SHOW_USERS_PART: &str = "show_users" ;

/// create_role часть uri 
pub const CREATE_ROLE_PART: &str = "create_role" ;

/// delete_role часть uri 
pub const DELETE_ROLE_PART: &str = "delete_role" ;

/// update_rolename часть uri 
pub const UPDATE_ROLENAME_PART: &str = "update_rolename" ;

/// update_rolepermissions часть uri 
pub const UPDATE_ROLEPERMISSIONS_PART: &str = "update_rolepermissions" ;

// show role часть uri
pub const SHOW_ROLE_PART: &str = "show_role" ;

// add_tole_to_user часть uri
pub const ADD_ROLE_TO_USER_PART: &str = "add_role_to_user" ;

// add_tole_to_user часть uri
pub const REMOVE_ROLE_FROM_USER_PART: &str = "remove_role_from_user" ;

// Ответы сервера
#[derive(
    Serialize, 
    Deserialize,
    ToSchema,
 )
]
pub enum Responce {
    // Успех
    Success(String),    // serialization -> {"Success":"Operation completed"}
    // Ошибка
    Error(String),      // serialization -> {"Error":"Database connection failed"}
    // Пользователь и tuj роли:
    UserWithRole(UserWithRole),
    // Пользователи и их роли:
    UsersRoles(Vec<UserWithRole>),
    // Роль
    Role(roles::Role),
    // Все роли
    ListRoles(Vec<roles::Role>),
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
#[allow(dead_code)]
pub fn get_initdb_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_initdb_uri())
}

/// получить create-user uri
pub fn get_create_user_uri() ->String {
    format!("{}{}", get_base_uri_path(), CREATE_USER_PART)
}

/// получить url создания пользователя
#[allow(dead_code)]
pub fn get_create_user_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_create_user_uri())
}

/// получить укороченный delete-user uri
pub fn get_delete_user_uri_short() ->String {
    format!("{}{}/", get_base_uri_path(), DELETE_USER_PART)
}

/// получить delete-user uri
#[allow(dead_code)]
pub fn get_delete_user_uri(id_user: u32) ->String {
    format!("{}{}", get_delete_user_uri_short(), id_user)
}

/// получить url delete-user
#[allow(dead_code)]
pub fn get_delete_user_url(host: &str, port: u32, id_user: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_delete_user_uri(id_user))
}

/// получить update-username uri
pub fn get_update_username_uri() ->String {
    format!("{}{}", get_base_uri_path(), UPDATE_USERNAME_PART)
}

/// получить url update-username
#[allow(dead_code)]
pub fn get_update_username_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_update_username_uri())
}

/// получить update-useremail uri
pub fn get_update_useremail_uri() ->String {
    format!("{}{}", get_base_uri_path(), UPDATE_USEREMAIL_PART)
}

/// получить url update-useremail
#[allow(dead_code)]
pub fn get_update_useremail_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_update_useremail_uri())
}

/// получить show-users uri
pub fn get_show_users_uri() ->String {
    format!("{}{}", get_base_uri_path(), SHOW_USERS_PART)
}

/// получить url show-users
#[allow(dead_code)]
pub fn get_show_users_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_show_users_uri())
}

/// получить url show-user
#[allow(dead_code)]
pub fn get_show_user_url(host: &str, port: u32, id_user: u32) ->String {
    format!("{}/{}", get_show_users_url(host, port), id_user)
}

/// получить create_role uri
pub fn get_create_role_uri() ->String {
    format!("{}{}", get_base_uri_path(), CREATE_ROLE_PART)
}

/// получить url create_role
#[allow(dead_code)]
pub fn get_create_role_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_create_role_uri())
}

/// получить укороченный delete_role uri 
pub fn get_delete_role_uri_short() ->String {
    format!("{}{}/", get_base_uri_path(), DELETE_ROLE_PART)
}

/// получить delete_role uri
#[allow(dead_code)]
pub fn get_delete_role_uri(slug: &str) ->String {
    format!("{}{}", get_delete_role_uri_short(), urlencoding::encode(slug))
}

/// получить url delete_role
#[allow(dead_code)]
pub fn get_delete_role_url(host: &str, port: u32, slug: &str) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_delete_role_uri(slug))
}

/// получить update_rolename uri
pub fn get_update_rolename_uri() ->String {
    format!("{}{}", get_base_uri_path(), UPDATE_ROLENAME_PART)
}

/// получить url delete_role
#[allow(dead_code)]
pub fn get_update_rolename_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_update_rolename_uri())
}

/// получить update_rolepermissions uri
pub fn get_update_rolepermissions_uri() ->String {
    format!("{}{}", get_base_uri_path(), UPDATE_ROLEPERMISSIONS_PART)
}

/// получить url update_rolepermissions
#[allow(dead_code)]
pub fn get_update_rolepermissions_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_update_rolepermissions_uri())
}

/// получить укороченный show_role uri
pub fn get_show_role_short_uri() ->String {
    format!("{}{}", get_base_uri_path(), SHOW_ROLE_PART)
}

/// получить show_role uri
#[allow(dead_code)]
pub fn get_show_role_uri(slug: &str) ->String {
    format!("{}/{}", get_show_role_short_uri(), urlencoding::encode(slug))
}

/// получить url show_role
#[allow(dead_code)]
pub fn get_show_role_url(host: &str, port: u32, slug: &str) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_show_role_uri(slug))
}

/// получить show_roles uri
#[allow(dead_code)]
pub fn get_show_roles_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_show_role_short_uri())
}

/// получить add_role_to_user uri
pub fn get_add_role_to_user_uri() ->String {
    format!("{}{}", get_base_uri_path(), ADD_ROLE_TO_USER_PART)
}

/// получить show_roles uri
#[allow(dead_code)]
pub fn get_add_role_to_user_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_add_role_to_user_uri())
}

/// получить remove_role_from_user uri
pub fn get_remove_role_from_user_uri() ->String {
    format!("{}{}", get_base_uri_path(), REMOVE_ROLE_FROM_USER_PART)
}

/// получить remove_role_from_user uri
#[allow(dead_code)]
/*
pub fn get_remove_role_from_user_url(host: &str, port: u32) ->String {
    format!("{}://{}:{}{}", HTTP_PROTOCOL, host, port, get_remove_role_from_user_uri())
}
 */
pub fn get_remove_role_from_user_url(host: &str, port: u32, id_user: u32, slug: &str) ->String {
    format!("{}://{}:{}{}/{}/{}", HTTP_PROTOCOL, host, port, get_remove_role_from_user_uri(), id_user, &urlencoding::encode(slug))
}