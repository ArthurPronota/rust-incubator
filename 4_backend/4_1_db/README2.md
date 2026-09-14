## Бэкенд-трек: CLI `step_4_1` для CRUD пользователей и ролей в **MySQL 8** через **sqlx**

## Задание

Три таблицы: `users`, `roles` (PK = `slug`), `users_roles` (M:N). CLI: создать/удалить/обновить, назначить/снять роль, списки. У пользователя **всегда есть хотя бы одна роль**. Выбрана MySQL, не Postgres/SQLite.

## Стек

- `sqlx` + `mysql` + `runtime-tokio-native-tls` — пул и SQL (query builder/ORM нет)
- `clap` — подкоманды
- `anyhow` — ошибки
- `dotenv` — `.env`
- `validator` + `serde` — проверка полей / (де)сериализация моделей

Строка подключения: переменная **`DB_PATH_CONNECT`**. Если есть `.env`, он подгружается, **не перезаписывая** уже заданные env. Нужны права на таблицы и триггеры. После пустой БД: `cargo run -- init-db`.

## Схема

| Таблица | Суть |
|---|---|
| `users` | `id_user` AI PK, `name`, уникальный `email`, utf8mb4 |
| `roles` | PK `slug`, `name`, `permissions` (строка через запятую) |
| `users_roles` | `(id_user, slug)` unique, FK на users/roles **ON DELETE CASCADE** |

Длины колонок берутся из констант модулей (`MAX_LENGTH_*`).

После таблиц: роль **`default` / reader / `read,write`**. Триггеры в транзакции:

- **BEF_UPD_ROLE** — нельзя менять `slug`
- **BEF_DEL_ROLE** — нельзя удалить `default`; нельзя удалить роль, если она **единственная** у какого-то пользователя (`FOR UPDATE` в подзапросе)

В `create_tables` после `begin` и `CREATE TRIGGER` **нет `commit`**: при drop транзакции триггеры могут откатиться. Имеет смысл проверить, появляются ли они в MySQL после `init-db`.

## CLI (`args.rs` → `executor.rs`)

Подкоманды: `init-db`, `create-user`, `delete-user`, `update-name-user`, `update-email-user`, `show-users-roles [id]`, `create-role`, `delete-role`, `update-name-role`, `update-perm-role`, `show-roles [slug]`, `role-to-user`, `remove-user-role`.

Почти все мутации: `pool.begin()` → операция → `commit`. Создание пользователя в одной транзакции: insert user + `users_roles` с `SLUG_DEFAULT`.

`create-role` идёт через `Role::create_role(&db_res, …)` без явного begin в executor (транзакция может быть внутри). Permissions с CLI — `Vec` с `value_delimiter = ','`, в БД склеиваются в строку.

## Модели

**`User`**: `FromRow`, `Validate` (длина имени, email). Сеттеры режут пробелы и проверяют длину/email. CRUD: `ins_user`, `update_*`, `delete_user`, поиск по id/email с опциональным **`FOR UPDATE`**. `UserWithRole` печатает пользователя и все роли.

**`Role`**: slug — нижний регистр, куски через `-` без пустых сегментов. Permissions нормализуются в сеттерах. `create_default_role` при init.

**`UsersRoles`**: связь; `ins_role_to_user` / `del_role_from_user` с проверкой существования user/role и блокировками.

SQL собирается строками + `bind` (`?`), не `query!` с проверкой схемы на compile-time. Это ближе к «ручному SQL», чем к ORM — как в вопросах README.

## Пул

`MySqlPoolOptions::new().max_connections(10).connect(...)`. Каждая команда берёт соединение из пула; транзакция держит его до commit/drop.

## Итог

Учебный **sqlx + MySQL + CLI**: пул, async, инварианты (роль обязательна, default нельзя снести, slug иммутабелен, email уникален). Не хватает слоя миграций (sqlx-cli / refinery): схема создаётся из Rust при `init-db`. Тестов в крейте нет.
