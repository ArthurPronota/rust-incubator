use anyhow::Result ;

use sqlx::{
        mysql::MySqlPoolOptions,
        MySqlPool
} ;

/// Пул соединений с базой данных
pub struct Database {
    pub pool:   MySqlPool,
}

impl Database {

    // Создание нового пула соединений с базой
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
    pub async fn create_tables(&self) ->Result<()> {

        // вектор с командами создания необходимых таблиц
        let sql_queries = vec![
// Создать таблицу пользователей            
r#"
create table if not exists users (
    id_user	int unsigned not null primary key auto_increment,
    name varchar(255) not null,
    email varchar(255) not null,
    unique key `email` (email)
) 
ENGINE=InnoDb 
CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
"#,
// Создать таблицу ролей
r#"
create table if not exists roles (
    slug varchar(50) not null primary key,
    name varchar(255) not null,
    permissions varchar(100) not null
) 
ENGINE=InnoDb
CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
"#,
// Создать таблицу соответсвия пользователя его правам
r#"
create table users_roles (
    id_user int unsigned not null,
    slug varchar(50) not null,
    key `slug` (slug),
    unique `id_user__slug` (id_user, slug),
    constraint `users_roles__id_user` foreign key (id_user) references users (id_user) on delete cascade,
    constraint `users_roles__slug` foreign key (slug) references roles (slug) on delete cascade
) 
ENGINE=InnoDb
CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
"#,
        ] ;

        for sql_query in sql_queries {
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