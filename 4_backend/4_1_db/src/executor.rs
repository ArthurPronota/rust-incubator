use crate::args::{
        self,
        Commands
    } ;
use crate::db ;
use crate::users ;
use crate::roles ;
use crate::users_roles ;

use anyhow::Result ;

/// Выполнить действие из аргументов командной строки
pub async fn any_command(arg_in: &args::Args, db_path_conn: &str) ->Result<()>{
    
    // Подключение и создание ресурса базы данных
    let db_res = 
            db::Database::new(db_path_conn) 
            .await ?;

    // Обработка команд из CLI
    match &arg_in.command {
        // Инициализация DB
        Commands::InitDb => {
            db_res
                .create_tables()
                .await ?;
            println!("Database objects created successfully.") ;
        },
        Commands::CreateUser { name, email } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

            let new_user = users::User::ins_user(
                                    &mut *trans,
                                    name,
                                    email
                                 )
                                 .await? ;

            // Добавить для пользователя роль по умолчанию
            users_roles::UsersRoles::ins_role_to_user(
                            &mut *trans,
                            new_user.id_user(),
                            roles::SLUG_DEFAULT
                        ).await? ;

            trans.commit().await? ;

            println!("User created successfully.") ;                
        },
        Commands::DeleteUser { id_user } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

            users::User::delete_user(&mut *trans, *id_user).await? ;
                      
            trans.commit().await? ;

            println!("The user has been deleted.") ;
        },
        Commands::ShowUsersRoles { id_user } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

            match id_user {
                Some(u) => {
                    let ur = users::UserWithRole::get_data(
                        &mut *trans,
                        *u
                    )
                    .await? ;
                    print!("{}", ur) ;
                },
                None => {
                    for id_user in users::User::get_all_id_user(&mut *trans).await? {
                        let ur = 
                            users::UserWithRole::get_data(
                                &mut *trans,
                                id_user
                            )
                            .await? ;
                        print!("{}", ur) ;
                        println!("--------------------------------------------") ;
                    }

                }
            }
        },
        Commands::CreateRole { slug, name, permissions } => {
            roles::Role::create_role(&db_res, slug, name, &permissions.join(","))
                .await? ;

            println!("Role created successfully.") ;
        },
        Commands::DeleteRole { slug } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;
            
            roles::Role::delete_role(&mut *trans, slug).await? ;

            trans.commit().await? ;

            println!("The role has been successfully added to the user.") ;                                
        },
        Commands::ShowRoles { slug } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

            match slug {
                Some(slug) => {
                    println!(
                        "{}", 
                        roles::Role::find_slug_raise(
                                &mut *trans,
                                slug
                            )
                            .await?                
                    ) ;
                },
                None => {
                    for sl in roles::Role::get_all_slugs(&mut *&mut trans).await? {
                        print!(
                            "{}", 
                            roles::Role::find_slug_raise(
                                    &mut *trans,
                                    &sl
                            )
                            .await?                
                        ) ;
                        println!("--------------------------------------------") ;
                    }
                }
            }                    
        },
        Commands::AddRoleToUser { slug, id_user } => {
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
                        *id_user,
                        slug
                    )
                    .await? ;

            trans.commit().await? ;

            println!("The role has been successfully added to the user.") ;
        },
        Commands::RemoveRoleFromUser { slug, id_user } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

            users_roles::UsersRoles::del_role_from_user(
                    &mut *trans,
                    *id_user,
                    slug
                ).await? ;

            trans.commit().await? ;

            println!("The role has been successfully removed from the user.") ;
        },
        Commands::UpdateNameUser { new_name, id_user } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

            users::User::update_name(
                    &mut *trans,
                    new_name,
                    *id_user
                ).await? ;

            trans.commit().await? ;

            println!("User's name changed successfully.") ;
        },
        Commands::UpdateEmailUser { new_email, id_user } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

            users::User::update_email(
                &mut *trans,
                new_email,
                *id_user
            )
            .await? ;

            trans.commit().await? ;

            println!("User's email changed successfully.") ;                      
        },
        Commands::UpdateNameRole { new_name, slug } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

            roles::Role::update_name(&mut *trans, slug, new_name)
                .await? ;

            trans.commit().await? ;

            println!("Role name successfully changed.") ;
        },
        Commands::UpdatePermissionsRole { slug, new_permissions } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

            roles::Role::update_permissions(
                        &mut *trans,
                        slug, 
                        &new_permissions.join(",")
                    )
                    .await? ;

            trans.commit().await? ;

            println!("Role permissions successfully changed.") ;
        }
    }

    Ok(())
}