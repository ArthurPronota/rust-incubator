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

        let mut sql_stat = r#"
create table if not exists users (
    id_user	int unsigned not null primary key auto_increment,
    name varchar(255) not null,
    email varchar(255) not null,
    unique key `email` (email)
) 
ENGINE=InnoDb 
CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
"# ;

        // Создать таблицу пользователей
        sqlx::query(
            sql_stat
        )
        .execute(&self.pool)
        .await
        .map_err(|err|
            anyhow::anyhow!("{}, sql_statement: {}", err, sql_stat)
        )? ;



        Ok(())
    }

}