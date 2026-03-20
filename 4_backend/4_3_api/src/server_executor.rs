use std::sync::Arc;

use const_format::concatcp;

use crate::{common, roles, users::{self, User}, users_roles} ;

use axum::{
        extract::{
            Json,
            State,
        }, http::{StatusCode, status}, response::IntoResponse
} ;

use anyhow::Result ;
use clap::Command;

use crate::db::Database ;

use crate::args ;

use utoipa::{OpenApi, ToSchema} ;


const DB_OBJ_CREATED_SUCCESS: &str = "Database objects created successfully." ;

/// Файл с данными документации
const OPENAPI_FILE: &str = "openapi.json" ;

/// url документации openapi
pub const OPENAPI_URL_DOCS: &str = "/docs" ;

/// url спецификации openapi
pub const OPENAPI_URL_SPECIFIC: &str = "/api-docs/openapi.json" ;

/// пользователь успешно создан
const USER_CREATED_SUCCESSFULY: &str = "User created successfully." ;

/// Запись в файл спецификации openapi если спецификация изменилась
pub fn write_to_openapi(op_api: &utoipa::openapi::OpenApi) ->Result<()> {

    // текущий контент сожержимого openapi
    let content_openapi_now = serde_json::to_string_pretty(&op_api)? ;

    // путь к файлу openapi
    let openapi_path = std::path::Path::new(OPENAPI_FILE) ;

    // чтение содержимого файла openapi
    let content_openapi = if openapi_path.exists() {
        std::fs::read_to_string(openapi_path)?
    } else {
        "".to_string()
    } ;

    if content_openapi_now != content_openapi {
        std::fs::write(
                openapi_path,
                content_openapi_now
            )? ;
    }

    Ok(())
}

// Отправка сообщения об успехе
fn success_message(mess: &str) -> common::Responce {
    common::Responce::Success(mess.to_owned())
}

// Отправка сообщения об ошибке
fn error_message(err: &str) ->common::Responce {
    common::Responce::Error(err.to_owned())
}

// OpenAPI документация
#[derive(OpenApi)]
#[openapi(
    paths(
        initdb_handle
    ),
    components(
        schemas(
            common::Responce,
            args::CreateUser,
        )
    ),
    tags(
        (
            name = "initdb",
            description = "Creating the necessary objects in the database",
        ),
    ),
    info(
        title = "API for working with users and their roles.",
        version = "1.0.0",
        description = "RESTful API for managing users and roles",
    ),
  )
 ]
pub struct ApiDoc;

// ****************** Раздел INIT-DB ******************

/// инициализация базы данных реальная внутрянная функция
async fn initdb_handle_int(
                db_res: &Database,
            ) ->Result<common::Responce>{
    db_res
        .create_tables()
        .await? ;

    Ok(success_message(
            DB_OBJ_CREATED_SUCCESS
        ))
}

// инициализация базы данных
#[utoipa::path(
    get,
    path = concatcp!(
                common::BASE_URI_PATH,        // "/api/",
                common::INIT_DB_PART      // "initdb"
            ),  // "/api/initdb"
    responses (
        (
            status = StatusCode::OK,  // 200, 
            description = "Creating database objects.", 
            body = common::Responce,
            example = json!({"Success": DB_OBJ_CREATED_SUCCESS})
        ),
        (
            status = StatusCode::SEE_OTHER,   // 303, 
            description = "Error creating database objects.", 
            body = common::Responce,
            example = json!({"Error": "Error creating trigger."})
        ),
    ),
    tag = "initdb",
  )
]
pub async fn initdb_handle(
                State(db_res): State<Arc<Database>>,
            ) 
                //->Json<common::Responce> 
                -> impl IntoResponse
            {

    match initdb_handle_int(&db_res)
                .await 
    {
        Ok(v) => (
                            StatusCode::OK,
                            Json(v)
                        ),
        Err(err) => (
                            StatusCode::SEE_OTHER,
                            Json(
                                error_message(
                                    &err.to_string()
                                )
                            ),
                        ),
    }
}

// ****************** Раздел User ******************

// создать пользователя,внутренний формат
async fn create_user_int(db_res: &Database,
                        cmd: &args::CreateUser,
                        ) ->Result<common::Responce> {
    let mut trans = db_res
                      .pool
                      .begin()
                      .await? ;

    let new_user = 
            users::User::ins_user(
                    &mut *trans,
                    &cmd.name, //        name, 
                    &cmd.email, //    email
                )
                .await? ;

    users_roles::UsersRoles::ins_role_to_user(
            &mut *trans, 
            new_user.id_user(),
            roles::SLUG_DEFAULT
        )
        .await? ;
        
    trans.commit().await? ;

    Ok(success_message(USER_CREATED_SUCCESSFULY))
}


// создать пользователя
#[utoipa::path(
    post,
    path = concatcp!(
                common::BASE_URI_PATH,        // "/api/",
                common::CREATE_USER_PART      // "create_user"
            ), // "/api/create_user",
    request_body = args::CreateUser,
    responses (
        (
            status = StatusCode::OK,  // 200, 
            description = "Creating database objects.", 
            body = common::Responce,
            example = json!({"Success": DB_OBJ_CREATED_SUCCESS})
        ),
        (
            status = StatusCode::SEE_OTHER,   // 303, 
            description = "Error creating user.", 
            body = common::Responce,
            example = json!({"Error": "duplicate user email."})
        ),
    ),
    tag = "users",
  )
]
pub async fn create_user(
                    State(db_res): State<Arc<Database>>,
                    //Json(cmd): Json<args::CreateUser>
                ) ->impl IntoResponse {

    /*
    match cmd {
        // команда создания пользователя
        args::Command::CreateUser { name, email } => 
            match create_user_int(&db_res, &name, &email).await {
                Ok(v) => (
                            StatusCode::OK,
                            Json(v)
                           ),
                Err(err) => (
                            StatusCode::SEE_OTHER,
                            Json(
                                error_message(&err.to_string())
                              )
                            ),
            }
        ,
        // Иная команда
        other_comm => (
                            StatusCode::SEE_OTHER,
                            Json(
                                error_message(
                                    &format!(
                                            "This is a different command: {:?}",
                                            other_comm
                                        )
                                )
                              )
                            ),
    }
     */

    /*
            match create_user_int(&db_res, &cmd).await {
                Ok(v) => (
                            StatusCode::OK,
                            Json(v)
                           ),
                Err(err) => (
                            StatusCode::SEE_OTHER,
                            Json(
                                error_message(&err.to_string())
                              )
                            ),
            }
    */
    (
                            StatusCode::SEE_OTHER,
                            Json(
                                error_message("abc")
                              )
                            )
}