use anyhow::{
        //Ok,
        Result
    } ;

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
        Commands::UpdateEmailUser { new_email, id_user } => {
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
                new_email,
                *id_user
            )
            .await? ;

            // Выполнить commit
            trans.commit().await? ;

            Ok(local_success("User's email changed successfully."))
        },
        Commands::ShowUsersRoles { id_user } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;
            // проверка кода пользователя
            match id_user {
                Some(u) => {    // код пользователя найден
                    // поиск пользователя
                    let ur = users::UserWithRole::get_data(
                        &mut *trans,
                        *u
                    )
                    .await? ;
                    //print!("{}", ur) ;  // перчать пользователя
                    Ok(common::Response::UserRole(ur))
                },
                None => {   // кода пользователя нет, печать всех пользователей
                    let mut list_ur = vec![] ;

                    for id_user in users::User::get_all_id_user(&mut *trans).await? {
                        /*
                        let ur = 
                            users::UserWithRole::get_data(
                                &mut *trans,
                                id_user
                            )
                            .await? ;
                        print!("{}", ur) ;  // печать пользователя
                        println!("--------------------------------------------") ;
                         */
                        list_ur.push(
                            users::UserWithRole::get_data(
                                &mut *trans,
                                id_user
                            )
                            .await?
                        );
                    }
                    //Ok(local_success("User's email changed successfully."))                                    
                    Ok(common::Response::ListUsersRoles(list_ur))
                }
            }
        },
        Commands::CreateRole { slug, name, permissions } => {
            roles::Role::create_role(&db_res, slug, name, &permissions.join(","))
                .await? ;

            Ok(local_success("Role created successfully."))
        },
        Commands::DeleteRole { slug } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;
            // удалить роль
            roles::Role::delete_role(&mut *trans, slug).await? ;

            // выполнить commit
            trans.commit().await? ;

            Ok(local_success("The role has been successfully added to the user."))
        },
        Commands::UpdateNameRole { new_name, slug } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

            // Модифицировать name в роли
            roles::Role::update_name(&mut *trans, slug, new_name)
                .await? ;

            // Выполнить commit
            trans.commit().await? ;

            Ok(local_success("Role name successfully changed."))
        },
        Commands::UpdatePermissionsRole { slug, new_permissions } => {
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
                        slug, 
                        &new_permissions.join(",")
                    )
                    .await? ;

            // Выполнить commit
            trans.commit().await? ;

            Ok(local_success("Role permissions successfully changed."))
        },
        Commands::ShowRoles { slug } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

            // Проверка кода роли
            match slug {
                Some(slug) => { // код роли найден, печать этой роли
                    /*
                    println!(
                        "{}", 
                        roles::Role::find_slug_raise(
                                &mut *trans,
                                slug,
                                false
                            )
                            .await?                
                    ) ;
                    */
                    Ok(common::Response::Role( 
                        roles::Role::find_slug_raise(
                                &mut *trans,
                                slug,
                                false
                            )
                            .await?
                         ))
                },
                None => {   // код роли не найден, печатьвсех ролей
                    let mut list_roles = vec![] ;

                    for sl in roles::Role::get_all_slugs(&mut *&mut trans).await? {
                        /*
                        print!(
                            "{}", 
                            roles::Role::find_slug_raise(
                                    &mut *trans,
                                    &sl,
                                    false
                            )
                            .await?                
                        ) ;
                        println!("--------------------------------------------") ;
                         */
                        list_roles.push(
                            roles::Role::find_slug_raise(
                                    &mut *trans,
                                    &sl,
                                    false
                            )
                            .await?                            
                        );
                    }
                    Ok(common::Response::ListRoles(list_roles))
                }
            }                    
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

