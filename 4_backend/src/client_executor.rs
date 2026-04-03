use anyhow::Result ;    // Импорт типа Result из библиотеки anyhow для упрощенной обработки ошибок

use crate::{    // Импорт модулей из текущего крейта
        args::{ // Импорт модуля args, содержащего парсинг аргументов командной строки
            self,       // Импорт самого модуля args
            Commands,   // Импорт перечисления Commands из модуля args для обработки различных команд CLI
        }, 
        common,         // Импорт модуля common для доступа к общим утилитам и константам
        graphql_client, // Импорт модуля graphql_client для работы с GraphQL-запросами
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
            let (user_info, jwt) = gr_client.register(name, password)? ;
            common::print_jw_token(jwt);
            println!("Registered user: {}", user_info) ;
        },
        // Логирование пользоватлем
        Commands::Login { name, password } => {
            let (user_info, jwt) = gr_client.login(name, password)? ;
            common::print_jw_token(jwt);
            println!("Logged in user: {}", user_info) ;
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
            println!("{}", gr_client.show_friend(jwt)?) ;
        },
    }

    Ok(())
}