## набор HTTP-эндпоинтов и толстый CLI

## Задание vs 4.2

| 4.2 (тонкий клиент) | 4.3 |
|---|---|
| Один `POST /anycommand` + JSON-enum команды | Отдельный URL и метод на операцию |
| Клиент шлёт «команду как есть» | Клиент сам выбирает метод, путь, тело |
| Без OpenAPI | Схема из кода + `/docs` |

Можно ходить в API **curl/Swagger без CLI**. Слой БД тот же (sqlx, транзакции, триггеры).

## Стек (новое)

`utoipa`, `utoipa-swagger-ui` (Axum), `serde_json`, `const_format`, `urlencoding`. Клиент по-прежнему **ureq** (sync), сервер **Axum + Tokio**.

## REST-поверхность

Префикс `/api/`. Имена ближе к **RPC** (`create_user`), не к ресурсам (`/users`), но глаголы HTTP соблюдены:

| Операция | Метод | Путь |
|---|---|---|
| init-db | GET | `/api/initdb` |
| создать пользователя | POST | `/api/create_user` + JSON |
| удалить | DELETE | `/api/delete_user/{id_user}` |
| имя / email | PUT | `/api/update_username`, `/api/update_useremail` + JSON |
| список / один | GET | `/api/show_users`, `/api/show_users/{id_user}` |
| роль создать | POST | `/api/create_role` |
| роль удалить | DELETE | `/api/delete_role/{slug}` |
| имя / permissions роли | PUT | `update_rolename`, `update_rolepermissions` |
| роли | GET | `/api/show_role`, `/api/show_role/{slug}` |
| выдать роль | POST | `/api/add_role_to_user` |
| снять роль | DELETE | `/api/remove_role_from_user/{id_user}/{slug}` |

`slug` в URL кодируется (`urlencoding`). Ответ — enum `Responce` (`ToSchema`): `Success` / `Error` / пользователь с ролями / списки. Хендлеры с `#[utoipa::path(...)]`, теги `users`, `roles`, `users_roles`.

## Сервер

Старт: env → `ApiDoc::openapi()` → при изменении записать **`openapi.json`** → `Arc<Database>` → роутер + `SwaggerUi` на **`/docs`**, JSON спецификации **`/api-docs/openapi.json`**.

UI: `http://127.0.0.1:8080/docs/` (порт из `HTTP_PORT`). Это **code-first**: контракт следует из хендлеров и `#[derive(OpenApi)]`, не из заранее написанного YAML.

## Толстый клиент

CLI как раньше (`init-db`, `create-user`, …), но `client_executor` **мапит команду на HTTP**: GET/POST/PUT/DELETE, JSON тел, path params.

До запроса клиент **валидирует** имя, email, slug через сеттеры `User`/`Role` — логика не только на сервере. Успех: HTTP 200 или 201, затем разбор `Responce`.

Нюанс: в ветке `UpdatePermissionsRole` в сеттер permissions, похоже, передаётся `slug` вместо списка прав — вероятная опечатка.

## Итог

Сдвиг **RPC-over-HTTP → набор эндпоинтов + толстый клиент + OpenAPI/Swagger**. Идиоматичный REST (`/users/{id}`) не выдержан: пути = имена действий. Для задания (curl, схема, HTML-доки) этого достаточно. GraphQL/gRPC в README — теория, в коде нет.
