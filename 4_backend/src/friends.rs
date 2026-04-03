use anyhow::Result ;    // Импорт типа Result из библиотеки anyhow для упрощенной обработки ошибок

use validator::{         // Импорт типов и трейтов из крейта validator для валидации данных
        Validate,        // Импорт трейта Validate, добавляющего метод validate() для структур
        ValidateRange,   // Импорт трейта ValidateRange для проверки диапазонов значений
        ValidationError, // Импорт типа ValidationError для создания и обработки ошибок валидации
} ;

use sqlx::FromRow ;      // Импорт трейта FromRow из SQLx для десериализации строк БД в структуры

use crate::db ;          // Импорт модуля db из текущего крейта для работы с базой данных

use crate::users::Users; // Импорт структуры Users из модуля users текущего крейта

/// Друзья
#[derive(   // Атрибут для автоматической реализации трейтов для структуры
    FromRow,    // Автоматически реализует трейт FromRow для десериализации из строки SQL-запроса
    Validate,   // Автоматически реализует трейт Validate для проверки правил валидации полей
    Default,    // Автоматически реализует трейт Default для создания экземпляра со значениями по умолчанию
 )
]
#[validate( // Атрибут для настройки правил валидации на уровне всей структуры
    schema(function = "Friends::validate_record")   // Указывает кастомную функцию для валидации всей структуры
 )
]
pub struct Friends {
    /// Код пользователя
    #[validate( // Атрибут для правил валидации поля
        range(  // Проверка вхождения значения в диапазон
            min = 1,    // Минимальное допустимое значение
            message = "id_user must be greater than zero",  // Сообщение об ошибке при нарушении
        ),
    )]
    user_id:    u32,    // Поле для хранения ID пользователя

    /// Код друга
    #[validate(    // Атрибут для правил валидации поля
        range(     // Проверка вхождения значения в диапазон
            min = 1,    // Минимальное допустимое значение
            message = "friend_id must be greater than zero",    // Сообщение об ошибке при нарушении
        )
    )]
    friend_id:  u32,    // Поле для хранения ID друга
}

impl Friends {

    // проверка полей Friends на совместимость
    fn validate_record(fr: &Friends) ->core::result::Result<(), ValidationError> {

        if fr.user_id == fr.friend_id {
            let mut err = ValidationError::new("user_id__eq__friend") ;
            err.message = Some("user_id must not equal friend_id".into()) ;
            return Err(err);
        }

        Ok(())
    }

    /// Установить user_id
    pub fn set_user_id(&mut self, user_id: u32) ->Result<()> {

        match user_id.validate_range(
                            Some(1),
                            None,
                            None,
                            None
                        ) 
        {
            v if v => {
                self.user_id = user_id ;
                Ok(())
            },
            _ => Err(anyhow::anyhow!("Invalid id_user: {}", user_id)),
        }
    }

    /// Получи user_id
    pub fn user_id(&self) ->u32 {
        self.user_id
    }

    /// Установить friend_id
    pub fn set_friend_id(&mut self, friend_id: u32) ->Result<()> {
        match friend_id.validate_range(
                Some(1), 
                None, 
                None, 
                None,
            ) {
            v if v => {
                self.friend_id = friend_id ;
                Ok(())
            },
            _ => Err(anyhow::anyhow!("Invalid friend_id: {}", friend_id)),
        }
    }

    /// Получить friend_id
    pub fn friend_id(&self) ->u32 {
        self.friend_id
    }

    /// поиск друга
    pub async fn find(
                    trans:      &mut sqlx::MySqlConnection,
                    user_id:    u32, 
                    friend_id:  u32,
                    is_lock:    bool,
                 ) ->Result<Option<Friends>> {

        let mut tmp_friend = Friends::default() ;

        tmp_friend.set_user_id(user_id)? ;

        tmp_friend.set_friend_id(friend_id)? ;

        tmp_friend.validate()? ;

        match sqlx::query_as::<_, Friends>(
            format!(
            r#"
            select * 
            from friends
            where user_id = ? and friend_id = ?
            {}
            "#,
            is_lock
                .then_some(db::FOR_UPDATE)
                .unwrap_or("")
            )
            .as_str()
        )
        .bind(tmp_friend.user_id)
        .bind(tmp_friend.friend_id())
        .fetch_one(&mut *trans)
        .await 
        {
            Ok(fr) => {
                fr.validate()? ;   
                Ok(Some(fr))
            },
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(anyhow::anyhow!("{}", err)),
        }
    }


    /// Обязательный поиск друга
    pub async fn find_raise(
                    trans:      &mut sqlx::MySqlConnection,
                    user_id:    u32, 
                    friend_id:  u32,
                    is_lock:    bool,
                 ) ->Result<Friends> {

        match Self::find(
                trans, 
                user_id, 
                friend_id, 
                is_lock
            ).await? {
            Some(fr) => Ok(fr),
            None => Err(anyhow::anyhow!("Not found friend for user_id: {}, friend_id: {}", user_id, friend_id)),
        }
    }

    /// Создание нового пользователя
    pub async fn insert(
                    trans:      &mut sqlx::MySqlConnection,
                    user_id:    u32, 
                    friend_id:  u32,
                 ) ->Result<Friends> {

        // поиск пользователя user_id
        Users::find_for_id_user_raise(
                    trans,
                    user_id,
                    false
                )
                .await? ;

        // поиск пользователя friend_id
        Users::find_for_id_user_raise(
                        trans, 
                        friend_id, 
                        false
                    )
                    .await ?;

        // поиск существующей дружбы
        if Self::find(
                    trans,
                    user_id,
                    friend_id,
                    true
                )
                .await?
                .is_some() {
            return Err(anyhow::anyhow!("I can't add a friend_id: {} for user: {}, he already exists.", friend_id, user_id));
        }
        
        // вставка друга
        sqlx::query(
            r#"
            insert into friends (user_id, friend_id)
            values (?, ?)
            "#
        )
        .bind(user_id)
        .bind(friend_id)
        .execute(&mut *trans)
        .await? ;

        // обязательный поиск друга
        Self::find_raise(
            trans,
            user_id,
            friend_id,
            false
        )
        .await
    }

    /// Удаление друга
    pub async fn delete(
                    trans:      &mut sqlx::MySqlConnection,
                    user_id:    u32,
                    friend_id:  u32,
                 ) ->Result<()> {

        // поиск друга с блокировкой
        Self::find_raise(
                trans,
                user_id,
                friend_id,
                true
            )
            .await? ;
        
        // удаление друга
        sqlx::query(
            r#"
            delete from friends
            where user_id = ? and friend_id = ?
            "#
        )
        .bind(user_id)        
        .bind(friend_id)
        .execute(&mut *trans)
        .await? ;

        // поиск друга
        match Self::find(
                trans,
                user_id,
                friend_id,
                false
            )
            .await?
        {
            Some(fr) => Err(anyhow::anyhow!("friend_id: {} for user_id: {} has not been deleted.", fr.friend_id(), fr.user_id())),
            None => Ok(()),
        }
    }

}

// unit тесты
#[cfg(test)]
mod tests {
    use super::* ;

    // тест проверки валидного user_id
    #[test]
    fn valid_user_id_check() {
        let mut tmp_friend = Friends::default() ;

        assert!(
            tmp_friend.set_user_id(1).is_ok()
        ) ;
    }

    // тест проверки инвалидного user_id
    #[test]
    fn invalid_user_id_check() {
        let mut tmp_friend = Friends::default() ;

        assert!(
            tmp_friend.set_user_id(0).is_err()
        ) ;
    }

    // тест проверки валидного friend_id
    #[test]
    fn valid_friend_id_check() {
        let mut tmp_friend = Friends::default() ;

        assert!(tmp_friend.set_friend_id(1).is_ok()) ;
    }

    // тест проверки инвалидного friend_id
    #[test]
    fn invalid_friend_id_check() {
        let mut tmp_friend = Friends::default() ;

        assert!(tmp_friend.set_friend_id(0).is_err()) ;
    }

    // валидный тест проверки совместимости user_id и friend_id
    #[test]
    fn valid_mix_ids_check() {
        let mut tmp_friend = Friends::default() ;

        tmp_friend.set_user_id(1).unwrap() ;

        tmp_friend.set_friend_id(2).unwrap() ;

        assert!(tmp_friend.validate().is_ok()) ;
    }

    // инвалидный тест проверки совместимости user_id и friend_id
    #[test]
    fn invalid_mix_ids_check() {
        let mut tmp_friend = Friends::default() ;

        tmp_friend.set_user_id(1).unwrap() ;

        tmp_friend.set_friend_id(1).unwrap() ;

        assert!(tmp_friend.validate().is_err()) ;
    }

}