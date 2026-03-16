/*
Пример запуска:
    cargo run --bin client -- -h



    Contact: https://artaudiochats.t.me/

        Общие положения:
1) Для работы программы необходим MySql версии 8.4.7
2) В файле .env указаны параметры соединения с MySql.
3) В переменную окужения DB_PATH_CONNECT можно так-же установить параметры соединения с MySql.
4) Пользователь DB болжен иметь права на создание таблиц и триггеров.
5) После создания пустой базы данных запустиие команду:
    $ cargo run -- --bin client init-db
6) Вся работа с DB выполняется в асинхронном режиме.

        Структура проекта:
    
4_1_db/
├── Cargo.toml              <- конфигурация программы
├── src/                    <- директорий для хранения исходныъ кодов
│   ├── main.rs             <- точка входа в программу
│   ├── db.rs               <- модуль общей работы с DB
│   ├── executor.rs         <- модуль обработки всех основных комманд
│   ├── args.rs             <- модуль обработки агрементов CLI
│   ├── roles.rs            <- модуль обработки ролей
│   ├── users.rs            <- модуль обработки пользователей
│   └── users_roles.rs      <- модуль обработки пользовательских ролей
├── .env                    <- файл для формирования переменных окружения
└── README.md               <- файл с документацией



    1. Общая помощь:
$ cargo run -- -h
Usage: step_4_1.exe <COMMAND>

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
$ cargo run -- create-user -h
Create a new user

Usage: step_4_1.exe create-user <Username> <Email>

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
$ cargo run -- create-role "read-data" "Reader" "read,write,access"
Role created successfully.
$

    5. Создание Пользователя:
$ cargo run -- create-user Arthur yhgvnhjk.986ght.jhgt543@gmail.com
User created successfully.
$

    6. Показать пользователей и их роли:
$ cargo run -- show-users-roles
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
$ cargo run -- show-roles
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
use anyhow::Result ;
use clap::Parser ;

// Подклбчение модуля args из родительского дирректория
#[path = "../args.rs"]
mod args ;

// Подклбчение модуля client_executor из родительского дирректория
#[path = "../client_executor.rs"]
mod client_executor ;

// Подклбчение модуля common из родительского дирректория
#[path = "../common.rs"]
mod common ;

#[path = "../users.rs"]
mod users ;

#[path = "../roles.rs"]
mod roles ;

#[path = "../db.rs"]
mod db ;

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
