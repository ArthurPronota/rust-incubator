use crate::args::{self, Commands} ;
use crate::db ;
use crate::users ;
use crate::roles ;

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
            users::User::create_user(&db_res, name, email)
                .await? ;
            println!("✅ User created successfully.") ;
        },
        Commands::CreateRole { slug, name, permissions } => {
            roles::Role::create_role(&db_res, slug, name, &permissions.join(","))
                .await? ;
            println!("✅ Role created successfully.") ;
        },
    }

    Ok(())
}