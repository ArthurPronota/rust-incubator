use anyhow::Result ;

use std::sync::Arc ;

use tokio::net::TcpListener ;

use axum::{
        Extension, 
        Router, 
        routing::{
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

#[path = "../common.rs"]
mod common ;

#[path = "../db.rs"]
mod db ;

#[path = "../graphql_server.rs"]
mod graphql_server ;

#[path = "../graphql_client.rs"]
mod graphql_client ;

#[tokio::main]
async fn main() ->Result<()> {

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

    let schema = 
                    Schema::build(
                        graphql_server::Query,    //query, 
                        graphql_server::Mutation,    // mutation, 
                        EmptySubscription,    // subscription
                    )
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