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

/// Модифицировать email у пользователя
#[derive(
    Serialize,
    Deserialize,
    Parser,
    ToSchema,
  )
]
pub struct UpdateEmailUser {
    #[arg(
        name = "New user email",
        help = "Modified user email",
      )
    ]
    pub new_email:  String,

    /// Id user
    #[arg(
        name = "id_user",
        help = "Id of user",
     )
    ]
    pub id_user:    u32,        
}


/// Показать пользователей и их роли
#[derive(
    Serialize,
    Deserialize,
    Parser,
    ToSchema,
  )
]
pub struct ShowUsersRoles {
    /// Id user
    #[arg(
        name = "id_user",
        help = "Id of user, optional",
     )
    ]
    id_user:    Option<u32>
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

    /// Модифицировать email у пользователя
    #[clap(
        name = "update-email-user",
        about = "Modify email name",
     )
    ]
    UpdateEmailUser(UpdateEmailUser),

    /// Показать пользователей и их роли
    #[clap(
        name = "show-users-roles",
        about = "Show users and their roles",
     )
    ]
    ShowUsersRoles(ShowUsersRoles)
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