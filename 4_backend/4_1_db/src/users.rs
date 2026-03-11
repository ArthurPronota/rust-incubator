use serde::{Serialize, Deserialize} ;

/*  Ручная реализация FromRow обусловленная типом pub id_user: usize 
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

use crate::{roles::{self, Role}, users_roles};

//use crate::db::Database;

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

    /*
    /// Проверка id_user
    pub fn check_id_user(&self) ->Result<()> {
        match self.id_user() {
            id if id == 0 => return Err(anyhow::anyhow!("Invalid id_user: {}", id)),
            _ => Ok(())
        }
    }
     */

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

    /// Поиск пользователя по id_user
    pub async fn find_for_id_user(
                    trans: &mut sqlx::MySqlConnection,
                    id_user:    u32,
                 ) ->Result<Option<User>> {
        let mut tmp_user = User::default() ;

        tmp_user.set_id_user(id_user)? ;

        match sqlx::query_as::<_, User>(
                r#"
                select *
                from users
                where id_user = ?
                "#
            )
            .bind(tmp_user.id_user())
            .fetch_one(&mut *trans)
            .await {
                Ok(u) => Ok(Some(u)),
                Err(sqlx::Error::RowNotFound) => Ok(None),
                Err(err) => Err(err.into()),
        }
    }

    /// Обязательный поиск пользователя по id_user
    pub async fn find_for_id_user_raise(
                    trans: &mut sqlx::MySqlConnection,
                    id_user:    u32,
                 ) ->Result<User> {
        match Self::find_for_id_user(&mut *trans, id_user).await {
            Ok(ur) => match ur {
                Some(u ) => Ok(u),
                None => Err(anyhow::anyhow!("Not found user for id_user: {}", id_user)),
            },
            Err(err) => Err(err),
        }
    }

    /// Поиск пользователя по email
    pub async fn find_for_email(
                        trans: &mut sqlx::MySqlConnection,
                        email: &str, 
                    ) ->Result<Option<User>> {
        let mut new_user = User::default() ;
        
        new_user.set_email(email)? ;

        // Поиск пользователя по email
        match sqlx::query_as::<_, User>(
            r#"
            select *
            from users
            where email = ?
            "#
        )
        .bind(new_user.email())
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

    /// Обязательный поиск пользователя по email
    pub async fn find_for_email_raise(
                        trans: &mut sqlx::MySqlConnection,
                        email: &str, 
                    ) ->Result<User> {
        match Self::find_for_email(&mut *trans, email).await {
            Ok(ur) => match ur {
                Some(u ) => Ok(u),
                None => Err(anyhow::anyhow!("Not found user for email: {}", email)),
            },
            Err(err) => Err(err),
        }
    }

    /// Вставить пользователя
    pub async fn ins_user(
                    trans: &mut sqlx::MySqlConnection,
                    name:  &str,
                    email: &str,
                 ) ->Result<User> {
        let mut new_user = User::default() ;

        new_user.set_email(email)? ;

        new_user.set_name(name)? ;

        match Self::find_for_email(&mut *trans, new_user.email()).await? {
            Some(u) => Ok(u),
            None => {
                sqlx::query(
                    r#"
                    insert into users (name, email)
                    values (?, ?)                    
                    "#
                )
                .bind(new_user.name())
                .bind(new_user.email())
                .execute(&mut *trans)
                .await? ;

                Self::find_for_email_raise(&mut *trans, new_user.email()).await
            },
        }
    }


    /// Модифицировать имя пользователя
    pub async fn update_name(
                    trans: &mut sqlx::MySqlConnection,
                    name:       &str,
                    id_user:    u32,
                 ) ->Result<User> {

        let mut tmp_user = User::default() ;

        tmp_user.set_name(name)? ;

        tmp_user.set_id_user(id_user)? ;

        let exist_user = 
                User::find_for_id_user_raise(&mut *trans, id_user).await? ;

        if exist_user.name() == tmp_user.name() {
            return Ok(exist_user) ;
        }

        sqlx::query(r#"
                update users
                set name = ?
                where id_user = ?
                "#
            )
            .bind(tmp_user.name())
            .bind(tmp_user.id_user())
            .execute(&mut *trans)
            .await? ;

        Ok(
            User::find_for_id_user_raise(
                    &mut *trans, 
                    tmp_user.id_user()
                )
                .await?
        )
    }

    /// Модифицировать email для пользователя
    pub async fn update_email(
                    trans: &mut sqlx::MySqlConnection,
                    email:      &str,
                    id_user:    u32,
                 ) ->Result<User> {
        let mut user_tmp = User::default() ;

        user_tmp.set_email(email)? ;

        user_tmp.set_id_user(id_user)? ;

        let user_now = 
                Self::find_for_id_user_raise(&mut *trans, id_user)
                    .await? ;

        if user_now.email() == user_tmp.email() {
            return Ok(user_now);
        }   

        sqlx::query(r#"
            update users
            set email = ?
            where id_user = ?
        "#
        )
        .bind(user_tmp.email())
        .bind(user_tmp.id_user())
        .execute(&mut *trans)
        .await? ;

        User::find_for_id_user_raise(
                &mut *trans,
                user_tmp.id_user()
            )
            .await
    }

    /// Удалить пользователя
    pub async fn delete_user(
                    trans: &mut sqlx::MySqlConnection,
                    id_user:    u32,
                 ) ->Result<()> {
        let mut user_tmp = User::default() ;

        user_tmp.set_id_user(id_user)? ;

        User::find_for_id_user_raise(&mut *trans, user_tmp.id_user())
            .await? ;

        sqlx::query(r#"
            delete from users
            where id_user = ?
        "#
        )
        .bind(user_tmp.id_user())
        .execute(&mut *trans)
        .await? ;

        match Self::find_for_id_user(&mut *trans, user_tmp.id_user())
            .await?
        {
            Some(_) => {
                Err(anyhow::anyhow!("The user id_user: {} has not been deleted", user_tmp.id_user()))
            },
            None => Ok(()),
        }
    }

    /// Получить всех id_user
    pub async fn get_all_id_user(trans: &mut sqlx::MySqlConnection,) ->Result<Vec<u32>> {
        Ok(
            sqlx::query_as::<_,(u32,)>(r#"
                    select id_user
                    from users
                    order by id_user
                    "#
            )
            .fetch_all(&mut *trans)
            .await?
            .iter()
            .map(|&u| u.0)
            .collect::<Vec<_>>() 
        )
    }

    /*
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
     */

    /*
        Commands::UserList { id } => {
            if let Some(id) = id {
                if let Some(user) = db.get_user_with_roles(id).await? {
                    println!("{}", user);
                } else {
                    println!("User {} not found", id);
                }
            } else {
                let users = db.list_users_with_roles().await?;
                for user in users {
                    println!("{}", user);
                    println!("---");
                }
            }
        }    
     */
}

/// Пользователь с его ролями
#[derive(Default)]
pub struct UserWithRole {
    user:   User,
    list_roles: Vec<Role>
}


// Реализация Display для UserWithRole
impl std::fmt::Display for UserWithRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "User #{}: {} ({})", 
            self.user.id_user(), 
            self.user.name(),
            self.user.email()
        )? ;

        writeln!(f, "Roles: ")? ;

        if self.list_roles.is_empty() {
            writeln!(f, "  No roles assigned")? ;
        } else {
            for rl in &self.list_roles {
                writeln!(
                    f,
                    "  Role #{}: {} perm: {}",
                    rl.slug(),
                    rl.name(),
                    rl.permissions(),
                )? ;
            }
        }

        Ok(())
    }
}

impl UserWithRole {
    /// Получить данные по пользователю и его роли
    pub async fn get_data(
                    trans: &mut sqlx::MySqlConnection,
                    id_user: u32
                 ) ->Result<UserWithRole> {
        let mut user_with_roles = UserWithRole::default() ;

        user_with_roles.user = 
                User::find_for_id_user_raise(
                    &mut *trans, 
                    id_user
                )
                .await? ;

        //user_with_roles.list_roles = 
        let lr = users_roles::UsersRoles::find_all_user_roles(
                &mut *trans,
                id_user
            )
            .await? ;

        for r in lr {
            let role = 
                    roles::Role::find_slug_raise(
                        &mut *trans,
                        r.slug()
                    )
                    .await? ;

            user_with_roles.list_roles.push(role);
        }

        // сортировка ролей по slug
        user_with_roles
            .list_roles
            .sort_by(|a, b|
                a.slug().cmp(b.slug())
            ) ;

        Ok(user_with_roles)
    }
}