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
    pub id_user:    Option<u32>
}


/// Создать роль
#[derive(
    Serialize,
    Deserialize,
    Parser,
    ToSchema,
  )
]
pub struct  CreateRole {
    /// Slug: name-of-role
    #[arg(
        name = "slug",
        help = "Slug string of role, unique",
      )
    ]
    pub slug:   String,

    /// Текстовое наименоывание роли
    #[arg(
        name = "name",
        help = "Description of the role of the slug",
      )
    ]
    pub name:   String,

    // Разрешения включенную в эту роль, разделённых запятой: r,i,d,u
    #[arg(
        name = "permissions",
        help = "The permissions for this role are comma separated.",
        value_delimiter = ',',
     )
    ]        
    pub permissions:    Vec<String>,
}

/// Удалить роль
#[derive(
    Serialize,
    Deserialize,
    Parser,
    ToSchema,
  )
]
pub struct DeleteRole {
    /// Slug: name-of-role
    #[arg(
        name = "slug",
        help = "Slug string of role",
     )
    ]
    pub slug:   String,
}

/// Модифицировать имя у роли
#[derive(
    Serialize,
    Deserialize,
    Parser,
    ToSchema,
  )
]
pub struct UpdateNameRole {
    /// Наименование роли
    #[arg(
        name = "New role name",
        help = "Modified role name",
     )
    ]
    pub new_name:   String,

    /// Код роли
    #[arg(
        name = "slug",
        help = "Slug string of role",
      )
    ]
    pub slug:       String,
}

// Модифицировать разрешения у роли
#[derive(
    Serialize,
    Deserialize,
    Parser,
    ToSchema,
  )
]
pub struct UpdatePermissionsRole {
    // Код роли
    #[arg(
        name = "slug",
        help = "Slug string of role",
      )
    ]
    pub slug:       String,

    // Разрешения включенную в эту роль, разделённых запятой: r,i,d,u
    // Значения типа Vec<_> должны быть в конце списка аргументов иначе возникает ошибка.
    #[arg(
        name = "new-permissions",
        help = "The new permissions for this role are comma separated.",
        value_delimiter = ',',
      )
    ]        
    pub new_permissions:    Vec<String>,
} 

// Показать роли
#[derive(
    Serialize,
    Deserialize,
    Parser,
    ToSchema,
  )
]
pub struct ShowRoles {
    // Slug: name-of-role
    #[arg(
        name = "slug",
        help = "Slug string of role, optional",
      )
    ]
    pub slug:   Option<String>,
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
    ShowUsersRoles(ShowUsersRoles),

    /// Создать роль
    #[clap(
        name = "create-role",
        about = "Create a new role",
     )
    ]
    CreateRole(CreateRole),

    /// Удалить роль
    #[clap(
        name = "delete-role",
        about = "Delete role",
     )
    ]
    DeleteRole(DeleteRole),

    // Модифицировать имя у роли
    #[clap(
        name = "update-name-role",
        about = "Modify role name",
     )
    ]
    UpdateNameRole(UpdateNameRole),

    // Модифицировать разрешения у роли
    #[clap(
        name = "update-perm-role",
        about = "Modify role permissions",
     )
    ]
    UpdatePermissionsRole(UpdatePermissionsRole),

    // Показать роли
    #[clap(
        name = "show-roles",
        about = "Show roles",
     )
    ]
    ShowRoles(ShowRoles),
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