use std::{
        //path::Path, 
        sync::Arc
    };

use const_format::concatcp;

use crate::{
        args::{
            UpdateEmailUser, 
            UpdateNameUser
        },
        common::{
            self, 
            //Responce
        }, 
        roles, 
        users::{
            self, 
            User
        },
        users_roles::{
            self, 
            UsersRoles
        }
} ;

use urlencoding ;


//use axum::extract::Path;
use axum::{
        extract::{
            self, Json, State
        }, http::{StatusCode, status}, response::IntoResponse
} ;

use anyhow::Result ;
use clap::Command;

use crate::db::Database ;

use crate::args ;

use utoipa::{OpenApi, ToSchema} ;

//use crate::users_roles::UsersRoles ;

const DB_OBJ_CREATED_SUCCESS: &str = "Database objects created successfully." ;

/// Файл с данными документации
const OPENAPI_FILE: &str = "openapi.json" ;

/// url документации openapi
pub const OPENAPI_URL_DOCS: &str = "/docs" ;

/// url спецификации openapi
pub const OPENAPI_URL_SPECIFIC: &str = "/api-docs/openapi.json" ;

/// пользователь успешно создан
const USER_CREATED_SUCCESSFULY: &str = "User created successfully." ;

/// роль успешно создана
const ROLE_CREATED_SUCCESSFULY: &str = "Role created successfully." ;

/// тэг пользователи
const TAG_USERS: &str = "users" ;

/// тэг роли
const TAG_ROLES: &str = "roles" ;

/// роли и пользователи
const TAG_USERS_ROLES: &str = "users_roles" ;

/// id_user ключ
pub const ID_USER_KEY: &str = "id_user" ;

/// slug ключ
pub const SLUG_KEY: &str = "slug" ;

/// Сообщение имя пользователя изменено успешно
const USER_NAME_CHANGED_SUCCESS: &str = "User's name changed successfully." ;

/// Сообщение email пользователя изменено успешно
const USER_EMAIL_CHANGED_SUCCESS: &str = "User's email changed successfully." ;

/// Сообщение роль была успешно удалена
const ROLE_WAS_SUCCESS_REMOVED: &str = "The role was successfully removed." ;

/// Сообщение наименование роли было успешно изменено
const ROLE_NAME_WASSUCCESS_CHANGED: &str = "Role name successfully changed." ;

/// Сообщение разрешения роли были успешно изменены
const ROLE_PERMISSIONS_WASSUCCESS_CHANGED: &str = "Role permissions successfully changed." ;

/// Роль успешно добавлена к пользователю.
const ROLE_SUCCESS_ADDED_TO_USER: & str = "The role has been successfully added to the user." ;

/// Роль успешно далена у пользователя
const ROLE_REMOVED_FROM_USER_SUCCESS: &str = "The role has been successfully removed from the user." ;

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
        initdb_handle,
        create_user,
        delete_user,
        update_username,
        update_useremail,
        show_user,
        show_users,
        create_role,
        delete_role,
        update_rolename,
        update_rolepermissions,
        show_role,
        get_show_roles,
        add_role_to_user,
        remove_role_from_user,
    ),
    components(
        schemas(
            common::Responce,
            args::CreateUser,
            args::UpdateNameUser,
            args::UpdateEmailUser,
            users::UserWithRole,
            args::CreateRole,
            args::UpdateNameRole,
            args::UpdatePermissionsRole,
            roles::Role,
            args::AddRoleToUser,
            args::RemoveRoleFromUser,
        )
    ),
    tags(
        (
            name = "initdb",
            description = "Creating the necessary objects in the database",
        ),
        (
            name = "users",
            description = "Working with users",
        ),
        (
            name = "roles",
            description = "Working with roles",
        ),
        (
            name = "users_roles",
            description = "Manipulating user roles",
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
    summary = "Creating database objects.",
    responses (
        (
            status = StatusCode::OK,  // 200, 
            description = "Successfully created database objects.", 
            body = common::Responce,
            example = json!({"Success": DB_OBJ_CREATED_SUCCESS})
        ),
        (
            status = StatusCode::CREATED,   // 303, 
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
                            StatusCode::CREATED,
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
    summary = "Creating a user.",
    request_body = args::CreateUser,
    responses (
        (
            status = StatusCode::OK,  // 200, 
            description = "Successful user creation.", 
            body = common::Responce,
            example = json!({"Success": "User created successfully."})
        ),
        (
            status = StatusCode::CREATED,   // 303, 
            description = "Error creating user.", 
            body = common::Responce,
            example = json!({"Error": "Duplicate user email."})
        ),
    ),
    tag = TAG_USERS,
  )
]
pub async fn create_user(
                    State(db_res): State<Arc<Database>>,
                    Json(cmd): Json<args::CreateUser>
                ) ->impl IntoResponse {

    match create_user_int(&db_res, &cmd).await {
            Ok(v) => (
                            StatusCode::OK,
                            Json(v)
                           ),
            Err(err) => (
                            StatusCode::CREATED,
                            Json(
                                error_message(&err.to_string())
                              )
                            ),
    }
}

// удалить пользователя, внутренний формат
async fn delete_user_int(
            db_res:     &Database,
            id_user:    u32,
         ) ->Result<common::Responce> {
    
    let mut tmp_user = User::default() ;

    tmp_user.set_id_user(id_user)? ;

    // Сформировать новую транзакцию
    let mut trans = 
                db_res
                    .pool
                    // Устанавливает соединение и немедленно начинает новую транзакцию.
                    .begin()
                    .await? ;

    // удалить пользователя
    User::delete_user(&mut *trans, tmp_user.id_user()).await? ;
    
    // выполнить commit в DB
    trans.commit().await? ;    

    Ok(success_message("The user has been deleted."))
}

// удалить пользователя
#[utoipa::path(
    delete,
    //path =  &format!("{}{{id_user}}", common::get_delete_user_uri_short()), // "/api/del_user/{id_user}",
    path =  &format!("{}{{{}}}", common::get_delete_user_uri_short(), ID_USER_KEY), // "/api/del_user/{id_user}",
    summary = "Deleting a user.",
    params(
        ("id_user" = u32, Path, description = "User ID to delete")
    ),
    responses(
        (
            status = StatusCode::OK, // 200, 
            description = "Successfully deleted user.", 
            body = common::Responce,
            example = json!({"Success": "The user has been deleted."}),
        ),
        (
            status = StatusCode::CREATED,  // 303,
            description = "Error deleting user.",
            body = common::Responce,
            example = json!({"Error": "The user does not exist."}),
        ),
    ),
    tag = TAG_USERS,
    /*
    operation_id = "delete_user",
    security(
        ("bearer_auth" = [])
    )
     */
)]
pub async fn delete_user(
                  State(db_res): State<Arc<Database>>,
                  extract::Path(id_user): extract::Path<u32>,
                ) ->impl IntoResponse {
    //println!("id_user: {}", id_user) ;
    match delete_user_int(&db_res, id_user).await {
        Ok(v) => (
                            StatusCode::OK,
                            Json(v)
                           ),
        Err(err) => (
                        StatusCode::CREATED,
                        Json(error_message(&err.to_string()))
                    ),
    }
}

// модифицировать имя пользователя, внутренний формат
async fn update_username_int(
            db_res:     &Database,
            com:        &UpdateNameUser,
         ) ->Result<common::Responce>{
    
    // Сформировать новую транзакцию
    let mut trans = 
              db_res
                .pool
                // Устанавливает соединение и немедленно начинает новую транзакцию.
                .begin()
                .await? ;

    // Модифицировать имя пользователя
    users::User::update_name(
                    &mut *trans,
                    &com.new_name,
                    com.id_user
                ).await? ;

    // Выполнить commit
    trans.commit().await? ;

    Ok(success_message(USER_NAME_CHANGED_SUCCESS))
}

// модифицировать имя пользователя
#[utoipa::path(
    put,
    path = concatcp!(
                common::BASE_URI_PATH,        // "/api/",
                common::UPDATE_USERNAME_PART      // "create_user"
            ), // "/api/create_user",
    summary = "Update user's username",
    request_body = args::UpdateNameUser,
    responses (
        (
            status = StatusCode::OK,  // 200, 
            description = "Successful modification of username.", 
            body = common::Responce,
            example = json!({"Success": USER_NAME_CHANGED_SUCCESS})
        ),
        (
            status = StatusCode::CREATED,   // 303, 
            description = "Error modifying username.", 
            body = common::Responce,
            example = json!({"Error": "Not found user for id_user: 100"})
        ),
    ),
    tag = TAG_USERS,
  )
]
pub async fn update_username(
                State(db_res): State<Arc<Database>>,
                Json(com): Json<args::UpdateNameUser> 
             ) ->impl IntoResponse {
    match update_username_int(&db_res, &com).await {
        Ok(v) => (
                            StatusCode::OK,
                            Json(v),
                           ),
        Err(err) => (
                            StatusCode::CREATED,
                            Json(
                                error_message(
                                    &err.to_string()
                                )
                            )
                           ),
    }
}

// модифицировать email пользователя, внутренний формат
async fn update_useremail_int(
                    db_res:     &Database,
                    com:  &UpdateEmailUser
                ) ->Result<common::Responce> {
    // Сформировать новую транзакцию
    let mut trans = 
               db_res
                .pool
                // Устанавливает соединение и немедленно начинает новую транзакцию.
                .begin()
                .await? ;

    // Модифицировать имя у пользователя
    users::User::update_email(
            &mut *trans,
            &com.new_email,
            com.id_user
          )
          .await? ;

    // Выполнить commit
    trans.commit().await? ;

    Ok(success_message(USER_EMAIL_CHANGED_SUCCESS))
}

// модифицировать email пользователя
#[utoipa::path(
    put,
    path = concatcp!(
                common::BASE_URI_PATH,        // "/api/",
                common::UPDATE_USEREMAIL_PART
            ), // "/api/create_user",
    summary = "Update user's email.",
    request_body = args::UpdateEmailUser,
    responses (
        (
            status = StatusCode::OK,  // 200, 
            description = "Successful modification of user email.", 
            body = common::Responce,
            example = json!({"Success": USER_EMAIL_CHANGED_SUCCESS})
        ),
        (
            status = StatusCode::CREATED,   // 303, 
            description = "Error modifying user email.", 
            body = common::Responce,
            example = json!({"Error": "Invalid email: n1#abc.com"})
        ),
    ),
    tag = TAG_USERS,
  )
]
pub async fn update_useremail(
                    State(db_res): State<Arc<Database>>,
                    Json(com):  Json<args::UpdateEmailUser>
                ) ->impl IntoResponse {
    match update_useremail_int(&db_res, &com).await {
        Ok(v) => (
                            StatusCode::OK,
                            Json(v)
                        ),
        Err(err) => (
                            StatusCode::CREATED,
                            Json(
                                error_message(&err.to_string())
                            )
                        )
    }
}

// Показ пользователя, внутренний код
async fn show_user_int(
            db_res:     &Database,
            id_user:    u32
         ) ->//Result<Vec<users::UserWithRole>> 
            Result<common::Responce>
         {

    //let mut list_ur = vec![] ;

    // Сформировать новую транзакцию
    let mut trans = 
                db_res
                  .pool
                  // Устанавливает соединение и немедленно начинает новую транзакцию.
                  .begin()
                  .await? ;
    /*
    list_ur.push(
        users::UserWithRole::get_data(
                                &mut *trans,
                                id_user
                            )
                            .await?
    );

    Ok(
        common::Responce::UsersRoles(list_ur)
    )
     */
    Ok(common::Responce::UserWithRole(
        users::UserWithRole::get_data(
                                &mut *trans,
                                id_user
                            )
                            .await?
        )
    )
}

// Показ пользователя и их ролей
#[utoipa::path(
    get,
    path =  &format!("{}/{{{}}}", common::get_show_users_uri(), ID_USER_KEY), // "/api/del_user/{id_user}",
    summary = "Show user and their roles",
    params(
        ("id_user" = u32, Path, description = "User ID to show")
    ),
    responses(
        (
            status = StatusCode::OK, // 200, 
            description = "The user and their roles have been successfully displayed.", 
            //body = Vec<users::UserWithRole> // common::Responce,
            body = users::UserWithRole, // common::Responce,
            //example = json!({"Success": "The user has been deleted."}),
        ),
        (
            status = StatusCode::CREATED,  // 303,
            description = "Error displaying user.",
            body = common::Responce,
            example = json!({"Error": "The user does not exist."}),
        ),
    ),
    tag = TAG_USERS,
)]
pub async fn show_user(
                State(db_res): State<Arc<Database>>,
                extract::Path(id_user): extract::Path<u32>
             ) ->impl IntoResponse {
    match show_user_int(&db_res, id_user).await {
        Ok(v) => (
                            StatusCode::OK,
                            Json(v)
                           ),
        Err(err) => (
                            StatusCode::CREATED, 
                            Json(error_message(&err.to_string()))
                           ),
    }
}

// показ пользователей и их ролей, внутренний код
async fn show_users_int(db_res: &Database) ->Result<common::Responce> {

        // Сформировать новую транзакцию
    let mut trans = 
                db_res
                  .pool
                  // Устанавливает соединение и немедленно начинает новую транзакцию.
                  .begin()
                  .await? ;

    let mut list_ur = vec![] ;

    for id_user in users::User::get_all_id_user(&mut *trans).await? {
            list_ur.push(
                users::UserWithRole::get_data(
                    &mut *trans,
                    id_user
                )
                .await?
            );
    }
    
    Ok(common::Responce::UsersRoles(list_ur))
}

// показ пользователей и их ролей
#[utoipa::path(
    get,
    path = common::get_show_users_uri(), // "/api/del_user/{id_user}",
    summary = "Show users and their roles.",
    responses(
        (
            status = StatusCode::OK, // 200, 
            description = "Successful display of users and their roles.", 
            //body = Vec<users::UserWithRole> // common::Responce,
            body = Vec<users::UserWithRole>, // common::Responce,
            //example = json!({"Success": "The user has been deleted."}),
        ),
        (
            status = StatusCode::CREATED,  // 303,
            description = "Error displaying users and their roles.",
            body = common::Responce,
            example = json!({"Error": "Database connection error."}),
        ),
    ),
    tag = TAG_USERS,
)]
pub async fn show_users(
                State(db_res): State<Arc<Database>>,
             ) ->impl IntoResponse {
    match show_users_int(&db_res).await {
        Ok(v) => (StatusCode::OK, Json(v)),
        Err(err) => (StatusCode::CREATED, Json(error_message(&err.to_string()))),
    }
}

// Создать роль, внутренний код
pub async fn create_role_int(
                db_res:     &Database,
                cmd:        &args::CreateRole
             ) ->Result<common::Responce> {
    roles::Role::create_role(
            &db_res, 
            &cmd.slug, 
            &cmd.name, 
            &cmd.permissions.join(",")
        )
        .await? ;

    Ok(success_message(ROLE_CREATED_SUCCESSFULY))
}

// Создать роль
#[utoipa::path(
    post,
    path = concatcp!(
                common::BASE_URI_PATH,        // "/api/",
                common::CREATE_ROLE_PART      // "create_user"
            ), // "/api/create_user",
    summary = "Create a role.",
    request_body = args::CreateRole,
    responses (
        (
            status = StatusCode::OK,  // 200, 
            description = "Successful role creation.", 
            body = common::Responce,
            example = json!({"Success": ROLE_CREATED_SUCCESSFULY})
        ),
        (
            status = StatusCode::CREATED,   // 303, 
            description = "Error creating role.", 
            body = common::Responce,
            example = json!({"Error": "name is empty"})
        ),
    ),
    tag = TAG_ROLES,
  )
]
pub async fn create_role(
                State(db_res): State<Arc<Database>>,
                Json(cmd):  Json<args::CreateRole>
             ) ->impl IntoResponse {
    match create_role_int(&db_res, &cmd).await {
        Ok(v) => (StatusCode::OK, Json(v)),
        Err(err) => (StatusCode::CREATED, Json(error_message(&err.to_string()))),
    }
}

// Удаление роли, внутренний код
async fn delete_role_int(
                db_res:     &Database,
                slug:       &str,
             ) ->Result<common::Responce> {
               
    // Сформировать новую транзакцию
    let mut trans = 
                db_res
                    .pool
                    // Устанавливает соединение и немедленно начинает новую транзакцию.
                    .begin()
                    .await? ;

    let slug = &urlencoding::decode(slug)?
                        //.unwrap()
                        .to_string()
                        //.as_str()
                        ;

    // удалить роль
    roles::Role::delete_role(&mut *trans, slug).await? ;

    // выполнить commit
    trans.commit().await? ;

    Ok(success_message(ROLE_WAS_SUCCESS_REMOVED))
}

// Удаление роли
#[utoipa::path(
    delete,
    //path =  &format!("{}{{id_user}}", common::get_delete_user_uri_short()), // "/api/del_user/{id_user}",
    path =  &format!("{}{{{}}}", common::get_delete_role_uri_short(), SLUG_KEY), // "/api/del_user/{id_user}",
    summary = "Deleting a role.",
    params(
        ("slug" = String, Path, description = "Slug of role to delete.")
    ),
    responses(
        (
            status = StatusCode::OK, // 200, 
            description = "Successful role deletion.", 
            body = common::Responce,
            example = json!({"Success": ROLE_WAS_SUCCESS_REMOVED}),
        ),
        (
            status = StatusCode::CREATED,  // 303,
            description = "Error deleting role.",
            body = common::Responce,
            example = json!({"Error": "The default role cannot be deleted."}),
        ),
    ),
    tag = TAG_ROLES,
)]
pub async fn delete_role(
                  State(db_res): State<Arc<Database>>,
                  extract::Path(slug): extract::Path<String>,
                ) ->impl IntoResponse {
    match delete_role_int(&db_res, &slug).await {
       Ok(v)  => (
                            StatusCode::OK,
                            Json(v)
                           ),
       Err(err) => (
                            StatusCode::CREATED,
                            Json(error_message(&err.to_string()))
                          ),
    }
}

/// Модификация наименование роли, внутренний код
async fn update_rolename_int(
                db_res: &Database,
                cmd:  &args::UpdateNameRole
             ) ->Result<common::Responce> {
    // Сформировать новую транзакцию
    let mut trans = 
                db_res
                    .pool
                    // Устанавливает соединение и немедленно начинает новую транзакцию.
                    .begin()
                    .await? ;

    // Модифицировать name в роли
    roles::Role::update_name(&mut *trans, &cmd.slug, &cmd.new_name)
            .await? ;

    // Выполнить commit
    trans.commit().await? ;

    Ok(success_message(ROLE_NAME_WASSUCCESS_CHANGED))
}

/// Модификация наименование роли
#[utoipa::path(
    put,
    path = concatcp!(
                common::BASE_URI_PATH,        // "/api/",
                common::UPDATE_ROLENAME_PART      // "create_user"
            ), // "/api/create_user",
    summary = "Modify role name",
    request_body = args::UpdateNameRole,
    responses (
        (
            status = StatusCode::OK,  // 200, 
            description = "Successful modification of role name.", 
            body = common::Responce,
            example = json!({"Success": ROLE_NAME_WASSUCCESS_CHANGED})
        ),
        (
            status = StatusCode::CREATED,   // 303, 
            description = "Error modifying role name.", 
            body = common::Responce,
            example = json!({"Error": "Not found role for slug: abc-mk"})
        ),
    ),
    tag = TAG_ROLES,
  )
]
pub async fn update_rolename(
                State(db_res): State<Arc<Database>>,
                Json(cmd):  Json<args::UpdateNameRole>
             ) ->impl IntoResponse {
    match update_rolename_int(&db_res, &cmd).await {
        Ok(v) => (
                            StatusCode::OK,
                            Json(v)
                           ),
        Err(err) => (
                            StatusCode::CREATED,
                            Json(
                                error_message(&err.to_string())
                            )
                        ),
    }
}

// Модифицировать разрешение у роли, внутренний код
async fn update_rolepermissions_int(
                db_res: &Database,
                cmd:    &args::UpdatePermissionsRole,
             ) ->Result<common::Responce> {
    // Сформировать новую транзакцию
    let mut trans = 
                db_res
                    .pool
                    // Устанавливает соединение и немедленно начинает новую транзакцию.
                    .begin()
                    .await? ;

    // Модифицировать разрешения у роли
    roles::Role::update_permissions(
                    &mut *trans,
                    &cmd.slug, 
                    &cmd.new_permissions.join(",")
                    )
                    .await? ;

    // Выполнить commit
    trans.commit().await? ;

    Ok(success_message(ROLE_PERMISSIONS_WASSUCCESS_CHANGED))
}

// Модифицировать разрешение у роли
#[utoipa::path(
    put,
    path = concatcp!(
                common::BASE_URI_PATH,        // "/api/",
                common::UPDATE_ROLEPERMISSIONS_PART      // "create_user"
            ), // "/api/create_user",
    summary = "Modify role permissions",
    request_body = args::UpdatePermissionsRole,
    responses (
        (
            status = StatusCode::OK,  // 200, 
            description = "Successful modification of role permission.", 
            body = common::Responce,
            example = json!({"Success": ROLE_PERMISSIONS_WASSUCCESS_CHANGED})
        ),
        (
            status = StatusCode::CREATED,   // 303, 
            description = "Error modifying role permissions.", 
            body = common::Responce,
            example = json!({"Error": "Not found role for slug: abc-mk"})
        ),
    ),
    tag = TAG_ROLES,
  )
]
pub async fn update_rolepermissions(
                State(db_res): State<Arc<Database>>,
                Json(cmd): Json<args::UpdatePermissionsRole>
             ) ->impl IntoResponse {
    match update_rolepermissions_int(&db_res, &cmd).await {
        Ok(v) => (StatusCode::OK, Json(v)),
        Err(err) => (StatusCode::CREATED, Json(error_message(&err.to_string()))),
    }
}

/*
async fn show_role_int(
            db_res: &Database,
            slug:   &str,
         ) ->Result<common::Responce> {
    // Сформировать новую транзакцию
    let mut trans = 
                db_res
                  .pool
                  // Устанавливает соединение и немедленно начинает новую транзакцию.
                  .begin()
                  .await? ;

    Ok(
      common::Responce::Role( 
        roles::Role::find_slug_raise(
            &mut *trans,
            slug,
            false
        )
        .await?
      )
    )
}
 */

/// Показать роль
#[utoipa::path(
    get,
    path =  &format!("{}/{{{}}}", common::get_show_role_short_uri(), SLUG_KEY), // "/api/del_user/{id_user}",
    summary = "Show role",
    params(
        ("slug" = String, Path, description = "Role slug to show")
    ),
    responses(
        (
            status = StatusCode::OK, // 200, 
            description = "Successfully displayed role.", 
            //body = Vec<users::UserWithRole> // common::Responce,
            body = roles::Role, // common::Responce,
            //example = json!({"Success": "The user has been deleted."}),
        ),
        (
            status = StatusCode::CREATED,  // 303,
            description = "Error displaying role.",
            body = common::Responce,
            example = json!({"Error": "Not found role for slug: abc-mk"}),
        ),
    ),
    tag = TAG_ROLES,
)]
pub async fn show_role(
                State(db_res): State<Arc<Database>>,
                extract::Path(slug): extract::Path<String>
             ) ->impl IntoResponse {

    async fn show_role_int(
                db_res: &Database,
                slug:   &str,
            ) ->Result<common::Responce> {
        // Сформировать новую транзакцию
        let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

        Ok(
            common::Responce::Role( 
                roles::Role::find_slug_raise(
                    &mut *trans,
                    slug,
                    false
                )
                .await?
            )
        )
    }

    match show_role_int(&db_res, &slug).await {
        Ok(v) => (StatusCode::OK, Json(v)),
        Err(err) => (StatusCode::CREATED, Json(error_message(&err.to_string()))),

    }
}


/// Показать все роли
#[utoipa::path(
    get,
    path =  &common::get_show_role_short_uri(), // "/api/del_user/{id_user}",
    summary = "Show all roles",
    responses(
        (
            status = StatusCode::OK, // 200, 
            description = "Successfully display all roles.", 
            //body = Vec<users::UserWithRole> // common::Responce,
            body = Vec<roles::Role>, // common::Responce,
            //example = json!({"Success": "The user has been deleted."}),
        ),
        (
            status = StatusCode::CREATED,  // 303,
            description = "Error displaying all roles.",
            body = common::Responce,
            example = json!({"Error": "Not found role for slug: abc-mk"}),
        ),
    ),
    tag = TAG_ROLES,
)]
pub async fn get_show_roles(
                State(db_res): State<Arc<Database>>,
            ) ->impl IntoResponse {

    async fn get_show_roles_int(db_res: &Database) ->Result<common::Responce> {
        // Сформировать новую транзакцию
        let mut trans = 
                  db_res
                    .pool
                    // Устанавливает соединение и немедленно начинает новую транзакцию.
                    .begin()
                    .await? ;

        let mut list_roles = vec![] ;

        for sl in roles::Role::get_all_slugs(&mut *&mut trans).await? {
                    list_roles.push(
                            roles::Role::find_slug_raise(
                                    &mut *trans,
                                    &sl,
                                    false
                            )
                            .await?                            
                    );
        }
        Ok(common::Responce::ListRoles(list_roles))
    }

    match get_show_roles_int(&db_res).await {
        Ok(v) => (StatusCode::OK, Json(v)),
        Err(err) => (StatusCode::CREATED, Json(error_message(&err.to_string()))),
    }

}

/// Добавить роль к пользователю
#[utoipa::path(
    post,
    path = concatcp!(
                common::BASE_URI_PATH,        // "/api/",
                common::ADD_ROLE_TO_USER_PART      // "create_user"
            ), // "/api/create_user",
    summary = "Add a role to a user",
    request_body = args::AddRoleToUser,
    responses (
        (
            status = StatusCode::OK,  // 200, 
            description = "Successfully added role to user.", 
            body = common::Responce,
            example = json!({"Success": ROLE_SUCCESS_ADDED_TO_USER})
        ),
        (
            status = StatusCode::CREATED,   // 303, 
            description = "Error adding role to user.", 
            body = common::Responce,
            example = json!({"Error": "Not found user for id_user: 100"})
        ),
    ),
    tag = TAG_USERS_ROLES,
  )
]
pub async fn add_role_to_user(
                State(db_res): State<Arc<Database>>,
                Json(com): Json<args::AddRoleToUser>
             ) ->impl IntoResponse {

    async fn add_role_to_user_int(
                    db_res: &Database,
                    cmd:    &args::AddRoleToUser
                ) ->Result<common::Responce> {
        // Сформировать новую транзакцию
        let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

        // Добавить роль для пользователя
        users_roles::UsersRoles::ins_role_to_user(
                        &mut *trans,
                        cmd.id_user,
                        &cmd.slug
                    )
                    .await? ;
        // выполнить commit
        trans.commit().await? ;

        Ok(success_message(ROLE_SUCCESS_ADDED_TO_USER))
    }

    match add_role_to_user_int(&db_res, &com).await {
        Ok(v) => (StatusCode::OK, Json(v)),
        Err(err) => (StatusCode::CREATED, Json(error_message(&err.to_string()))),
    }
}

/// Удалить роль у пользователи
#[utoipa::path(
    post,
    path = concatcp!(
                common::BASE_URI_PATH,        // "/api/",
                common::REMOVE_ROLE_FROM_USER_PART      // "create_user"
            ), // "/api/create_user",
    summary = "Removing a role from a user",
    request_body = args::RemoveRoleFromUser,
    responses (
        (
            status = StatusCode::OK,  // 200, 
            description = "Successfully removed a role from a user.", 
            body = common::Responce,
            example = json!({"Success": ROLE_REMOVED_FROM_USER_SUCCESS})
        ),
        (
            status = StatusCode::CREATED,   // 303, 
            description = "Error deleting role from user.", 
            body = common::Responce,
            example = json!({"Error": "Invalid number: 1 of roles for id_user: 10"})
        ),
    ),
    tag = TAG_USERS_ROLES,
  )
]
pub async fn remove_role_from_user(
                State(db_res): State<Arc<Database>>,
                Json(cmd): Json<args::RemoveRoleFromUser>
             ) ->impl IntoResponse {

    async fn remove_role_from_user_int(
                    db_res: &Database,
                    cmd:    &args::RemoveRoleFromUser
                ) ->Result<common::Responce> {
        // Сформировать новую транзакцию
        let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;
        // удаление роли у пользователя
        users_roles::UsersRoles::del_role_from_user(
                    &mut *trans,
                    cmd.id_user,
                    &cmd.slug
                ).await? ;
        // выпонить commit
        trans.commit().await? ;

        Ok(success_message(ROLE_REMOVED_FROM_USER_SUCCESS))
    }

    match remove_role_from_user_int(&db_res, &cmd).await {
        Ok(v) => (StatusCode::OK, Json(v)),
        Err(err) => (StatusCode::CREATED, Json(error_message(&err.to_string()))),
    }
}