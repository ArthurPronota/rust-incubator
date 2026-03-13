use anyhow::Result ;  // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок
use crate::users ;    // Импорт модуля users из текущего крейта

use crate::roles ;    // Импорт модуля roles из текущего крейта

use sqlx::{ // Импорт типов из крейта sqlx для работы с MySQL базой данных
        mysql::MySqlPoolOptions,    // MySqlPoolOptions - строитель для настройки и создания пула соединений с БД
        MySqlPool   // MySqlPool - пул соединений с MySQL для выполнения асинхронных запросов
} ;

/// часть выражения для блокировки строки
pub const FOR_UPDATE: &str = "FOR UPDATE" ;

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

        // Создание триггеров для контроля целостности данных
        use sqlx::Executor ;    // это трейт (trait) в sqlx, который определяет общий интерфейс для выполнения SQL-запросов. 

        let sql_triggers = vec![
// Триггер запрета модификации slug роли
r#"
CREATE TRIGGER IF NOT EXISTS BEF_UPD_ROLE
BEFORE UPDATE ON roles
FOR EACH ROW
BEGIN
    IF OLD.SLUG != NEW.SLUG THEN
        SIGNAL SQLSTATE '45000'
        SET MESSAGE_TEXT = 'Cannot update role' ;
    END IF ;
END;
"#,
// Триггер контроля удаления роли
// фраза "for update" обеспечивает целостность данных
r#"
CREATE TRIGGER IF NOT EXISTS BEF_DEL_ROLE
BEFORE DELETE ON roles
FOR EACH ROW
BEGIN
    DECLARE v_user_exists INT UNSIGNED DEFAULT NULL ;
    DECLARE v_mess_text VARCHAR(255) DEFAULT '' ;

    IF OLD.slug = 'default' THEN
        SIGNAL SQLSTATE '45000'
        SET MESSAGE_TEXT = 'Cannot delete default role' ;
    END IF ;

    select id_user
        into v_user_exists
        from (
            select ur_2.id_user as id_user, count(*) as count_roles
            from users_roles ur_2
            where ur_2.id_user in (
                select ur_1.id_user
                from users_roles ur_1
                where ur_1.slug = OLD.slug
                for update
            )
            group by ur_2.id_user
        ) as res
    where res.count_roles = 1 ;

    IF v_user_exists IS NOT NULL THEN
        SET v_mess_text = CONCAT(
                'You cannot delete role: ',
                OLD.slug,
                ' because it is the only one for id_user: ',
                v_user_exists
            ) ;

        SIGNAL SQLSTATE '45000'
        SET MESSAGE_TEXT = v_mess_text ;
    END IF ;

END;
"#
        ] ;

        let mut trans = 
                    self
                      .pool
                      // Устанавливает соединение и немедленно начинает новую транзакцию.
                      .begin()
                      .await? ;

        for unit_trg in sql_triggers {
            trans.execute(
            unit_trg
            )
            .await? ;
        }

        Ok(())
    }
}