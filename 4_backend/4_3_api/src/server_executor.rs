use std::sync::Arc;

use crate::common ;

use axum::{
        extract::{
            Json,
            State,
        }, http::{StatusCode, status}, response::IntoResponse
} ;

use anyhow::Result ;

use crate::db::Database ;

use utoipa::{OpenApi, ToSchema} ;

const DB_OBJ_CREATED_SUCCESS: &str = "Database objects created successfully." ;

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
            common::Responce
        )
    ),
    tags(
        (
            name = "initdb",
            description = "Creating the necessary objects in the database",
        ),
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
    path = "/api/initdb",
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
                State(db_res): State<Arc<Database>>
            ) 
                //->Json<common::Responce> 
                -> impl IntoResponse
            {

    let v = StatusCode::SEE_OTHER.as_u16() ;

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