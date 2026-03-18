
use anyhow::Result ;

use clap::Parser ;

#[path = "../args.rs"]
mod args ;

#[path = "../common.rs"]
mod common ;

#[path = "../client_executor.rs"]
mod client_executor ;

#[path = "../users.rs"]
mod users ;

#[path = "../roles.rs"]
mod roles ;

#[path = "../users_roles.rs"]
mod users_roles ;

#[path = "../db.rs"]
mod db ;

fn main() ->Result<()>{

    // получить все необходтиые для работы параметры 
    let (port, host, _) = common::get_all_env_cars()? ;

    // Получение параметров из командной строки
    let args = args::Args::parse() ;

    client_executor::any_command(&args.commands, &host, port)? ;

    Ok(())
}
