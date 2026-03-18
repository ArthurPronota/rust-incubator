use std::path;

use anyhow::Result ;

use clap::Parser ;

#[path = "../args.rs"]
mod args ;

#[path = "../common.rs"]
mod common ;

fn main() ->Result<()>{

    // Получение параметров из командной строки
    let args = args::Args::parse() ;

    Ok(())
}
