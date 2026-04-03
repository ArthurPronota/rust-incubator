use std::path::Path ;   // Импорт типа Path из стандартной библиотеки для работы с путями к файлам и директориям

use dotenv ;  // Импорт крейта dotenv для загрузки переменных окружения из файла .env

use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок

use std::net::IpAddr ;  // Импорт типа IpAddr из стандартной библиотеки для работы с IP-адресами

/// Переменная окружения порт http сервера
const HTTP_PORT: &str = "HTTP_PORT" ;

/// Переменная окружения хост http сервера
const HTTP_HOST: &str = "HTTP_HOST" ;

/// Обозначение localhost
const LOCALHOST: &str = "localhost" ;

/// переменная окружения с путём соединения с DB
const DB_PATH_CONNECT: &str = "DB_PATH_CONNECT" ;    

/// Срок истечения JSON Web Token в часах
const JWT_EXPIRATION: &str = "JWT_EXPIRATION" ;

/// Секретная часть Json Web Token
const JWT_SECRET: &str = "JWT_SECRET" ;

/// Максимальная глубина GRAPHQL DEEP LIMIT
const GRAPHQL_DEEP_LIMIT: &str = "GRAPHQL_DEEPLIM" ;

/// Uri передачи сообщений для graphql
pub const GRAPHQL_URI: &str = "/graphql" ;

/// Заголовок Content-Type
#[allow(dead_code)]
pub const CONTENT_TYPE_HEADER: &str = "Content-Type" ;

/// Json тип данных
#[allow(dead_code)]
pub const JSON_TYPE_VAL: &str = "application/json" ;

/// Неверный пароль
#[allow(dead_code)]
pub const INVALID_USERNAME_PASSWORD: &str = "Invalid username or password" ;


/// Получить все необходимые переменные окружения
pub fn get_all_env_vars() ->Result<(
                              u32,      // http_port,
                              String,   // http_host,
                              String,   // db_path,
                              u32,      // jwt_expir,
                              String,   // jwt_secret,
                              usize,    // graphql_deep_limit
                            )>{
    // Считывание содержимого из .env файла с установкой переменных 
    // окружения если таковые не определены
    if Path::new(".env")    // путь к .env файлу 
            .exists() { // файл .env существует 
        dotenv::dotenv()    // загрузка переменных окружения из .env файла                
                .map_err(|err| anyhow::anyhow!("{}", err))
                ?;
    }

    // подучение http port
    let http_port = 
            match std::env::var(HTTP_PORT)
                    .map_err(|err| 
                        anyhow::anyhow!("The environment variable: {} does not exist, error: {}", HTTP_PORT, err)
                    )?
                    .trim()
                    .parse::<u32>()
                    .map_err(|err| anyhow::anyhow!("Error converting HTTP_PORT to u32: {}", err))?
    {
        port if port == 0 => return Err(anyhow::anyhow!("Invalid value: {} of http_port", port)),
        port => port,
    } ;

    // получение http host
    let http_host = 
            match std::env::var(HTTP_HOST)
                .map_err(|err| anyhow::anyhow!("The environment variable: {} does not exist, error: {}", HTTP_PORT, err))?
                .trim()
                .to_string()
    {
        host if host.is_empty() => return Err(anyhow::anyhow!("host is empty")),
        host if host == LOCALHOST => host,
        host => host.parse::<IpAddr>()
                            .map_err(|err| anyhow::anyhow!("Error conver host: {} to IpAddr, error: {}", host, err))?
                            .to_string()
    } ;

    // получение db_path
    let db_path = 
            match std::env::var(DB_PATH_CONNECT)
                    .map_err(|err| anyhow::anyhow!("The environment variable: {} does not exist, error: {}", DB_PATH_CONNECT, err))?
                    .trim()
    {
        path if path.is_empty() => return Err(anyhow::anyhow!("db_path is empty")),
        path=> path.to_string(),
    } ;

    // получение Json Web Token expiration
    let jwt_expir = match
                        std::env::var(JWT_EXPIRATION)
                            .map_err(|err| anyhow::anyhow!("The environment variable: {} does not exist, error: {}", JWT_EXPIRATION, err))?
                            .trim()
                            .parse::<u32>()
                            .map_err(|err| anyhow::anyhow!("Error converting JWT_EXPIRATION to u32: {}", err))?
    {
        exp if exp == 0 => return Err(anyhow::anyhow!("Invalid value: {} of jwt_expir", exp)),
        exp => exp,
    } ;

    // Получить Json Web Token Secret
    let jwt_secret = 
            match std::env::var(JWT_SECRET)
                    .map_err(|err|anyhow::anyhow!("The environment variable: {} does not exist, error: {}", JWT_SECRET, err))?
                    .trim()
    {
        secret  if secret.is_empty() => return Err(anyhow::anyhow!("jwt_secret is empty")),
        secret=> secret.to_string(),
    } ;


    let graphql_deep_limit = std::env::var(GRAPHQL_DEEP_LIMIT)
                            .map_err(|err| anyhow::anyhow!("The environment variable: {} does not exist, error: {}", GRAPHQL_DEEP_LIMIT, err))?
                            .trim()
                            .parse::<usize>()
                            .map_err(|err| anyhow::anyhow!("Error converting JWT_EXPIRATION to u32: {}", err))?
                            ;

    Ok((
        http_port,
        http_host,
        db_path,
        jwt_expir,
        jwt_secret,
        graphql_deep_limit,
       )
    )
}

/// Получить graphql url
#[allow(dead_code)]
pub fn get_graphql_url(host: &str, port: u32) ->String {
    format!("http://{}:{}{}", host, port, GRAPHQL_URI)
}

/// Печать JSON Web Token
#[allow(dead_code)]
pub fn print_jw_token(jw_token:  &str) {
    println!("JWT: {}", jw_token) ;
}