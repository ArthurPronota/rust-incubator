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
    let mut gr_client = 
                graphql_client::GraphQLClient::new(http_host, http_port)? ;
                
    // Выполнение полученной команды
    match &arg.commands {
        // Регистрация нового пользователя
        Commands::UserRegister { name, password } => {
            //println!("{}", gr_client.login(name, password)?) ;
            println!("{}", gr_client.register(name, password)?) ;
        },
        // Логирование пользоватлем
        Commands::Login { name, password } => {
            println!("Logged in user: {}", 
                     gr_client.login(name, password)?
            ) ;
        },
        // Добавить друга
        Commands::AddFriend { friend_id, jwt } => {
            println!("Added friend: {}", 
                      gr_client.add_friend( *friend_id, jwt)?
            ) ;
        },
        // Удалить друга
        Commands::DelFriend { friend_id, jwt } => {
            println!(
                "Deleted friend: {}",
                gr_client.del_friend(*friend_id, jwt)?
            ) ;
        },
        // Показать друзей
        Commands::ShowFriends { jwt } => {
            println!("{:#?}", gr_client.show_friend(jwt)?) ;
        },
    }

    Ok(())
}