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


// Краткая информация о друге
#[derive(
    Serialize,
    Deserialize,
 )
]
pub struct FriendShortInfo {
    pub id:     u32,
    pub name:   String,
}

/// Реализация Display для FriendShortInfo
impl std::fmt::Display for FriendShortInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "FriendId: {}, FriendName: {}", self.id, self.name)
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

// Данные по другу
#[derive(
    Deserialize,
  )
]
pub struct ShortFriendData {
    pub friend: FriendShortInfo
}

// Сырой ответ логирования
#[derive(
    Deserialize
  )
 ]
pub struct LoginRawResponce {
    login:  TokenAndShortUser,
}

// Сырой ответ добавления друга
#[derive(
    Deserialize
 )
]
pub struct AddFriendRawResponce {
    addfriend:     ShortFriendData,
}

// Сырой ответ регистрации нового пользователя
#[derive(
    Deserialize
  )
]
pub struct RegisterRawResponce {
    register:  TokenAndShortUser,
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

// Ответ сервера на попытку регистрации нового пользователя
#[derive(
    Deserialize
  )
]
struct RegisterResponce {
    data:   Option<RegisterRawResponce>,
    errors: Option<Vec<GraphQLError>>
}

// Ответ сервера на попытку добавления друга
#[derive(
    Deserialize
  )
]
struct AddFriendResponce {
    data:   Option<AddFriendRawResponce>,
    errors: Option<Vec<GraphQLError>>
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
        // inp - название аргемента у метода graphql_server::Mutation::login(.., inp: LoginInputObject,)
        r#"
            mutation Login($name: String!, $password: String!) {
                login(inp: { name: $name, password: $password }) {
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
        // inp - название аргемента у метода graphql_server::Mutation::login(.., inp: LoginInputObject,)
        r#"
            mutation Login($name: String!, $password: String!) {
                login(inp: { name: $name, password: $password }) {
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
        // inp - название аргемента у метода graphql_server::Mutation::login(.., inp: LoginInputObject,)
        r#"
            mutation Login($name: String!, $password: String!) {
                login(inp: { name: $name, password: $password }) {
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
        // inp - название аргемента у метода graphql_server::Mutation::login(.., inp: LoginInputObject,)
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
        // {"data":null,"errors":[{"message":"Invalid password !!!!!!!!!!!","locations":[{"line":3,"column":17}],"path":["login"]}]}
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
            return Err(anyhow::anyhow!("{:?}", err[0].message));
        }

        match log_resp.data {
            Some(data) => {
                self.set_token(&data.login.token)? ;
                common::print_jw_token(&self.token);
                Ok(data.login.user)
            },
            None => {
                Err(anyhow::anyhow!("Not found log_resp.data"))
            }
        }
    }

    // Выполнить регистацию нового пользователя
    pub fn register(&mut self, name: &String, password: &str) ->Result<UserShortInfo> {
        // строка запроса в формате GraphQL
        let query = 
        // 1) формат: {"data":{"register":{"token":"aaasdasdsfsdgdrghdfgdgh","user":{"id":10,"name":"123"}}}}
        // inp - название аргемента у метода graphql_server::Mutation::register(.., inp: LoginInputObject,)
        r#"
            mutation Register($name: String!, $password: String!) {
                register(inp: { name: $name, password: $password }) {
                    token
                    user {
                        id
                        name
                    }
                }
            }
        "#
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
        // {"data":null,"errors":[{"message":"Invalid password !!!!!!!!!!!","locations":[{"line":3,"column":17}],"path":["login"]}]}
        */

        //*
        // получение ответа от сервера
        let reg_resp = resp
                        .body_mut()
                        .read_json::<RegisterResponce>()
                        .map_err(|err| 
                            anyhow::anyhow!("read_json to RegisterResponce error: {}", err)
                        )? ;

        // проверка ошибки в ответе сервера
        if let Some(err) = reg_resp.errors {
            return Err(anyhow::anyhow!("{:?}", err[0].message));
        }

        match reg_resp.data {
            Some(data) => {
                self.set_token(&data.register.token)? ;
                common::print_jw_token(&self.token);
                Ok(data.register.user)
            },
            None => {
                Err(anyhow::anyhow!("Not found reg_resp.data"))
            }
        }
    }

    // Выполнить регистацию нового пользователя
    pub fn add_friend(&mut self, friend_id: u32, jwt: &str) ->Result<FriendShortInfo> {

        println!("Begin add_friend") ;

        // строка запроса в формате GraphQL
        let query = 
        // 1) формат: {"data":{"add_friend":{"user":{"id":10,"name":"123"}}}}
        // inp - название аргемента у метода graphql_server::Mutation::add_friend(.., inp: LoginInputObject,)
        r#"
            mutation Addfriend($friendid: Int!, $jwt: String!) {
                addfriend(inp: { friendid: $friendid, jwt: $jwt }) {
                    friend {
                        id
                        name
                    }
                }
            }
        "#
        ;

        println!("1) add_friend") ;

        /*
        println!("{}",
    &json!({
                            "query": query, // запрос в формате GraphQL
                            "variables": {  // Переменные участвующие в формировании запроса
                                "friendid": friend_id,
                                "jwt":  jwt,
                            }
                         }
                        )
    
        ) ;
         */

        let mut resp = 
                ureq::post(&self.url)
                    .header(common::CONTENT_TYPE_HEADER, common::JSON_TYPE_VAL)
                    .send_json(
                        // Создайте объект serde_json::Value из JSON-литерала.
                        &json!({
                            "query": query, // запрос в формате GraphQL
                            "variables": {  // Переменные участвующие в формировании запроса
                                "friendid": friend_id,
                                "jwt":  jwt,
                            }
                         }
                        )
                    )? ;

        println!("2) add_friend") ;

        // проверка кода возврата ответа сервера
        if resp.status() != StatusCode::OK {
            return Err(anyhow::anyhow!("Server error: {}", resp.status()));
        }

        /*
        let v = resp
                            .body_mut()
                            .read_to_string()? 
                            ;
        println!("{}", v) ; 
        //{"data":{"addfriend":{"friend":{"id":3,"name":"Tom"}}}}
        //{"data":null,"errors":[{"message":"I can't add a friend_id: 3 for user: 2, he already exists.","locations":[{"line":3,"column":17}],"path":["addfriend"]}]}
        */

        //*
        // получение ответа от сервера
        let add_friend_resp = resp
                        .body_mut()
                        .read_json::<AddFriendResponce>()
                        .map_err(|err| 
                            anyhow::anyhow!("read_json to AddFriendResponce error: {}", err)
                        )? ;

        // проверка ошибки в ответе сервера
        if let Some(err) = add_friend_resp.errors {
            return Err(anyhow::anyhow!("{:?}", err[0].message));
        }

        match add_friend_resp.data {
            Some(data) => {
                /*
                self.set_token(&data.register.token)? ;
                common::print_jw_token(&self.token);
                */
                Ok(data.addfriend.friend)
            },
            None => {
                Err(anyhow::anyhow!("Not found reg_resp.data"))
            }
        }        
    }

}