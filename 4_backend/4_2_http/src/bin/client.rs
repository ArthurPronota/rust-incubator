/*
    Contact: https://artaudiochats.t.me/

    ***************** HTTP тонкий клиент. *****************

        Общие положения:
1) Для работы программы необходим MySql версии 8.4.7
2) В файле .env указаны параметры соединения с MySql, данные по порту и хосту http сервера.
3) В переменную окужения DB_PATH_CONNECT можно так-же установить параметры соединения с MySql.
4) В переменную окужения HTTP_PORT можно так-же установить порт http сервера
5) В переменную окужения HTTP_HOST можно так-же установить хост http сервера
6) Пользователь DB болжен иметь права на создание таблиц и триггеров.
7) После создания пустой базы данных, запустиие команду:
    $ cargo run --bin client -- init-db


        Структура проекта:
    
4_2_http/
├── Cargo.toml              <- конфигурация программы
├── src/                    <- директорий для хранения исходных кодов
│   │  │
│   │  bin/                  <- директорий для исполняемых файлов
│   │     ├── client.rs      <- точка входа в программу для клиента
│   │     └── server.rs      <- точка входа в программу для сервера
│   ├── client_executor.rs  <- модуль обработки всех основных комманд клмента
│   ├── db.rs               <- модуль общей работы с DB
│   ├── server_executor.rs  <- модуль обработки всех основных комманд сервера
│   ├── common.rs           <- модуль общих данных
│   ├── args.rs             <- модуль обработки агрементов CLI
│   ├── roles.rs            <- модуль обработки ролей
│   ├── users.rs            <- модуль обработки пользователей
│   └── users_roles.rs      <- модуль обработки пользовательских ролей
├── .env                    <- файл для формирования переменных окружения
└── README.md               <- файл с документацией



    1. Общая помощь:
$ cargo run --bin client -- -h
Usage: client.exe <COMMAND>

Commands:
  init-db            Creating the required database objects.
  create-user        Create a new user
  delete-user        Delete user
  update-name-user   Modify user name
  update-email-user  Modify email name
  show-users-roles   Show users and their roles
  create-role        Create a new role
  delete-role        Delete role
  update-name-role   Modify role name
  update-perm-role   Modify role permissions
  show-roles         Show roles
  role-to-user       Add a role to a user
  remove-user-role   Remove a role from a user
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
$

    2. Помощь для конкретной команды:
$ cargo run --bin client -- create-user -h
Create a new user

Usage: client.exe create-user <Username> <Email>

Arguments:
  <Username>  User name, not unique
  <Email>     Email, unique

Options:
  -h, --help  Print help
$

    3. Создание объектов базы данных:
$ cargo run -- init-db
Database objects created successfully.
$


    4. Создание роли:
$ cargo run --bin client -- create-role "read-data" "Reader" "read,write,access"
Role created successfully.
$

    5. Создание Пользователя:
$ cargo run --bin client -- create-user Arthur yhgvnhjk.986ght.jhgt543@gmail.com
User created successfully.
$

    6. Показать пользователей и их роли:
$ cargo run --bin client -- show-users-roles
User #3: Arthur (yhgvnhjk.986ght.jhgt543@gmail.com)
Roles:
  Role #default: reader perm: read,write
  Role #manager-1: Level 1 Manager perm: access,approve,read,write
  Role #read-data: Reader perm: access,read,write
--------------------------------------------
User #4: Bob (bob@gmail.com)
Roles:
  Role #default: reader perm: read,write
--------------------------------------------
$

    7. Показать роли:
$ cargo run --bin client -- show-roles
Role: #default: reader perm: read,write
--------------------------------------------
Role: #manager-1: Level 1 Manager perm: access,approve,read,write
--------------------------------------------
Role: #read-data: Reader perm: access,read,write
--------------------------------------------
$
    8. Пример соединение с mysqlsh (опционально)
\connect arthur@localhost:3306

*/

use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок
use clap::Parser ;  // Импорт трейта Parser из крейта clap для декларативного парсинга аргументов командной строки

// Подключение модуля args из родительского дирректория
#[path = "../args.rs"]
mod args ;

// Подключение модуля client_executor из родительского дирректория
#[path = "../client_executor.rs"]
mod client_executor ;

// Подключение модуля common из родительского дирректория
#[path = "../common.rs"]
mod common ;

// Подключение модуля users из родительского дирректория
#[path = "../users.rs"]
mod users ;

// Подключение модуля roles из родительского дирректория
#[path = "../roles.rs"]
mod roles ;

// Подключение модуля db из родительского дирректория
#[path = "../db.rs"]
mod db ;

// Подключение модуля users_roles из родительского дирректория
#[path = "../users_roles.rs"]
mod users_roles ;

fn main() ->Result<()> {

    // получить все необходтиые для работы параметры
    let (http_port, http_host, _) = common::get_all_env_vars()? ;

    // разбор аргументов командной строки
    let args = args::Args::parse() ;

    // выполнить действия в соответствии с полученными аргументами командной строки
    client_executor::any_command(&args, &http_host, http_port)? ;

    Ok(())
}
