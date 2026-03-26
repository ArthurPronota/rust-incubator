use anyhow::Result ;

use ureq::{
        self,
        http::StatusCode
    } ;

use crate::{
        args::{
            self, 
            Commands
        }, 
        common,
        graphql_client,
    } ;

/// Выполнить действие из аргументов командной строки    
pub fn any_command(
            arg:        &args::Args,
            http_host:  &str,
            http_port:  u32,
        ) ->Result<()> {

    // создание GraphQLClient 
    let gr_client = 
            graphql_client::GraphQLClient::new(http_host, http_port)? ;
                
    // Выполнение полученной команды
    match &arg.commands {
        Commands::UserRegister { name, password } => {
            gr_client.login(name, password)? ;
        },

    }

    /*
    let resp = 
                ureq::post(&common::get_graphql_url(http_host, http_port))
                    .header("Content-Type", "application/json")
                    .send_json(&arg.commands)? ;
    
    // проверка кода возврата ответа сервера
    if resp.status() != StatusCode::OK {
        return Err(anyhow::anyhow!("Server error: {}", resp.status()));
    }
     */

    Ok(())
}