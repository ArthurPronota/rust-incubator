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
