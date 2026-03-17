
📁 Структура проекта

rest_api_project/
├── Cargo.toml
├── src/
│   ├── main.rs              # Точка входа (клиент)
│   ├── bin/
│   │   └── server.rs         # Отдельный бинарник сервера
│   └── common.rs              # Общие типы

------------------------------------------------------------------

📦 Cargo.toml

[package]
name = "rest_api_project"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "server"
path = "src/bin/server.rs"

[dependencies]
# HTTP сервер
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }

# HTTP клиент
reqwest = { version = "0.11", features = ["json"] }

# Сериализация
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# CLI
clap = { version = "4.0", features = ["derive"] }

# OpenAPI
utoipa = { version = "4.0", features = ["axum"] }
utoipa-swagger-ui = { version = "4.0", features = ["axum"] }

# Логирование
env_logger = "0.11"
log = "0.4"

# Обработка ошибок
anyhow = "1.0"

# Для дат
chrono = "0.4"

------------------------------------------------------------------

📄 src/common.rs - Общие типы

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub slug: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub slug: String,
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleRequest {
    pub user_id: i32,
    pub role_slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: i32,
    pub role_slug: String,
    pub assigned_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

------------------------------------------------------------------

📄 src/bin/server.rs - RESTful API сервер

use axum::{
    Router,
    routing::{get, post},
    Json,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::{Arc, Mutex};
use chrono::Utc;
use log::info;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use rest_api_project::common::*;

// Состояние сервера (in-memory база данных)
#[derive(Clone)]
struct AppState {
    users: Arc<Mutex<Vec<User>>>,
    roles: Arc<Mutex<Vec<Role>>>,
    user_roles: Arc<Mutex<Vec<UserRole>>>,
    next_user_id: Arc<Mutex<i32>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
            roles: Arc::new(Mutex::new(Vec::new())),
            user_roles: Arc::new(Mutex::new(Vec::new())),
            next_user_id: Arc::new(Mutex::new(1)),
        }
    }
}

// OpenAPI документация
#[derive(OpenApi)]
#[openapi(
    paths(
        create_user,
        get_users,
        get_user,
        create_role,
        get_roles,
        get_role,
        assign_role_to_user,
        get_user_roles,
    ),
    components(
        schemas(
            User, CreateUserRequest, ApiResponse<User>,
            Role, CreateRoleRequest, ApiResponse<Role>,
            AssignRoleRequest, UserRole, ApiResponse<UserRole>,
            ApiResponse<Vec<User>>, ApiResponse<Vec<Role>>, ApiResponse<Vec<UserRole>>
        )
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "roles", description = "Role management endpoints"),
        (name = "assignments", description = "User-role assignment endpoints"),
    ),
    info(
        title = "User Role Management API",
        version = "1.0.0",
        description = "RESTful API for managing users and roles",
    )
)]
struct ApiDoc;

// ==================== Обработчики для пользователей ====================

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = ApiResponse<User>),
        (status = 400, description = "Invalid request", body = ApiResponse<String>),
    ),
    tag = "users"
)]
async fn create_user(
    state: axum::extract::State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> impl IntoResponse {
    info!("Creating user: {} ({})", req.name, req.email);
    
    let mut users = state.users.lock().unwrap();
    let mut next_id = state.next_user_id.lock().unwrap();
    
    let user = User {
        id: *next_id,
        name: req.name,
        email: req.email,
        created_at: Utc::now(),
    };
    
    users.push(user.clone());
    *next_id += 1;
    
    (StatusCode::CREATED, Json(ApiResponse::success(user)))
}

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of users", body = ApiResponse<Vec<User>>),
    ),
    tag = "users"
)]
async fn get_users(state: axum::extract::State<AppState>) -> Json<ApiResponse<Vec<User>>> {
    let users = state.users.lock().unwrap().clone();
    Json(ApiResponse::success(users))
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = ApiResponse<User>),
        (status = 404, description = "User not found", body = ApiResponse<String>),
    ),
    tag = "users"
)]
async fn get_user(
    state: axum::extract::State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let users = state.users.lock().unwrap();
    
    match users.iter().find(|u| u.id == id) {
        Some(user) => (StatusCode::OK, Json(ApiResponse::success(user.clone()))),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(format!("User {} not found", id))),
        ),
    }
}

// ==================== Обработчики для ролей ====================

#[utoipa::path(
    post,
    path = "/api/roles",
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Role created successfully", body = ApiResponse<Role>),
        (status = 400, description = "Invalid request or duplicate slug", body = ApiResponse<String>),
    ),
    tag = "roles"
)]
async fn create_role(
    state: axum::extract::State<AppState>,
    Json(req): Json<CreateRoleRequest>,
) -> impl IntoResponse {
    info!("Creating role: {} ({})", req.name, req.slug);
    
    let mut roles = state.roles.lock().unwrap();
    
    // Проверка на дубликат slug
    if roles.iter().any(|r| r.slug == req.slug) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(format!("Role with slug {} already exists", req.slug))),
        );
    }
    
    let role = Role {
        slug: req.slug,
        name: req.name,
        permissions: req.permissions,
        created_at: Utc::now(),
    };
    
    roles.push(role.clone());
    (StatusCode::CREATED, Json(ApiResponse::success(role)))
}

#[utoipa::path(
    get,
    path = "/api/roles",
    responses(
        (status = 200, description = "List of roles", body = ApiResponse<Vec<Role>>),
    ),
    tag = "roles"
)]
async fn get_roles(state: axum::extract::State<AppState>) -> Json<ApiResponse<Vec<Role>>> {
    let roles = state.roles.lock().unwrap().clone();
    Json(ApiResponse::success(roles))
}

#[utoipa::path(
    get,
    path = "/api/roles/{slug}",
    params(
        ("slug" = String, Path, description = "Role slug")
    ),
    responses(
        (status = 200, description = "Role found", body = ApiResponse<Role>),
        (status = 404, description = "Role not found", body = ApiResponse<String>),
    ),
    tag = "roles"
)]
async fn get_role(
    state: axum::extract::State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let roles = state.roles.lock().unwrap();
    
    match roles.iter().find(|r| r.slug == slug) {
        Some(role) => (StatusCode::OK, Json(ApiResponse::success(role.clone()))),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(format!("Role {} not found", slug))),
        ),
    }
}

// ==================== Обработчики для назначений ====================

#[utoipa::path(
    post,
    path = "/api/assignments",
    request_body = AssignRoleRequest,
    responses(
        (status = 201, description = "Role assigned successfully", body = ApiResponse<UserRole>),
        (status = 400, description = "Invalid request", body = ApiResponse<String>),
        (status = 404, description = "User or role not found", body = ApiResponse<String>),
    ),
    tag = "assignments"
)]
async fn assign_role_to_user(
    state: axum::extract::State<AppState>,
    Json(req): Json<AssignRoleRequest>,
) -> impl IntoResponse {
    info!("Assigning role {} to user {}", req.role_slug, req.user_id);
    
    // Проверка существования пользователя
    let users = state.users.lock().unwrap();
    if !users.iter().any(|u| u.id == req.user_id) {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(format!("User {} not found", req.user_id))),
        );
    }
    
    // Проверка существования роли
    let roles = state.roles.lock().unwrap();
    if !roles.iter().any(|r| r.slug == req.role_slug) {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(format!("Role {} not found", req.role_slug))),
        );
    }
    
    // Проверка дубликата
    let mut user_roles = state.user_roles.lock().unwrap();
    if user_roles.iter().any(|ur| ur.user_id == req.user_id && ur.role_slug == req.role_slug) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Role already assigned to this user".to_string())),
        );
    }
    
    let user_role = UserRole {
        user_id: req.user_id,
        role_slug: req.role_slug,
        assigned_at: Utc::now(),
    };
    
    user_roles.push(user_role.clone());
    (StatusCode::CREATED, Json(ApiResponse::success(user_role)))
}

#[utoipa::path(
    get,
    path = "/api/users/{id}/roles",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User's roles", body = ApiResponse<Vec<Role>>),
        (status = 404, description = "User not found", body = ApiResponse<String>),
    ),
    tag = "assignments"
)]
async fn get_user_roles(
    state: axum::extract::State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    // Проверка существования пользователя
    let users = state.users.lock().unwrap();
    if !users.iter().any(|u| u.id == id) {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(format!("User {} not found", id))),
        );
    }
    
    let user_roles = state.user_roles.lock().unwrap();
    let roles = state.roles.lock().unwrap();
    
    let user_role_slugs: Vec<String> = user_roles
        .iter()
        .filter(|ur| ur.user_id == id)
        .map(|ur| ur.role_slug.clone())
        .collect();
    
    let user_roles_full: Vec<Role> = roles
        .iter()
        .filter(|r| user_role_slugs.contains(&r.slug))
        .cloned()
        .collect();
    
    (StatusCode::OK, Json(ApiResponse::success(user_roles_full)))
}

// ==================== Запуск сервера ====================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let state = AppState::new();
    
    // Создаем OpenAPI документацию
    let openapi = ApiDoc::openapi();
    
    // Сохраняем OpenAPI схему в файл
    std::fs::write(
        "openapi.json",
        serde_json::to_string_pretty(&openapi)?,
    )?;
    println!("✅ OpenAPI schema saved to openapi.json");
    
    // Создаем роутер с Swagger UI
    let app = Router::new()
        .route("/api/users", post(create_user).get(get_users))
        .route("/api/users/:id", get(get_user))
        .route("/api/roles", post(create_role).get(get_roles))
        .route("/api/roles/:slug", get(get_role))
        .route("/api/assignments", post(assign_role_to_user))
        .route("/api/users/:id/roles", get(get_user_roles))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
        .with_state(state);
    
    let addr = "127.0.0.1:8080";
    println!("🚀 Server running on http://{}", addr);
    println!("📚 OpenAPI documentation: http://{}/docs", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

------------------------------------------------------------------

📄 src/main.rs - "Толстый" CLI клиент

use clap::{Parser, Subcommand};
use reqwest::Client;
use anyhow::{Result, Context};
use log::info;
use rest_api_project::common::*;

#[derive(Parser)]
#[command(author, version, about = "User Role Management CLI")]
struct Cli {
    /// API server URL
    #[arg(short, long, default_value = "http://127.0.0.1:8080")]
    server: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new user
    CreateUser {
        name: String,
        email: String,
    },
    
    /// List all users
    ListUsers,
    
    /// Get user by ID
    GetUser {
        id: i32,
    },
    
    /// Create a new role
    CreateRole {
        slug: String,
        name: String,
        #[arg(short, long, value_delimiter = ',')]
        permissions: Vec<String>,
    },
    
    /// List all roles
    ListRoles,
    
    /// Get role by slug
    GetRole {
        slug: String,
    },
    
    /// Assign role to user
    AssignRole {
        user_id: i32,
        role_slug: String,
    },
    
    /// Get user's roles
    GetUserRoles {
        user_id: i32,
    },
}

struct ApiClient {
    base_url: String,
    http_client: Client,
}

impl ApiClient {
    fn new(base_url: String) -> Self {
        Self {
            base_url,
            http_client: Client::new(),
        }
    }
    
    async fn create_user(&self, name: &str, email: &str) -> Result<User> {
        let url = format!("{}/api/users", self.base_url);
        let req = CreateUserRequest {
            name: name.to_string(),
            email: email.to_string(),
        };
        
        let resp = self.http_client
            .post(&url)
            .json(&req)
            .send()
            .await
            .context("Failed to send request")?;
        
        self.handle_response(resp).await
    }
    
    async fn get_users(&self) -> Result<Vec<User>> {
        let url = format!("{}/api/users", self.base_url);
        let resp = self.http_client.get(&url).send().await?;
        self.handle_response(resp).await
    }
    
    async fn get_user(&self, id: i32) -> Result<User> {
        let url = format!("{}/api/users/{}", self.base_url, id);
        let resp = self.http_client.get(&url).send().await?;
        self.handle_response(resp).await
    }
    
    async fn create_role(&self, slug: &str, name: &str, permissions: Vec<String>) -> Result<Role> {
        let url = format!("{}/api/roles", self.base_url);
        let req = CreateRoleRequest {
            slug: slug.to_string(),
            name: name.to_string(),
            permissions,
        };
        
        let resp = self.http_client
            .post(&url)
            .json(&req)
            .send()
            .await?;
        
        self.handle_response(resp).await
    }
    
    async fn get_roles(&self) -> Result<Vec<Role>> {
        let url = format!("{}/api/roles", self.base_url);
        let resp = self.http_client.get(&url).send().await?;
        self.handle_response(resp).await
    }
    
    async fn get_role(&self, slug: &str) -> Result<Role> {
        let url = format!("{}/api/roles/{}", self.base_url, slug);
        let resp = self.http_client.get(&url).send().await?;
        self.handle_response(resp).await
    }
    
    async fn assign_role(&self, user_id: i32, role_slug: &str) -> Result<UserRole> {
        let url = format!("{}/api/assignments", self.base_url);
        let req = AssignRoleRequest {
            user_id,
            role_slug: role_slug.to_string(),
        };
        
        let resp = self.http_client
            .post(&url)
            .json(&req)
            .send()
            .await?;
        
        self.handle_response(resp).await
    }
    
    async fn get_user_roles(&self, user_id: i32) -> Result<Vec<Role>> {
        let url = format!("{}/api/users/{}/roles", self.base_url, user_id);
        let resp = self.http_client.get(&url).send().await?;
        self.handle_response(resp).await
    }
    
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T> {
        let status = resp.status();
        let text = resp.text().await?;
        
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }
        
        let api_resp: ApiResponse<T> = serde_json::from_str(&text)
            .context("Failed to parse response")?;
        
        if api_resp.success {
            api_resp.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("API error: {}", api_resp.error.unwrap_or_default()))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    let client = ApiClient::new(cli.server);
    
    match cli.command {
        Commands::CreateUser { name, email } => {
            info!("Creating user: {} ({})", name, email);
            let user = client.create_user(&name, &email).await?;
            println!("✅ User created: {:?}", user);
        }
        
        Commands::ListUsers => {
            info!("Listing all users");
            let users = client.get_users().await?;
            println!("📋 Users ({}):", users.len());
            for user in users {
                println!("  • {}: {} ({})", user.id, user.name, user.email);
            }
        }
        
        Commands::GetUser { id } => {
            info!("Getting user {}", id);
            let user = client.get_user(id).await?;
            println!("✅ User found: {:?}", user);
        }
        
        Commands::CreateRole { slug, name, permissions } => {
            info!("Creating role: {} ({})", name, slug);
            let role = client.create_role(&slug, &name, permissions).await?;
            println!("✅ Role created: {:?}", role);
        }
        
        Commands::ListRoles => {
            info!("Listing all roles");
            let roles = client.get_roles().await?;
            println!("📋 Roles ({}):", roles.len());
            for role in roles {
                println!("  • {}: {} (permissions: {:?})", role.slug, role.name, role.permissions);
            }
        }
        
        Commands::GetRole { slug } => {
            info!("Getting role {}", slug);
            let role = client.get_role(&slug).await?;
            println!("✅ Role found: {:?}", role);
        }
        
        Commands::AssignRole { user_id, role_slug } => {
            info!("Assigning role {} to user {}", role_slug, user_id);
            let assignment = client.assign_role(user_id, &role_slug).await?;
            println!("✅ Role assigned: {:?}", assignment);
        }
        
        Commands::GetUserRoles { user_id } => {
            info!("Getting roles for user {}", user_id);
            let roles = client.get_user_roles(user_id).await?;
            println!("📋 User {} roles ({}):", user_id, roles.len());
            for role in roles {
                println!("  • {}: {} (permissions: {:?})", role.slug, role.name, role.permissions);
            }
        }
    }
    
    Ok(())
}

------------------------------------------------------------------

🚀 Запуск и использование

1. Запуск сервера

```bash
RUST_LOG=info cargo run --bin server
```

2. Работа через CLI клиента

```basg
# Создание пользователя
cargo run -- create-user --name "Иван Петров" --email ivan@example.com

# Создание роли
cargo run -- create-role --slug admin --name "Администратор" --permissions read,write,delete

# Назначение роли пользователю
cargo run -- assign-role --user-id 1 --role-slug admin

# Просмотр пользователей
cargo run -- list-users

# Просмотр ролей пользователя
cargo run -- get-user-roles --user-id 1
```

3. Работа через curl (прямой доступ к API)

```bash
# Создание пользователя
curl -X POST http://127.0.0.1:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Мария Сидорова","email":"maria@example.com"}'

# Создание роли
curl -X POST http://127.0.0.1:8080/api/roles \
  -H "Content-Type: application/json" \
  -d '{"slug":"editor","name":"Редактор","permissions":["read","write"]}'

# Назначение роли
curl -X POST http://127.0.0.1:8080/api/assignments \
  -H "Content-Type: application/json" \
  -d '{"user_id":2,"role_slug":"editor"}'

# Получение списка пользователей
curl http://127.0.0.1:8080/api/users

# Получение ролей пользователя
curl http://127.0.0.1:8080/api/users/2/roles
```

4. OpenAPI документация

1. JSON схема: http://127.0.0.1:8080/api-docs/openapi.json
2. Swagger UI: http://127.0.0.1:8080/docs
3. Локальный файл: openapi.json в корне проекта

------------------------------------------------------------------

📊 Структура API

Метод	Endpoint	            Описание
------- ----------------------- -----------------------
POST	/api/users	            Создание пользователя
GET	    /api/users	            Список пользователей
GET	    /api/users/{id}	        Получение пользователя
POST	/api/roles	            Создание роли
GET	    /api/roles	            Список ролей
GET	    /api/roles/{slug}	    Получение роли
POST	/api/assignments	    Назначение роли
GET	    /api/users/{id}/roles	Роли пользователя

------------------------------------------------------------------

🎯 Особенности реализации

1. RESTful API: Отдельные эндпоинты для каждой операции
2. Толстый клиент: CLI сам анализирует команды и отправляет точные запросы
3. OpenAPI: Автоматическая генерация документации
4. Swagger UI: Интерактивная документация по адресу /docs
5. cURL доступ: Все операции можно выполнять напрямую через API
6. In-memory БД: Для простоты данные хранятся в памяти

====================================================================

🤔 Почему этот клиент считается "толстым"?

Клиент называется "толстым" (fat client/thick client) потому что он 
содержит значительную часть логики обработки и принятия решений, 
в отличие от "тонкого клиента", который только передаёт команды серверу.

🔍 Сравнение с "тонким" клиентом

Тонкий клиент (из предыдущего примера)

// Тонкий клиент - только отправка команд "как есть"
match cmd {
    Commands::CreateUser { name, email } => {
        // Просто отправляет команду на сервер
        client.send_command(Command::CreateUser { name, email }).await?
    }
}
// Вся логика на сервере


Наш "толстый" клиент

// Толстый клиент - сам решает, какие запросы отправлять
match cli.command {
    Commands::CreateUser { name, email } => {
        // Клиент ЗНАЕТ:
        // - какой эндпоинт использовать (/api/users)
        // - какой HTTP метод (POST)
        // - как сформировать JSON
        // - как обработать ответ
        
        let url = format!("{}/api/users", self.base_url);
        let req = CreateUserRequest { name, email };
        
        // Клиент сам строит запрос
        let resp = self.http_client
            .post(&url)           // POST метод
            .json(&req)            // JSON формат
            .send()
            .await?;
        
        // Клиент сам обрабатывает ответ
        self.handle_response(resp).await
    }
}

📊 Критерии "толстого" клиента

Аспект	            Тонкий клиент	            Толстый клиент (наш)
------------------- --------------------------- -------------------------------
Знание API	        Только одна конечная точка	Знает все эндпоинты
HTTP методы	        Только POST	                GET, POST и др.
URL построение	    Одинаковый для всех команд	Разные для каждой операции
Обработка ответов	Просто выводит	            Парсит, валидирует, форматирует
Бизнес-логика	    На сервере	                Распределена

🏋️ Что делает наш клиент "толстым"?

1. Знает структуру API

// Клиент знает все эндпоинты
let url = match command {
    CreateUser => format!("{}/api/users", self.base_url),
    GetUser(id) => format!("{}/api/users/{}", self.base_url, id),
    CreateRole => format!("{}/api/roles", self.base_url),
    AssignRole => format!("{}/api/assignments", self.base_url),
};

2. Использует правильные HTTP методы

// Клиент выбирает метод
match method {
    "GET"  => self.http_client.get(&url),
    "POST" => self.http_client.post(&url),
    "PUT"  => self.http_client.put(&url),
    "DELETE" => self.http_client.delete(&url),
}

3. Формирует специфичные для каждого запроса JSON

// Разные структуры для разных запросов
let json = match command {
    CreateUser => serde_json::to_string(&CreateUserRequest { name, email }),
    CreateRole => serde_json::to_string(&CreateRoleRequest { slug, name, permissions }),
    AssignRole => serde_json::to_string(&AssignRoleRequest { user_id, role_slug }),
};

4. Обрабатывает ответы по-разному

// Клиент по-разному обрабатывает ответы
match command {
    ListUsers => {
        let users: Vec<User> = self.handle_response(resp).await?;
        // Форматирует вывод для пользователей
        for user in users {
            println!("  • {}: {} ({})", user.id, user.name, user.email);
        }
    }
    GetUserRoles { user_id } => {
        let roles: Vec<Role> = self.handle_response(resp).await?;
        // Форматирует вывод для ролей
        println!("📋 User {} roles:", user_id);
        for role in roles {
            println!("  • {}: {}", role.slug, role.name);
        }
    }
}

5. Содержит логику валидации

// Клиент проверяет данные перед отправкой
if name.is_empty() {
    return Err(anyhow::anyhow!("Name cannot be empty"));
}

if !email.contains('@') {
    return Err(anyhow::anyhow!("Invalid email format"));
}

📈 Преимущества толстого клиента

Преимущество	            Объяснение
--------------------------- -------------------------------------------
Меньше нагрузки на сервер	Часть логики выполняется на клиенте
Лучшая производительность	Меньше запросов к серверу
Работа офлайн	            Можно кэшировать данные
Богатый UI/UX	            Более сложный интерфейс
Гибкость	                Клиент может адаптироваться под разные API

🎯 Итог

1. Наш клиент считается толстым, потому что он:
2. Знает структуру API (разные эндпоинты)
3. Принимает решения (какой метод использовать)
4. Формирует запросы (строит URL, JSON)
5. Обрабатывает ответы (парсит, валидирует, форматирует)
6. Содержит логику (проверки, преобразования)

В отличие от тонкого клиента, который просто передаёт команды на 
единственный эндпоинт, толстый клиент активно участвует в коммуникации 
с сервером и обработке данных.

==========================================================================

📊 Соответствие HTTP методов и операций


HTTP метод	REST операция	CRUD	    Когда использовать
----------- --------------- ----------- -------------------------------------------
GET	        Read	        Чтение	    Получение данных (списки, отдельные записи)
POST	    Create	        Создание	Создание новых ресурсов
PUT	        Update/Replace	Обновление	Полное обновление существующего ресурса
DELETE	    Delete	        Удаление	Удаление ресурса

🎯 Конкретные примеры из нашего кода

1. GET - получение данных

// Получение списка пользователей
let url = "http://server/api/users";
self.http_client.get(&url)  // GET запрос

// Получение конкретного пользователя
let url = "http://server/api/users/42";
self.http_client.get(&url)  // GET запрос

2. POST - создание новых ресурсов

// Создание нового пользователя
let url = "http://server/api/users";
self.http_client.post(&url)  // POST запрос
    .json(&CreateUserRequest { name, email })
    
// Создание новой роли
let url = "http://server/api/roles";
self.http_client.post(&url)  // POST запрос
    .json(&CreateRoleRequest { slug, name, permissions })

3. PUT - обновление существующих ресурсов

// Полное обновление пользователя
let url = "http://server/api/users/42";
self.http_client.put(&url)  // PUT запрос
    .json(&UpdateUserRequest { name, email })

4. DELETE - удаление ресурсов

// Удаление пользователя
let url = "http://server/api/users/42";
self.http_client.delete(&url)  // DELETE запрос

📝 Почему это важно в REST API?

Идемпотентность методов

Метод	Идемпотентность*	Безопасность**
------- ------------------- ---------------
GET	    ✅ Да	          ✅ Да
POST	❌ Нет	          ❌ Нет
PUT	    ✅ Да	          ❌ Нет
DELETE	✅ Да	          ❌ Нет

Пример работы методов

// GET - просто читает, безопасно вызывать много раз
for _ in 0..5 {
    client.get_user(42).await?;  // Сервер не меняется
}

// POST - создаёт новый ресурс каждый раз
for _ in 0..5 {
    client.create_user("Иван").await?;  // Создаст 5 пользователей!
}

// PUT - обновляет, можно вызывать много раз с тем же результатом
for _ in 0..5 {
    client.update_user(42, "Иван").await?;  // Всё равно один пользователь с именем Иван
}

🎯 В контексте нашего API

// Клиент сам выбирает метод в зависимости от команды
match command {
    // Получение данных - GET
    Commands::ListUsers => self.http_client.get(&url),
    Commands::GetUser { id } => self.http_client.get(&url),
    Commands::ListRoles => self.http_client.get(&url),
    Commands::GetUserRoles { .. } => self.http_client.get(&url),
    
    // Создание - POST
    Commands::CreateUser { .. } => self.http_client.post(&url),
    Commands::CreateRole { .. } => self.http_client.post(&url),
    Commands::AssignRole { .. } => self.http_client.post(&url),
    
    // Обновление - PUT (если бы было)
    // Commands::UpdateUser { .. } => self.http_client.put(&url),
    
    // Удаление - DELETE (если бы было)
    // Commands::DeleteUser { .. } => self.http_client.delete(&url),
}

📋 Почему это делает клиент "толстым"?

Потому что клиент знает семантику каждой операции:

1. Какая операция требует GET (чтение)
2. Какая требует POST (создание)
3. Какая требует PUT (обновление)
4. Какая требует DELETE (удаление)

Тонкий клиент просто шлёт всё на один эндпоинт методом POST, 
а толстый клиент использует правильные HTTP методы для каждой операции, 
следуя REST архитектуре.
