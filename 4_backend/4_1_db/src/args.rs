use clap::{
    Parser,         // Трейт для парсинга аргументов в структуру
    Subcommand      // Трейт для создания вложенных команд
} ;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Создание необхожимых таблиц
    #[clap(
        name = "init-db",
        about = "Creating the required database objects.",
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

    /// Модифицировать email у пользователю
    #[clap(
        name = "update-email-user",
        about = "Modify email name",
     )
    ]
    UpdateEmailUser {
        #[arg(
            name = "New user email",
            help = "Modified user email",
         )
        ]
        new_email:  String,

        /// Id user
        #[arg(
            name = "id_user",
            help = "Id of user",
         )
        ]
        id_user:    u32,        
    },


    /// Показать пользователей и из роли
    #[clap(
        name = "show-users-roles",
        about = "Show users and their roles",
     )
    ]
    ShowUsersRoles {
        /// Id user
        #[arg(
            name = "id_user",
            help = "Id of user, optional",
         )
        ]
        id_user:    Option<u32>
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

    /// Удалить роль
    #[clap(
        name = "delete-role",
        about = "Delete role",
     )
    ]
    DeleteRole {
        /// Slug: name-of-role
        #[arg(
            name = "slug",
            help = "Slug string of role",
         )
        ]
        slug:   String,
    },

    /// Модифицировать имя у роли
    #[clap(
        name = "update-name-role",
        about = "Modify role name",
     )
    ]
    UpdateNameRole {
        /// Наименование роли
        #[arg(
            name = "New role name",
            help = "Modified role name",
         )
        ]
        new_name:   String,

        /// Код роли
        #[arg(
            name = "slug",
            help = "Slug string of role",
         )
        ]
        slug:       String,
    },

    /// Модифицировать разрешения у роли
    #[clap(
        name = "update-perm-role",
        about = "Modify role permissions",
     )
    ]    
    UpdatePermissionsRole {
        /// Код роли
        #[arg(
            name = "slug",
            help = "Slug string of role",
         )
        ]
        slug:       String,

        /// Разрешения включенную в эту роль, разделённых запятой: r,i,d,u
        /// Значения типа Vec<_> должны быть в конце списка аргументов иначе возникает ошибка.
        #[arg(
            name = "new-permissions",
            help = "The new permissions for this role are comma separated.",
            value_delimiter = ',',
         )
        ]        
        new_permissions:    Vec<String>,
    },

    /// Показать роли
    #[clap(
        name = "show-roles",
        about = "Show roles",
     )
    ]    
    ShowRoles {
        /// Slug: name-of-role
        #[arg(
            name = "slug",
            help = "Slug string of role, optional",
         )
        ]
        slug:   Option<String>,
    },

    /// Добвить роль к пользователю
    #[clap(
        name = "role-to-user",
        about = "Add a role to a user",
     )
    ]
    AddRoleToUser {
        /// Код роли
        #[arg(
            name = "slug",
            help = "Slug string of role",
         )
        ]
        slug:       String,

        /// Id user
        #[arg(
            name = "id_user",
            help = "Id of user",
         )
        ]
        id_user:    u32,
    },

    /// Удалить роль у пользователю
    #[clap(
        name = "remove-user-role",
        about = "Remove a role from a user",
     )
    ]
    RemoveRoleFromUser {
        /// Код роли
        #[arg(
            name = "slug",
            help = "Slug string of role",
         )
        ]
        slug:       String,

        /// Id user
        #[arg(
            name = "id_user",
            help = "Id of user",
         )
        ]        
        id_user:    u32,
    },    
}

// Автоматически реализует трейт Parser
#[derive(Parser, Debug)]
pub struct Args {
    // Указывает, что это поле содержит подкоманду
    #[command(subcommand)]
    pub command: Commands,  // Поле для хранения выбранной команды
}

