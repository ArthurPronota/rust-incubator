/*
    1. Запуск сервера
$ cargo run --bin server

    2. Просмотрт документации
Загрузить в браузере http://127.0.0.1:8080/api_docs.html

    3. Пример соединение с mysqlsh (опционально)
\connect arthur@localhost:3306

    4. Пример создания базы данных 4_db
CREATE DATABASE `4_db` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci

*/
use anyhow::Result ;
use sqlx::migrate;

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
            SimpleObject,
};

use async_graphql_axum::{
            GraphQLRequest,
            GraphQLResponse,
};

//use jsonwebtoken::crypto ;
//use jsonwebtoken::crypto::CryptoProvider;

use async_graphql::dataloader::DataLoader;

use tower_http::services::ServeFile;

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

#[path = "../passw.rs"]
mod passw ;

#[path = "../friends.rs"]
mod friends ;

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
         jwt_secret,
         graphql_deep_limit
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

    let friend_loader = 
            DataLoader::new(
                graphql_server::FriendDataLoader{
                            pool: Arc::new(
                                db::Database::new(&db_path).await?
                            )
                }, 
                tokio::spawn
            );

    // Создать таблицы в DB      
    db_res.create_tables()
        .await? ;

    /* Выполнение миграций
    При выполнении миграции создаётся таблица _sqlx_migrations.

 MySQL  localhost:3306 ssl  4_db  SQL > desc _sqlx_migrations ;
+----------------+------------+------+-----+-------------------+-------------------+
| Field          | Type       | Null | Key | Default           | Extra             |
+----------------+------------+------+-----+-------------------+-------------------+
| version        | bigint     | NO   | PRI | NULL              |                   |
| description    | text       | NO   |     | NULL              |                   |
| installed_on   | timestamp  | NO   |     | CURRENT_TIMESTAMP | DEFAULT_GENERATED |
| success        | tinyint(1) | NO   |     | NULL              |                   |
| checksum       | blob       | NO   |     | NULL              |                   |
| execution_time | bigint     | NO   |     | NULL              |                   |
+----------------+------------+------+-----+-------------------+-------------------+
6 rows in set (0.0027 sec)
 MySQL  localhost:3306 ssl  4_db  SQL >    

 MySQL  localhost:3306 ssl  4_db  SQL > select * from _sqlx_migrations \G
*************************** 1. row ***************************
       version: 20260401000000
   description: initial
  installed_on: 2026-04-01 10:04:35
       success: 1
      checksum: 0x81997E09F9A43D55633C5C258C868DD64F173FC204F03CC973AE5B8456DD8816A76B06AEAFC4EF29ACF8B6ED61657959
execution_time: 18625100
*************************** 2. row ***************************
       version: 20260401000001
   description: two
  installed_on: 2026-04-01 10:04:35
       success: 1
      checksum: 0xEC4064DDFCC9878E3AA294AEA757BE0D10A5938B5E7C6134A35E7AB5B643CCC73C8C04CE2ABF705CF73AAA26E8426E5D
execution_time: 19653300
2 rows in set (0.0007 sec)
 MySQL  localhost:3306 ssl  4_db  SQL >

    Формат имени файла: migrations\20260401000001_two.sql
        20260401000001 - YYYYmmddHHmiss (version)
        two - description

    Если в файле миграции одно sql предложени то не нужно ставить ';'
    Если в файле миграции несколько sql предложения то ';' ставится 
        после всех sql предложений.

    Код ниже:
    1. Находит все миграции в папке migrations/
    2. Проверяет, какие миграции уже выполнены
    3. Выполняет новые миграции в правильном порядке
    4. Создает и обновляет таблицу _sqlx_migrations для отслеживания состояния

     */

    sqlx::migrate!().run(&db_res.pool).await?;

    let schema = 
                    Schema::build(
                        graphql_server::Query,    //query, 
                        graphql_server::Mutation,    // mutation, 
                        EmptySubscription,    // subscription
                    )
                    // добавить пул соединений с базой
                    .data(db_res.clone())
                    .data(auth_serv.clone())
                    .data(friend_loader)
                    // Установить максимальную глубину запроса.
                    .limit_depth(graphql_deep_limit)
                    .finish() ;

    let route: Router<()> = Router::new()
                                .route(
                                    common::GRAPHQL_URI, 
                                    post(graphql_server::graph_handler),  // method_router
                                )
                                //.with_state(db_res)
                                .route_service(
                                    "/api_docs.html",
                                    ServeFile::new("docs/api_docs.html")
                                )
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