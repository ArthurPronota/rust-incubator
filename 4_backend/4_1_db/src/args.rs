use clap::{
    Parser,         // Трейт для парсинга аргументов в структуру
    Subcommand      // Трейт для создания вложенных команд
} ;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Создание необхожимых таблиц
    #[clap(
        name = "init-db",
        about = "Creating the necessary tables.",
     )
    ]
    InitDb,

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

    /// Создать роль
    #[clap(
        name = "create-role",
        about = "Create a new role",
     )
    ] 
    CreateRole {
        /// Slug: name-of-role
        #[arg(
            name = "slug",
            help = "Slug string of role, unique",
         )
        ]
        slug:   String,

        /// Текстовое наисеноывание роли
        #[arg(
            name = "name",
            help = "Description of the role of the slug",
         )
        ]
        name:   String,

        /// Разрешения включенную в эту роль, разделённых запятой: r,i,d,u
        #[arg(
            name = "permissions",
            help = "The permissions for this role are comma separated.",
            value_delimiter = ',',
         )
        ]        
        permissions:    Vec<String>,
    },
}

// Автоматически реализует трейт Parser
#[derive(Parser, Debug)]
pub struct Args {
    // Указывает, что это поле содержит подкоманду
    #[command(subcommand)]
    pub command: Commands,  // Поле для хранения выбранной команды
}

