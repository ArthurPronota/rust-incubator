use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок

use tokio::net::TcpListener ;
use utoipa::OpenApi;  // Импорт TcpListener из tokio для асинхронного прослушивания TCP соединений

use std::sync::Arc ;

use axum::{
        Router,
        routing/*::{
            get,
            post,
            put,
            delete,
        }
        */
        ,
} ;

use utoipa_swagger_ui::SwaggerUi ;

// Подклбчение модуля common из родительского дирректория
#[path = "../common.rs"]
mod common ;

// Подклбчение модуля server_executor из родительского дирректория
#[path = "../server_executor.rs"]
mod server_executor ;

#[path = "../roles.rs"]
mod roles ;

#[path = "../users.rs"]
mod users ;

#[path ="../users_roles.rs"]
mod users_roles ;

// Подклбчение модуля db из родительского дирректория
#[path = "../db.rs"]
mod db ;

#[tokio::main]
async fn main() ->Result<()> {
    
    // получить все необходимые для работы параметры
    let (http_port, http_host, db_path) = common::get_all_env_cars()? ;

    let openapi = server_executor::ApiDoc::openapi() ;
    std::fs::write(
            "openapi.json",
            serde_json::to_string_pretty(&openapi)?
        )? ;

    let db_res = 
            // Оборачивание пула соединений с DB в Arc
            Arc::new(
                db::Database::new(&db_path)
                .await? 
            ) ;

    // создание роутера
    let rout: Router<()> = 
                Router::new()
                    // инициализация объектов DB
                    .route(
                        &common::get_initdb_uri(), 
                        routing::get(server_executor::initdb_handle)
                    )
                    .merge(
                        SwaggerUi::new("/docs")
                            .url(
                                "/api-docs/openapi.json", 
                                openapi
                            )
                    )
                    .with_state(db_res) ;
    
    let listener = 
                TcpListener::bind(
                    format!("{}:{}", http_host, http_port)
                )
                .await? ;

    axum::serve(
            listener, 
            rout
        )
        .await? ;

    Ok(())
}
