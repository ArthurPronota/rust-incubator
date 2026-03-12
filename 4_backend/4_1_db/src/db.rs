use anyhow::Result ;
use crate::users::{
                self,
                //User,
            } ;

use crate::roles::{
                self,
} ;

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
format!(
r#"
create table if not exists users (
    id_user	int unsigned not null primary key auto_increment,
    name varchar({}) not null,
    email varchar({}) not null,
    unique key `email` (email)
) 
ENGINE=InnoDb 
CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
"#,
users::MAX_LENGTH_NAME,
users::MAX_LENGTH_EMAIL,
),
// Создать таблицу ролей
format!(
r#"
create table if not exists roles (
    slug varchar({}) not null primary key,
    name varchar({}) not null,
    permissions varchar({}) not null
) 
ENGINE=InnoDb
CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
"#,
roles::MAX_LENGTH_SLUG,
roles::MAX_LENGTH_NAME,
roles::MAX_LENGTH_PERMISSIONS,
),
// Создать таблицу соответсвия пользователя его правам
format!(
r#"
create table if not exists users_roles (
    id_user int unsigned not null,
    slug varchar({}) not null,
    key `slug` (slug),
    unique `id_user__slug` (id_user, slug),
    constraint `users_roles__id_user` foreign key (id_user) references users (id_user) on delete cascade,
    constraint `users_roles__slug` foreign key (slug) references roles (slug) on delete cascade
) 
ENGINE=InnoDb
CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
"#,
roles::MAX_LENGTH_SLUG,
),
/*

r#"
DELIMITER //

CREATE TRIGGER IF NOT EXISTS BEF_DEL_ROLE
BEFORE DELETE ON roles
FOR EACH ROW
BEGIN

    IF OLD.slug = 'default' THEN
        SIGNAL SQLSTATE '45000'
        SET MESSAGE_TEXT = 'Cannot delete default role' ;
    END IF;

END //

DELIMITER ;
"#

*/
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

        // создать роль по умолчанию
        roles::Role::create_default_role(&self)
            .await? ;

        // Создать триггер контроля удаления роли по умолчанию
        use sqlx::Executor ;

        let mut trans = 
                    self
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

        trans.execute(r#"
CREATE TRIGGER IF NOT EXISTS BEF_DEL_ROLE
BEFORE DELETE ON roles
FOR EACH ROW
BEGIN

    IF OLD.slug = 'default' THEN
        SIGNAL SQLSTATE '45000'
        SET MESSAGE_TEXT = 'Cannot delete default role' ;
    END IF;

END;        
        "#
        )
        .await? ;

        Ok(())
    }

    /*
    /// Создать пользователя
    pub async fn create_user(&self, name: &str, email: &str) ->Result<User> {
        
        let mut user_new = User::default() ;

        user_new.set_name(name)? ;
        
        user_new.set_email(email)? ;

        let mut trans = 
                self
                    .pool
                    // Устанавливает соединение и немедленно начинает новую транзакцию.
                    .begin()
                    .await? ;

        let new_user = sqlx::query_as::<_, User>(
            r#"
            insert into users (name, email)
            values (?, ?)
            returning id_user, name, email
            "#
        )
        .bind(user_new.name())
        .bind(user_new.email())
        .fetch_one(&mut *trans)
        .await? ;

        trans
            // Подтверждает эту транзакцию или точку сохранения.
            .commit()
            .await? ;

         Ok(new_user)
    }
     */
}