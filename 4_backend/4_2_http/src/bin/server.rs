/*
Пример запуска:
    cargo run --bin server

*/
use anyhow::Result ;

//use std::path::Path ;

use axum::{
    Router, 
    routing::{
        post,
        //get
    },
    extract::State,
} ;

use std::{path, sync::Arc};

use tokio::net::TcpListener;

use crate::db::Database;

// Необходимо подключить все используемые модули в точке входа 
// для их дальнейшего использования.

// Подклбчение модуля args из родительского дирректория
#[path ="../args.rs"]
mod args ;

// Подклбчение модуля common из родительского дирректория
#[path = "../common.rs"]
mod common ;

// Подклбчение модуля server_executor из родительского дирректория
#[path = "../server_executor.rs"]
mod server_executor ;

// Подклбчение модуля db из родительского дирректория
#[path = "../db.rs"]
mod db ;

// Подклбчение модуля users из родительского дирректория
#[path = "../users.rs"]
mod users ;

#[path = "../roles.rs"]
mod roles ;

#[path = "../users_roles.rs"]
mod users_roles ;

#[tokio::main]
async fn main() ->Result<()> {

    // получить все необходимые для работы параметры
    let (http_port, http_host, db_path_conn) = common::get_all_env_vars()? ;

    // server::db::Database

    let mut db_res = 
            Arc::new(
                db::Database::new(&db_path_conn) 
                    .await? 
            ) ;

    let app: //Router<Arc<Database>> 
             Router<()>
                = 
                Router::new()
                    .route(
                        common::get_base_path_for_url(),
                        post(server_executor::handle_commmand)
                        //get(server_executor::handle_commmand)
                    )  
                    .with_state(db_res) ;
    
    let addr = format!("{}:{}", http_host, http_port) ;
    
    let listener = TcpListener::bind(&addr).await?;

    println!("{} -> {}", 
        common::get_base_path_for_url(),
        addr,
    ) ;

    axum::serve(
            listener,
            app
        )
        .await? ;


    /* Так:
    let app = 
                Router::<common::Response>::new()
                    .route(
                        &common::get_base_url(&http_host, http_port),
                        post(server_executor::handle_commmand)
                    )  ;
     */
    /* Ити так
    let app = 
                Router::<()>::new()
                    .route(
                        &common::get_base_url(&http_host, http_port),
                        post(server_executor::handle_commmand)
                    )  ;
     */

    /*
    use std::sync::Arc;

    #[derive(Clone)]
    struct ServerState {
        db_pool: Vec<u32>, 
    }

    let mut data = Arc::new(ServerState{db_pool: vec![1, 2, 3]});
    let app: Router<Arc<ServerState>> = 
                Router::new()
                    .route(
                        &common::get_base_url(&http_host, http_port),
                        post(server_executor::handle_commmand)
                    )  
                    .with_state(data) ;
     */


    Ok(())
}