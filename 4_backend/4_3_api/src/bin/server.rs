use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок

use tokio::net::TcpListener ;  // Импорт TcpListener из tokio для асинхронного прослушивания TCP соединений

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

#[path = "../common.rs"]
mod common ;


#[path = "../db.rs"]
mod db ;

#[tokio::main]
async fn main() ->Result<()> {
    
    // получить все необходимые для работы параметры
    let (http_port, http_host, db_path) = common::get_all_env_cars()? ;

    let db_res = 
            // Оборачивание пула соединений с DB в Arc
            Arc::new(
                db::Database::new(&db_path)
                .await? 
            ) ;

    let app: Router<()> = 
                Router::new()
                    // инициализация объектов DB
                    .route(
                        &common::get_initdb_uri(), 
                        routing::get()
                    )
                    .with_state(db_res)
                ;

    Ok(())
}
