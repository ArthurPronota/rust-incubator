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