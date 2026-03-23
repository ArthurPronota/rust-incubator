use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок

use tokio::net::TcpListener ;   // TCP-сервер, принимающий соединения.
use utoipa::OpenApi;  // Импорт TcpListener из tokio для асинхронного прослушивания TCP соединений

use std::sync::Arc ;  // Импорт тип атомарного счетчика ссылок Arc для разделяемого владения данными между потоками.

use axum::{ // фреймворк для веб-приложений, ориентированный на эргономику и модульность.
        Router, // Тип маршрутизатора для компоновки обработчиков и служб.
        routing,    // Маршрутизация между сервисами и обработчиками.
} ;

// Импортирует структуру SwaggerUi для встраивания Swagger UI в 
// веб-приложение, чтобы автоматически генерировать интерактивную 
// документацию API на основе OpenAPI-спецификации из крейта utoipa.
use utoipa_swagger_ui::SwaggerUi ;

// Подклбчение модуля common из родительского дирректория
#[path = "../common.rs"]
mod common ;

// Подклбчение модуля server_executor из родительского дирректория
#[path = "../server_executor.rs"]
mod server_executor ;

// Подклбчение модуля roles из родительского дирректория
#[path = "../roles.rs"]
mod roles ;

// Подклбчение модуля users из родительского дирректория
#[path = "../users.rs"]
mod users ;

// Подклбчение модуля users_roles из родительского дирректория
#[path = "../users_roles.rs"]
mod users_roles ;

// Подклбчение модуля args из родительского дирректория
#[path = "../args.rs"]
mod args ;

// Подклбчение модуля db из родительского дирректория
#[path = "../db.rs"]
mod db ;

#[tokio::main]  // макрос, который преобразует асинхронную функцию main в синхронную, автоматически создавая и запуская среду выполнения Tokio (runtime) для выполнения асинхронного кода.
async fn main() ->Result<()> {
    
    // получить все необходимые для работы параметры
    let (http_port, http_host, db_path) = common::get_all_env_cars()? ;

    // Возвращает экземпляр openapi::OpenApi, который можно разобрать с помощью serde 
    let openapi = server_executor::ApiDoc::openapi() ;

    // Запись в файл спецификации openapi если спецификация изменилась
    server_executor::write_to_openapi(&openapi)? ;

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
                    // Показ роли 
                    .route(
                        &format!("{}/{{{}}}", &common::get_show_role_short_uri(), server_executor::SLUG_KEY),
                        routing::get(server_executor::show_role)
                    )
                    // Показ всех ролей
                    .route(
                        &common::get_show_role_short_uri(), 
                        routing::get(server_executor::get_show_roles)
                    )
                    // Добавить роль к пользователю
                    .route(
                        &common::get_add_role_to_user_uri(),
                        routing::post(server_executor::add_role_to_user)
                    )
                    // Удалить роль у пользователя
                    .route(
                        &format!("{}/{{{}}}/{{{}}}", common::get_remove_role_from_user_uri(), server_executor::ID_USER_KEY, server_executor::SLUG_KEY),
                        routing::delete(server_executor::remove_role_from_user)
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
    
    // Создает новый объект TcpListener, который будет привязан к 
    // указанному адресу.
    let listener = 
                TcpListener::bind(
                    format!("{}:{}", http_host, http_port)
                )
                .await? ;

    // Запустите сервис, используя предоставленный обработчик событий.
    axum::serve(
            listener, 
            rout
        )
        .await? ;

    Ok(())
}
