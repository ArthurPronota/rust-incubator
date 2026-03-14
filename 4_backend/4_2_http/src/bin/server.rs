use anyhow::Result ;

//use std::path::Path ;

#[path = "../common.rs"]
mod common ;

fn main() ->Result<()> {

    let (http_port, http_host, db_path_conn) = common::get_all_env_vars()? ;

    
    Ok(())
}