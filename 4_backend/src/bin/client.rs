/*
        Contact: https://artaudiochats.t.me/

        ***************** HTTP клиент. *****************

        Общие положения:
1) Для работы серверной части необходим MySql версии 8.4.7
2) В файле .env указаны параметры соединения с MySql, данные по порту и хосту http сервера.
3) В переменную окружения DB_PATH_CONNECT можно так-же установить параметры соединения с MySql.
4) В переменную окружения HTTP_PORT можно так-же установить порт http сервера.
5) В переменную окружения HTTP_HOST можно так-же установить хост http сервера.
6) В переменную окружения JWT_EXPIRATION можно так-же установить время жизни токена JSON Web Token в часах, содержащего код текущего пользователя.
7) В переменную окружения JWT_SECRET можно также установить секретную фразу для формирования JWT.
8) В переменную окружения GRAPHQL_DEEPLIM можно также установить максимальную глубину для GRAPHQL запроса.
9) Пользователь DB болжен иметь права на создание таблиц.

        Структура проекта:

4_backend/
├── Cargo.toml              <- конфигурация программы
├── src/                    <- директорий для хранения исходных кодов
│   │  │
│   │  bin/                 <- директорий для исполняемых файлов
│   │     ├── client.rs     <- точка входа в программу для клиента
│   │     └── server.rs     <- точка входа в программу для сервера
│   ├── client_executor.rs  <- модуль обработки всех основных комманд клмента
│   ├── db.rs               <- модуль общей работы с DB
│   ├── graphql_server.rs   <- модуль работы с GRAPHQL для сервера
│   ├── common.rs           <- модуль общих данных
│   ├── args.rs             <- модуль обработки агрементов CLI
│   ├── graphql_client.rs   <- модуль работы с GRAPHQL для клиента
│   ├── users.rs            <- модуль обработки пользователей
│   ├── jwt.rs              <- модуль обработки JSON Web Token
│   ├── passw.rs            <- модуль обработки паролей
│   └── friends.rs          <- модуль обработки друзей
├── .env                    <- файл для формирования переменных окружения
├── README.md               <- файл с документацией
├── tests/                  <- директорий для хранения интегральных тестов (E2E)
│    └── e2e_tests.rs       <- E2E тест
├── migrations/             <- директорий для хранения миграций базы данных
│    ├── 20260401000000_initial.sql <- начальный файл миграции DB от 2026/04/01 00:00:00
│    └── 20260401000001_two.sql     <- второй файл миграции DB от 2026/04/01 00:00:01
└── docs/                   <- директорий для хранения API документации
     └── api_docs.html      <- файл API документации (http://127.0.0.1:8080/api_docs.html)

            1. Показ помощи
$ cargo run --bin client -- -h
Usage: client.exe <COMMAND>

Commands:
  user-register  Register a new user
  login          Log in as a user
  add-friend     Add a friend
  del-friend     Delete a friend
  show-friends   Show friends
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

            2. Запуск тестов
$ cargo test
     Running unittests src\bin\client.rs (C:\Users\user\work\MyWorks\Rust\rust-incubator\target\debug\deps\client-3f2ef1d0803a829e.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\bin\server.rs (C:\Users\user\work\MyWorks\Rust\rust-incubator\target\debug\deps\server-130ccf3e27a6e493.exe)

running 22 tests
test friends::tests::invalid_user_id_check ... ok
test friends::tests::invalid_friend_id_check ... ok
test friends::tests::valid_friend_id_check ... ok
test friends::tests::invalid_mix_ids_check ... ok
test friends::tests::valid_user_id_check ... ok
test passw::tests::invalid_hash_password ... ok
test friends::tests::valid_mix_ids_check ... ok
test jwt::tests::check_new_authservice ... ok
test jwt::tests::check_generate_token ... ok
test users::tests::chack_empty_name ... ok
test users::tests::chack_long_name ... ok
test jwt::tests::valid_check_validate_token ... ok
test jwt::tests::invalid_check_validate_token ... ok
test users::tests::chack_long_password ... ok
test users::tests::check_empty_password ... ok
test users::tests::check_normal_nane ... ok
test users::tests::check_normal_password ... ok
test users::tests::invalid_id_user_check ... ok
test users::tests::valid_id_user_check ... ok
test passw::tests::valid_hash_password ... ok
test passw::tests::invalid_check_password ... ok
test passw::tests::valid_check_password ... ok

test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.54s

     Running tests\e2e_tests.rs (C:\Users\user\work\MyWorks\Rust\rust-incubator\target\debug\deps\e2e_tests-37d9979f5b98a21e.exe)

running 9 tests
test users::tests::chack_empty_name ... ok
test users::tests::chack_long_name ... ok
test users::tests::check_empty_password ... ok
test users::tests::chack_long_password ... ok
test users::tests::check_normal_nane ... ok
test users::tests::check_normal_password ... ok
test users::tests::invalid_id_user_check ... ok
test users::tests::valid_id_user_check ... ok
test test_e2e ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.47s

            3. Зарегистрировать нового пользователя
$ cargo run --bin client -- user-register Arthur Pass10
JWT: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc3NDg0NTA5OSwiaWF0IjoxNzc0ODQxNDk5fQ.6x6eY79uL8D6sYvud7Rry4vrubM_1tpJM96Dl056DIE
UserId: 2, UserName: Arthur

$ cargo run --bin client -- user-register Tom Pass20
JWT: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjMsImV4cCI6MTc3NDg0NTQ1OCwiaWF0IjoxNzc0ODQxODU4fQ.IemC6q7S82YK7R1n1jNCcbhZ7bIFAZa_ugDj7R8XTcw
UserId: 3, UserName: Tom

            4. Выполнить login
$ cargo run --bin client -- login Arthur Pass10
JWT: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc3NDg0NTIwNCwiaWF0IjoxNzc0ODQxNjA0fQ.WU7VQdxy4l0WyIf7kQSnkotI91uItcn3xOviwdxlPdU
Registered user: UserId: 2, UserName: Arthur

            5. Добавить друга
$ cargo run --bin client -- add-friend 3 eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc3NDg0OTk4MiwiaWF0IjoxNzc0ODQ2MzgyfQ.Rx_NBrq5oqcnvzZ6i_QI9tdVpMfhKOD1p07GZKicVs0
Added friend: FriendId: 3, FriendName: Tom

            6. Удалить друга
$ cargo run --bin client -- del-friend 3 eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc3NDg1ODc2MiwiaWF0IjoxNzc0ODU1MTYyfQ.jw624nSVrSA3HjbdrqC4S8uWwfJAY1DU8uAKC838LK4
Deleted friend: FriendId: 3, FriendName: Tom

            7. Показать друзей и их друзей

$ cargo run --bin client -- show-friends eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc3NTAwNTU2MywiaWF0IjoxNzc1MDAxOTYzfQ.0r-s1Fw8F0j2GdwqQ-_-dozTjwdOl9Ul-v3YuMwt4L8
Структура данных:
{
    "data": {
        "userplus": {
            "id": 2,
            "name": "Arthur",
            "friends": [
                {
                    "id": 3,
                    "name": "Tom",
                    "friends": []
                },
                {
                    "id": 4,
                    "name": "Sem",
                    "friends": [
                        {
                            "id": 3,
                            "name": "Tom"
                        }
                    ]
                }
            ]
        }
    }
}

$ cargo run --bin client -- show-friends eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjMsImV4cCI6MTc3NTAwMjQ2OCwiaWF0IjoxNzc0OTk4ODY4fQ.X6dgMdCyFQtUa8SItpP07-GRIF0Ou7H5cDPPargAtuE
Структура данных:
{
    "data": {
        "userplus": {
            "id": 3,
            "name": "Tom",
            "friends": []
        }
    }
}

$ cargo run --bin client -- show-friends eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjQsImV4cCI6MTc3NTAwNTQ0MiwiaWF0IjoxNzc1MDAxODQyfQ.1uSc3PMeSjRUSA-TftvyQcbD5iUkLApjsDoNNXLabIw
Структура данных:
UserPlusNode {
    id: 2,
    name: "Arthur",
    friends: Some(
        [
            UserPlusNode {
                id: 3,
                name: "Tom",
                friends: Some(
                    [],
                ),
            },
            UserPlusNode {
                id: 4,
                name: "Sem",
                friends: Some(
                    [
                        UserPlusNode {
                            id: 3,
                            name: "Tom",
                            friends: None,
                        },
                    ],
                ),
            },
        ],
    ),
}

Реальный вывод данных:
- Id: 2, Name: Arthur
 Friends:
  - Id: 3, Name: Tom
  - Id: 4, Name: Sem
   Friends:
    - Id: 3, Name: Tom

*/

use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок
use clap::Parser;       // Импорт трейта Parser из крейта clap для автоматического парсинга аргументов командной строки

// Подключение модуля common из родительского дирректория
#[path = "../common.rs"]
mod common ;

// Подключение модуля args из родительского дирректория
#[path = "../args.rs"]
mod args ;

// Подключение модуля client_executor из родительского дирректория
#[path = "../client_executor.rs"]
mod client_executor ;

// Подключение модуля graphql_client из родительского дирректория
#[path = "../graphql_client.rs"]
mod graphql_client ;

fn main() ->Result<()>{
    
    // получить все необходтиые для работы параметры, остальные игнорируем
    let (http_port, http_host, ..) = common::get_all_env_vars()? ;

    let args = args::Args::parse() ;

    // Выполнить действие из аргументов командной строки
    client_executor::any_command(&args, &http_host, http_port)? ;

    Ok(())
}