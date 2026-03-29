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

use crate::common;
use crate::db ;

use crate::jwt ;
use crate::passw;

use crate::users::{
        self,
        Users
    } ;

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
        inp: LoginInputObject,
      ) ->Result<LoginResult>
    {
        // получить пул соединений с DB
        /*
        let db_res = match ctx.data::<Arc<db::Database>>() {
            Ok(db) => db,
            Err(err) => return Err(anyhow::anyhow!("{:?}", err)),
        } ;
          */
        let db_res = 
                ctx.data::<Arc<db::Database>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // получить auth_serv для работы с JSON Web Token
        /*
        let auth_serv = match ctx.data::<Arc<jwt::AuthService>>() {
            Ok(auth) => auth,
            Err(err) => return Err(anyhow::anyhow!("{:?}", err)),
        } ;
          */
        let auth_serv = 
                ctx.data::<Arc<jwt::AuthService>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        let mut tmp_user = Users::default() ;

        tmp_user.set_name(&inp.name)? ;

        tmp_user.set_password(&inp.password)? ;

        // Сформировать новую транзакцию
        let mut trans = db_res
                                            .pool
                                            .begin()
                                            .await
                                            ?;
        // контроль наличия вставляемого пользователя
        let found_user = match Users::find_for_name(
                &mut *trans,
                tmp_user.name(),
                false
            )
            .await? {
           Some(u) => u,
           None => return Err(anyhow::anyhow!(common::INVALID_USERNAME_PASSWORD)),
        } ;

        if ! passw::check_password(tmp_user.password(), found_user.password())? {
            return Err(anyhow::anyhow!(common::INVALID_USERNAME_PASSWORD)) ;
        }

        /*
            .is_none()
            {
                return Err(anyhow::anyhow!("Invalid username or password"));
            }
         */

        /*
        // вставить нового пользователя
        tmp_user = Users::int_user(
            &mut *trans,
            tmp_user.name(),
            &passw::hash_password(tmp_user.password())?,    // сгенерировать hash of password
        )
        .await? ;

        // выполнить commit
        trans
            .commit()
            .await? ;
         */

        /*
        // hash верного пароля для проверки
        let real_pass_hash = passw::hash_password("bbbb")? ;

        if passw::check_password(&inp.password, &real_pass_hash)? {
            println!("Valid password.")
        } else {
            //println!("Invalid password !!!!!!!!!!!") ;
            return Err(anyhow::anyhow!("Invalid password !!!!!!!!!!!")) ;
        }
         */

        /* 
            Возврат данных полного формата, клиент может запросить часть
            (всё что ниже login:)
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

или вариант с ошибкой:
{   
    "data":null,
    "errors":[
        {
            "message":"Invalid password !!!!!!!!!!!",
            "locations":[{"line":3,"column":17}],
            "path":["login"]
        }
    ]
}    
        */

        Ok(
            LoginResult {
                token:  auth_serv.generate_token(found_user.id_user())?,
                user:   UserShortInfo { id: found_user.id_user(), name: found_user.name().to_owned() }
            }
        )
    }


    // регистрация нового пользователя
    async fn register(
        &self, 
        ctx: &Context<'_>, 
        inp: LoginInputObject,
      ) ->Result<LoginResult>
    {
        // получить пул соединений с DB
        let db_res = 
                ctx.data::<Arc<db::Database>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // получить auth_serv для работы с JSON Web Token
        let auth_serv = 
                ctx.data::<Arc<jwt::AuthService>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        let mut tmp_user = Users::default() ;

        tmp_user.set_name(&inp.name)? ;

        tmp_user.set_password(&inp.password)? ;

        // Сформировать новую транзакцию
        let mut trans = db_res
                                            .pool
                                            .begin()
                                            .await
                                            ?;
        // контроль наличия вставляемого пользователя
        if Users::find_for_name(
                &mut *trans,
                tmp_user.name(),
                true
            )
            .await?
            .is_some() 
            {
                return Err(anyhow::anyhow!("A user named: {} already exists.", tmp_user.name()));
            }

        // вставить нового пользователя
        tmp_user = Users::int_user(
            &mut *trans,
            tmp_user.name(),
            &passw::hash_password(tmp_user.password())?,    // сгенерировать hash of password
        )
        .await? ;

        // выполнить commit
        trans
            .commit()
            .await? ;

        /* 
            Возврат данных полного формата, клиент может запросить часть
            (всё что ниже login:)
Формат: 
{
    "data": {   <- добавлен автоматически
        "register": {  <- добавлен автоматически, название метода
            "token":"aaasdasdsfsdgdrghdfgdgh",
            "user": {
                "id": 10,
                "name": "123"
            }
        }
    }
}

или вариант с ошибкой:
{   
    "data":null,
    "errors":[
        {
            "message":"Invalid password !!!!!!!!!!!",
            "locations":[{"line":3,"column":17}],
            "path":["login"]
        }
    ]
}    
        */

        Ok(
            LoginResult {
                token:  auth_serv.generate_token(tmp_user.id_user())?,
                user:   UserShortInfo { id: tmp_user.id_user(), name: tmp_user.name().to_owned() }
            }
        )
    }    
}

// Создание типа данных GraphQL схема
type MySchema = Schema<
                    Query, 
                    Mutation,    // Mutation,
                    EmptySubscription
                >;

// Обработчик запросов от клиента
pub async fn graph_handler(
                schema:     Extension<MySchema>,
                req:        GraphQLRequest
             ) ->GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}