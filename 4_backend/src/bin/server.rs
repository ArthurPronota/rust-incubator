/*
            Contact: https://artaudiochats.t.me/

            ***************** HTTP сервер. *****************

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
├── image.png               <- Скриншот API документации
├── tests/                  <- директорий для хранения интегральных тестов (E2E)
│    └── e2e_tests.rs       <- E2E тест
├── migrations/             <- директорий для хранения миграций базы данных
│    ├── 20260401000000_initial.sql <- начальный файл миграции DB от 2026/04/01 00:00:00
│    └── 20260401000001_two.sql     <- второй файл миграции DB от 2026/04/01 00:00:01
└── docs/                   <- директорий для хранения API документации
     └── api_docs.html      <- файл API документации (http://127.0.0.1:8080/api_docs.html)

                1. Запуск сервера
$ cargo run --bin server

                2. Просмотрт документации
Загрузить в браузере http://127.0.0.1:8080/api_docs.html

                3. Пример соединение с mysqlsh (опционально)
\connect arthur@localhost:3306

                4. Пример создания базы данных 4_db
CREATE DATABASE `4_db` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci

*/
use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок

use std::sync::Arc ;    // Импорт Arc (Atomic Reference Counting) для потокобезопасного разделяемого владения

use tokio::net::TcpListener ;   // Импорт TcpListener из Tokio для асинхронного прослушивания TCP портов

use axum::{     // Импорт компонентов из веб-фреймворка Axum
        Extension,  // Импорт типа Extension для внедрения зависимостей в обработчики
        Router,     // Импорт Router для объединения маршрутов в одно приложение
        routing::{  // Импорт модуля routing для определения HTTP методов
            post    // Импорт функции post для создания маршрутов обрабатывающих POST запросы
        }
} ;

use async_graphql::{    // Импорт компонентов из крейта async-graphql
            EmptySubscription,  // Импорт EmptySubscription - пустой тип для подписок (без real-time)
            Schema,     // Импорт Schema для объединения Query, Mutation и Subscription
};


use async_graphql::dataloader::DataLoader;  // Импорт DataLoader для пакетной загрузки данных и решения N+1 проблемы

use tower_http::services::ServeFile;    // Импорт ServeFile из tower_http для обслуживания статических файлов

// Подключение модуля common из родительского дирректория
#[path = "../common.rs"]
mod common ;

// Подключение модуля db из родительского дирректория
#[path = "../db.rs"]
mod db ;

// Подключение модуля graphql_server из родительского дирректория
#[path = "../graphql_server.rs"]
mod graphql_server ;

// Подключение модуля graphql_client из родительского дирректория
#[path = "../graphql_client.rs"]
mod graphql_client ;

// Подключение модуля users из родительского дирректория
#[path = "../users.rs"]
mod users ;

// Подключение модуля jwt из родительского дирректория
#[path = "../jwt.rs"]
mod jwt ;

// Подключение модуля passw из родительского дирректория
#[path = "../passw.rs"]
mod passw ;

// Подключение модуля friends из родительского дирректория
#[path = "../friends.rs"]
mod friends ;

// Атрибут, преобразующий асинхронную функцию main в синхронную точку входа с запуском токио рантайма
#[tokio::main]
async fn main() ->Result<()> {

    // Получить все необходимые переменные окружения
    let (port_http, 
         host_http, 
         db_path, 
         jwt_expir, 
         jwt_secret,
         graphql_deep_limit
        ) = common::get_all_env_vars()? ;


    // Оборачивание пула соединений с DB в Arc
    let db_res = Arc::new(
                    db::Database::new(&db_path).await?
                ) ;

    // Обернуть в Arc структуру AuthService
    let auth_serv = Arc::new(
            jwt::AuthService::new(
                    &jwt_secret,
                    jwt_expir
            )? 
        ) ;

    let friend_loader = 
            DataLoader::new(    // Создание нового экземпляра DataLoader для пакетной загрузки данных
                graphql_server::FriendDataLoader{   // Создание экземпляра кастомного загрузчика друзей
                            pool: Arc::new( // Оборачиваем пул соединений в Arc для потокобезопасного разделения
                                db::Database::new(&db_path).await?  // Создаем новое подключение к базе данных по указанному пути
                            )
                }, 
                tokio::spawn    // Указываем исполнитель (токио спаун) для асинхронных операций загрузчика
            );

    // Создать таблицы в DB      
    db_res.create_tables()
        .await? ;

    /* Выполнение миграций
    При выполнении миграции создаётся таблица _sqlx_migrations.

 MySQL  localhost:3306 ssl  4_db  SQL > desc _sqlx_migrations ;
+----------------+------------+------+-----+-------------------+-------------------+
| Field          | Type       | Null | Key | Default           | Extra             |
+----------------+------------+------+-----+-------------------+-------------------+
| version        | bigint     | NO   | PRI | NULL              |                   |
| description    | text       | NO   |     | NULL              |                   |
| installed_on   | timestamp  | NO   |     | CURRENT_TIMESTAMP | DEFAULT_GENERATED |
| success        | tinyint(1) | NO   |     | NULL              |                   |
| checksum       | blob       | NO   |     | NULL              |                   |
| execution_time | bigint     | NO   |     | NULL              |                   |
+----------------+------------+------+-----+-------------------+-------------------+
6 rows in set (0.0027 sec)
 MySQL  localhost:3306 ssl  4_db  SQL >    

 MySQL  localhost:3306 ssl  4_db  SQL > select * from _sqlx_migrations \G
*************************** 1. row ***************************
       version: 20260401000000
   description: initial
  installed_on: 2026-04-01 10:04:35
       success: 1
      checksum: 0x81997E09F9A43D55633C5C258C868DD64F173FC204F03CC973AE5B8456DD8816A76B06AEAFC4EF29ACF8B6ED61657959
execution_time: 18625100
*************************** 2. row ***************************
       version: 20260401000001
   description: two
  installed_on: 2026-04-01 10:04:35
       success: 1
      checksum: 0xEC4064DDFCC9878E3AA294AEA757BE0D10A5938B5E7C6134A35E7AB5B643CCC73C8C04CE2ABF705CF73AAA26E8426E5D
execution_time: 19653300
2 rows in set (0.0007 sec)
 MySQL  localhost:3306 ssl  4_db  SQL >

    Формат имени файла: migrations\20260401000001_two.sql
        20260401000001 - YYYYmmddHHmiss (version)
        two - description

    Если в файле миграции одно sql предложени то не нужно ставить ';'
    Если в файле миграции несколько sql предложения то ';' ставится 
        после всех sql предложений.

    Код ниже:
    1. Находит все миграции в папке migrations/
    2. Проверяет, какие миграции уже выполнены
    3. Выполняет новые миграции в правильном порядке
    4. Создает и обновляет таблицу _sqlx_migrations для отслеживания состояния

     */
    sqlx::migrate!().run(&db_res.pool).await?;

    let schema = 
                    Schema::build(  // Начинаем построение схемы с помощью билдера
                        graphql_server::Query,       // Первый параметр: тип Query (корневой тип для операций чтения)
                        graphql_server::Mutation,    // Второй параметр: тип Mutation (корневой тип для операций изменения)
                        EmptySubscription,  // Третий параметр: тип Subscription (корневой тип для подписок, здесь пустой)
                    )
                    // Добавляем клонированный пул подключений к БД в контекст схемы
                    .data(db_res.clone())   
                    // Добавляем клонированный сервис аутентификации в контекст схемы
                    .data(auth_serv.clone())    
                    // Добавляем DataLoader для загрузки друзей в контекст схемы
                    .data(friend_loader)   
                    // Устанавливаем лимит глубины вложенности GraphQL запроса
                    .limit_depth(graphql_deep_limit)
                    // Завершаем построение схемы и получаем готовый экземпляр Schema
                    .finish()
                    ;

    let route: Router<()> = Router::new()   // Создаем новый маршрутизатор Axum с типом состояния ()
                                .route(     // Добавляем маршрут для обработки HTTP запросов
                                    common::GRAPHQL_URI,    // Путь из константы
                                    post(graphql_server::graph_handler),    // Указываем обработчик для POST запросов (функция graph_handler)
                                )
                                .route_service( // Добавляем маршрут для обслуживания статического файла
                                    "/api_docs.html",   // Путь, по которому будет доступна документация API
                                    ServeFile::new("docs/api_docs.html")    // Обслуживаем статический HTML файл из директории docs
                                )
                                .layer(Extension(schema))   // Добавляем слой middleware, внедряющий GraphQL схему в расширения запроса
                                ;

    let listener = 
            TcpListener::bind(  // Привязываемся к указанному сетевому адресу
                format!("{}:{}", host_http, port_http)  // Формируем строку адреса из хоста и порта 
            )
            .await     // Асинхронно ожидаем привязки
            ?;      // оператор ? распространяет ошибку при неудаче

    axum::serve(    // Вызываем функцию serve из крейта axum для запуска HTTP сервера
            listener,   // Передаем TcpListener, который слушает входящие соединения
            route,  // Передаем настроенный Router с маршрутами и обработчиками
        )
        .await  // Асинхронно ожидаем завершения работы сервера (бесконечно)
        ?;  // Оператор ? распространяет ошибку, если сервер завершился с ошибкой

    Ok(())
}