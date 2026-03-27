use std::sync::Arc;

use anyhow::Result ;

use async_graphql::{
            Context,
            InputObject,
            Object,
            Schema, 
            SimpleObject,
            ID,
            EmptyMutation,
            EmptySubscription,
            OutputType,
            //Result as GraphQLResult,
            Error,
};

use async_graphql_axum::{
            GraphQLRequest,
            GraphQLResponse
};

use axum::{Extension} ;

use crate::db ;

//use crate::graphql_client ;
/*
// Корневой Query тип
struct Query ;

impl Query {
    
}

// Корневой Mutation тип
struct Mutation ;

#[Object]
impl Mutation {
    
}

 */

// 1. Определяем нашу структуру данных. 
// SimpleObject позволяет async-graphql автоматически превратить её в тип GraphQL.
#[derive(SimpleObject, Clone)]
struct User {
    name: String,
    email: String,
}

// Краткая информация о пользователе
#[derive(
    SimpleObject
  )
]
pub struct UserShortInfo {
    pub id:     u32,
    pub name:   String,
}


// Возвразаемая информация о процессе Login
#[derive(
    SimpleObject
  )
]
pub struct LoginResult {
    token:  String,
    user:   UserShortInfo,
}

// 2. Определяем корневой запрос (Query).
pub struct Query;

#[Object]
impl Query {
    // Наш резолвер. Он принимает name и email и возвращает структуру User.
    async fn get_user(&self, name: String, email: String) -> Result<User> {
        Ok(
            User {name, email}
        )
    }
}

// Входные данные для логирования
#[derive(InputObject)]
struct LoginInputObject {
    name:       String,
    password:   String,
}

// Корневой Mutation тип
pub struct Mutation ;

#[Object]
impl Mutation {
    // Выполнение login
    async fn login(
        &self, 
        ctx: &Context<'_>, 
        input: LoginInputObject,
      ) ->Result<LoginResult>
    {
        // получить пул соединений с DB
        let db_res = match ctx.data::<Arc<db::Database>>() {
            Ok(db) => db,
            Err(err) => return Err(anyhow::anyhow!("{:?}", err)),
        } ;

        /* 
        Возврат данных
        Формат: 
            {
                "data": {   <- добавлен автоматически
                    "login": {  <- добавлен автоматически, название метода
                        "token":"aaasdasdsfsdgdrghdfgdgh",
                        "user": {
                            "id": 10,
                            "name": "123"
                        }
                    }
                }
            }

        */
        Ok(
            LoginResult {
                token:  "aaasdasdsfsdgdrghdfgdgh".to_string(),
                user:   UserShortInfo { id: 10, name: input.name }
            }
        )
    }
}

// Создание типа данных MySchema
type MySchema = Schema<
                    Query, 
                    Mutation,    // Mutation,
                    EmptySubscription
                >;

pub async fn graph_handler(
                schema:     Extension<MySchema>,
                req:        GraphQLRequest
             ) ->GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}