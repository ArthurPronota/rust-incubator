use anyhow::Result ;

use ureq ;

use ureq::http::StatusCode;

use crate::args::{
            self,
            Commands 
        } ;

use crate::common::{
            self,
            get_base_url 
        };


/// Выполнить действие из аргументов командной строки
pub fn any_command(
                args:   &args::Args,
                host:   &str,
                port:   u32,
             ) ->Result<()> {

    // сформировать бащовый url
    //let base_url = get_base_url(host, port) ;

    let mut resp = 
            // сформировать POST запрос
            ureq::post(
                    //base_url
                    get_base_url(host, port)
                )
                // добавить заголовок
                .header("Content-Type", "application/json")
                // Отправьте данные тела запроса в формате JSON.
                .send_json(&args.commands)? ;

    if resp.status() != StatusCode::OK {
        return Err(anyhow::anyhow!("Server error: {}", resp.status()));
    }

    // разбор ответа сервера
    match resp.body_mut().read_json::<common::Response>()? {
        common::Response::Success(mess) => println!("{}", mess),
        common::Response::Error(err) => return Err(anyhow::anyhow!("{}", err)),
    }

    Ok(())
}