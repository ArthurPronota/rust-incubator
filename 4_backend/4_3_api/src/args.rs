use clap::{
        Parser,
        Subcommand,
} ;

use serde::{
        Serialize,
        Deserialize
    } ;

 use utoipa::ToSchema;

#[derive(
    Serialize,
    Deserialize,
    Parser,
    ToSchema,
    Clone,
    Debug,
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
    CreateUser 
    /*
    {
        #[arg(
            name = "Username",
            help = "User name, not unique",
         )
        ]
        name:     String,
        #[arg(
            name = "Email",
            help = "Email, unique",
          )
        ]
        email:    String,
    }
    */
    (CreateUser),

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