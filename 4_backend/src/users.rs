use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок

use sqlx::{         // Импорт компонентов из крейта sqlx для работы с БД
        FromRow,    // Импорт трейта FromRow для десериализации строк таблицы в структуры
        Pool,       // Импорт типа Pool для пула соединений с базой данных
        mysql::MySql,   // Импорт типа MySql для указания драйвера MySQL
} ;

use validator::{    // Импорт компонентов из крейта validator для валидации данных
        Validate,          // Импорт трейта Validate для валидации структур
        ValidateLength,    // Импорт трейта ValidateLength для проверки длины строк/коллекций
        ValidateRange,     // Импорт трейта ValidateRange для проверки диапазонов числовых значений
} ;

use crate::db ;     // Импорт модуля db из текущего крейта для работы с базой данных

/// Минимальная длина наименование пользователя
pub const MIN_LENGTH_NAME: u64 = 1 ;

/// Максимальная длина наименование пользователя
pub const MAX_LENGTH_NAME: u64 = 255 ;

/// Минимальная длина пароля пользователя
pub const MIN_LENGTH_PASSWORD: u64 = 1 ;

/// Максимальная длина пароля пользователя
pub const MAX_LENGTH_PASSWORD: u64 = 255 ;

/// Структура пользователей
#[derive(       // Атрибут для автоматической реализации трейтов
    FromRow,    // Автоматически реализует трейт FromRow для преобразования строки БД в структуру
    Validate,   // Автоматически реализует трейт Validate для проверки правил валидации полей
    Default,    // Автоматически реализует трейт Default для создания экземпляра со значениями по умолчанию
)]
pub struct Users {
    /// код пользователя
    #[validate(     // Атрибут для правил валидации поля
        range(      // Проверка вхождения значения в числовой диапазон
            min = 1,    // Минимальное допустимое значение
            message = "id_user must be greater than zero",  // Сообщение об ошибке при нарушении
        )
    )]
    id_user:    u32,    // Поле для хранения уникального идентификатора пользователя 

    // имя пользователя
    #[validate(     // Атрибут для правил валидации поля
        length(     // Проверка длины строки
            min = MIN_LENGTH_NAME,  // Минимальная допустимая длина имени
            max = MAX_LENGTH_NAME,  // Максимальная допустимая длина имени
        )
    )]
    name:       String,     // Поле для хранения имени пользователя

    // хеш пароля пользователя
    #[validate(     // Атрибут для правил валидации поля
        length(     // Проверка длины строки
            min = MIN_LENGTH_PASSWORD,  // Минимальная допустимая длина хэша пароля
            max = MAX_LENGTH_PASSWORD,  // Максимальная допустимая длина хэша пароля
        )
    )]
    password:   String,     // Поле для хранения хэшированного пароля пользователя
}

impl Users {

    // Установка id_user
    pub fn set_id_user(&mut self, id_user: u32) ->Result<()> {
        self.id_user = match id_user.validate_range(
                Some(1),     // min: Нижняя граница (включительно). 
                None,        // max: Верхняя граница (включительно). 
                None,  // исключая min, Если true, то id_user > min
                None,   // исключая max, Если true, то id_user < max
           ) 
        {
            v if v => id_user,
            _ => return Err(anyhow::anyhow!("Invalid id_user: {}", id_user)),
        } ;

        Ok(())
    }

    // получить id_user
    pub fn id_user(&self) ->u32 {
        self.id_user
    }

    /// Установить Username
    pub fn set_name(&mut self, name: &str) ->Result<()> {

        self.name = match name
                            .trim()
        {
            n if n.is_empty() => return Err(anyhow::anyhow!("Username os empty")),
            n if n.validate_length( // проверка длины хеша пароля
                            Some(MIN_LENGTH_NAME),
                            Some(MAX_LENGTH_NAME),
                            None
                        ) => n.to_owned(),
            n => return Err(anyhow::anyhow!("Invalid length: {} of Username", n.chars().count()))
        } ;

        Ok(())
    }

    /// Получить Username
    pub fn name(&self) ->&str {
        &self.name
    }

    /// Установить password
    pub fn set_password(&mut self, password: &str) ->Result<()> {

        self.password = match password
                                .trim() {
            p if p.is_empty() => return Err(anyhow::anyhow!("password is empty")),
            p if p.validate_length(     // проверка длины хеша пароля
                            Some(MIN_LENGTH_PASSWORD),
                            Some(MAX_LENGTH_PASSWORD),
                            None
                        ) => p.to_owned(),
            p => return Err(anyhow::anyhow!("Invalid length: {} of password", p.chars().count()))
        } ;

        Ok(())
    }

    /// Получить password
    #[allow(dead_code)]
    pub fn password(&self) ->&str {
        &self.password
    }

    // Поиск по id_user без блокировок
    #[allow(dead_code)]
    pub async fn find_no_trans(
                    db_res:   &Pool<MySql>, // Асинхронный пул подключений к базе данных SQLx.
                    id_user:  u32,
                 ) ->Result<Option<Self>>
    {
        let mut tmp_user = Users::default() ;

        tmp_user.set_id_user(id_user)? ;

        match sqlx::query_as::<_, Self>(
            r#"
            select *
            from users
            where id_user = ?
            "#
        )
        .bind(tmp_user.id_user())
        .fetch_one(db_res)
        .await {
            Ok(u) => {
                u.validate()? ;
                Ok(Some(u))
            },
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    // Обязательный поиск по id_user без блокировок
    #[allow(dead_code)]
    pub async fn find_no_trans_raise(
                    db_res:   &Pool<MySql>, // Асинхронный пул подключений к базе данных SQLx.
                    id_user:  u32,
                 ) ->Result<Self>
    {
        match Self::find_no_trans(
                db_res, 
                id_user
            )
            .await?
        {
            Some(u) => Ok(u),
            None => Err(anyhow::anyhow!("Not found user for id_user: {}", id_user)),
        }
    }

    /// Поиск по id_user
    pub async fn find_for_id_user(
                trans:   &mut sqlx::MySqlConnection, // транзакция
                id_user: u32,
                is_lock: bool,  // признак блокировки при поиске
            ) ->Result<Option<Self>> {

        let mut tmp_user = Users::default() ;

        tmp_user.set_id_user(id_user)? ;

        match sqlx::query_as::<_, Self>(
            format!(
            r#"
                select * 
                from users
                where id_user = ?
                {}
            "#,
            is_lock
                .then_some(db::FOR_UPDATE)
                .unwrap_or("")
            )
            .as_str()
        )
        .bind(tmp_user.id_user())
        .fetch_one(&mut *trans)
        .await {
            Ok(u) => {
                u.validate()? ;
                Ok(Some(u))
            },
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    /// Обязательный поиск пользователя по id_user
    pub async fn find_for_id_user_raise(
                    trans:   &mut sqlx::MySqlConnection,    // транзакция
                    id_user: u32,
                    is_lock: bool,  // признак блокировки при поиске
                 ) ->Result<Self> {
        match Self::find_for_id_user(
                trans, 
                id_user, 
                is_lock
            )
            .await?
        {
            Some(u) => Ok(u),
            None => Err(anyhow::anyhow!("Not found user for id_user: {}", id_user)),
        }
    }

    /// Поиск по Username
    pub async fn find_for_name(
                    trans:   &mut sqlx::MySqlConnection, // транзакция
                    name:    &str,
                    is_lock: bool,  // признак блокировки при поиске
                 ) ->Result<Option<Self>> {
        let mut tmp_user = Self::default() ;

        tmp_user.set_name(name)? ;

        match sqlx::query_as::<_, Self>(
            format!(
                r#"
                select *
                from users
                where name = ?
                {}
                "#,
                is_lock
                    .then_some(db::FOR_UPDATE)
                    .unwrap_or("")
            )
            .as_str()
        )
        .bind(tmp_user.name())
        .fetch_one(&mut *trans)
        .await {
            Ok(u) => {
                u.validate()? ;
                Ok(Some(u))
            },
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err.into())
        }
    }

    /// Обязательный поиск по Username
    #[allow(dead_code)]
    pub async fn find_for_name_raise(
                    trans:   &mut sqlx::MySqlConnection,    // транзакция
                    name:    &str,
                    is_lock: bool,  // признак блокировки при поиске
                 ) ->Result<Self> {
        match Self::find_for_name(
                trans, 
                name, 
                is_lock
            )
            .await? 
        {
            Some(u) => Ok(u),
            None => Err(anyhow::anyhow!("Not found user for Username: {}", name)),
        }
    }

    /// Вставить нового пользователя
    #[allow(dead_code)]
    pub async fn int_user(
                    trans:    &mut sqlx::MySqlConnection,   // транзакция
                    name:     &str,
                    password: &str,
                 ) ->Result<Self> {

        let mut tmp_user = Users::default() ;

        tmp_user.set_name(name)? ;

        tmp_user.set_password(password)? ;

        match Self::find_for_name(
                    trans,
                    name,
                    true,
                )
                .await?
        {
            Some(u) => Ok(u),
            None => {
                sqlx::query(
                    r#"
                    insert into users (name, password)
                    values (?, ?)
                    "#
                    )
                    .bind(tmp_user.name())
                    .bind(tmp_user.password())
                    .execute(&mut *trans)
                    .await? ;

                Self::find_for_name_raise(
                        trans, 
                        tmp_user.name(),
                        false
                    )
                    .await
            }
        }
    }

    /// Удалить пользователя по id_user
    #[allow(dead_code)]
    pub async fn del_user_by_id(
                trans:    &mut sqlx::MySqlConnection,   // транзакция
                id_user:  u32
            ) ->Result<()> {
        let mut tmp_user = Users::default() ;

        tmp_user.set_id_user(id_user)? ;

        // заблокировать пользователя
        Self::find_for_id_user_raise(
                trans,
                id_user,
                true
            )
            .await? ;
        
        sqlx::query(
            r#"
                delete from users
                where id_user = ?            
            "#
        )
        .bind(tmp_user.id_user())
        .execute(&mut *trans)
        .await? ;

        // контроль удаления пользователя
        Self::find_for_id_user(
                trans,
                tmp_user.id_user(),
                false
            )
            .await?
            .map_or(
                Ok(()),
                |_| Err(anyhow::anyhow!("The user id_user: {} has not been deleted", tmp_user.id_user()))
            )
    }
}

// Условная компиляция - этот модуль включается только при запуске тестов (cargo test)
#[cfg(test)]
mod tests {         // Определение модуля для Unit тестов
    use super::* ;  // Импортируем все элементы из родительского модуля (выше mod tests)

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // Проверка валидного id_user
    fn valid_id_user_check() {
        let mut tmp_user = Users::default() ;

        assert!(tmp_user.set_id_user(1).is_ok()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // Проверка инвалидного id_user
    fn invalid_id_user_check() {
        let mut tmp_user = Users::default() ;

        assert!(tmp_user.set_id_user(0).is_err()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // Проверка пустого имени пользователя
    fn chack_empty_name() {
        let mut tmp_user = Users::default() ;

        assert!(tmp_user.set_name("").is_err()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // Проверка нормального имени пользователя
    fn check_normal_nane() {
        let mut tmp_user = Users::default() ;

        assert!(tmp_user.set_name("abc").is_ok()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // Проверка длинного имени пользователя
    fn  chack_long_name() {
        let mut tmp_user = Users::default() ;

        assert!(tmp_user.set_name(
                             &"a".repeat((MAX_LENGTH_NAME + 1) as usize)
                        ).is_err()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // Проверка пустого пароля
    fn  check_empty_password() {
        let mut tmp_user = Users::default() ;

        assert!(tmp_user.set_password("").is_err()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // Проверка валидного пароля
    fn  check_normal_password() {
        let mut tmp_user = Users::default() ;

        assert!(tmp_user.set_password("abc").is_ok()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // Проверка длинного пароля
    fn  chack_long_password() {
        let mut tmp_user = Users::default() ;

        assert!(tmp_user.set_password(
                             &"a".repeat((MAX_LENGTH_PASSWORD + 1) as usize)
                        ).is_err()
        ) ;
    }

}