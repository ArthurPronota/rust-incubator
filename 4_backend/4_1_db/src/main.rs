/*

    Общая помощь:
cargo run -- -h

    Помощь для конкретной команды:
cargo run -- create-user -h


*/
mod args ;
mod parser ;
mod db ;

use anyhow::Result ;
use clap::Parser;
use std::{
        path::Path
    } ;

const DB_PATH_CONNECT: &str = "DB_PATH_CONNECT" ;

fn main() ->Result<()> {

    let env_file = Path::new(".env") ;
    if env_file.exists() {
        // Загрузка в переменные окружения сожержимого файла .env
        // Если переменная уже существует в окружении то она не затирается 
        // содержимым из .env
        dotenv::dotenv()
            // преобразование ошибки в anyhow формат
            .map_err(|err|
                anyhow::anyhow!("{} from file: {:?}", err, env_file)
            )? ;
    }

    // Получить содержимое переменной окружения
    let db_path_conn  = 
                std::env::var(DB_PATH_CONNECT)
                    .map_err(|err| {
                        anyhow::anyhow!("{}, for var: {}", err, DB_PATH_CONNECT)
                    })? ;

    println!("db_path_conn: {}", db_path_conn) ;

    let cl_args = args::Args::parse() ;

    println!("v: {:?}", cl_args) ;

    Ok(())
}

