use anyhow::Result ;

use validator::{
        Validate,
        ValidateRange,
        ValidationError,
} ;

use sqlx::FromRow ;

use crate::db ;

use crate::users::Users;

/// Друзья
#[derive(
    FromRow,
    Validate,
    Default,
 )
]
#[validate(
    schema(function = "Friends::validate_record")
 )
]
pub struct Friends {
    /// Код пользователя
    #[validate(
        range(
            min = 1,
            message = "id_user must be greater than zero",
        ),
    )]
    user_id:    u32,

    /// Код друга
    #[validate(
        range(
            min = 1,
            message = "friend_id must be greater than zero",
        )
    )]
    friend_id:  u32,
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

        Self::find_raise(
                trans,
                user_id,
                friend_id,
                true
            )
            .await? ;
        
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

#[cfg(test)]
mod tests {
    use super::* ;

    #[test]
    fn valid_user_id_check() {
        let mut tmp_friend = Friends::default() ;

        assert!(
            tmp_friend.set_user_id(1).is_ok()
        ) ;
    }

    #[test]
    fn invalid_user_id_check() {
        let mut tmp_friend = Friends::default() ;

        assert!(
            tmp_friend.set_user_id(0).is_err()
        ) ;
    }

    #[test]
    fn valid_friend_id_check() {
        let mut tmp_friend = Friends::default() ;

        assert!(tmp_friend.set_friend_id(1).is_ok()) ;
    }

    #[test]
    fn invalid_friend_id_check() {
        let mut tmp_friend = Friends::default() ;

        assert!(tmp_friend.set_friend_id(0).is_err()) ;
    }

    #[test]
    fn valid_mix_ids_check() {
        let mut tmp_friend = Friends::default() ;

        tmp_friend.set_user_id(1).unwrap() ;

        tmp_friend.set_friend_id(2).unwrap() ;

        assert!(tmp_friend.validate().is_ok()) ;
    }

    #[test]
    fn invalid_mix_ids_check() {
        let mut tmp_friend = Friends::default() ;

        tmp_friend.set_user_id(1).unwrap() ;

        tmp_friend.set_friend_id(1).unwrap() ;

        assert!(tmp_friend.validate().is_err()) ;
    }

}