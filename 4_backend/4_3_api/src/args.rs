use clap::{
        Parser,
        Subcommand,
} ;

use serde::{
        Serialize,
        Deserialize
    } ;

// Перечень команд
#[derive(
    Serialize,
    Deserialize,
    Subcommand,
)]
pub enum Command {
    
    /// Создание необходимых объектов DB
    #[clap(
        name = "init-db",
        about = "Creating the required database objects.",
     )
    ]
    InitDb,

}

// Структура с агрументами
#[derive(
   Parser
  )
 ]
pub struct Args {
    #[command(subcommand)]
    pub commands: Command,
}