//use std::path;

use anyhow::Result ;
use clap::Parser;

#[path = "../common.rs"]
mod common ;

#[path = "../args.rs"]
mod args ;

#[path = "../client_executor.rs"]
mod client_executor ;

#[path = "../graphql_client.rs"]
mod graphql_client ;

fn main() ->Result<()>{
    
    // получить все необходтиые для работы параметры, остальные игнорируем
    let (http_port, http_host, ..) = common::get_all_env_vars()? ;

    let args = args::Args::parse() ;

    //println!("args: {:?}", args) ;

    // Выполнить действие из аргументов командной строки
    client_executor::any_command(&args, &http_host, http_port)? ;

    Ok(())
}
