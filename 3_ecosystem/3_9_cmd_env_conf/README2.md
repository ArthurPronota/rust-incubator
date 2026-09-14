## Типизированная иерархия настроек из трёх слоёв для программ.

## Зависимости

- **clap 4** (`derive`, `env`) — CLI и путь к файлу из `CONF_FILE`
- **serde** + **toml 0.8** — разбор TOML
- **thiserror** — `ConfigError`

## Модель конфигурации

Вложенные структуры с `Serialize`/`Deserialize` и `Default`:

```text
Config
├── mode.debug: bool                          // false
├── server: external_url, http/grpc/healthz/metrics ports
├── db.mysql: host, port, dating, user, pass
│     └── connections: max_idle, max_open     // в Default 30/30
├── log.app.level: LogApp                     // info
└── background.watchdog: period, limit, lock_timeout
```

У корневого `Config` стоит `#[serde(default)]`: отсутствующие ключи в TOML заполняются `Default`, а не падают с ошибкой.

`LogApp` — enum `error|warn|info|debug|trace` (`rename_all = "lowercase"` + `FromStr` для переменных окружения).

Дефолты в коде и в `config.toml` **не везде совпадают**: в файле `max_idle`/`max_open` = 300, `watchdog.limit` = 100; в `Default` — 30/30 и 10. Если файла нет, будут значения из Rust.

`config2.toml` почти как `config.toml`, отличие — `external_url = "http://127.0.0.2"`.

## CLI (`CliArgs`)

`#[derive(Parser)]` + свой `help_template` (в clap 4 flags слиты в options, шаблон задания с `[FLAGS]` устарел).

| Флаг | Смысл |
|---|---|
| `-d` / `--debug` | включить `mode.debug` |
| `-c` / `--conf` | путь к TOML; иначе env **`CONF_FILE`**, иначе `config.toml` |
| `-h`, `-V` | help / version |

`ConfigLoader::new()` вызывает `CliArgs::parse()` (при ошибке clap завершает процесс).

## Ошибки

`ConfigError`: чтение файла (`io::Error`), разбор TOML (`toml::de::Error`), `VarError`. Вариант env в `load_from_env` почти не задействован: там `std::env::vars()`, не `var()`.

## Приоритет слияния (`load`)

Как в задании, плюс CLI:

1. `Config::default()`
2. Если файл **существует** — целиком заменить на `toml::from_str` (частичные поля дополняются serde default). Нет файла → тихий `Ok(None)`, остаются дефолты кода (не ошибка).
3. `load_from_env`: все `CONF_*`. Суффикс после `CONF_` мапится вручную (`DEBUG`, `HTTP_PORT`, `MYSQL_PASS`, `LOG_APP_LEVEL`, …). Пустые строки для URL/host/dating/pass не перезаписывают; числа/`bool`/`LogApp` при ошибке `parse` оставляют старое значение. **`CONF_MYSQL_USER` нет.** `CONF_FILE` сюда не входит — его обрабатывает clap.
4. Если задан `-d`, `mode.debug = true` (выключить debug флагом нельзя, только не передавать `-d`).

Для каждой найденной `CONF_*` ещё `println!` ключа и значения.

## `main`

```rust
let config = ConfigLoader::new().load().unwrap();
println!("{:#?}", config);
```

Пример: `cargo run -- -c config2.toml` или `set CONF_FILE=config2.toml` и `cargo run`. Help: `cargo run -- -h`.

## Итог по коду

Типичный 12-factor без крейта `config`: дерево структур, дефолты в `impl Default`, файл как слой, env как оверлей, clap для пути и debug. Слабые места: ручной список env (легко забыть поле вроде `user`), полная замена конфига файлом а не deep-merge полей, дефолты кода ≠ комментарии в TOML, нет тестов.
