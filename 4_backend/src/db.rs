use anyhow::Result ;  // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок
use crate::users ;    // Импорт модуля users из текущего крейта

use sqlx::{ // Импорт типов из крейта sqlx для работы с MySQL базой данных
        mysql::MySqlPoolOptions,    // MySqlPoolOptions - строитель для настройки и создания пула соединений с БД
        MySqlPool   // MySqlPool - пул соединений с MySQL для выполнения асинхронных запросов
} ;

/// часть выражения для блокировки строки
#[allow(dead_code)]
pub const FOR_UPDATE: &str = "FOR UPDATE" ;

/// Пул соединений с базой данных
#[allow(dead_code)]
pub struct Database {
    pub pool:   MySqlPool,
}

impl Database {

    /// Создание нового пула соединений с базой
    #[allow(dead_code)]
    pub async fn new(path_to_mysql: &str) ->Result<Self> {
        let pool = 
                // Возвращает конфигурацию по умолчанию
                MySqlPoolOptions::new()
                    // Установите максимальное количество соединений, которые должен 
                    // поддерживать этот пул.
                    .max_connections(10)
                    // Создайте новый пул, используя эти параметры пула (PoolOptions), 
                    // и немедленно откройте хотя бы одно соединение.
                    .connect(path_to_mysql)
                    // Приостановить выполнение до тех пор, пока результат выполнения 
                    // Future не будет готов.
                    .await
                    // Отобразить ошибку в формат anyhow
                    .map_err(|err|
                        anyhow::anyhow!("{} for path: {}", err, path_to_mysql)
                    )? ;

        // Вертуть результат
        Ok(Database { pool })
    }

    /// Создать таблицы для программы
    #[allow(dead_code)]
    pub async fn create_tables(&self) ->Result<()> {

        // вектор с командами создания необходимых таблиц
        let sql_queries = vec![
// Создать таблицу пользователей  
format!(
r#"
create table if not exists users (
    id_user	int unsigned not null primary key auto_increment,
    name varchar({}) not null,
    password varchar({}) not null,
    unique key `name` (name)
)
ENGINE=InnoDb 
CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
"#,
users::MAX_LENGTH_NAME,
users::MAX_LENGTH_PASSWORD,
),
// Создать таблицу друзей
r#"
create table if not exists friends (
    user_id     int unsigned not null,
    friend_id   int unsigned not null,
    primary key (user_id, friend_id),
    foreign key (user_id) references users(id_user) on delete cascade,
    foreign key (friend_id) references users(id_user) on delete cascade,
    constraint not_equal CHECK (user_id != friend_id),
    index id_user (user_id)
) 
ENGINE=InnoDb
CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
"#
.to_owned(),
        ] ;

        for sql_query in sql_queries.iter() {
            // Выполните один SQL-запрос в виде подготовленного запроса (с прозрачным кэшированием).
            sqlx::query(
                sql_query
            )
            // Выполните запрос и верните общее количество затронутых строк.
            .execute(&self.pool)
            // Приостановить выполнение до тех пор, пока результат выполнения 
            // Future не будет готов.            
            .await
            // Отобразить ошибку в формат anyhow
            .map_err(|err|
                anyhow::anyhow!("{}, sql_statement: {}", err, sql_query)
            )? ;
        }

        Ok(())
    }
}