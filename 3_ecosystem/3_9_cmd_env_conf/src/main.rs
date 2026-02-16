use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Конфигурация режима работы.
pub struct ModeConfig {
    /// Режим отладки
    debug:  bool,
}

// Реализации занчения по умолчанию для ModeConfig
impl Default for ModeConfig {
    // Возвращает "значение по умолчанию" для заданного типа.
    fn default() -> Self {
        Self { debug: false }
    }
}

/// Конфигурация сервера приложения
pub struct ServerConfig {
    /// URL-адрес, по которому данное приложение доступно извне.
    pub external_url: String,

    /// Порт, предоставляющий HTTP-интерфейс клиентам, доступен извне.
    pub http_port:  u16,

    /// Порт, предоставляющий клиентам доступ к gRPC-интерфейсу (Google Remote Procedure Call).
    pub grpc_port:  u16,

    /// Этот порт должен быть доступен только внутри Kubernetes Pod.
    pub healthz_port:   u16,

    /// Порт, предоставляющий конечную точку для сбора метрик Prometheus, доступен только внутри кластера Kubernetes.
    pub metrics_port:   u16,
}

/// Реализация значений по умолчанию для ServerConfig
impl Default for ServerConfig {

    // Возвращает "значение по умолчанию" для заданного типа.
    fn default() -> Self {
        Self { 
            external_url: "http://127.0.0.1".to_string(), 
            http_port: 8081, 
            grpc_port: 8082, 
            healthz_port: 10025, 
            metrics_port: 9199,
        }
    }
}

/// Конфигурация сервера MySql
pub struct DbMysqlConfig {
    
    /// Хост сервера базы данных MySQL.
    pub host:   String,

    /// Порт, на котором сервер базы данных MySQL принимает соединения.
    pub port:   u16,

    /// Имя базы данных для использования на сервере MySQL.
    pub dating: String,

    /// Пользователь базы данных MySQL для подключения к серверу MySQL.
    pub user:   String,

    /// Пароль пользователя базы данных MySQL, используемый для аутентификации на сервере MySQL.
    pub pass:   String,

}

/// Реализация значений по умолчанию для DbMysqlConfig
impl Default for DbMysqlConfig {
    
    // Возвращает "значение по умолчанию" для заданного типа.
    fn default() -> Self {
        Self { 
            host: "127.0.0.1".to_string(),
            port: 3306,
            dating: "default".to_string(),
            user: "root".to_string(),
            pass: "".to_string(),
        }
    }
}

/// Конфигурация количества соединений с MySql
pub struct DbMysqlConnectionsConfig {
    /// Максимально допустимое количество соединений в пуле неактивных соединений.
    max_idle:   u16,
    /// Максимально допустимое количество открытых соединений с сервером базы данных MySQL одновременно.
    max_open:   u16,
}

/// Реализация значений по умолчанию для DbMysqlConnectionsConfig
impl Default for DbMysqlConnectionsConfig {
    // Возвращает "значение по умолчанию" для заданного типа.
    fn default() -> Self {
        Self { max_idle: 30, max_open: 30 }
    }
}


/// Логи приложения в порядке убывания
pub enum LogApp {
    error,
    warn,
    info,
    debug,
    trace,
}

/// Конфигурация максимально допустимого уровень записей в логе приложения.
pub struct LogAppConfig {
    /// Максимальный уровень лога в приложении
    level:  LogApp,
}

/// Реализация значений по умолчанию для LogAppConfig
impl Default for LogAppConfig {
    // Возвращает "значение по умолчанию" для заданного типа.
    fn default() -> Self {
        Self { level: LogApp::info }
    }
}

/// Конфигурация запуска фоновой задачи мониторинга.
pub struct BackgroundWatchdogConfig {
    /// Период времени для запуска фоновой задачи мониторинга.
    pub period: String,

    /// Максимальное количество записей, которые могут быть завершены за один запуск задания.
    pub limit:  u16,

    /// Тайм-аут для удержания сторожевой блокировки записей.
    pub lock_timeout:   String,
}

/// Реализация значений по умолчанию для LogAppConfig
impl Default for BackgroundWatchdogConfig {
    // Возвращает "значение по умолчанию" для заданного типа.
    fn default() -> Self {
        Self { 
            period: "5s".to_string(),
            limit: 10,
            lock_timeout: "4s".to_string(),
        }
    }
}

/// Общая конфигурация
pub struct Config {
    pub mode:       ModeConfig,
    pub server:     ServerConfig,
    pub mysql:      DbMysqlConfig,
    pub mysql_connections:   DbMysqlConnectionsConfig,
    pub log_app:    LogAppConfig,
    pub background_watchdog:   BackgroundWatchdogConfig,
}


fn main() {
    println!("Implement me!");
}
/*

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

// ==================== СТРУКТУРЫ КОНФИГУРАЦИИ ====================

/// Корневая структура конфигурации
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Config {
    /// Режим отладки
    pub debug: bool,
    
    /// Настройки сервера
    pub server: ServerConfig,
    
    /// Настройки логирования
    pub logging: LoggingConfig,
    
    /// Настройки базы данных
    pub database: DatabaseConfig,
    
    /// Произвольные дополнительные параметры
    #[serde(flatten)]
    pub extra: HashMap<String, toml::Value>,
}

/// Настройки сервера
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ServerConfig {
    /// Хост сервера
    pub host: String,
    
    /// Порт сервера
    pub port: u16,
    
    /// Таймаут в секундах
    pub timeout: u64,
    
    /// Максимальный размер тела запроса в байтах
    pub max_body_size: usize,
    
    /// Включить SSL
    pub ssl_enabled: bool,
    
    /// Путь к SSL сертификату
    pub ssl_cert_path: Option<String>,
    
    /// Путь к SSL ключу
    pub ssl_key_path: Option<String>,
}

/// Настройки логирования
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct LoggingConfig {
    /// Уровень логирования (debug, info, warn, error)
    pub level: String,
    
    /// Путь к файлу лога
    pub file_path: Option<String>,
    
    /// Формат логов (json, text)
    pub format: String,
    
    /// Максимальный размер файла лога в байтах
    pub max_size: u64,
    
    /// Количество резервных копий
    pub backups: u32,
}

/// Настройки базы данных
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DatabaseConfig {
    /// URL подключения к базе данных
    pub url: String,
    
    /// Имя базы данных
    pub name: String,
    
    /// Имя пользователя
    pub username: String,
    
    /// Пароль
    pub password: String,
    
    /// Максимальное количество соединений в пуле
    pub max_connections: u32,
    
    /// Таймаут подключения в секундах
    pub connection_timeout: u64,
}

// ==================== РЕАЛИЗАЦИЯ ЗНАЧЕНИЙ ПО УМОЛЧАНИЮ ====================

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
            database: DatabaseConfig::default(),
            extra: HashMap::new(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            timeout: 30,
            max_body_size: 1024 * 1024, // 1MB
            ssl_enabled: false,
            ssl_cert_path: None,
            ssl_key_path: None,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file_path: None,
            format: "text".to_string(),
            max_size: 10 * 1024 * 1024, // 10MB
            backups: 5,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "localhost".to_string(),
            name: "app".to_string(),
            username: "user".to_string(),
            password: "password".to_string(),
            max_connections: 10,
            connection_timeout: 5,
        }
    }
}

// ==================== CLI АРГУМЕНТЫ ====================

/// Prints its configuration to STDOUT.
#[derive(Parser, Debug)]
#[clap(name = "step_3_9", version = "0.1.0", author)]
struct CliArgs {
    /// Enables debug mode
    #[clap(short = 'd', long = "debug")]
    debug: bool,
    
    /// Path to configuration file
    #[clap(short = 'c', long = "conf", env = "CONF_FILE", default_value = "config.toml")]
    conf_file: PathBuf,
}

// ==================== ЗАГРУЗЧИК КОНФИГУРАЦИИ ====================

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileReadError(#[from] std::io::Error),
    
    #[error("Failed to parse TOML: {0}")]
    TomlParseError(#[from] toml::de::Error),
    
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),
}

pub struct ConfigLoader {
    args: CliArgs,
}

impl ConfigLoader {
    pub fn new() -> Self {
        Self {
            args: CliArgs::parse(),
        }
    }
    
    /// Загружает конфигурацию с применением правил приоритета
    pub fn load(&self) -> Result<Config, ConfigError> {
        // 1. Начинаем со значений по умолчанию
        let mut config = Config::default();
        
        println!("1. Default config: {:?}", config);
        
        // 2. Загружаем из TOML файла (переопределяет значения по умолчанию)
        if let Some(file_config) = self.load_from_file()? {
            config = self.merge_configs(config, file_config);
            println!("2. After TOML file: {:?}", config);
        }
        
        // 3. Загружаем из переменных окружения с префиксом CONF_ (высший приоритет)
        config = self.apply_env_vars(config)?;
        println!("3. After env vars: {:?}", config);
        
        // 4. CLI флаг --debug имеет наивысший приоритет
        if self.args.debug {
            config.debug = true;
            println!("4. CLI --debug flag applied");
        }
        
        Ok(config)
    }
    
    /// Загружает конфигурацию из TOML файла
    fn load_from_file(&self) -> Result<Option<Config>, ConfigError> {
        if !self.args.conf_file.exists() {
            println!("Config file not found: {:?}, using defaults", self.args.conf_file);
            return Ok(None);
        }
        
        let contents = fs::read_to_string(&self.args.conf_file)?;
        let file_config: Config = toml::from_str(&contents)?;
        
        println!("Loaded config from file: {:?}", self.args.conf_file);
        Ok(Some(file_config))
    }
    
    /// Применяет переменные окружения с префиксом CONF_
    fn apply_env_vars(&self, mut config: Config) -> Result<Config, ConfigError> {
        // Проходим по всем переменным окружения
        for (key, value) in std::env::vars() {
            if key.starts_with("CONF_") {
                let config_key = &key[5..]; // Убираем префикс "CONF_"
                self.apply_env_value(&mut config, config_key, &value)?;
            }
        }
        
        Ok(config)
    }
    
    /// Применяет значение из переменной окружения к конфигурации
    fn apply_env_value(&self, config: &mut Config, key: &str, value: &str) -> Result<(), ConfigError> {
        match key {
            // Корневые поля
            "DEBUG" => config.debug = value.parse().unwrap_or(false),
            
            // Поля сервера
            "SERVER_HOST" => config.server.host = value.to_string(),
            "SERVER_PORT" => config.server.port = value.parse().unwrap_or(config.server.port),
            "SERVER_TIMEOUT" => config.server.timeout = value.parse().unwrap_or(config.server.timeout),
            "SERVER_MAX_BODY_SIZE" => config.server.max_body_size = value.parse().unwrap_or(config.server.max_body_size),
            "SERVER_SSL_ENABLED" => config.server.ssl_enabled = value.parse().unwrap_or(false),
            "SERVER_SSL_CERT_PATH" => config.server.ssl_cert_path = Some(value.to_string()),
            "SERVER_SSL_KEY_PATH" => config.server.ssl_key_path = Some(value.to_string()),
            
            // Поля логирования
            "LOGGING_LEVEL" => config.logging.level = value.to_string(),
            "LOGGING_FILE_PATH" => config.logging.file_path = Some(value.to_string()),
            "LOGGING_FORMAT" => config.logging.format = value.to_string(),
            "LOGGING_MAX_SIZE" => config.logging.max_size = value.parse().unwrap_or(config.logging.max_size),
            "LOGGING_BACKUPS" => config.logging.backups = value.parse().unwrap_or(config.logging.backups),
            
            // Поля базы данных
            "DATABASE_URL" => config.database.url = value.to_string(),
            "DATABASE_NAME" => config.database.name = value.to_string(),
            "DATABASE_USERNAME" => config.database.username = value.to_string(),
            "DATABASE_PASSWORD" => config.database.password = value.to_string(),
            "DATABASE_MAX_CONNECTIONS" => config.database.max_connections = value.parse().unwrap_or(config.database.max_connections),
            "DATABASE_CONNECTION_TIMEOUT" => config.database.connection_timeout = value.parse().unwrap_or(config.database.connection_timeout),
            
            // Неизвестные ключи сохраняем в extra
            _ => {
                config.extra.insert(key.to_string(), toml::Value::String(value.to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Объединяет две конфигурации (приоритет у второй)
    fn merge_configs(&self, base: Config, override_config: Config) -> Config {
        Config {
            debug: override_config.debug,
            server: self.merge_server(base.server, override_config.server),
            logging: self.merge_logging(base.logging, override_config.logging),
            database: self.merge_database(base.database, override_config.database),
            extra: base.extra.into_iter().chain(override_config.extra).collect(),
        }
    }
    
    fn merge_server(&self, base: ServerConfig, over: ServerConfig) -> ServerConfig {
        ServerConfig {
            host: over.host,
            port: over.port,
            timeout: over.timeout,
            max_body_size: over.max_body_size,
            ssl_enabled: over.ssl_enabled,
            ssl_cert_path: over.ssl_cert_path.or(base.ssl_cert_path),
            ssl_key_path: over.ssl_key_path.or(base.ssl_key_path),
        }
    }
    
    fn merge_logging(&self, base: LoggingConfig, over: LoggingConfig) -> LoggingConfig {
        LoggingConfig {
            level: over.level,
            file_path: over.file_path.or(base.file_path),
            format: over.format,
            max_size: over.max_size,
            backups: over.backups,
        }
    }
    
    fn merge_database(&self, base: DatabaseConfig, over: DatabaseConfig) -> DatabaseConfig {
        DatabaseConfig {
            url: over.url,
            name: over.name,
            username: over.username,
            password: over.password,
            max_connections: over.max_connections,
            connection_timeout: over.connection_timeout,
        }
    }
}

// ==================== ОСНОВНАЯ ПРОГРАММА ====================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Конфигурация приложения ===\n");
    
    let loader = ConfigLoader::new();
    let config = loader.load()?;
    
    println!("\n=== Финальная конфигурация ===");
    println!("{:#?}", config);
    
    println!("\n=== JSON формат ===");
    println!("{}", serde_json::to_string_pretty(&config)?);
    
    Ok(())
}

// ------ Пример конфигурационного файла config.toml -----

debug = true

[server]
host = "0.0.0.0"
port = 3000
timeout = 60
max_body_size = 2097152
ssl_enabled = true
ssl_cert_path = "/etc/ssl/cert.pem"
ssl_key_path = "/etc/ssl/key.pem"

[logging]
level = "debug"
file_path = "/var/log/app.log"
format = "json"
max_size = 10485760
backups = 7

[database]
url = "postgres://localhost"
name = "myapp_prod"
username = "admin"
password = "secret123"
max_connections = 20
connection_timeout = 10

# Произвольные дополнительные параметры
environment = "production"
region = "us-east-1"

// ----- 1. Без аргументов (используются значения по умолчанию)

$ cargo run

// ----- 2. С указанием файла конфигурации

$ cargo run -- --conf custom.toml

// ----- 3. С флагом debug

$ cargo run -- --debug

// ----- 4. С переменными окружения

$ export CONF_SERVER_PORT=9999
$ export CONF_DATABASE_URL="postgres://remote"
$ export CONF_DEBUG=true
$ cargo run

// ---- 5. Комбинация всего

$ export CONF_SERVER_TIMEOUT=120
$ export CONF_LOGGING_FORMAT="json"
$ cargo run -- --conf prod.toml --debug

// -------------------

Ключевые особенности:

1. Приоритет конфигурации (от низшего к высшему):

    - Значения по умолчанию в коде
    - TOML файл
    - Переменные окружения с префиксом CONF_
    - CLI флаг --debug (специальный случай)

2. Типизированная иерархическая структура с вложенными конфигами

3. Поддержка произвольных полей через extra: HashMap<String, toml::Value>

4. Интеграция с clap для парсинга CLI аргументов

5. Детальный вывод процесса загрузки для отладки

6. Поддержка всех требований из задания:
    - Флаг -d, --debug
    - Опция -c, --conf <conf> с переменной окружения CONF_FILE
    - Помощь -h, --help
    - Версия -V, --version

*/