use std::path::Display;

// GraphQL клиент
 
use ureq::{
        self,
        http::StatusCode,
} ;

use anyhow::Result ;

use serde::{
        Serialize,
        Deserialize,
} ;

use serde_json::json ;

use crate::common ;

use async_graphql::{
        //OutputType
        //SimpleObject,
    } ;

// Ошибка в GraphQL
#[derive(
    Debug,
    Deserialize
)]
struct GraphQLError {
    message: String,
}

// Информация о пользователе
#[derive(
    Serialize,
    Deserialize,
    //OutputType,
    //SimpleObject,   // Чтобы этот тип мог быть вызвразаемым в graphql_server::Mutation::login
    //Debug,
 )
]
pub struct UserShortInfo {
    pub id:     u32,
    pub name:   String,
}

// реализация Display для UserShortInfo
impl std::fmt::Display for UserShortInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "UserId: {}, UserName: {}", self.id, self.name)
    }
}


// Данные по токену и короткие данные по пользователю
#[derive(
    Deserialize,
    //SimpleObject,
)]
pub struct TokenAndShortUser {
    pub token:  String,
    pub user:   UserShortInfo,
}

// Сырой ответ логирования
#[derive(
    Deserialize
  )
 ]
pub struct LoginRawResponce {
    login:  TokenAndShortUser,
}


// Ответ сервера на попытку логирования
#[derive(
    Deserialize
  )
]
struct LoginResponce {
    data:    Option<LoginRawResponce>,  //Option<UserInfo>,
    errors:  Option<Vec<GraphQLError>>,
}

// GraphQL Client
#[derive(Deserialize, Debug)]
pub struct GraphQLClient {
    http_host:   String,
    http_port:   u32,
    url:         String,
    token:       String,
}

// реализация методов для GraphQLClient
impl GraphQLClient {

    // создать экземпляр GraphQLClient
    pub fn new(http_host: &str, http_port: u32) ->Result<GraphQLClient> {
        Ok(
            Self {
                http_host:  http_host.to_string(),
                http_port:  http_port,
                url:        common::get_graphql_url(http_host, http_port),
                token:      "".to_owned(),
            }
        )
    }

    // Установить токен
    pub fn set_token(&mut self, token: &str) ->Result<()> {
        match token {
            t if t.is_empty() => Err(anyhow::anyhow!("token is empty")),
            t => {
                self.token = t.to_owned() ;
                Ok(())
            }
        }
    }

    // получить token
    pub fn token(&self) ->&str {
        &self.token
    }

    // Выполнить login
    pub fn login(&mut self, name: &String, password: &str) ->Result<UserShortInfo> {
        // строка запроса в формате GraphQL
        let query = 
        // 1) формат: {"data":{"login":{"token":"aaasdasdsfsdgdrghdfgdgh","user":{"id":10,"name":"123"}}}}
        r#"
            mutation Login($name: String!, $password: String!) {
                login(input: { name: $name, password: $password }) {
                    token
                    user {
                        id
                        name
                    }
                }
            }
        "#
        /*
        // 2) формат: {"data":{"login":{"user":{"id":10,"name":"123"}}}}
        r#"
            mutation Login($name: String!, $password: String!) {
                login(input: { name: $name, password: $password }) {
                    user {
                        id
                        name
                    }
                }
            }
        "#
         */
        /*
        // 3) формат: {"data":{"login":{"token":"aaasdasdsfsdgdrghdfgdgh","user":{"id":10}}}}
        r#"
            mutation Login($name: String!, $password: String!) {
                login(input: { name: $name, password: $password }) {
                    token
                    user {
                        id
                    }
                }
            }
        "#
         */
        /*
        // 4) формат: {"data":{"login":{"token":"aaasdasdsfsdgdrghdfgdgh"}}}
        r#"
            mutation Login($name: String!, $password: String!) {
                login(input: { name: $name, password: $password }) {
                    token
                }
            }
        "#
         */
        ;

        let mut resp = 
                ureq::post(&self.url)
                    .header(common::CONTENT_TYPE_HEADER, common::JSON_TYPE_VAL)
                    .send_json(
                        // Создайте объект serde_json::Value из JSON-литерала.
                        &json!({
                            "query": query, // запрос в формате GraphQL
                            "variables": {  // Переменные участвующие в формировании запроса
                                "name": name,
                                "password": password,
                            }
                         }
                        )
                    )? ;

        // проверка кода возврата ответа сервера
        if resp.status() != StatusCode::OK {
            return Err(anyhow::anyhow!("Server error: {}", resp.status()));
        }

        /*
        let v = resp
                            .body_mut()
                            .read_to_string()? 
                            ;
        println!("{}", v) ; // {"data":{"login":{"id":10,"name":"123"}}}
        // {"data":{"login":{"user":{"id":10,"name":"123"}}}}
        */

        //*
        // получение ответа от сервера
        let log_resp = resp
                        .body_mut()
                        .read_json::<LoginResponce>()
                        .map_err(|err| 
                            anyhow::anyhow!("read_json to LoginResponce error: {}", err)
                        )? ;

        // проверка ошибки в ответе сервера
        if let Some(err) = log_resp.errors {
            return Err(anyhow::anyhow!("{:?}", err));
        }

        match log_resp.data {
            Some(data) => {
                self.set_token(&data.login.token) ;
                common::print_jw_token(&self.token);
                Ok(data.login.user)
            },
            None => {
                Err(anyhow::anyhow!("Not found log_resp.data"))
            }
        }
    }
}