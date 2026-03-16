use serde::{
        Serialize,  // Трейт для преобразования структуры в различные форматы (JSON, YAML, и т.д.)
        Deserialize // Трейт для создания структуры из различных форматов
    } ;

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

use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок
use sqlx::FromRow ; // Импорт трейта FromRow из крейта sqlx для преобразования строк БД в структуры
use validator::{
        Validate,   // Основной трейт для валидации структур с методом validate()
        ValidateEmail,  // Трейт для валидации email адресов (используется с #[validate(email)])
        ValidateLength, // Трейт для валидации длины строк/коллекций (используется с #[validate(length)])
    } ;

use crate::{
        db, // Модуль для работы с базой данных (подключение, пул соединений)
        roles::{    // Модуль для работы с ролями пользователей
            self,   // Импорт самого модуля roles для доступа к его функциям (roles::get_default())
            Role    // Импорт структуры Role для прямого использования без префикса roles::
        },
        users_roles // Модуль для работы со связями пользователей и ролей (таблица users_roles)
};

/// Минимальная длина наименование пользователя
pub const MIN_LENGTH_NAME: u64 = 1 ;

/// Максимальная длина наименование пользователя
pub const MAX_LENGTH_NAME: u64 = 255 ;

/// Минимальная длина email
pub const MIN_LENGTH_EMAIL: u64 = 5 ;

/// Максимальная длина email
pub const MAX_LENGTH_EMAIL: u64 = 255 ;

/// Структура пользователя
#[derive(
    Debug,  // Реализует форматирование {:?} для вывода в консоль и отладки
    Default,    // Реализует метод default() для создания экземпляра со значениями по умолчанию
    Serialize,  // Реализует сериализацию структуры в форматы JSON/YAML/etc. (из крейта serde)
    Deserialize, // Реализует десериализацию структуры из форматов JSON/YAML/etc. (из крейта serde)
    FromRow,    // Реализует преобразование строки из БД в структуру (из крейта sqlx)
    Validate,   // Реализует метод validate() для валидации полей структуры (из крейта validator)
    Clone,
 )
]
pub struct User {
    
    /// Код пользователя
    id_user: u32,

    /// Натменование пользователя
    #[validate(
        length(
            min = MIN_LENGTH_NAME, 
            max = MAX_LENGTH_NAME,
        )
      )
    ]
    name:    String,

    /// email пользователя
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

    /// Установка name
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

    /// Поиск пользователя по id_user
    pub async fn find_for_id_user(
                    trans: &mut sqlx::MySqlConnection,
                    id_user:    u32,
                    is_lock:    bool,   // признак блокировки
                 ) ->Result<Option<User>> {
        let mut tmp_user = User::default() ;

        tmp_user.set_id_user(id_user)? ;

        match sqlx::query_as::<_, User>(
                format!(
                    r#"
                    select *
                    from users
                    where id_user = ?
                    {}
                    "#
                    ,
                    is_lock.then_some(db::FOR_UPDATE).unwrap_or("")
                ).as_str()
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
                    is_lock:    bool,   // признак блокировки
                 ) ->Result<User> {
        match Self::find_for_id_user(&mut *trans, id_user, is_lock).await {
            Ok(ur) => match ur {
                Some(u ) => Ok(u),
                None => Err(anyhow::anyhow!("Not found user for id_user: {}", id_user)),
            },
            Err(err) => Err(err),
        }
    }

    /// Поиск пользователя по email
    #[allow(dead_code)]
    pub async fn find_for_email(
                        trans: &mut sqlx::MySqlConnection,
                        email: &str,
                        is_lock:    bool,   // признак блокировки
                    ) ->Result<Option<User>> {
        let mut new_user = User::default() ;
        
        new_user.set_email(email)? ;

        // Поиск пользователя по email
        match sqlx::query_as::<_, User>(
            format!(
                r#"
                select *
                from users
                where email = ?
                {}
            "#
            ,
            is_lock.then_some(db::FOR_UPDATE).unwrap_or("")
            ).as_str()
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
    #[allow(dead_code)]
    pub async fn find_for_email_raise(
                        trans: &mut sqlx::MySqlConnection,
                        email: &str, 
                        is_lock:    bool,   // признак блокировки
                    ) ->Result<User> {
        match Self::find_for_email(&mut *trans, email, is_lock).await {
            Ok(ur) => match ur {
                Some(u ) => Ok(u),
                None => Err(anyhow::anyhow!("Not found user for email: {}", email)),
            },
            Err(err) => Err(err),
        }
    }

    /// Вставить пользователя
    #[allow(dead_code)]
    pub async fn ins_user(
                    trans: &mut sqlx::MySqlConnection,
                    name:  &str,
                    email: &str,
                 ) ->Result<User> {
        let mut new_user = User::default() ;

        new_user.set_email(email)? ;

        new_user.set_name(name)? ;

        match Self::find_for_email(&mut *trans, new_user.email(), true).await? {
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

                Self::find_for_email_raise(
                        &mut *trans,
                        new_user.email(),
                        false
                    ).await
            },
        }
    }


    /// Модифицировать имя пользователя
    #[allow(dead_code)]
    pub async fn update_name(
                    trans: &mut sqlx::MySqlConnection,
                    name:       &str,
                    id_user:    u32,
                 ) ->Result<User> {

        let mut tmp_user = User::default() ;

        tmp_user.set_name(name)? ;

        tmp_user.set_id_user(id_user)? ;

        let exist_user = 
                User::find_for_id_user_raise(&mut *trans, id_user, true).await? ;

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
                    tmp_user.id_user(),
                    false
                )
                .await?
        )
    }

    /// Модифицировать email для пользователя
    #[allow(dead_code)]
    pub async fn update_email(
                    trans: &mut sqlx::MySqlConnection,
                    email:      &str,
                    id_user:    u32,
                 ) ->Result<User> {
        let mut user_tmp = User::default() ;

        user_tmp.set_email(email)? ;

        user_tmp.set_id_user(id_user)? ;

        let user_now = 
                Self::find_for_id_user_raise(&mut *trans, id_user, true)
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
                user_tmp.id_user(),
                false
            )
            .await
    }

    /// Удалить пользователя
    #[allow(dead_code)]
    pub async fn delete_user(
                    trans: &mut sqlx::MySqlConnection,
                    id_user:    u32,
                 ) ->Result<()> {
        let mut user_tmp = User::default() ;

        user_tmp.set_id_user(id_user)? ;

        // заблокировать пользователя
        User::find_for_id_user_raise(&mut *trans, user_tmp.id_user(), true)
            .await? ;

        sqlx::query(r#"
            delete from users
            where id_user = ?
        "#
        )
        .bind(user_tmp.id_user())
        .execute(&mut *trans)
        .await? ;

        // проверка наличия пользователя
        Self::find_for_id_user(
                &mut *trans,
                user_tmp.id_user(),
                false
            )
            .await?
            .map_or(
                Ok(()),
                |_| Err(anyhow::anyhow!("The user id_user: {} has not been deleted", user_tmp.id_user()))
            )
    }

    /// Получить всех id_user
    #[allow(dead_code)]
    pub async fn get_all_id_user(trans: &mut sqlx::MySqlConnection,) ->Result<Vec<u32>> {
        Ok(
            sqlx::query_scalar(r#"
            select id_user
            from users
            order by id_user
            "#
            )
            .fetch_all(&mut *trans)
            .await?
        )
    }
}

/// Пользователь с его ролями
#[derive(
    Default,
    Serialize,
    Deserialize,
    Debug,
    Clone,
  )
 ]
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
    #[allow(dead_code)]
    pub async fn get_data(
                    trans: &mut sqlx::MySqlConnection,
                    id_user: u32
                 ) ->Result<UserWithRole> {
        let mut user_with_roles = UserWithRole::default() ;

        // получить данные по пользователю
        user_with_roles.user = 
                User::find_for_id_user_raise(
                    &mut *trans, 
                    id_user,
                    false
                )
                .await? ;

        // получить список пользовательских ролей
        for r in users_roles::UsersRoles::find_all_user_roles(
                                    &mut *trans,
                                    id_user
                                )
                                .await? 
        {
            // заполнить данными по ролям
            user_with_roles
                .list_roles
                .push(
                    roles::Role::find_slug_raise(
                        &mut *trans,
                        r.slug(),
                        false
                    )
                    .await?                    
                );
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