use anyhow::Result ;

use sqlx::FromRow ;

use validator::{
        Validate,
        //ValidateLength
    } ;

use crate::roles ;

use crate::users ;

//use crate::db::Database ;
//use crate::users::User;

#[derive(
    Default,
    Validate,
    FromRow,
 )
]
pub struct UsersRoles {

    id_user: u32,

    #[validate(
        length(
            min = roles::MIN_LENGTH_SLUG,
            max = roles::MAX_LENGTH_SLUG,
        )
     )
    ]
    slug:           String,    
}

impl UsersRoles {

    /// Установть id_user
    pub fn set_id_user(&mut self, id_user: u32) ->Result<()> {

        let mut user_tmp = users::User::default() ;

        user_tmp.set_id_user(id_user)? ;

        self.id_user = user_tmp.id_user() ;

        Ok(())
    }

    /// Получить id_user
    pub fn id_user(&self) ->u32 {
        self.id_user
    }

    /// Установить slug
    pub fn set_slug(&mut self, slug: &str) ->Result<()> {
        
        let mut role_tmp = roles::Role::default() ;

        role_tmp.set_slug(slug)? ;

        self.slug = role_tmp.slug().to_owned() ;

        Ok(())
    }

    /// Получить slug
    pub fn slug(&self) ->&str {
        &self.slug
    }

    /// Поиск роли для пользователя
    pub async fn find(
                    trans: &mut sqlx::MySqlConnection,
                    id_user:    u32,
                    slug:       &str,
                 ) ->Result<Option<UsersRoles>> {

        let mut new_users_roles = UsersRoles::default() ;

        new_users_roles.set_id_user(id_user)? ;

        new_users_roles.set_slug(slug)? ;

        match sqlx::query_as::<_, UsersRoles>(
                r#"
                select *
                from users_roles
                where id_user = ? and slug = ?
                "#
            )
            .bind(new_users_roles.id_user())
            .bind(new_users_roles.slug())
            .fetch_one(trans)
            .await {
                Ok(ur) => Ok(Some(ur)),
                Err(sqlx::Error::RowNotFound) => Ok(None),
                Err(err) => Err(err.into())
            }
    }

    /// Обязательный поиск роли для пользователя
    pub async fn find_raise(
                    trans: &mut sqlx::MySqlConnection,
                    id_user:    u32,
                    slug:       &str,
                 ) ->Result<UsersRoles> {
        match Self::find(trans, id_user, slug).await {
            Ok(ur) => match ur {
                Some(ur) => Ok(ur),
                None => Err(anyhow::anyhow!("Not found user role for id_user: {}, slug: {}", id_user, slug)),
            },
            Err(err) => Err(err),
        }
    }

    /// Вставка в role_to_user
    pub async fn ins_role_to_user(
                trans: &mut sqlx::MySqlConnection,
                id_user:    u32,
                slug:       &str,
            ) ->Result<UsersRoles> {

        let mut new_users_roles = UsersRoles::default() ;

        new_users_roles.set_id_user(id_user)? ;

        new_users_roles.set_slug(slug)? ;

        // Проверка пользователя по id_user
        users::User::find_for_id_user_raise(trans, new_users_roles.id_user()).await? ;
        // Проверка роли по slug
        roles::Role::find_slug_raise(trans, new_users_roles.slug()).await? ;

        match Self::find(trans, new_users_roles.id_user(), new_users_roles.slug()).await {
            Ok(ur) => {
                match ur {
                    Some(ur) => Ok(ur),
                    None => {
                        sqlx::query(
                            r#"
                            insert into users_roles
                            (id_user, slug)
                            values (?, ?)
                            "#
                        )
                        .bind(new_users_roles.id_user())
                        .bind(new_users_roles.slug())
                        .execute(&mut *trans)
                        .await? ;

                        match Self::find_raise(
                                    trans,
                                    new_users_roles.id_user(),
                                    new_users_roles.slug()
                                ).await {
                            Ok(ur) => {
                                Ok(ur)
                            },
                            Err(err) => Err(err),
                        }
                    }
                }
            },
            Err(err) => Err(err),
        }
    }

    /*
    /// Добавить роль к пользователю
    pub async fn add_role_to_user(
            db:         &Database,
            id_user:    u32,
            slug:       &str,
        ) ->Result<UsersRoles> {

        let mut trans = 
                    db
                    .pool
                    .begin()
                    .await? ;

        match Self::ins_role_to_user(&mut trans, id_user, slug).await {
            Ok(ur) => {
                trans.commit().await? ;
                Ok(ur)
            },
            Err(err) => Err(err),
        }
    }
     */
}