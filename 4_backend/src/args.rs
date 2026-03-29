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
        /// Имя нового пользователя
        #[arg(
            name = "user name",
            help = "Username",
         )
        ]
        name:       String,

        /// Пароль пользователя        
        #[arg(
            name = "User password",
            help = "User password",
         )
        ]
        password:   String,
    },

    /// Залогироваться как пользователь
    #[clap(
        name = "login",
        about = "Log in as a user",
    )]
    Login {
        /// Имя пользователя
        #[arg(
            name = "user name",
            help = "Username",
         )
        ]
        name:       String,

        /// Пароль пользователя        
        #[arg(
            name = "User password",
            help = "User password",
         )
        ]
        password:   String,
    }
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