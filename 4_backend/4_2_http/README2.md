## Тонкокий CLI-клиента и **HTTP-демон**.

## Задание

Клиент не ходит в БД: парсит CLI, шлёт команду на сервер, печатает ответ. Сервер — **одна** HTTP-точка, внутри разбирает команду и работает с MySQL.

## Стек

| Часть | Крейты |
|---|---|
| Сервер | Axum 0.8, Tokio, sqlx MySQL |
| Клиент | **синхронный** `ureq` (не reqwest) |
| Общее | clap, serde, anyhow, dotenv, validator |

Клиент без async-рантайма; сервер — классический Tokio work-stealing. Это как раз контраст «HTTP-клиенты Rust» из README: ureq простой и блокирующий.

## Конфиг (`.env` / окружение)

`common::get_all_env_vars()`:

- `HTTP_PORT`, `HTTP_HOST` (`localhost` или IP)
- `DB_PATH_CONNECT` — клиент тоже читает её, но **не использует** (берёт `_`)

## Протокол

Единственный маршрут: **`POST /anycommand`**, JSON.

Тело запроса — enum **`Commands`** (тот же, что clap-подкоманды): `InitDb`, `CreateUser { name, email }`, … Serde помечает вариант внешне (например `"InitDb"`).

Ответ — enum **`Response`**:

- `Success(String)` / `Error(String)`
- `UserRole` / `ListUsersRoles`
- `Role` / `ListRoles`

HTTP почти всегда **200**; ошибка БД/валидации — JSON `Error(...)`, не 4xx/5xx. Клиент считает ошибкой только `status != OK`.

## Клиент (`cargo run --bin client -- <subcommand>`)

`clap` → `Commands` → `ureq::post(http://host:port/anycommand).send_json(&commands)` → `read_json::<Response>` → `println!` как в 4.1.

Тонкий клиент: нет транзакций и sqlx в `client_executor`. Модули `db`/`users` всё равно компилируются в бинарник из‑за типов в `Response`.

Первый шаг после пустой БД: `cargo run --bin client -- init-db` (сервер уже должен слушать).

## Сервер (`cargo run --bin server`)

1. Env → пул `Database::new` в **`Arc`** (state Axum, `Send + Sync`).
2. `Router`: `POST /anycommand` → `handle_commmand`.
3. `TcpListener::bind(host:port)` + `axum::serve`.

Хендлер: `State<Arc<Database>>` + `Json<Commands>` → `handle_commmand_int` (логика как `executor` из 4.1: транзакции, default-роль, CRUD) → `Json<Response>`. `Err` превращается в `Response::Error`.

Слой `db` / `users` / `roles` / `users_roles` — по сути перенос 4.1 (пул 10 соединений, триггеры, `FOR UPDATE`).

## Архитектура

```text
CLI  --JSON POST-->  Axum  --sqlx-->  MySQL
         Commands              Response
```

Не REST (нет `/users/:id`), не WebSocket: RPC одной ручкой. Для учебного «тонкого клиента» этого достаточно.

Нюансы: клиент синхронный, сервер async; нет auth/TLS; путь захардкожен; бинарники тянут общие `.rs` через `#[path]`, а не через `lib.rs`.
