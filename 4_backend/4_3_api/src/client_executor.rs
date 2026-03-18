use anyhow::Result ;

use ureq::{self, http::StatusCode} ;

use crate::args::Command ;

use crate::users::User;
use crate::{common, users} ;


/// Выполнить любую команду
pub fn any_command(
        args:   &Command,
        host:   &str,
        port:   u32,
       ) ->Result<()> {

    // Выполнение команд на основе аргументов CLI
    let mut resp = match args {
        // Создание в DB необходимых объектов
        Command::InitDb => {
            // https://docs.rs/ureq/latest/ureq/
            ureq::get(&common::get_initdb_uri())
                // Отправляет запрос и блокирует вызывающего до получения ответа.
                .call()?
        },

    } ;

    if resp.status() != StatusCode::OK {
        return Err(anyhow::anyhow!("Server error: {}", resp.status()));  
    }

    match resp
            .body_mut()
            .read_json::<common::Responce>()? 
    {
        common::Responce::Success(mess) => println!("{}", mess),
        common::Responce::Error(err) => println!("{}", err),
    }

    Ok(())
}