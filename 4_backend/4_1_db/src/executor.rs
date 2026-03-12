use crate::args::{
        self,       // Импортирует сам модуль args
        Commands    // Импортирует структуры Commands из модуля args
    } ;
use crate::db ;     // Импортирует модуль db
use crate::users ;  // Импортирует модуль users
use crate::roles ;  // Импортирует модуль roles
use crate::users_roles ;    // Импортирует модуль users_roles

use anyhow::Result ;    // Импортирует тип Result из крейта anyhow

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
        // Создание пользователя
        Commands::CreateUser { name, email } => {
            // Сформировать новую транзакцию
            let mut trans = 
                    db_res
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;
            // вставить нового пользователя
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

            // выполнить commit в DB
            trans.commit().await? ;

            println!("User created successfully.") ;                
        },
        // Удаление пользователя
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

            println!("The user has been deleted.") ;
        },
        // Показать пользователей и их роли
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
                    print!("{}", ur) ;  // перчать пользователя
                },
                None => {   // кода пользователя нет, печать всех пользователей
                    for id_user in users::User::get_all_id_user(&mut *trans).await? {
                        let ur = 
                            users::UserWithRole::get_data(
                                &mut *trans,
                                id_user
                            )
                            .await? ;
                        print!("{}", ur) ;  // печать пользователя
                        println!("--------------------------------------------") ;
                    }

                }
            }
        },
        // Создать роль
        Commands::CreateRole { slug, name, permissions } => {
            roles::Role::create_role(&db_res, slug, name, &permissions.join(","))
                .await? ;

            println!("Role created successfully.") ;
        },
        // Удалить роль
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

            println!("The role has been successfully added to the user.") ;                                
        },
        // Показать роль
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
                    println!(
                        "{}", 
                        roles::Role::find_slug_raise(
                                &mut *trans,
                                slug,
                                false
                            )
                            .await?                
                    ) ;
                },
                None => {   // код роли не найден, печатьвсех ролей
                    for sl in roles::Role::get_all_slugs(&mut *&mut trans).await? {
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
                    }
                }
            }                    
        },
        // Добавить роль к пользователю
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
            // выполнить commit
            trans.commit().await? ;

            println!("The role has been successfully added to the user.") ;
        },
        // Удалить роль у пользователя
        Commands::RemoveRoleFromUser { slug, id_user } => {
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
                    *id_user,
                    slug
                ).await? ;
            // выпонить commit
            trans.commit().await? ;

            println!("The role has been successfully removed from the user.") ;
        },
        // Модифицировать имя у пользователя
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

            println!("User's name changed successfully.") ;
        },
        // Модифицировать email у пользователю
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

            println!("User's email changed successfully.") ;                      
        },
        // Модифицировать наименование роли
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

            println!("Role name successfully changed.") ;
        },
        // Модифицировать разрешения у роли
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

            println!("Role permissions successfully changed.") ;
        }
    }

    Ok(())
}