use anyhow::Result ;

use axum::{
    extract::{Json, State},
    response::Json as JsonResponse,
} ;

use crate::args ;

use crate::args::Commands;
use crate::common ;
use std::sync::Arc ;

use crate::db::Database ;

use crate::users ;

use crate::roles ;

use crate::users_roles ;

/// Внутренний обработчик полученных команд от клиента
async fn handle_commmand_int(
                    db_res: &Database,
                    cmd:    &args::Commands
                ) ->Result<common::Response /*, String*/> {

    /// Локальная функция возвращения успеха
    fn local_success(mess: &str) ->common::Response {
        common::Response::Success(mess.to_owned())
    }

    match cmd {
        Commands::InitDb => {
            db_res
                .create_tables()
                .await? ;
                Ok(local_success("Database objects created successfully."))
        },
        Commands::CreateUser { name, email } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                    .pool
                    // Устанавливает соединение и немедленно начинает новую транзакцию.
                    .begin()
                    .await? ;

            // вставить нового пользователя
            let new_user = 
                    users::User::ins_user(
                            &mut *trans,
                            &name,
                            &email
                          )
                          .await? ;

            // Добавить для пользователя роль по умолчанию
            users_roles::UsersRoles::ins_role_to_user(
                            &mut *trans,
                            new_user.id_user(),
                            roles::SLUG_DEFAULT
                        )
                        .await? ;

            // выполнить commit в DB
            trans.commit().await? ;

            Ok(local_success("User created successfully."))            
        },
        Commands::DeleteUser { id_user } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;
            // удалить пользователя
            users::User::delete_user(&mut *trans, *id_user).await? ;
            // выполнить commit в DB
            trans.commit().await? ;

            Ok(local_success("The user has been deleted."))
        },
        Commands::UpdateNameUser { new_name, id_user } => {
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
                    new_name,
                    *id_user
                ).await? ;
            // Выполнить commit
            trans.commit().await? ;

            Ok(local_success("User's name changed successfully."))
        },
    }
}

/// Обработка полученных команд от клиента
pub async fn handle_commmand(
                    State(db_res): State<Arc<Database>>,
                    Json(cmd): Json<args::Commands>
                ) 
                ->  JsonResponse<common::Response> 
                //Json<common::Response>
{
    // Внутренная обработка команд
    match handle_commmand_int(&db_res, &cmd).await {
        Ok(v) => JsonResponse(v),
        Err(err) => JsonResponse(common::Response::Error(err.to_string())),
    } 

    /*
    match cmd {
        Commands::InitDb => {

            match db_res.create_tables().await {
                Ok(_) => JsonResponse(common::Response::Success("Database objects created successfully.".to_owned())),
                Err(err) => JsonResponse(common::Response::Error(err.to_string())),
            }
            //JsonResponse(common::Response::Success("Database objects created successfully.".to_owned()))
            //Json(common::Response::Success("Database objects created successfully.".to_owned()))
        },
        // Создание пользователя        
        Commands::CreateUser { name, email } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    match 
                        db_res
                            .pool
                            // Устанавливает соединение и немедленно начинает новую транзакцию.
                            .begin()
                            .await 
                    {
                      Ok(v) => v,
                      Err(err) => return JsonResponse(common::Response::Error(err.to_string())),
                    } ;

            // вставить нового пользователя
            let new_user = 
                    match users::User::ins_user(
                            &mut *trans,
                            &name,
                            &email
                          )
                          .await {
                        Ok(v) => v,
                        Err(err) => return JsonResponse(common::Response::Error(err.to_string())),
                    } ;
            
            // Добавить для пользователя роль по умолчанию
            if let Err(err) = users_roles::UsersRoles::ins_role_to_user(
                            &mut *trans,
                            new_user.id_user(),
                            roles::SLUG_DEFAULT
                        )
                        .await
            {
                return JsonResponse(common::Response::Error(err.to_string())) ;
            } 

            // выполнить commit в DB
            if let Err(err) = trans.commit().await {
                return JsonResponse(common::Response::Error(err.to_string())) ;
            }

            // println!("User created successfully.") ;                
            JsonResponse(common::Response::Success("User created successfully.".to_owned()))
        },

    }
    */
}

