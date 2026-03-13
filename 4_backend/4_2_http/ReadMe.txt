
Структура проекта

client-server/
├── Cargo.toml
├── src/
│   ├── main.rs              # Точка входа (запуск клиента или сервера)
│   ├── client.rs             # CLI клиент
│   ├── server.rs             # HTTP сервер
│   └── common.rs              # Общие типы и функции

-------------------------------------------------------------

 Cargo.toml

 [package]
name = "client-server"
version = "0.1.0"
edition = "2021"

[dependencies]
# Для сервера
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Для клиента
ureq = { version = "2.9", features = ["json"] }

# Общие
anyhow = "1.0"
clap = { version = "4.0", features = ["derive"] }
log = "0.4"
env_logger = "0.11"

# Для работы с датами
chrono = "0.4"

-------------------------------------------------------------

 src/common.rs - Общие типы

 use serde::{Deserialize, Serialize};
use std::fmt;

// Команды, которые клиент отправляет на сервер
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    CreateUser {
        name: String,
        email: String,
    },
    CreateRole {
        slug: String,
        name: String,
        permissions: Vec<String>,
    },
    AssignRole {
        user_id: i32,
        role_slug: String,
    },
}

// Ответы от сервера
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    Success(String),
    UserCreated(User),
    RoleCreated(Role),
    RoleAssigned(UserRole),
    Error(String),
}

// Модели данных
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub slug: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: i32,
    pub role_slug: String,
    pub assigned_at: chrono::DateTime<chrono::Utc>,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Success(msg) => write!(f, "✅ {}", msg),
            Response::UserCreated(user) => write!(f, "✅ User created: {} ({})", user.name, user.email),
            Response::RoleCreated(role) => write!(f, "✅ Role created: {} ({})", role.name, role.slug),
            Response::RoleAssigned(ur) => write!(f, "✅ Role {} assigned to user {}", ur.role_slug, ur.user_id),
            Response::Error(msg) => write!(f, "❌ {}", msg),
        }
    }
}

-------------------------------------------------------------

src/client.rs - Тонкий CLI клиент

use anyhow::Result;
use clap::Parser;
use log::{info, error};
use std::io::{self, Write};

use crate::common::{Command, Response};

// Аргументы командной строки для клиента
#[derive(Parser, Debug)]
#[command(author, version, about = "Thin CLI client for user-role management")]
pub struct ClientArgs {
    /// Адрес сервера (по умолчанию http://localhost:8080)
    #[arg(short, long, default_value = "http://localhost:8080")]
    pub server: String,

    #[command(subcommand)]
    pub command: ClientCommand,
}

// Подкоманды клиента
#[derive(Parser, Debug)]
pub enum ClientCommand {
    /// Создать нового пользователя
    CreateUser {
        name: String,
        email: String,
    },
    
    /// Создать новую роль
    CreateRole {
        slug: String,
        name: String,
        #[arg(short, long, value_delimiter = ',')]
        permissions: Vec<String>,
    },
    
    /// Назначить роль пользователю
    AssignRole {
        #[arg(short, long)]
        user_id: i32,
        #[arg(short, long)]
        role_slug: String,
    },
}

pub struct Client {
    server_url: String,
}

impl Client {
    pub fn new(server_url: String) -> Self {
        Self { server_url }
    }

    pub fn execute_command(&self, cmd: ClientCommand) -> Result<()> {
        // Преобразуем команду CLI в формат для сервера
        let command = match cmd {
            ClientCommand::CreateUser { name, email } => Command::CreateUser { name, email },
            ClientCommand::CreateRole { slug, name, permissions } => {
                Command::CreateRole { slug, name, permissions }
            }
            ClientCommand::AssignRole { user_id, role_slug } => {
                Command::AssignRole { user_id, role_slug }
            }
        };

        // Отправляем команду на сервер
        info!("Sending command to server: {:?}", command);
        
        let response = self.send_command(&command)?;
        
        // Отображаем ответ сервера
        println!("{}", response);
        
        Ok(())
    }

    fn send_command(&self, command: &Command) -> Result<Response> {
        let url = format!("{}/api/command", self.server_url);
        
        // Отправляем POST запрос с JSON
        let response = ureq::post(&url)
            .set("Content-Type", "application/json")
            .send_json(serde_json::to_value(command)?)?;

        // Проверяем статус ответа
        if response.status() != 200 {
            return Err(anyhow::anyhow!(
                "Server error: {} - {}",
                response.status(),
                response.into_string()?
            ));
        }

        // Парсим ответ
        let response: Response = response.into_json()?;
        Ok(response)
    }
}

-------------------------------------------------------------

src/server.rs - HTTP сервер (демон)

use axum::{
    extract::State,
    routing::post,
    Json,
    Router,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use log::{info, error};

use crate::common::*;

// Состояние сервера (база данных в памяти)
#[derive(Clone)]
struct AppState {
    users: Arc<Mutex<HashMap<i32, User>>>,
    roles: Arc<Mutex<HashMap<String, Role>>>,
    user_roles: Arc<Mutex<Vec<UserRole>>>,
    next_user_id: Arc<Mutex<i32>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            roles: Arc::new(Mutex::new(HashMap::new())),
            user_roles: Arc::new(Mutex::new(Vec::new())),
            next_user_id: Arc::new(Mutex::new(1)),
        }
    }

    // Создание пользователя
    fn create_user(&self, name: String, email: String) -> User {
        let mut users = self.users.lock().unwrap();
        let mut next_id = self.next_user_id.lock().unwrap();
        
        let user = User {
            id: *next_id,
            name,
            email,
            created_at: chrono::Utc::now(),
        };
        
        users.insert(*next_id, user.clone());
        *next_id += 1;
        
        user
    }

    // Создание роли
    fn create_role(&self, slug: String, name: String, permissions: Vec<String>) -> Result<Role, String> {
        let mut roles = self.roles.lock().unwrap();
        
        if roles.contains_key(&slug) {
            return Err(format!("Role with slug '{}' already exists", slug));
        }
        
        let role = Role {
            slug: slug.clone(),
            name,
            permissions,
            created_at: chrono::Utc::now(),
        };
        
        roles.insert(slug, role.clone());
        Ok(role)
    }

    // Назначение роли пользователю
    fn assign_role(&self, user_id: i32, role_slug: String) -> Result<UserRole, String> {
        // Проверяем существование пользователя
        let users = self.users.lock().unwrap();
        if !users.contains_key(&user_id) {
            return Err(format!("User with id {} not found", user_id));
        }
        
        // Проверяем существование роли
        let roles = self.roles.lock().unwrap();
        if !roles.contains_key(&role_slug) {
            return Err(format!("Role with slug '{}' not found", role_slug));
        }
        
        // Проверяем, не назначена ли уже роль
        let mut user_roles = self.user_roles.lock().unwrap();
        if user_roles.iter().any(|ur| ur.user_id == user_id && ur.role_slug == role_slug) {
            return Err(format!("Role '{}' already assigned to user {}", role_slug, user_id));
        }
        
        let user_role = UserRole {
            user_id,
            role_slug: role_slug.clone(),
            assigned_at: chrono::Utc::now(),
        };
        
        user_roles.push(user_role.clone());
        Ok(user_role)
    }

    // Обработка команды
    fn handle_command(&self, command: Command) -> Response {
        match command {
            Command::CreateUser { name, email } => {
                info!("Creating user: {} ({})", name, email);
                let user = self.create_user(name, email);
                Response::UserCreated(user)
            }
            
            Command::CreateRole { slug, name, permissions } => {
                info!("Creating role: {} ({})", name, slug);
                match self.create_role(slug, name, permissions) {
                    Ok(role) => Response::RoleCreated(role),
                    Err(e) => Response::Error(e),
                }
            }
            
            Command::AssignRole { user_id, role_slug } => {
                info!("Assigning role {} to user {}", role_slug, user_id);
                match self.assign_role(user_id, role_slug) {
                    Ok(ur) => Response::RoleAssigned(ur),
                    Err(e) => Response::Error(e),
                }
            }
        }
    }
}

// Обработчик HTTP POST запросов
async fn handle_command(
    State(state): State<AppState>,
    Json(command): Json<Command>,
) -> Json<Response> {
    info!("Received command: {:?}", command);
    let response = state.handle_command(command);
    Json(response)
}

// Запуск сервера
pub async fn run_server(port: u16) -> Result<(), anyhow::Error> {
    let state = AppState::new();
    
    let app = Router::new()
        .route("/api/command", post(handle_command))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    
    info!("Server running on http://{}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

-------------------------------------------------------------

src/main.rs - Точка входа

mod common;
mod client;
mod server;

use clap::{Parser, Subcommand};
use log::info;
use std::process;

// Главные аргументы командной строки
#[derive(Parser)]
#[command(author, version, about = "User-Role Management System")]
struct Cli {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand)]
enum Mode {
    /// Запустить в режиме клиента
    Client(client::ClientArgs),
    
    /// Запустить в режиме сервера
    Server {
        /// Порт для сервера
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.mode {
        Mode::Client(args) => {
            info!("Starting in client mode");
            let client = client::Client::new(args.server);
            
            if let Err(e) = client.execute_command(args.command) {
                eprintln!("Client error: {}", e);
                process::exit(1);
            }
        }
        
        Mode::Server { port } => {
            info!("Starting in server mode on port {}", port);
            if let Err(e) = server::run_server(port).await {
                eprintln!("Server error: {}", e);
                process::exit(1);
            }
        }
    }
    
    Ok(())
}

-------------------------------------------------------------

1. Запуск сервера:

# В одном терминале
RUST_LOG=info cargo run -- server --port 8080

2. Запуск клиента (в другом терминале):

# Создание пользователя
RUST_LOG=info cargo run -- client --server http://localhost:8080 create-user "Иван Петров" ivan@example.com

# Создание роли
RUST_LOG=info cargo run -- client --server http://localhost:8080 create-roles admin "Администратор" --permissions read,write,delete

# Назначение роли пользователю
RUST_LOG=info cargo run -- client --server http://localhost:8080 assign-role --user-id 1 --role-slug admin

-------------------------------------------------------------

Примеры вывода

# Создание пользователя
✅ User created: Иван Петров (ivan@example.com)

# Создание роли
✅ Role created: Администратор (admin)

# Назначение роли
✅ Role admin assigned to user 1

# Ошибка (если роль уже назначена)
❌ Role 'admin' already assigned to user 1

-------------------------------------------------------------

🔑 Ключевые особенности
Тонкий клиент: только отправка команд и отображение ответов

Сервер-демон: вся бизнес-логика на сервере

Единая конечная точка: /api/command для всех операций

JSON сериализация: обмен данными через JSON

Многопоточность: сервер обрабатывает множество клиентов

-------------------------------------------------------------

📊 Архитектура

┌─────────┐     Команда (JSON)     ┌─────────┐
│ Клиент  │ ────────────────────── │ Сервер  │
│ (ureq)  │ ◀───────────────────── │ (axum)  │
└─────────┘     Ответ (JSON)       └─────────┘
      │                                  │
      │ 1. Парсит CLI аргументы          │ 1. Принимает команду
      │ 2. Отправляет на сервер          │ 2. Выполняет бизнес-логику
      │ 3. Выводит ответ                 │ 3. Возвращает результат


==========================================================

📁 Структура проекта с несколькими бинарниками

Вариант 1: Бинарники в src/bin/ (рекомендуемый)

my_project/
├── Cargo.toml
└── src/
    ├── bin/
    │   ├── client.rs      # первый бинарник
    │   └── server.rs      # второй бинарник
    └── lib.rs             # общий код (опционально)

Вариант 2: Явное указание в Cargo.toml

my_project/
├── Cargo.toml
└── src/
    ├── bin/
    │   ├── client.rs
    │   └── server.rs
    └── lib.rs

📄 Настройка Cargo.toml

[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

# Автоматически найдет все файлы в src/bin/
# Никаких дополнительных настроек не нужно!

[dependencies]
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
# ... другие зависимости

📝 Примеры файлов

src/lib.rs - общий код

// Общие типы и функции для обоих бинарников
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    CreateUser { name: String, email: String },
    CreateRole { slug: String, name: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Success(String),
    Error(String),
}

pub fn common_function() {
    println!("Это общая функция");
}

src/bin/client.rs - первый бинарник

use anyhow::Result;
use my_project::{Command, Response};  // импорт из lib.rs

fn main() -> Result<()> {
    println!("Клиент запущен");
    
    // Используем общий код
    let cmd = Command::CreateUser {
        name: "Иван".to_string(),
        email: "ivan@example.com".to_string(),
    };
    
    println!("Отправляем команду: {:?}", cmd);
    
    Ok(())
}

src/bin/server.rs - второй бинарник

use anyhow::Result;
use my_project::{Command, Response};  // импорт из lib.rs

fn main() -> Result<()> {
    println!("Сервер запущен");
    
    // Используем общий код
    let response = Response::Success("OK".to_string());
    
    println!("Ответ: {:?}", response);
    
    Ok(())
}

🚀 Запуск бинарников

# Запуск клиента
cargo run --bin client

# Запуск сервера
cargo run --bin server

# Сборка всех бинарников
cargo build

# Сборка конкретного бинарника
cargo build --bin client

# Запуск с аргументами
cargo run --bin client -- --help

📋 Вариант с явным указанием в Cargo.toml

[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

# Явное указание бинарников
[[bin]]
name = "client"
path = "src/client_main.rs"

[[bin]]
name = "server"
path = "src/server_main.rs"

[dependencies]
anyhow = "1.0"

📁 Структура для явного указания

my_project/
├── Cargo.toml
└── src/
    ├── client_main.rs     # точка входа клиента
    ├── server_main.rs     # точка входа сервера
    └── lib.rs             # общий код

🔧 Для вашего клиент-серверного проекта

[package]
name = "user_role_system"
version = "0.1.0"
edition = "2021"

# Автоматически найдет bin/client.rs и bin/server.rs
# Ничего писать не нужно!

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
ureq = { version = "2.9", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
clap = { version = "4.0", features = ["derive"] }
log = "0.4"
env_logger = "0.11"

📁 Структура для вашего случая

user_role_system/
├── Cargo.toml
└── src/
    ├── bin/
    │   ├── client.rs      # CLI клиент
    │   └── server.rs      # HTTP сервер
    ├── lib.rs             # общие типы (Command, Response)
    ├── common.rs          # дополнительные общие функции
    ├── db.rs              # работа с БД (для сервера)
    └── models.rs          # модели данных

📝 lib.rs для общего кода

// src/lib.rs
pub mod common;
pub mod models;

pub use common::*;
pub use models::*;

🎯 Преимущества подхода

Разделение кода - бинарники используют общий код из lib.rs

Быстрая компиляция - общий код компилируется один раз

Тестирование - можно тестировать общий код отдельно

Гибкость - легко добавлять новые бинарники

Чистота - точка входа отделена от логики

📊 Сравнение подходов

Подход	        Преимущества	                        Недостатки
--------------- --------------------------------------- ----------------------------
src/bin/	    Автоматическое обнаружение, простота	Меньше контроля над именами
Явное указание	Полный контроль, любые пути	            Больше настроек в Cargo.toml

🚀 Запуск вашего проекта

# Запуск сервера
RUST_LOG=info cargo run --bin server -- --port 8080

# Запуск клиента
RUST_LOG=info cargo run --bin client -- --server http://localhost:8080 create-user "Иван" ivan@example.com

# Сборка обоих бинарников для продакшена
cargo build --release
# ./target/release/server
# ./target/release/client

