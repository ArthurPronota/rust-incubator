/*

    Contact: https://artaudiochats.t.me/

    Для работы программы необходим MySql версии 8.4.7
    В файле .env указаны параметры соединения с MySql.
    В переменную окружения DB_PATH_CONNECT можно так-же установить параметры соединения с MySql.

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

*/
mod args ;
mod executor ;
mod db ;
mod users ;
mod roles ;
mod users_roles;

use anyhow::Result ;
use clap::Parser;
use std::{
        path::Path
    } ;

const DB_PATH_CONNECT: &str = "DB_PATH_CONNECT" ;

#[tokio::main]
async fn main() ->Result<()> {
    // Путь к файлу с переменными ркружения и из значениями
    let env_file = Path::new(".env") ;
    if env_file.exists() {
        // Загрузка в переменные окружения сожержимого файла .env
        // Если переменная уже существует в окружении то содержимое не затирается 
        // содержимым из .env
        dotenv::dotenv()
            // преобразование ошибки в anyhow формат
            .map_err(|err|
                anyhow::anyhow!("{} from file: {:?}", err, env_file)
            )? ;
    }

    // Получить содержимое переменной окружения
    let db_path_conn  = 
                std::env::var(DB_PATH_CONNECT)
                    .map_err(|err| {
                        anyhow::anyhow!("{}, for var: {}", err, DB_PATH_CONNECT)
                    })? ;

    //println!("db_path_conn: {}", db_path_conn) ;

    let cl_args = args::Args::parse() ;

    //println!("v: {:?}", cl_args) ;

    // Выполнить полученную команду
    executor::any_command(
                &cl_args, 
                &db_path_conn
      ) 
      .await? ;

    Ok(())
}

