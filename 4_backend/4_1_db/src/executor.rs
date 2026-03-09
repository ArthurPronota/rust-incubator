use crate::args::{self, Commands} ;
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

    match &arg_in.command {
        Commands::InitDb => {
            db_res
                .create_tables()
                .await ?;
            println!("✅ Tables created successfully.") ;
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
            let new_user_role = 
                        users_roles::UsersRoles::ins_role_to_user(
                            &mut *trans,
                            new_user.id_user(),
                            roles::SLUG_DEFAULT
                        ).await? ;

            trans.commit().await? ;

            println!("✅ User created successfully.") ;                
        },
        Commands::CreateRole { slug, name, permissions } => {
            roles::Role::create_role(&db_res, slug, name, &permissions.join(","))
                .await? ;

            println!("✅ Role created successfully.") ;
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
            let usr_roles = 
                    users_roles::UsersRoles::ins_role_to_user(
                        &mut *trans,
                        *id_user,
                        slug
                    )
                    .await? ;

            println!("✅ The role has been successfully added to the user.") ;
        },
    }

    Ok(())
}