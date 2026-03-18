use std::path;

use anyhow::Result ;

use clap::Parser ;

#[path = "../args.rs"]
mod args ;

#[path = "../common.rs"]
mod common ;

#[path = "../client_executor.rs"]
mod client_executor ;

fn main() ->Result<()>{

    // получить все необходтиые для работы параметры 
    let (port, host, _) = common::get_all_env_cars()? ;

    // Получение параметров из командной строки
    let args = args::Args::parse() ;


    Ok(())
}
