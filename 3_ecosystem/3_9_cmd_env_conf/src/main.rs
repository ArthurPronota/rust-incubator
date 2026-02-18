/*
    Contact: https://artaudiochats.t.me/
    
    Внешний вид help устарел, т.к. используем крейт clap версии "4.5.5". 
    С версии 3 в clap все FLAGS перенесены в OPTIONS.
    Ошибка при попытке установть FLAGS:
        Command step_3_9: `{flags}` template variable was removed in clap3, they are now included in `{options}`

    Пример установки переменной окружения:
        set CONF_MYSQL_PASS=my_root

    Добавлен новый файл конфигурации:   config2.toml

    Получение помощи:   cargo run -- -h
    
    Пример запуска: cargo run -- -c config2.toml

 */

use clap::Parser;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;



/// Конфигурация режима работы.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

    /// Ограничения количества соединений с MySql
    pub connections:    DbMysqlConnectionsConfig,
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
            connections: DbMysqlConnectionsConfig::default(),
        }
    }
}

/// Конфигурация количества соединений с MySql
#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// Приведение всех полей при сериализации к нижнему регистру
#[serde(rename_all = "lowercase")]
pub enum LogApp {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

// реализация преобразования from String to LogApp
impl FromStr for LogApp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "error" => Ok(LogApp::Error),
            "warn" => Ok(LogApp::Warn),
            "info" => Ok(LogApp::Info),
            "debug" => Ok(LogApp::Debug),
            "trace" => Ok(LogApp::Trace),
            v => Err(format!("Invalid value: {} for LogApp", v)),
        }
    }
}

/// Конфигурация максимально допустимого уровень записей в логе приложения.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LogAppConfig {
    /// Максимальный уровень лога в приложении
    level:  LogApp,
}

/// Реализация значений по умолчанию для LogAppConfig
impl Default for LogAppConfig {
    // Возвращает "значение по умолчанию" для заданного типа.
    fn default() -> Self {
        Self { level: LogApp::Info }
    }
}

/// Конфигурация запуска фоновой задачи мониторинга.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

/// Родительский тип DbConfig
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DbConfig {
    pub mysql:  DbMysqlConfig,
}

/// Родительский тип LogConfig
#[derive(Serialize, Default, Debug, Deserialize)]
pub struct LogConfig {
    pub app:    LogAppConfig,
}

/// Родительский тип BackgroundConfig
#[derive(Default, Serialize, Debug, Deserialize)]
pub struct BackgroundConfig {
    pub watchdog:   BackgroundWatchdogConfig,
}

/// Общая конфигурация
#[derive(Default, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub mode:       ModeConfig,
    pub server:     ServerConfig,
    pub db:         DbConfig,  
    pub log:        LogConfig,
    pub background: BackgroundConfig,
}


/// Аргументы CLI
#[derive(
    Parser, //  генерирует код для разбора std::env::args()
    Debug
)]
#[clap(
    // метаданные программы - name
    name = "step_3_9",
    // метаданные программы - version, позволяет выводить -V или --vesion
    version = "0.1.0",
    // шаблон полсказки при выводе помощи
    help_template = "\
{name} {version}
Prints its configuration to STDOUT.

USAGE:
    {usage}

OPTIONS:
{options}
"
)]
struct CliArgs {
    // Признак debug режима
    #[clap(
        // Позволяет вызвать -d
        short = 'd',
        // Позволяет вызвать --debug
        long = "debug",
        // Задает описание аргумента для краткой справки (-h).
        help = "Enables debug output",
    )]
    debug: bool,

    /// Путь к конфигурационному файлу
    #[clap(
        // Позволяет вызвать -c
        short = 'c',
        // Позволяет вызвать --conf
        long = "conf",
        // Если аргумента нет, clap проверит переменную окружения CONF_FILE
        env = "CONF_FILE",
        // Значение аргумента, если он отсутствует.
        default_value = "config.toml",
        // Задает описание аргумента для краткой справки (-h).
        help = "Path to configuration file",
    )]
    conf_file: PathBuf,
}

#[derive(Debug)]
/// Загрузка конфигурации
pub struct ConfigLoader {
    args:   CliArgs,
}

/// Перечисляемый тип ошибок конфигурации
/// для 3-х типов ошибок
#[derive(
    Debug, 
    thiserror::Error    // генерирует реализацию трейта std::error::Error
)]
pub enum ConfigError {
    // Определяет сообщение об ошибке
    // {0} подставляется на место первого аргумента (std::io::Error)
    #[error("Failed to read config file: {0}")]
    // Хранит оригинальную ошибку ввода-вывода
    // #[from] автоматически реализует преобразование From<std::io::Error>
    FileReadError(#[from] std::io::Error),
    
    // Определяет сообщение об ошибке parse TOML
    // {0} подставляется на место первого аргумента (toml::de::Error)
    #[error("Failed to parse TOML: {0}")]
    // #[from] автоматически реализует преобразование From<toml::de::Error>
    TomlParseError(#[from] toml::de::Error),
    
    // Определяет сообщение об ошибке Environment variable (переменная не найдена или содержит невалидный Unicode)
    // {0} подставляется на место первого аргумента (std::env::VarError)
    #[error("Environment variable error: {0}")]
    // #[from] автоматически реализует преобразование From<std::env::VarError>
    EnvVarError(#[from] std::env::VarError),
}


// Реализация загрузкии конфигурации
impl ConfigLoader {

    /// Разбор и заполнение значений аргуметов от CLI
    pub fn new() ->Self {
        Self { 
            args: CliArgs::parse(), // разбор от std::env::args_os(), завершение работы при ошибке.
        }
    }

    /// Загрузка конфигурации из файла
    pub fn load_from_file(&self) ->Result<Option<Config>, ConfigError> {

        // Проверка наличия конфигурационного файла
        if !self
            .args
            .conf_file
            .exists() {
          return Ok(None);
        }

        // получение контента конфигурационного файла
        let file_content = fs::read_to_string(&self.args.conf_file)? ;

        // десеариализовать контент конфигурационного файла в структуру конфигурации
        let conf_from_file = toml::from_str::<Config>(&file_content)? ;

        Ok(Some(conf_from_file))
    }

    /// Загрузка из переменных окружения
    pub fn load_from_env(&self, cfg: &mut Config) ->Result<(), ConfigError> {

        for (key, val) in std::env::vars() {
            // имя переменной начинается с CONF_
            if key.starts_with("CONF_") {
                println!("key: {}, val: {}", key, val) ;
                match &key[5..] {
                    "DEBUG" => cfg.mode.debug = val.parse().unwrap_or(cfg.mode.debug),
                    "EXTERNAL_URL" => cfg.server.external_url = (!val.is_empty()).then(|| val).unwrap_or_else(|| cfg.server.external_url.clone()),
                    "HTTP_PORT" => cfg.server.http_port = val.parse().unwrap_or(cfg.server.http_port),
                    "GRPC_PORT" => cfg.server.grpc_port = val.parse().unwrap_or(cfg.server.grpc_port),
                    "HEALTHZ_PORT" => cfg.server.healthz_port = val.parse().unwrap_or(cfg.server.healthz_port),
                    "METRICS_PORT" => cfg.server.metrics_port = val.parse().unwrap_or(cfg.server.metrics_port),
                    "MYSQL_HOST" => cfg.db.mysql.host = (!val.is_empty()).then(|| val).unwrap_or_else(|| cfg.db.mysql.host.clone()),
                    "MYSQL_PORT" => cfg.db.mysql.port = val.parse().unwrap_or(cfg.db.mysql.port),
                    "MYSQL_DATING" => cfg.db.mysql.dating = (!val.is_empty()).then(|| val).unwrap_or_else(|| cfg.db.mysql.dating.clone()),
                    "MYSQL_PASS" => cfg.db.mysql.pass = (!val.is_empty()).then(|| val).unwrap_or_else(|| cfg.db.mysql.pass.clone()),
                    "MYSQL_CONNECTIONS_MAX_IDLE" => cfg.db.mysql.connections.max_idle = val.parse().unwrap_or(cfg.db.mysql.connections.max_idle),
                    "MYSQL_CONNECTIONS_MAX_OPEN" => cfg.db.mysql.connections.max_open = val.parse().unwrap_or(cfg.db.mysql.connections.max_open),
                    "LOG_APP_LEVEL" => cfg.log.app.level = val.parse().unwrap_or(cfg.log.app.level.clone()),
                    _ => {}, 
                }
            }
        }

        Ok(())
    }


    /// Загрузка конфигурации всеми досиупеыми способами.
    pub fn load(&self) ->Result<Config, ConfigError> {
        // 1. Загрузка парамеров конфигурации данными установленными в программе по умолчанию.
        let mut conf = Config::default() ;

        // 2. Загрузка парамеров из конфигурационного файла toml по умолчанию
        if let Some(cfg_toml) = self.load_from_file()? {
            //println!("cfg_toml: {:#?}\n", cfg_toml) ;
            conf = cfg_toml ;
        }

        // 3. Загрузка параметров из переменных окружения
        self.load_from_env(&mut conf)? ;

        // 4. Модификация режима работы
        if self.args.debug {
            conf.mode.debug = self.args.debug ;
        }

        Ok(conf)
    }
}

fn main() {
    let loader = ConfigLoader::new();

    let config = loader.load().unwrap() ;

    println!("{:#?}", config) ;
}