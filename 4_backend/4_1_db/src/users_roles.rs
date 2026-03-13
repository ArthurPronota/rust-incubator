use anyhow::Result ; // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок

use sqlx::FromRow ; // Импорт трейта FromRow из крейта sqlx для автоматического преобразования строк базы данных в структуры Rust 

use validator::Validate ;   // Импорт трейта Validate из крейта validator для валидации полей структур

use crate::roles ;  // Импорт модуля roles из текущего крейта

use crate::users ;  // Импорт модуля users из текущего крейта

use crate::db ;     // Импорт модуля db из текущего крейта (crate) для работы с базой данных

/// Роля для пользователей
#[derive(   // Автоматическая реализация стандартных трейтов для структуры
    Default,    // Реализует метод default() для создания экземпляра со значениями по умолчанию
    Validate,   // Реализует метод validate() из крейта validator для проверки полей
    FromRow,    // Реализует преобразование строки из БД в структуру (из крейта sqlx)
 )
]
pub struct UsersRoles {

    /// Код пользователя
    id_user: u32,

    /// Код роли
    #[validate(
        length(
            min = roles::MIN_LENGTH_SLUG,
            max = roles::MAX_LENGTH_SLUG,
        )
     )
    ]
    slug:           String,    
}

/// Реализация методов для UsersRoles
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
                    is_lock:    bool,  // признак необходимости блокировки
                 ) ->Result<Option<UsersRoles>> {

        let mut new_users_roles = UsersRoles::default() ;

        new_users_roles.set_id_user(id_user)? ;

        new_users_roles.set_slug(slug)? ;

        match sqlx::query_as::<_, UsersRoles>(
                format!(
                    r#"
                    select *
                    from users_roles
                    where id_user = ? and slug = ?
                    {}
                    "#
                    ,
                    is_lock.then_some(db::FOR_UPDATE).unwrap_or("")
                ).as_str()
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
                    is_lock:    bool,  // признак необходимости блокировки
                 ) ->Result<UsersRoles> {
        match Self::find(trans, id_user, slug, is_lock).await {
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
        users::User::find_for_id_user_raise(trans, new_users_roles.id_user(), false).await? ;
        // Проверка роли по slug
        roles::Role::find_slug_raise(trans, new_users_roles.slug(), false).await? ;

        match Self::find(trans, new_users_roles.id_user(), new_users_roles.slug(), true).await {
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

                        Self::find_raise(
                                    trans,
                                    new_users_roles.id_user(),
                                    new_users_roles.slug(),
                                    false
                                )
                                .await
                    }
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Получить все роли для пользователя
    pub async fn find_all_user_roles(
                    trans: &mut sqlx::MySqlConnection,
                    id_user:    u32,
                ) ->Result<Vec<UsersRoles>> {

        let mut user_tmp = users::User::default() ;

        user_tmp.set_id_user(id_user)? ;

        Ok(sqlx::query_as::<_, UsersRoles>(
            r#"
                  select *
                  from users_roles
                  where id_user = ?
                "#
            )
            .bind(id_user)
            .fetch_all(&mut *trans)
            .await? 
        )    
    }

    /// Удалить роль у пользователя
    pub async fn del_role_from_user(
                    trans: &mut sqlx::MySqlConnection,
                    id_user:    u32,
                    slug:       &str,
                 ) ->Result<()> {
        let mut new_user_role = UsersRoles::default() ;

        new_user_role.set_id_user(id_user)? ;

        new_user_role.set_slug(slug)? ;

        // Проверка пользователя по id_user
        users::User::find_for_id_user_raise(&mut *trans, id_user, false).await? ;
        // Проверка роли по slug
        roles::Role::find_slug_raise(&mut *trans, slug, false).await? ;

        let all_roles = 
                Self::find_all_user_roles(&mut *trans, id_user)
                        .await? ;

        // У пользователя должна остаться хотя бы одна роль.
        if all_roles.len() <= 1 {
            return Err(anyhow::anyhow!("Invalid number: {} of roles for id_user: {}", all_roles.len(), new_user_role.id_user()));
        }

        if all_roles
            .iter()
            .filter(|v|
                v.slug == new_user_role.slug()
            )
            .collect::<Vec<_>>()
            .len() != 1 {
              return Err(anyhow::anyhow!("There is no role: {} for id_user: {}", new_user_role.slug(), new_user_role.id_user()));
        }

        sqlx::query(
            r#"
            delete from users_roles
            where id_user = ?
            and slug = ?
            "#
        )
        .bind(new_user_role.id_user())
        .bind(new_user_role.slug())
        .execute(&mut *trans)
        .await? ;

        Self::find(
                &mut *trans,
                new_user_role.id_user(),
                new_user_role.slug(),
                false
                )
                .await?
                .map_or(
                    Ok(()), |_|
                    Err(anyhow::anyhow!("Role: {} for user: {} is not deleted", new_user_role.slug(), new_user_role.id_user()))
                )
    }
}