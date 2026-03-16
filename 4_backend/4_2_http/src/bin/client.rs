/*

Пример запуска:
    cargo run --bin client -- -h


*/
use anyhow::Result ;
use clap::Parser ;
/*
use std::path::{
            self, 
            Path
        } ;
use std::net::IpAddr ;
 */

// Подклбчение модуля args из родительского дирректория
#[path = "../args.rs"]
mod args ;

// Подклбчение модуля client_executor из родительского дирректория
#[path = "../client_executor.rs"]
mod client_executor ;

// Подклбчение модуля common из родительского дирректория
#[path = "../common.rs"]
mod common ;

#[path = "../users.rs"]
mod users ;

#[path = "../roles.rs"]
mod roles ;

#[path = "../db.rs"]
mod db ;

#[path = "../users_roles.rs"]
mod users_roles ;


/*
/// Переменная окружения порт http сервера
const HTTP_PORT: &str = "HTTP_PORT" ;

/// Переменная окружения хост http сервера
const HTTP_HOST: &str = "HTTP_HOST" ;

/// Обозначение localhost
const LOCALHOST: &str = "localhost" ;
 */

fn main() ->Result<()> {

    /*
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
     */
    // получить все необъодтиые для работы параметры
    let (http_port, http_host, _) = common::get_all_env_vars()? ;

    //println!("host: {}, port: {}", http_host, http_port) ;

    let args = args::Args::parse() ;

    //println!("args: {:?}", args) ;

    client_executor::any_command(&args, &http_host, http_port)? ;

    Ok(())
}
