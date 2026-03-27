/*


    8. Пример соединение с mysqlsh (опционально)
\connect arthur@localhost:3306

    9. Пример создания базы данных 4_db
CREATE DATABASE `4_db` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci

*/
use anyhow::Result ;

use std::sync::Arc ;

use tokio::net::TcpListener ;

use axum::{
        Extension, Router, extract::path, routing::{
            self,
            post
        }
} ;

use async_graphql::{
            Context,
            EmptyMutation,
            EmptySubscription,
            Object,
            Schema,
            SimpleObject
};

use async_graphql_axum::{
            GraphQLRequest,
            GraphQLResponse,
};

//use jsonwebtoken::crypto ;
//use jsonwebtoken::crypto::CryptoProvider;


#[path = "../common.rs"]
mod common ;

#[path = "../db.rs"]
mod db ;

#[path = "../graphql_server.rs"]
mod graphql_server ;

#[path = "../graphql_client.rs"]
mod graphql_client ;

#[path = "../users.rs"]
mod users ;

#[path = "../jwt.rs"]
mod jwt ;

#[tokio::main]
async fn main() ->Result<()> {
    /*
    CryptoProvider::set_default_provider(CryptoProvider::ring())
        .expect("Failed to set default crypto provider");
     */
    //CryptoProvider::install_default()?;

    let (port_http, 
         host_http, 
         db_path, 
         jwt_expir, 
         jwt_secret
        ) = common::get_all_env_vars()? ;

    
    let db_res = Arc::new(
                    // Оборачивание пула соединений с DB в Arc
                    db::Database::new(&db_path).await?
                ) ;

    let auth_serv = Arc::new(
            jwt::AuthService::new(
                    &jwt_secret,
                    jwt_expir
            )? 
        ) ;

    // Создать таблицы в DB      
    db_res.create_tables()
        .await? ;

    let schema = 
                    Schema::build(
                        graphql_server::Query,    //query, 
                        graphql_server::Mutation,    // mutation, 
                        EmptySubscription,    // subscription
                    )
                    // добавить пул соединений с базой
                    .data(db_res.clone())
                    .data(auth_serv.clone())
                    .finish() ;

    let route: Router<()> = Router::new()
                                .route(
                                    common::GRAPHQL_URI, 
                                    post(graphql_server::graph_handler),  // method_router
                                )
                                //.with_state(db_res)
                                .layer(Extension(schema))
                                ;

    let listener = 
            TcpListener::bind(
                format!("{}:{}", host_http, port_http)
            )
            .await? ;

    axum::serve(listener, route)
            .await? ;

    Ok(())
}