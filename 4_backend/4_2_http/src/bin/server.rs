/*
Пример запуска:
    cargo run --bin server

*/
use anyhow::Result ;

//use std::path::Path ;

#[path = "../common.rs"]
mod common ;

fn main() ->Result<()> {

    // получить все необъодтиые для работы параметры
    let (http_port, http_host, db_path_conn) = common::get_all_env_vars()? ;

    
    Ok(())
}