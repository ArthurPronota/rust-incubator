use clap::{
        Parser,
        Subcommand,
} ;

use serde::{
        Serialize,
        Deserialize
    } ;

use utoipa::ToSchema;

// Структура создания пользователя
#[derive(
    Serialize,
    Deserialize,
    Parser,
    ToSchema,
  )
]
pub struct CreateUser {
    #[arg(
        name = "Username",
        help = "User name, not unique",
     )
    ]
    pub name:     String,

    #[arg(
        name = "Email",
        help = "Email, unique",
     )
    ]
    pub email:    String,
}

// Структура удаления пользователя
#[derive(
    Serialize,
    Deserialize,
    Parser,
    ToSchema,
  )
]
pub struct DeleteUser {
    /// Id user
    #[arg(
        name = "id_user",
        help = "Id of user",
     )
    ]
    pub id_user:    u32,
}


/// Модифицировать имя у пользователю
#[derive(
    Serialize,
    Deserialize,
    Parser,
    ToSchema,
  )
]
pub struct UpdateNameUser {
    #[arg(
        name = "New username",
        help = "Modified username",
     )
    ]
    pub new_name:   String,

    /// Id user
    #[arg(
        name = "id_user",
        help = "Id of user",
     )
    ]        
    pub id_user:    u32,
}



// Перечень команд
#[derive(
    Subcommand,
    Serialize,
    Deserialize,
    //Debug,
)]
pub enum Command {
    
    /// Создание необходимых объектов DB
    #[clap(
        name = "init-db",
        about = "Creating the required database objects.",
     )
    ]
    InitDb,

    /// Создать пользователя
    #[clap(
        name = "create-user",
        about = "Create a new user",
      )
    ]
    CreateUser(CreateUser),
    
    /// Удаление пользователя
    #[clap(
        name = "delete-user",
        about = "Delete user",
     )
    ]   
    DeleteUser(DeleteUser),

    /// Модифицировать имя у пользователя
    #[clap(
        name = "update-name-user",
        about = "Modify user name",
     )
    ]
    UpdateNameUser(UpdateNameUser),
}

// Структура с агрументами CLI
#[derive(
   Parser,
  )
 ]
pub struct Args {
    #[command(subcommand)]
    pub commands: Command,
}