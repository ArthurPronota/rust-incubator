1. Cargo.toml

[package]
name = "user_role_cli"
version = "0.1.0"
edition = "2021"

[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "mysql", "macros"] }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
chrono = "0.4"
dotenv = "0.15"
thiserror = "1.0"
tabled = "0.15"

[dev-dependencies]
rand = "0.8"

// ------------------------------

2. .env файл

DATABASE_URL=mysql://root:password@localhost:3306/user_role_db

// ------------------------------

3. src/main.rs

mod cli;
mod db;
mod error;
mod models;

use clap::Parser;
use cli::{Cli, Commands};
use db::Database;
use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let db = Database::new(&db_url).await?;
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::UserCreate { name, email, role_slugs } => {
            let user = db.create_user(&name, &email, &role_slugs).await?;
            println!("✅ User created: {:?}", user);
        }
        Commands::UserDelete { id } => {
            db.delete_user(id).await?;
            println!("✅ User {} deleted", id);
        }
        Commands::UserUpdate { id, name, email } => {
            let user = db.update_user(id, name.as_deref(), email.as_deref()).await?;
            println!("✅ User updated: {:?}", user);
        }
        Commands::UserList { id } => {
            if let Some(id) = id {
                if let Some(user) = db.get_user_with_roles(id).await? {
                    println!("{}", user);
                } else {
                    println!("User {} not found", id);
                }
            } else {
                let users = db.list_users_with_roles().await?;
                for user in users {
                    println!("{}", user);
                    println!("---");
                }
            }
        }
        Commands::RoleCreate { slug, name, permissions } => {
            let role = db.create_role(&slug, &name, &permissions).await?;
            println!("✅ Role created: {:?}", role);
        }
        Commands::RoleDelete { slug } => {
            db.delete_role(&slug).await?;
            println!("✅ Role {} deleted", slug);
        }
        Commands::RoleUpdate { slug, name, permissions } => {
            let role = db.update_role(&slug, name.as_deref(), permissions.as_deref()).await?;
            println!("✅ Role updated: {:?}", role);
        }
        Commands::RoleList { slug } => {
            if let Some(slug) = slug {
                if let Some(role) = db.get_role(&slug).await? {
                    println!("{}", role);
                } else {
                    println!("Role {} not found", slug);
                }
            } else {
                let roles = db.list_roles().await?;
                for role in roles {
                    println!("{}", role);
                }
            }
        }
        Commands::AssignRole { user_id, role_slug } => {
            db.assign_role_to_user(user_id, &role_slug).await?;
            println!("✅ Role {} assigned to user {}", role_slug, user_id);
        }
        Commands::UnassignRole { user_id, role_slug } => {
            db.unassign_role_from_user(user_id, &role_slug).await?;
            println!("✅ Role {} unassigned from user {}", role_slug, user_id);
        }
        Commands::InitDb => {
            db.create_tables().await?;
            println!("✅ Database initialized");
        }
    }
    
    Ok(())
}

// ------------------------------

4. src/cli.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize database tables
    InitDb,
    
    /// Create a new user
    UserCreate {
        name: String,
        email: String,
        #[arg(short, long, value_delimiter = ',')]
        role_slugs: Vec<String>,
    },
    
    /// Delete a user
    UserDelete {
        id: i32,
    },
    
    /// Update a user
    UserUpdate {
        id: i32,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        email: Option<String>,
    },
    
    /// List users (with optional ID)
    UserList {
        #[arg(short, long)]
        id: Option<i32>,
    },
    
    /// Create a new role
    RoleCreate {
        slug: String,
        name: String,
        permissions: String, // JSON string
    },
    
    /// Delete a role
    RoleDelete {
        slug: String,
    },
    
    /// Update a role
    RoleUpdate {
        slug: String,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        permissions: Option<String>,
    },
    
    /// List roles (with optional slug)
    RoleList {
        #[arg(short, long)]
        slug: Option<String>,
    },
    
    /// Assign role to user
    AssignRole {
        user_id: i32,
        role_slug: String,
    },
    
    /// Unassign role from user
    UnassignRole {
        user_id: i32,
        role_slug: String,
    },
}

// ------------------------------

5. src/models.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub slug: String,
    pub name: String,
    pub permissions: String, // JSON string
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserRole {
    pub user_id: i32,
    pub role_slug: String,
    pub assigned_at: DateTime<Utc>,
}

// For displaying users with their roles
#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithRoles {
    pub user: User,
    pub roles: Vec<Role>,
}

impl fmt::Display for UserWithRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "User #{}: {} ({})", self.user.id, self.user.name, self.user.email)?;
        writeln!(f, "Created: {}", self.user.createdat.format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(f, "Roles:")?;
        
        if self.roles.is_empty() {
            writeln!(f, "  No roles assigned")?;
        } else {
            for role in &self.roles {
                writeln!(f, "  - {} ({})", role.name, role.slug)?;
                writeln!(f, "    Permissions: {}", role.permissions)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Role: {} (slug: {})\nPermissions: {}\nCreated: {}",
            self.name,
            self.slug,
            self.permissions,
            self.created_at.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

#[derive(Tabled)]
pub struct RoleTable {
    pub slug: String,
    pub name: String,
    pub permissions: String,
}

// ------------------------------

6. src/error.rs

use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
}

// ------------------------------

7. src/db.rs

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use crate::models::{Role, User, UserRole, UserWithRoles};
use crate::error::{Result, AppError};
use chrono::Utc;

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        Ok(Self { pool })
    }
    
    pub async fn create_tables(&self) -> Result<()> {
        // Create users table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        // Create roles table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS roles (
                slug VARCHAR(100) PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                permissions JSON NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        // Create users_roles junction table with foreign keys
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users_roles (
                user_id INT NOT NULL,
                role_slug VARCHAR(100) NOT NULL,
                assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (user_id, role_slug),
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
                FOREIGN KEY (role_slug) REFERENCES roles(slug) ON DELETE CASCADE
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // User operations
    pub async fn create_user(&self, name: &str, email: &str, role_slugs: &[String]) -> Result<UserWithRoles> {
        if role_slugs.is_empty() {
            return Err(AppError::InvalidInput("User must have at least one role".to_string()));
        }
        
        // Verify all roles exist
        for slug in role_slugs {
            if !self.role_exists(slug).await? {
                return Err(AppError::NotFound(format!("Role {} not found", slug)));
            }
        }
        
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        // Insert user
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (name, email, created_at, updated_at)
            VALUES (?, ?, ?, ?)
            RETURNING id, name, email, created_at, updated_at
            "#
        )
        .bind(name)
        .bind(email)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(&mut *tx)
        .await?;
        
        // Assign roles
        for slug in role_slugs {
            sqlx::query(
                r#"
                INSERT INTO users_roles (user_id, role_slug, assigned_at)
                VALUES (?, ?, ?)
                "#
            )
            .bind(user.id)
            .bind(slug)
            .bind(Utc::now())
            .execute(&mut *tx)
            .await?;
        }
        
        // Commit transaction
        tx.commit().await?;
        
        // Get user with roles
        self.get_user_with_roles(user.id).await?
            .ok_or_else(|| AppError::NotFound("User not found after creation".to_string()))
    }
    
    pub async fn delete_user(&self, user_id: i32) -> Result<()> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("User {} not found", user_id)));
        }
        
        Ok(())
    }
    
    pub async fn update_user(&self, user_id: i32, name: Option<&str>, email: Option<&str>) -> Result<User> {
        // Get current user data
        let current = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        
        let current = match current {
            Some(u) => u,
            None => return Err(AppError::NotFound(format!("User {} not found", user_id))),
        };
        
        // Update with new values or keep old ones
        let new_name = name.unwrap_or(&current.name);
        let new_email = email.unwrap_or(&current.email);
        
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users 
            SET name = ?, email = ?, updated_at = ?
            WHERE id = ?
            RETURNING id, name, email, created_at, updated_at
            "#
        )
        .bind(new_name)
        .bind(new_email)
        .bind(Utc::now())
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn get_user_with_roles(&self, user_id: i32) -> Result<Option<UserWithRoles>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        
        let user = match user {
            Some(u) => u,
            None => return Ok(None),
        };
        
        let roles = sqlx::query_as::<_, Role>(
            r#"
            SELECT r.* FROM roles r
            JOIN users_roles ur ON r.slug = ur.role_slug
            WHERE ur.user_id = ?
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(Some(UserWithRoles { user, roles }))
    }
    
    pub async fn list_users_with_roles(&self) -> Result<Vec<UserWithRoles>> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY id")
            .fetch_all(&self.pool)
            .await?;
        
        let mut result = Vec::new();
        
        for user in users {
            let roles = sqlx::query_as::<_, Role>(
                r#"
                SELECT r.* FROM roles r
                JOIN users_roles ur ON r.slug = ur.role_slug
                WHERE ur.user_id = ?
                "#
            )
            .bind(user.id)
            .fetch_all(&self.pool)
            .await?;
            
            result.push(UserWithRoles { user, roles });
        }
        
        Ok(result)
    }
    
    // Role operations
    pub async fn create_role(&self, slug: &str, name: &str, permissions: &str) -> Result<Role> {
        // Validate JSON
        if let Err(e) = serde_json::from_str::<serde_json::Value>(permissions) {
            return Err(AppError::InvalidInput(format!("Invalid JSON: {}", e)));
        }
        
        let role = sqlx::query_as::<_, Role>(
            r#"
            INSERT INTO roles (slug, name, permissions, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            RETURNING slug, name, permissions, created_at, updated_at
            "#
        )
        .bind(slug)
        .bind(name)
        .bind(permissions)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;
        
        Ok(role)
    }
    
    pub async fn delete_role(&self, slug: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM roles WHERE slug = ?")
            .bind(slug)
            .execute(&self.pool)
            .await?;
        
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Role {} not found", slug)));
        }
        
        Ok(())
    }
    
    pub async fn update_role(&self, slug: &str, name: Option<&str>, permissions: Option<&str>) -> Result<Role> {
        // Get current role data
        let current = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE slug = ?")
            .bind(slug)
            .fetch_optional(&self.pool)
            .await?;
        
        let current = match current {
            Some(r) => r,
            None => return Err(AppError::NotFound(format!("Role {} not found", slug))),
        };
        
        // Validate JSON if provided
        if let Some(perms) = permissions {
            if let Err(e) = serde_json::from_str::<serde_json::Value>(perms) {
                return Err(AppError::InvalidInput(format!("Invalid JSON: {}", e)));
            }
        }
        
        let new_name = name.unwrap_or(&current.name);
        let new_permissions = permissions.unwrap_or(&current.permissions);
        
        let role = sqlx::query_as::<_, Role>(
            r#"
            UPDATE roles 
            SET name = ?, permissions = ?, updated_at = ?
            WHERE slug = ?
            RETURNING slug, name, permissions, created_at, updated_at
            "#
        )
        .bind(new_name)
        .bind(new_permissions)
        .bind(Utc::now())
        .bind(slug)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(role)
    }
    
    pub async fn get_role(&self, slug: &str) -> Result<Option<Role>> {
        let role = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE slug = ?")
            .bind(slug)
            .fetch_optional(&self.pool)
            .await?;
        
        Ok(role)
    }
    
    pub async fn list_roles(&self) -> Result<Vec<Role>> {
        let roles = sqlx::query_as::<_, Role>("SELECT * FROM roles ORDER BY slug")
            .fetch_all(&self.pool)
            .await?;
        
        Ok(roles)
    }
    
    pub async fn role_exists(&self, slug: &str) -> Result<bool> {
        let result = sqlx::query("SELECT 1 FROM roles WHERE slug = ?")
            .bind(slug)
            .fetch_optional(&self.pool)
            .await?;
        
        Ok(result.is_some())
    }
    
    // User-Role assignment operations
    pub async fn assign_role_to_user(&self, user_id: i32, role_slug: &str) -> Result<()> {
        // Check if user exists
        let user_exists = sqlx::query("SELECT 1 FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        
        if user_exists.is_none() {
            return Err(AppError::NotFound(format!("User {} not found", user_id)));
        }
        
        // Check if role exists
        let role_exists = sqlx::query("SELECT 1 FROM roles WHERE slug = ?")
            .bind(role_slug)
            .fetch_optional(&self.pool)
            .await?;
        
        if role_exists.is_none() {
            return Err(AppError::NotFound(format!("Role {} not found", role_slug)));
        }
        
        // Check if already assigned
        let assigned = sqlx::query("SELECT 1 FROM users_roles WHERE user_id = ? AND role_slug = ?")
            .bind(user_id)
            .bind(role_slug)
            .fetch_optional(&self.pool)
            .await?;
        
        if assigned.is_some() {
            return Ok(()); // Already assigned, do nothing
        }
        
        // Assign role
        sqlx::query(
            r#"
            INSERT INTO users_roles (user_id, role_slug, assigned_at)
            VALUES (?, ?, ?)
            "#
        )
        .bind(user_id)
        .bind(role_slug)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn unassign_role_from_user(&self, user_id: i32, role_slug: &str) -> Result<()> {
        let result = sqlx::query(
            "DELETE FROM users_roles WHERE user_id = ? AND role_slug = ?"
        )
        .bind(user_id)
        .bind(role_slug)
        .execute(&self.pool)
        .await?;
        
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                format!("Assignment between user {} and role {} not found", user_id, role_slug)
            ));
        }
        
        Ok(())
    }
}

// ------------------------------

8. Пример использования

    Инициализация базы данных

# Сначала создайте базу данных в MySQL
mysql -u root -p -e "CREATE DATABASE user_role_db;"

# Затем инициализируйте таблицы
cargo run -- init-db

    Создание ролей

# Создание роли admin
cargo run -- role-create admin Administrator '{"permissions": ["read", "write", "delete"]}'

# Создание роли user
cargo run -- role-create user "Regular User" '{"permissions": ["read"]}'

# Создание роли moderator
cargo run -- role-create moderator Moderator '{"permissions": ["read", "moderate"]}'

    Создание пользователей

# Создание пользователя с ролью admin
cargo run -- user-create "John Doe" john@example.com --role-slugs admin

# Создание пользователя с несколькими ролями
cargo run -- user-create "Jane Smith" jane@example.com --role-slugs user,moderator

# Попытка создать пользователя без роли (выдаст ошибку)
cargo run -- user-create "Bob Wilson" bob@example.com --role-slugs

    Просмотр списков

# Список всех пользователей с их ролями
cargo run -- user-list

# Просмотр конкретного пользователя
cargo run -- user-list --id 1

# Список всех ролей
cargo run -- role-list

# Просмотр конкретной роли
cargo run -- role-list --slug admin

    Обновление данных

# Обновление пользователя
cargo run -- user-update 1 --name "John Updated" --email john.new@example.com

# Обновление роли
cargo run -- role-update admin --name "Super Admin" --permissions '{"permissions": ["read", "write", "delete", "admin"]}'

    Управление назначением ролей

# Назначить роль пользователю
cargo run -- assign-role 1 user

# Удалить роль у пользователя
cargo run -- unassign-role 1 moderator

# Проверить результат
cargo run -- user-list --id 1

    Удаление

# Удаление пользователя
cargo run -- user-delete 3

# Удаление роли (только если она не используется)
cargo run -- role-delete user

    Ключевые особенности реализации

1. Согласованность данных: Используются внешние ключи с ON DELETE CASCADE для автоматического удаления связей при удалении пользователя или роли .

2. Транзакции: При создании пользователя используется транзакция, чтобы гарантировать, что пользователь и его роли создаются атомарно .

3. Валидация: Проверяется существование ролей перед назначением и корректность JSON для permissions .

4. Уникальность: Email пользователя уникален, роль использует slug как первичный ключ .

5. CRUD операции: Полная поддержка всех CRUD операций для обеих таблиц с проверками целостности .

// ------------------------------

    MySql v8.4.3

https://metanit.com/sql/mysql/

Путь к папке, где будут располагаться базы данных:
C:\ProgramData\MySQL\MySQL Server 8.4\

Tcp/Ip
Port:   3306
X protocol Port:   33060

My User:
arthur

Windows Service name:   MySQL84

MySQL Shell:
C:\Program Files\MySQL\MySQL Shell 8.4\

\sql
\connect arthur@localhost:3306

 MySQL  localhost:3306 ssl  SQL > show databases ;
+--------------------+
| Database           |
+--------------------+
| 4_1_db             |
| information_schema |
| mysql              |
| performance_schema |
| sys                |
+--------------------+
5 rows in set (0.0014 sec)

Вывести данные вертикально:
show INDEX FROM users_roles \G
