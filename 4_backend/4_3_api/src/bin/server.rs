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

#[path = "../users_roles.rs"]
mod users_roles ;

#[path = "../args.rs"]
mod args ;

// Подклбчение модуля db из родительского дирректория
#[path = "../db.rs"]
mod db ;

#[tokio::main]
async fn main() ->Result<()> {
    
    // получить все необходимые для работы параметры
    let (http_port, http_host, db_path) = common::get_all_env_cars()? ;

    // Возвращает экземпляр openapi::OpenApi, который можно разобрать с помощью serde 
    let openapi = server_executor::ApiDoc::openapi() ;

    // Запись в файл спецификации openapi если спецификация изменилась
    server_executor::write_to_openapi(&openapi)? ;

    //println!("{}", format!("{}{{id_user}}", common::get_delete_user_uri_short())) ;

    //println!("{}", format!("{}{{{}}}", common::get_delete_role_uri_short(), server_executor::SLUG_KEY)) ;


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
                    // создаение пользователя
                    .route(
                        &common::get_create_user_uri(),
                        routing::post(server_executor::create_user)
                    )
                    // Удаление пользователя
                    .route(
                        &format!("{}{{{}}}", common::get_delete_user_uri_short(), server_executor::ID_USER_KEY),
                        routing::delete(server_executor::delete_user)
                    )
                    // Модификация имени пользователя
                    .route(
                        &common::get_update_username_uri(),
                        routing::put(server_executor::update_username)
                    )
                    // Модификация email пользователя
                    .route(
                        &common::get_update_useremail_uri(),
                        routing::put(server_executor::update_useremail)
                    )
                    // Показ пользователя и его ролей
                    .route(
                        &format!("{}/{{{}}}", common::get_show_users_uri(), server_executor::ID_USER_KEY), //path, 
                        routing::get(server_executor::show_user)
                    )
                    // Показ пользователей и их ролей
                    .route(
                        &common::get_show_users_uri(),
                        routing::get(server_executor::show_users)
                    )
                    // Создать роль
                    .route(
                        &common::get_create_role_uri(),
                        routing::post(server_executor::create_role)
                    )
                    // Удаление роли
                    .route(
                        &format!("{}{{{}}}", common::get_delete_role_uri_short(), server_executor::SLUG_KEY),
                        routing::delete(server_executor::delete_role)
                    )
                    // Модификация наименования роли
                    .route(
                        &common::get_update_rolename_uri(),
                        routing::put(server_executor::update_rolename)
                    )
                    // Модификация разрешений у роли
                    .route(
                        &common::get_update_rolepermissions_uri(), 
                        routing::put(server_executor::update_rolepermissions)
                    )
                    // Добавляем Swagger UI в наш роутер (объединяем с основными маршрутами)
                    .merge(
                        // Создаем новый экземпляр Swagger UI, который будет доступен по пути "/docs"
                        // Пользователь может открыть в браузере http://127.0.0.1:8080/docs
                        // Там будет интерактивная документация с возможностью тестировать API
                        SwaggerUi::new(server_executor::OPENAPI_URL_DOCS) // пользовательский интерфейс: http://127.0.0.1:8080/docs
                            // Добавляем URL, по которому будет доступна OpenAPI спецификация в JSON формате
                            // Swagger UI загрузит этот JSON, чтобы построить интерфейс
                            .url(
                                // Путь к JSON с описанием API: http://127.0.0.1:8080/api-docs/openapi.json
                                    server_executor::OPENAPI_URL_SPECIFIC, // "/api-docs/openapi.json",   // данные API: http://127.0.0.1:8080/api-docs/openapi.json
                                // Сама OpenAPI спецификация, сгенерированная из кода с помощью макроса #[derive(OpenApi)]
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
