use std::u32;

use anyhow::Result ;

use chrono::format;
use sqlx::{
        FromRow,
        Row,
        mysql::MySql
} ;

use tokio::task::id;
use validator::{
        Validate,
        ValidateLength, 
        ValidateRange,
} ;

use crate::{
        db, users,
} ;

/// Минимальная длина наименование пользователя
pub const MIN_LENGTH_NAME: u64 = 1 ;

/// Максимальная длина наименование пользователя
pub const MAX_LENGTH_NAME: u64 = 255 ;


/// Минимальная длина пароля пользователя
pub const MIN_LENGTH_PASSWORD: u64 = 1 ;

/// Максимальная длина пароля пользователя
pub const MAX_LENGTH_PASSWORD: u64 = 255 ;

/// Структура пользователей
#[derive(
    FromRow,
    Validate,
    Default,
)]
pub struct Users {
    /// код пользователя
    #[validate(
        range(
            min = 1,
            //max = u32::MAX,
            message = "id_user must be greater than zero",
        )
    )]
    id_user:    u32,

    // имя пользователя
    #[validate(
        length(
            min = MIN_LENGTH_NAME,
            max = MAX_LENGTH_NAME,
        )
    )]
    name:       String,

    // хеш пароля пользователя
    #[validate(
        length(
            min = MIN_LENGTH_PASSWORD,
            max = MAX_LENGTH_PASSWORD,
        )
    )]
    password:   String,
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
            v if true => id_user,
            _ => return Err(anyhow::anyhow!("Invalid id_user: {}", id_user))
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
            n if n.validate_length(
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
            p if p.validate_length(
                            Some(MIN_LENGTH_PASSWORD),
                            Some(MAX_LENGTH_PASSWORD),
                            None
                        ) => p.to_owned(),
            p => return Err(anyhow::anyhow!("Invalid length: {} of password", p.chars().count()))
        } ;

        Ok(())
    }

    /// Получить password
    pub fn password(&self) ->&str {
        &self.password
    }

    /// Поиск по id_user
    pub async fn find_for_id_user(
                trans:   &mut sqlx::MySqlConnection, 
                id_user: u32,
                is_lock: bool,
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
                    trans:   &mut sqlx::MySqlConnection,
                    id_user: u32,
                    is_lock: bool,
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
                    trans:   &mut sqlx::MySqlConnection,
                    name:    &str,
                    is_lock: bool,
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
    pub async fn find_for_name_raise(
                    trans:   &mut sqlx::MySqlConnection,
                    name:    &str,
                    is_lock: bool,
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

    // Вставить нового пользователя
    pub async fn int_user(
                    trans:    &mut sqlx::MySqlConnection,
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
    pub async fn del_user_by_id(
                trans:    &mut sqlx::MySqlConnection,
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
                delete from user
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