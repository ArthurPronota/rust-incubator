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
};

use async_graphql_axum::{
            GraphQLRequest,
            GraphQLResponse
};

use axum::{Extension} ;

use crate::graphql_client ;
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

/*
pub struct UserInfo {
    pub id:     u32,
    pub name:   String,
}
 */

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
   async fn login(
        &self, 
        ctx: &Context<'_>, 
        input: LoginInputObject,
      ) ->
        //Result<User>
            //Result<graphql_client::UserInfo>
        //GraphQLRequest<graphql_client::UserInfo>
        Result<graphql_client::UserTop>
    {
        Ok(
            //User { name: input.name, email: "2".to_string() }
            /*
            graphql_client::UserInfo{
                id: 10,
                name:   "123".to_string()
            }
             */
            graphql_client::UserTop {
                token:  "aaasdasdsfsdgdrghdfgdgh".to_string(),
                user:   graphql_client::UserInfo {id: 10, name:   "123".to_string()}
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