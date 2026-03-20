use anyhow::Result ;

use ureq::{
        self,
        http::StatusCode
    } ;

use crate::args::Command ;

use crate::users::User;
use crate::{common, users} ;

const CONTENT_TYPE: &str = "Content-Type" ;
const JSON_TYPE: &str =  "application/json" ;

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
            ureq::get(&common::get_initdb_url(host, port))
                // Отправляет запрос и блокирует вызывающего до получения ответа.
                .call()?
        },
        // Создание пользователя
        Command::CreateUser(arg_unit)  => {
            let mut tmp_user = User::default() ;
            // проверка имени пользователя
            tmp_user.set_name(&arg_unit.name)? ;
            // проверка email пользователя
            tmp_user.set_email(&arg_unit.email)? ;

            ureq::post(common::get_create_user_url(host, port))
                .header(CONTENT_TYPE, JSON_TYPE)
                .send_json(&arg_unit)? 
        },
        // Удаление пользователя
        Command::DeleteUser(arg_unit) => {
            let mut tmp_user = User::default() ;

            tmp_user.set_id_user(arg_unit.id_user)? ;

            //println!("{}", common::get_delete_user_url(host, port, tmp_user.id_user())) ;

            ureq::delete(
                common::get_delete_user_url(host, port, tmp_user.id_user())
            )
            .call()?
        }
    } ;

    if !matches!(resp.status(), StatusCode::OK | StatusCode::CREATED) {
        return Err(anyhow::anyhow!("Server error: {}", resp.status())) ;
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