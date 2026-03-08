use serde::{Serialize, Deserialize} ;

/*  Ручеая реализация FromRow обусловленная типом pub id_user: usize 
    для которого нет автоматического преобразования из MySql типа int unsigned
    в Rust тип usize

use sqlx::{FromRow, Row, mysql::MySqlRow} ;

/// Структура пользователя
#[derive(
    Debug,
    Serialize,
    Deserialize,
 )
]
pub struct User {
    pub id_user: usize,
    pub name:    String,
    pub email:   String,
}

impl<'r> FromRow<'r, MySqlRow> for User {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(
            Self {
                id_user: row.try_get::<u64, _>("id_user")? as usize,
                name: row.try_get("name")?,
                email: row.try_get("email")?,
            }
        )
    }
}
 */

use anyhow::Result ;
use sqlx::FromRow ;
use validator::{
        Validate,
        ValidateEmail,
        ValidateLength,
    } ;

use crate::db::Database;

/*
use const_format::concatcp ;
 */

/*
type MySqlTrans<'a> = sqlx::Transaction<'a, sqlx::MySql> ;
 */

pub const MIN_LENGTH_NAME: u64 = 1 ;

pub const MAX_LENGTH_NAME: u64 = 255 ;

pub const MIN_LENGTH_EMAIL: u64 = 1 ;

pub const MAX_LENGTH_EMAIL: u64 = 255 ;

/*
static INVLENGT_USER_NAME: &str = concatcp!(
                    "Username must be between ",
                    MIN_LENGTH_USER_NAME,
                    " and ",
                    MAX_LENGTH_USER_NAME,
                    "characters long"
                ) ;
 */

#[derive(
    Debug,
    Default,
    //Clone,
    Serialize,
    Deserialize,
    // автоматическая реализация для типа u32
    FromRow,
    Validate,
 )
]
pub struct User {
    
    id_user: u32,

    #[validate(
        length(
            min = MIN_LENGTH_NAME, 
            max = MAX_LENGTH_NAME,
        )
      )
    ]
    name:    String,

    #[validate(
        email,
        length(
            min = MIN_LENGTH_EMAIL, 
            max = MAX_LENGTH_EMAIL,
        )        
     )
    ]
    email:   String,
}

impl User {

    /*
    /// Получить значение User по умолчанию
    pub fn new() ->Self {
        Self::default()
    }
     */

    /// Установка id_user
    pub fn set_id_user(&mut self, id_user: u32) ->Result<()> {

        self.id_user = match id_user {
            v if v > 0 => v,
            v => return Err(anyhow::anyhow!("Invalid id_user: {}", v))
        } ;

        Ok(())
    }

    /// Получить id_user
    pub fn id_user(&self) ->u32 {
        self.id_user
    }

    /// Проверка id_user
    pub fn check_id_user(&self) ->Result<()> {
        match self.id_user() {
            id if id == 0 => return Err(anyhow::anyhow!("Invalid id_user: {}", id)),
            _ => Ok(())
        }
    }

    /// Установка name
    pub fn set_name(&mut self, name: &str) ->Result<()> {
        
        self.name = match name.trim() {
            n if n.is_empty() => return Err(anyhow::anyhow!("User name is empty")),
            n if n.validate_length(
                            Some(MIN_LENGTH_NAME),
                            Some(MAX_LENGTH_NAME),
                            None
                        ) => n.to_owned(),
            n => return Err(anyhow::anyhow!("Invalid length: {} of user name: {}", n.chars().count(), n)),
        } ;

        Ok(())
    }

    /// Получить name
    pub fn name(&self) ->&str {
        &self.name
    }

    /// Установка email
    pub fn set_email(&mut self, email: &str) ->Result<()> {

        self.email = match email.trim() {
            e if e.is_empty() => return Err(anyhow::anyhow!("Email is empty.")),
            e if ! e.validate_length(
                            Some(MIN_LENGTH_EMAIL),
                            Some(MAX_LENGTH_EMAIL), 
                            None
                        ) => return Err(anyhow::anyhow!("Invalid length: {} of email: {}",e.chars().count(), e)),
            e if e.validate_email() => e.to_owned(),
            e => return Err(anyhow::anyhow!("Invalid email: {}", e)),
        } ;

        Ok(())
    }

    /// Получить email
    pub fn email(&self) ->&str {
        &self.email
    }

    /*
    /// Создать пользователя в DB
    pub async fn create_user(
                &mut self,
                db:    &Database,                
                name:  &str,
                email: &str,
            ) ->Result<()> {

        let mut user_new = User::default() ;

        user_new.set_name(name)? ;
        
        user_new.set_email(email)? ;

        let mut trans = 
                db
                    .pool
                    // Устанавливает соединение и немедленно начинает новую транзакцию.
                    .begin()
                    .await? ;

        *self = sqlx::query_as::<_, User>(
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

        self
            .validate()
            .map_err(|err|
                anyhow::anyhow!("{}", err)
            )? ;

        trans
            // Подтверждает эту транзакцию или точку сохранения.
            .commit()
            .await? ;

        Ok(())
    }
     */

    /// Создать пользователя в DB
    pub async fn create_user(
                db:    &Database,                
                name:  &str,
                email: &str,
            ) ->Result<User> {

        let mut user_new = User::default() ;

        user_new.set_name(name)? ;
        
        user_new.set_email(email)? ;

        let mut trans = 
                db
                    .pool
                    // Устанавливает соединение и немедленно начинает новую транзакцию.
                    .begin()
                    .await? ;

        // Поиск пользователя перед его созданием
        match sqlx::query_as::<_, User>(
            r#"
            select *
            from users
            where email = ?
            "#
        )
        .bind(user_new.email())
        .fetch_one(&mut *trans)
        .await {
            Ok(u) => { // пользователь найден
                u.validate()? ;
                Ok(u)
            },
            Err(sqlx::Error::RowNotFound) => {  // пользователь не найден
                // создание пользователя
                sqlx::query(
                    r#"
                    insert into users (name, email)
                    values (?, ?)
                    "#
                )
                .bind(user_new.name())
                .bind(user_new.email())
                .execute(&mut *trans)
                .await? ;

                // поиск созданного пользователя
                user_new = sqlx::query_as::<_, User>(
                    r#"
                    select *
                    from users
                    where email = ?
                    "#
                )
                .bind(user_new.email())
                .fetch_one(&mut *trans)
                .await? ;

                user_new
                    .validate()? ;

                //Self::f(&mut *trans) ;

                trans
                    // Подтверждает эту транзакцию или точку сохранения.
                    .commit()
                    .await? ;

                Ok(user_new)
            },
            Err(err) => Err(err.into()),
        }
    }

    /// Поиск пользователя по email
    pub async fn find_for_email(
                        trans: &mut sqlx::MySqlConnection,        
                        email: &str, 
                    ) ->Result<Option<User>> {
        // Проверка email
        if ! email.validate_email() {
            return Err(anyhow::anyhow!("Invalid email: {}", email)) ;
        }

        // Поиск пользователя по email
        match sqlx::query_as::<_, User>(
            r#"
            select *
            from user
            where email = ?
            "#
        )
        .bind(email)
        .fetch_one(&mut *trans)
        .await {
            Ok(u) => {  // пользователь найден
                u.validate()? ;
                Ok(Some(u))
            },
            Err(sqlx::Error::RowNotFound) => Ok(None),  // пользователь не найден
            Err(err) => Err(err.into()),    // возникла ошибка
        }
    }
}