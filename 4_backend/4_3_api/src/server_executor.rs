use std::sync::Arc ;    // Импорт атомарного счетчика ссылок для разделяемого владения данными между потоками

use const_format::concatcp; // Импорт макроса для конкатенации строковых литералов на этапе компиляции

use urlencoding ;   // Импорт крейта для кодирования/декодирования URL-строк (например, пробелов в %20)

use axum::{
        extract::{
            self,   // Импорт модуля extract целиком для доступа к экстракторам запросов (JSON, State, Path и др.)
            Json,   // Импорт экстрактора для извлечения JSON-данных из тела запроса
            State   // Импорт экстрактора для доступа к разделяемому состоянию приложения
        }, 
        http::{
            StatusCode, // Импорт перечисления HTTP-статусов (200, 404, 500 и т.д.)
        },
        response::IntoResponse, // Импорт трейта для преобразования типов в HTTP-ответы
} ;

use anyhow::Result ;    // Импорт типа Result из anyhow для гибкой обработки ошибок с контекстом

use utoipa::OpenApi ;   // Импорт трейта OpenApi для генерации OpenAPI-документации из аннотаций

use crate::db::Database ;   // Импорт структуры Database из модуля db текущего крейта для работы с подключением к БД

use crate::args ;   // Импорт модуля args целиком для доступа к структурам аргументов командной строки

use crate::{
        args::{
            UpdateEmailUser, // Импорт структуры аргументов для обновления email пользователя
            UpdateNameUser   // Импорт структуры аргументов для обновления имени пользователя
        },
        common, // Импорт модуля common с общими утилитами и функциями
        roles,  // Импорт модуля roles для работы с ролями пользователей
        users::{
            self, // Импорт модуля users с псевдонимом для доступа к его функциям и типам
            User  // Импорт структуры User из модуля users
        },
        users_roles,    // Импорт модуля users_roles для работы со связями пользователей и ролей
} ;

// Сообщение об успешном создании объектов базы данных
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

    // текущий контент содержимого openapi
    let content_openapi_now = serde_json::to_string_pretty(&op_api)? ;

    // путь к файлу openapi
    let openapi_path = std::path::Path::new(OPENAPI_FILE) ;

    // чтение содержимого файла openapi
    let content_openapi = if openapi_path.exists() {
        std::fs::read_to_string(openapi_path)?
    } else {
        "".to_string()
    } ;

    // Перезаписать контент openapi файла если контент изменился
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
#[derive(OpenApi)]  // Автоматически реализует трейт OpenApi для генерации OpenAPI-документации
#[openapi(  // Атрибут для настройки OpenAPI-спецификации
    paths(  // Секция для регистрации всех endpoint-обработчиков API
        initdb_handle,  // Регистрация эндпоинта инициализации базы данных
        create_user,    // Регистрация эндпоинта создания пользователя
        delete_user,    // Регистрация эндпоинта удаления пользователя
        update_username,    // Регистрация эндпоинта обновления имени пользователя
        update_useremail,   // Регистрация эндпоинта обновления email пользователя
        show_user,          // Регистрация эндпоинта получения одного пользователя
        show_users,         // Регистрация эндпоинта получения списка всех пользователей
        create_role,        // Регистрация эндпоинта создания роли
        delete_role,        // Регистрация эндпоинта удаления роли
        update_rolename,    // Регистрация эндпоинта обновления названия роли
        update_rolepermissions, // Регистрация эндпоинта обновления разрешений роли
        show_role,          // Регистрация эндпоинта получения одной роли
        get_show_roles,     // Регистрация эндпоинта получения всех ролей
        add_role_to_user,   // Регистрация эндпоинта назначения роли пользователю
        remove_role_from_user,  // Регистрация эндпоинта удаления роли у пользователя
    ),
    components(     // Секция для описания схем данных, используемых в API
        schemas(    // Список структур, которые будут документированы как OpenAPI-схемы
            common::Responce,   // Схема стандартного ответа API (опечатка в оригинале: Responce -> Response)
            args::CreateUser,   // Схема данных для создания пользователя
            args::UpdateNameUser,   // Схема данных для обновления имени пользователя
            args::UpdateEmailUser,  // Схема данных для обновления email пользователя
            users::UserWithRole,    // Схема пользователя с его ролью
            args::CreateRole,       // Схема данных для создания роли
            args::UpdateNameRole,   // Схема данных для обновления названия роли
            args::UpdatePermissionsRole,    // Схема данных для обновления разрешений роли
            roles::Role,    // Схема роли
            args::AddRoleToUser,    // Схема данных для назначения роли пользователю
            args::RemoveRoleFromUser,   // Схема данных для удаления роли у пользователя
        )
    ),
    tags(   // Секция группировки эндпоинтов по категориям в документации Swagger UI
        (
            name = "initdb",    // Название категории для эндпоинтов инициализации БД
            description = "Creating the necessary objects in the database", // Описание категории
        ),
        (
            name = "users", // Название категории для эндпоинтов работы с пользователями
            description = "Working with users", // Описание категории
        ),
        (
            name = "roles", // Название категории для эндпоинтов работы с ролями
            description = "Working with roles", // Описание категории
        ),
        (
            name = "users_roles",   // Название категории для эндпоинтов управления ролями пользователей
            description = "Manipulating user roles",    // Описание категории
        ),        
    ),
    info(   // Секция с общей информацией об API
        title = "API for working with users and their roles.",  // Заголовок документации API
        version = "1.0.0",  // Версия API
        description = "RESTful API for managing users and roles",   // Описание функциональности API
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
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    get,    // Указывает, что этот обработчик отвечает на GET-запросы
    path = concatcp!(   // Определяет URL-путь эндпоинта с конкатенацией на этапе компиляции
                common::BASE_URI_PATH,  // Базовая часть пути из модуля common
                common::INIT_DB_PART,   // Конкретная часть пути для инициализации БД
            ),
    summary = "Creating database objects.", // Краткое описание функциональности эндпоинта
    responses ( // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном выполнении
            description = "Successfully created database objects.", // Описание успешного ответа
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа
            example = json!({"Success": DB_OBJ_CREATED_SUCCESS})    // Пример успешного JSON-ответа
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 (для ощибки)
            description = "Error creating database objects.",   // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "Error creating trigger."}),  // Пример ответа с ошибкой
        ),
    ),
    tag = "initdb", // Группирует эндпоинт в категорию "initdb" в Swagger UI документации
  )
]
pub async fn initdb_handle(
                State(db_res): State<Arc<Database>>,
            ) -> impl IntoResponse
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
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    post,   // Указывает, что этот обработчик отвечает на POST-запросы
    path = concatcp!(   // Определяет URL-путь эндпоинта с конкатенацией на этапе компиляции
                common::BASE_URI_PATH,  // Базовая часть пути из модуля common
                common::CREATE_USER_PART,   // Конкретная часть пути для создания пользователя
            ),
    summary = "Creating a user.",   // Краткое описание функциональности эндпоинта
    request_body = args::CreateUser,    // Описывает структуру JSON-тела запроса для создания пользователя
    responses ( // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном создании пользователя
            description = "Successful user creation.",  // Описание успешного ответа
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа
            example = json!({"Success": "User created successfully."})  // Пример успешного JSON-ответа
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке создания 
            description = "Error creating user.",   // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "Duplicate user email."}) // Пример ответа с ошибкой дублирования email
        ),
    ),
    tag = TAG_USERS,    // Группирует эндпоинт в категорию "users" в Swagger UI документации
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
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    delete, // Указывает, что этот обработчик отвечает на DELETE-запросы
    path =  &format!("{}{{{}}}", common::get_delete_user_uri_short(), ID_USER_KEY), // Динамическое формирование пути с параметром id_user в фигурных скобках
    summary = "Deleting a user.",   // Краткое описание функциональности эндпоинта
    params( // Секция описания параметров запроса
        ("id_user" = u32, Path, description = "User ID to delete")
    ),
    responses(  // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном удалении пользователя
            description = "Successfully deleted user.", // Описание успешного ответа
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа
            example = json!({"Success": "The user has been deleted."}), // Пример успешного JSON-ответа
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке удаления
            description = "Error deleting user.",   // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "The user does not exist."}), // Пример ответа с ошибкой отсутствия пользователя
        ),
    ),
    tag = TAG_USERS,    // Пример ответа с ошибкой отсутствия пользователя
)]
pub async fn delete_user(
                  State(db_res): State<Arc<Database>>,
                  extract::Path(id_user): extract::Path<u32>,
                ) ->impl IntoResponse {

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
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    put,    // Указывает, что этот обработчик отвечает на PUT-запросы (обновление ресурса)
    path = concatcp!(   // Определяет URL-путь эндпоинта с конкатенацией на этапе компиляции
                common::BASE_URI_PATH,  // Базовая часть пути из модуля common
                common::UPDATE_USERNAME_PART    // Конкретная часть пути для обновления имени пользователя
            ),
    summary = "Update user's username", // Краткое описание функциональности эндпоинта
    request_body = args::UpdateNameUser,    // Описывает структуру JSON-тела запроса с ID пользователя и новым именем
    responses ( // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном обновлении имени
            description = "Successful modification of username.",   // Описание успешного ответа
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа
            example = json!({"Success": USER_NAME_CHANGED_SUCCESS}) // Пример успешного JSON-ответа
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке обновления
            description = "Error modifying username.",  // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "Not found user for id_user: 100"})   // Пример ответа с ошибкой отсутствия пользователя
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
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    put,    // Указывает, что этот обработчик отвечает на PUT-запросы (обновление ресурса)
    path = concatcp!(   // Определяет URL-путь эндпоинта с конкатенацией на этапе компиляции
                common::BASE_URI_PATH,  // Базовая часть пути из модуля common
                common::UPDATE_USEREMAIL_PART   // Конкретная часть пути для обновления email пользователя
            ),
    summary = "Update user's email.",   // Краткое описание функциональности эндпоинта
    request_body = args::UpdateEmailUser,   // Описывает структуру JSON-тела запроса с ID пользователя и новым email
    responses ( // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном обновлении email
            description = "Successful modification of user email.", // Описание успешного ответа
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа
            example = json!({"Success": USER_EMAIL_CHANGED_SUCCESS})    // Пример успешного JSON-ответа 
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке обновления
            description = "Error modifying user email.",    // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "Invalid email: n1#abc.com"}) // Пример ответа с ошибкой невалидного email
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
         ) ->Result<common::Responce> {

    // Сформировать новую транзакцию
    let mut trans = 
                db_res
                  .pool
                  // Устанавливает соединение и немедленно начинает новую транзакцию.
                  .begin()
                  .await? ;

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
#[utoipa::path(// Атрибут для документирования эндпоинта в OpenAPI спецификации
    get,    // Указывает, что этот обработчик отвечает на GET-запросы
    path =  &format!("{}/{{{}}}", common::get_show_users_uri(), ID_USER_KEY),   // Динамическое формирование пути с параметром id_user в фигурных скобках
    summary = "Show user and their roles",  // Краткое описание функциональности эндпоинта
    params( // Секция описания параметров запроса
        ("id_user" = u32, Path, description = "User ID to show")    // Описание параметра пути id_user типа u32 с пояснением, Path — обозначает, что параметр является частью URL-пути
    ),
    responses(  // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном отображении пользователя
            description = "The user and their roles have been successfully displayed.", // Описание успешного ответа
            body = users::UserWithRole, // Структура данных пользователя с ролями, возвращаемая в теле ответа
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке отображения
            description = "Error displaying user.", // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "The user does not exist."}), // Пример ответа с ошибкой отсутствия пользователя
        ),
    ),
    tag = TAG_USERS,    // Группирует эндпоинт в категорию "users" в Swagger UI документации
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
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    get,    // Указывает, что этот обработчик отвечает на GET-запросы
    path = common::get_show_users_uri(),    // URL-путь эндпоинта, возвращаемый функцией из модуля common
    summary = "Show users and their roles.",    // Краткое описание функциональности эндпоинта
    responses(  // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном отображении списка пользователе
            description = "Successful display of users and their roles.",   // Описание успешного ответа
            body = Vec<users::UserWithRole>,    // Массив структур пользователей с ролями, возвращаемый в теле ответа
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке формирования данных
            description = "Error displaying users and their roles.",    // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "Database connection error."}),   // Пример ответа с ошибкой подключения к БД
        ),
    ),
    tag = TAG_USERS,    // Группирует эндпоинт в категорию "users" в Swagger UI документации
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
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    post,   // Указывает, что этот обработчик отвечает на POST-запросы
    path = concatcp!(   // Определяет URL-путь эндпоинта с конкатенацией на этапе компиляции
                common::BASE_URI_PATH,  // Базовая часть пути из модуля common
                common::CREATE_ROLE_PART    // Конкретная часть пути для создания роли
            ),
    summary = "Create a role.", // Краткое описание функциональности эндпоинта
    request_body = args::CreateRole,    // Описывает структуру JSON-тела запроса с названием и правами новой роли
    responses ( // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном создании роли
            description = "Successful role creation.",  // Описание успешного ответа
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа
            example = json!({"Success": ROLE_CREATED_SUCCESSFULY})  // Пример успешного JSON-ответа
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке создания
            description = "Error creating role.",   // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "name is empty"}) // Пример ответа с ошибкой пустого названия роли
        ),
    ),
    tag = TAG_ROLES,    // Группирует эндпоинт в категорию "roles" в Swagger UI документации
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
                    .begin()
                    .await? ;

    let slug = &urlencoding::decode(slug)?
                        .to_string()
                        ;

    // удалить роль
    roles::Role::delete_role(&mut *trans, slug).await? ;

    // выполнить commit
    trans.commit().await? ;

    Ok(success_message(ROLE_WAS_SUCCESS_REMOVED))
}

// Удаление роли
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    delete, // Указывает, что этот обработчик отвечает на DELETE-запросы (удаление ресурса)
    path =  &format!("{}{{{}}}", common::get_delete_role_uri_short(), SLUG_KEY),    // Динамическое формирование пути с параметром slug в фигурных скобках
    summary = "Deleting a role.",   // Краткое описание функциональности эндпоинта
    params( // Секция описания параметров запроса
        ("slug" = String, Path, description = "Slug of role to delete.")    // Описание параметра пути slug типа String с пояснением
    ),
    responses(  // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном удалении роли
            description = "Successful role deletion.",  // Описание успешного ответа
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа
            example = json!({"Success": ROLE_WAS_SUCCESS_REMOVED}), // Пример успешного JSON-ответа
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке удаления
            description = "Error deleting role.",   // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "The default role cannot be deleted."}),  // Пример ответа с ошибкой удаления защищенной роли по умолчанию
        ),
    ),
    tag = TAG_ROLES,    // Группирует эндпоинт в категорию "roles" в Swagger UI документации
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
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    put,    // Указывает, что этот обработчик отвечает на PUT-запросы (обновление ресурса)
    path = concatcp!(   // Определяет URL-путь эндпоинта с конкатенацией на этапе компиляции
                common::BASE_URI_PATH,  // Базовая часть пути из модуля common
                common::UPDATE_ROLENAME_PART    // Конкретная часть пути для обновления названия роли
            ),
    summary = "Modify role name",   // Краткое описание функциональности эндпоинта
    request_body = args::UpdateNameRole,    // Описывает структуру JSON-тела запроса с slug роли и новым названием
    responses ( // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,     // HTTP статус 200 при успешном обновлении названия роли
            description = "Successful modification of role name.",  // Описание успешного ответа
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа
            example = json!({"Success": ROLE_NAME_WASSUCCESS_CHANGED})  // Пример успешного JSON-ответа 
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке обновления
            description = "Error modifying role name.", // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "Not found role for slug: abc-mk"})   // Пример ответа с ошибкой отсутствия роли по указанному slug
        ),
    ),
    tag = TAG_ROLES,    // Группирует эндпоинт в категорию "roles" в Swagger UI документации
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
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    put,    // Указывает, что этот обработчик отвечает на PUT-запросы (обновление ресурса)
    path = concatcp!(   // Определяет URL-путь эндпоинта с конкатенацией на этапе компиляции
                common::BASE_URI_PATH,  // Базовая часть пути из модуля common 
                common::UPDATE_ROLEPERMISSIONS_PART // Конкретная часть пути для обновления прав роли
            ),
    summary = "Modify role permissions",    // Краткое описание функциональности эндпоинта
    request_body = args::UpdatePermissionsRole, // Описывает структуру JSON-тела запроса с slug роли и новым набором прав
    responses ( // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном обновлении прав роли
            description = "Successful modification of role permission.",    // Описание успешного ответа
            body = common::Responce,    // // Структура данных, возвращаемая в теле ответа
            example = json!({"Success": ROLE_PERMISSIONS_WASSUCCESS_CHANGED})   // Пример успешного JSON-ответа
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке обновления
            description = "Error modifying role permissions.",  // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "Not found role for slug: abc-mk"})   // Пример ответа с ошибкой отсутствия роли по указанному slug
        ),
    ),
    tag = TAG_ROLES,    // Группирует эндпоинт в категорию "roles" в Swagger UI документации
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

/// Показать роль
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    get,    // Указывает, что этот обработчик отвечает на GET-запросы (получение данных)
    path =  &format!("{}/{{{}}}", common::get_show_role_short_uri(), SLUG_KEY), // Динамическое формирование пути с параметром slug в фигурных скобках
    summary = "Show role",  // Краткое описание функциональности эндпоинта
    params( // Секция описания параметров запроса
        ("slug" = String, Path, description = "Role slug to show")  // Описание параметра пути slug типа String с пояснением 
    ),
    responses(  // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном отображении роли
            description = "Successfully displayed role.",   // Описание успешного ответа
            body = roles::Role, // Структура данных роли, возвращаемая в теле ответа
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке отображения
            description = "Error displaying role.", // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "Not found role for slug: abc-mk"}),  // Пример ответа с ошибкой отсутствия роли по указанному slug
        ),
    ),
    tag = TAG_ROLES,    // Группирует эндпоинт в категорию "roles" в Swagger UI документации
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
#[utoipa::path( // Атрибут для документирования эндпоинта в OpenAPI спецификации
    get,    // Указывает, что этот обработчик отвечает на GET-запросы (получение данных)
    path =  &common::get_show_role_short_uri(), // URL-путь эндпоинта, возвращаемый функцией из модуля common
    summary = "Show all roles", // Краткое описание функциональности эндпоинта
    responses(  // Секция описания возможных HTTP-ответов
        (
            status = StatusCode::OK,    // HTTP статус 200 при успешном отображении списка всех ролей
            description = "Successfully display all roles.",    // Описание успешного ответа
            body = Vec<roles::Role>,    // Массив структур ролей, возвращаемый в теле ответа
        ),
        (
            status = StatusCode::CREATED,   // HTTP статус 201 при ошибке отображения
            description = "Error displaying all roles.",    // Описание ответа при ошибке
            body = common::Responce,    // Структура данных, возвращаемая в теле ответа при ошибке
            example = json!({"Error": "Not found role for slug: abc-mk"}),  // Пример ответа с ошибкой
        ),
    ),
    tag = TAG_ROLES,    // Группирует эндпоинт в категорию "roles" в Swagger UI документации
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

// ****************** Раздел Users-Roles ******************

/// Добавить роль к пользователю
#[utoipa::path(
    post,
    path = concatcp!(
                common::BASE_URI_PATH,
                common::ADD_ROLE_TO_USER_PART
            ),
    summary = "Add a role to a user",
    request_body = args::AddRoleToUser,
    responses (
        (
            status = StatusCode::OK,
            description = "Successfully added role to user.", 
            body = common::Responce,
            example = json!({"Success": ROLE_SUCCESS_ADDED_TO_USER})
        ),
        (
            status = StatusCode::CREATED,
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
                common::BASE_URI_PATH,
                common::REMOVE_ROLE_FROM_USER_PART
            ),
    summary = "Removing a role from a user",
    request_body = args::RemoveRoleFromUser,
    responses (
        (
            status = StatusCode::OK,
            description = "Successfully removed a role from a user.", 
            body = common::Responce,
            example = json!({"Success": ROLE_REMOVED_FROM_USER_SUCCESS})
        ),
        (
            status = StatusCode::CREATED,
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