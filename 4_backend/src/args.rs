use clap::{
        Parser,
        Subcommand,
} ;

use serde::{
        Serialize,
        Deserialize
} ;

/// Перечень команд
#[derive(
    Subcommand,
    //Debug,
    Serialize,
    Deserialize,
 )
]
pub enum Commands {
    #[clap(
        name = "user-register",
        about = "Register a new user",
    )]
    UserRegister {
        /// Тмя нового пользователя
        #[arg(
            name = "user name",
            help = "Username",
         )
        ]
        name:       String,
        #[arg(
            name = "User password",
            help = "User password",
         )
        ]
        password:   String,
    },
}

/// Агрументы командной строки
#[derive(
    Parser,
    //Debug,
  )
]
pub struct Args {
    #[command(subcommand)]
    pub commands: Commands,
}