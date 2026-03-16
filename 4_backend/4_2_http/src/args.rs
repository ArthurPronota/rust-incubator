use clap::{
        Parser,
        Subcommand,
    } ;

use serde::{Serialize, Deserialize};

/// Перечень команд
#[derive(
    Subcommand, 
    //Debug                 // Убрать после отладки
    Serialize,      // Для формирования POST запроса к серверу
    Deserialize,
  )
 ]
pub enum Commands {
    /// Создание необходимых таблиц
    #[clap(
        name = "init-db",
        about = "Creating the required database objects.",
      )
    ]
    InitDb, // serialization -> "InitDb"

    /// Создание пользователя
    #[clap(
        name = "create-user",
        about = "Create a new user",
     )
    ]
    CreateUser {
        #[arg(
            name = "Username",
            help = "User name, not unique",
         )
        ]
        name:   String,

        #[arg(
            name = "Email",
            help = "Email, unique",
         )
        ]
        email:  String,
    },

    /// Удаление пользователя
    #[clap(
        name = "delete-user",
        about = "Delete user",
     )
    ]    
    DeleteUser {
        /// Id user
        #[arg(
            name = "id_user",
            help = "Id of user",
         )
        ]
        id_user:    u32,
    },

    /// Модифицировать имя у пользователю
    #[clap(
        name = "update-name-user",
        about = "Modify user name",
     )
    ]
    UpdateNameUser {
        #[arg(
            name = "New username",
            help = "Modified username",
         )
        ]
        new_name:   String,

        /// Id user
        #[arg(
            name = "id_user",
            help = "Id of user",
         )
        ]        
        id_user:    u32,
    },    
}

/// Реализация трейта Parser
#[derive(
    Parser,
    //Debug,      // Убрать после отладки
  )
 ]
pub struct Args {
    // указание что поле содержит подкоманду
    #[command(subcommand)]
    pub commands: Commands,
}