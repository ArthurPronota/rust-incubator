//use std::f64::consts::E;

//use core::sync;

use anyhow::{Result} ;

use sqlx::FromRow ;

use validator::{Validate, ValidateLength} ;

use crate::{db::Database, 
            // users_roles::UsersRoles
        } ;


pub const MIN_LENGTH_SLUG: u64 = 1 ;

pub const MAX_LENGTH_SLUG: u64 = 50 ;

pub const MIN_LENGTH_NAME: u64 = 1 ;

pub const MAX_LENGTH_NAME: u64 = 255 ;

pub const MIN_LENGTH_PERMISSIONS: u64 = 1 ;

pub const MAX_LENGTH_PERMISSIONS: u64 = 255 ;

// slug по умолчанию
pub const SLUG_DEFAULT: &str = "default" ;
// name по умолчанию
pub const NAME_DEFAULT: &str = "reader" ;
// permission по умолчанию
pub const PERMISSION_DEFAULT: &str = "read,write" ;


/// Роль
#[derive(
    Default,
    Validate,
    FromRow,
)]
pub struct Role {

    #[validate(
        length(
            min = MIN_LENGTH_SLUG,
            max = MAX_LENGTH_SLUG,
        )
     )
    ]
    slug:           String,

    #[validate(
        length(
            min = MIN_LENGTH_NAME,
            max = MAX_LENGTH_NAME,
        )
     )
    ]
    name:           String,

    #[validate(
        length(
            min = MIN_LENGTH_PERMISSIONS,
            max = MAX_LENGTH_PERMISSIONS,
        )
      )
    ]
    permissions:    String,
}

// Реализация Display для Role
impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Role: #{}: {} perm: {}", 
            self.slug(),
            self.name(),
            self.permissions(),
        )? ;

        Ok(())
    }
}

impl Role {

    /// Установить slug
    pub fn set_slug(&mut self, slug: &str) ->Result<()> {

        self.slug = match slug.trim() {
            sl if sl.is_empty() => return Err(anyhow::anyhow!("slug is empty")),
            sl if sl.validate_length(
                            Some(MIN_LENGTH_SLUG), 
                            Some(MAX_LENGTH_SLUG),
                            None
                        ) => {
                // Проверка верхнего регистра в slug.
                if sl != sl.to_lowercase() {
                    return Err(anyhow::anyhow!("The slug is made in upper caseЖ {}", sl));
                }

                // Элемент слизняка пуст
                if sl
                    .split('-')
                    .any(|v| v.is_empty()) {
                  return Err(anyhow::anyhow!("Invalid slug value: {}", sl));
                }

                sl.to_owned()
            },
            sl => return Err(anyhow::anyhow!("Invalid length: {} of slug", sl.chars().count())),
        } ;

        Ok(())
    }

    /// Получить slug
    pub fn slug(&self) ->&str {
        &self.slug
    }

    /// Уситановить name
    pub fn set_name(&mut self, name: &str) ->Result<()> {
        self.name = match name.trim() {
            n if n.is_empty() => return Err(anyhow::anyhow!("name is empty")),
            n if n.validate_length(
                        Some(MIN_LENGTH_NAME),
                        Some(MAX_LENGTH_NAME),
                        None
                        ) => n.to_owned(),
            n => return Err(anyhow::anyhow!("Invalid length: {} of name", n.chars().count())),
        } ;

        Ok(())
    }

    /// Получить name
    pub fn name(&self) ->&str {
        &self.name
    }

    /// Установить permissions
    pub fn set_permissions(&mut self, permissions: &str) ->Result<()> {

        self.permissions = match permissions.trim() {
            pr if pr.is_empty() => return Err(anyhow::anyhow!("permissions is empty")),
            pr if pr.validate_length(
                    Some(MIN_LENGTH_PERMISSIONS),
                    Some(MAX_LENGTH_PERMISSIONS),
                    None
                ) => {
                    let mut v_perm = 
                            pr
                                .split(',')
                                .filter(|v| 
                                    ! v.trim().is_empty()
                                )
                                .map(|v| v.trim().to_owned())
                                .collect::<Vec<_>>() ;

                    v_perm.sort();

                    v_perm.dedup();

                    if v_perm.is_empty() {
                        return Err(anyhow::anyhow!("permissions is empty"));
                    }

                    match v_perm.join(",") {
                        pr if pr.is_empty() => return Err(anyhow::anyhow!("permissions is empty")),
                        pr if pr.validate_length(
                                        Some(MIN_LENGTH_PERMISSIONS),
                                        Some(MAX_LENGTH_PERMISSIONS),
                                        None
                                    ) => pr,
                        pr => return Err(anyhow::anyhow!("Invalid length: {} of permissions", pr.chars().count())),
                    }
                },
            pr => return Err(anyhow::anyhow!("Invalid length: {} of permissions", pr.chars().count())),
        } ;

        Ok(())
    }

    /// Получить permissions
    pub fn permissions(&self) ->&str {
        &self.permissions
    }

    /*
    /// Прербпазовать permissions в Vec<String>
    pub fn from_permissions_to_vec(&self) ->Vec<String> {
        let mut vec_tmp = self
            .permissions
            .split(",")
            .filter(|v| ! v.trim().is_empty())
            .map(|v| v.trim().to_owned())
            .collect::<Vec<String>>() ;

        vec_tmp.sort();

        vec_tmp.dedup();

        vec_tmp
    }
     */

    /*
    /// Установка permissions из Vec
    pub fn set_permissions_from_vec(&mut self, v_perm: &Vec<String>) ->Result<()> {

        let mut v_tmp_perm = 
                    v_perm
                        .iter()
                        .filter(|v| !v.trim().is_empty())
                        .map(|v| v.trim().to_owned())
                        .collect::<Vec<String>>() ;

        v_tmp_perm.sort();
        v_tmp_perm.dedup();

        let perm_new = v_tmp_perm.join(",") ;

        self.set_permissions(&perm_new)
    }
     */
    
    pub async fn create_default_role(db: &Database) ->Result<Role> {
        Self::create_role(
                db,
                SLUG_DEFAULT,
                NAME_DEFAULT,
                PERMISSION_DEFAULT,
            )
            .await
    }

    /// Поиск роли по slug
    pub async fn find_slug(
                    trans: &mut sqlx::MySqlConnection,
                    slug:  &str,
                 ) ->Result<Option<Role>> {
        let mut tmp_role = Role::default() ;

        tmp_role.set_slug(slug)? ;

        match sqlx::query_as::<_, Role>(
                r#"
                select *
                from roles
                where slug = ?
                "#
            )
            .bind(tmp_role.slug())
            .fetch_one(&mut *trans)
            .await {
          Ok(r) => Ok(Some(r)),
          Err(sqlx::Error::RowNotFound) => Ok(None),
          Err(err) => Err(err.into())
        }
    }

    /// Обязательный поиск роли по slug
    pub async fn find_slug_raise(
                    trans: &mut sqlx::MySqlConnection,
                    slug:  &str,
                 ) ->Result<Role> {
        match Self::find_slug(trans, slug).await {
            Ok(rr) => match rr {
                Some(r) => Ok(r),
                None => Err(anyhow::anyhow!("Not found role for slud: {}", slug)),
            },
            Err(err) => Err(err),
        }
    }

    /// Вставить роль
    pub async fn ins_role(
                    trans: &mut sqlx::MySqlConnection,
                    slug:           &str,
                    name:           &str,
                    permissions:    &str,
                 ) ->Result<Role> {
        let mut new_role = Role::default() ;
        
        new_role.set_slug(slug)? ;

        new_role.set_name(name)? ;

        new_role.set_permissions(permissions)? ;

        match Self::find_slug(&mut *trans, slug).await? {
            Some(r) => Ok(r),
            None => {
                sqlx::query(r#"
                    insert into roles (slug, name, permissions)
                    values (?, ?, ?)                
                    "#
                )
                .bind(new_role.slug())
                .bind(new_role.name())
                .bind(new_role.permissions())
                .execute(&mut *trans)
                .await? ;

                Self::find_slug_raise(
                        &mut *trans,
                        new_role.slug()
                    )
                    .await
            },
        }
    }

    /// Модифицировать name в роли
    pub async fn update_name(
                    trans: &mut sqlx::MySqlConnection,
                    slug:           &str,
                    name:           &str,
                ) ->Result<Role> {
        let mut role_tmp = Role::default() ;

        role_tmp.set_slug(slug)? ;

        role_tmp.set_name(name)? ;

        let role_now = 
                Self::find_slug_raise(
                    &mut *trans,
                    role_tmp.slug()
                )
                .await? ;

        if role_now.name() == role_tmp.name() {
            return Ok(role_now);
        }

        sqlx::query(
            r#"
                update roles
                set name = ?
                where slug = ?
                "#
            )
            .bind(role_tmp.name())
            .bind(role_tmp.slug())
            .execute(&mut *trans)
            .await? ;

        Self::find_slug_raise(&mut *trans, role_tmp.slug())
            .await
    }

    pub async fn update_permissions(
                    trans: &mut sqlx::MySqlConnection,
                    slug:           &str,
                    permissions:    &str,
                 ) ->Result<Role> {
        let mut role_tmp = Role::default() ;

        role_tmp.set_slug(slug)? ;

        role_tmp.set_permissions(permissions)? ;

        let role_now = 
                Role::find_slug_raise(&mut *trans, role_tmp.slug())
                .await? ;

        if role_tmp.permissions() == role_now.permissions() {
            return Ok(role_now) ;
        }

        sqlx::query(r#"
                    update roles
                    set permissions = ?
                    where slug = ?
                    "#
                )
                .bind(role_tmp.permissions())
                .bind(role_tmp.slug())
                .execute(&mut *trans)
                .await? ;

        Self::find_slug_raise(
                    &mut *trans,
                    role_tmp.slug()
                )
                .await
    }

    /// Удалить роль 
    pub async fn delete_role(
                    trans: &mut sqlx::MySqlConnection,
                    slug:           &str,
                 ) ->Result<()> {

        let mut role_tmp = Role::default() ;

        role_tmp.set_slug(slug)? ;

        if role_tmp.slug() == SLUG_DEFAULT {
            return Err(anyhow::anyhow!("The default role cannot be deleted."));
        }

        Role::find_slug_raise(
                        &mut *trans,
                        role_tmp.slug()
                    )
                    .await? ;

/*
                select id_user
                from (
                    select ur_2.id_user as id_user, count(*) as count_roles
                    from users_roles ur_2
                    where ur_2.id_user in (
                        select ur_1.id_user
                        from users_roles ur_1
                        where ur_1.slug = "read-data"
                    )
                    group by ur_2.id_user
                ) as res
                where res.count_roles = 1

select ur_2.id_user as id_user, count(*) as count_roles
from users_roles ur_2
where ur_2.id_user = 2
group by ur_2.id_user


*/
        // В случа получения одного значения использовать список: (u32,)
        match sqlx::query_as::<_, (u32,)>(
            r#"
                select id_user
                from (
                    select ur_2.id_user as id_user, count(*) as count_roles
                    from users_roles ur_2
                    where ur_2.id_user in (
                        select ur_1.id_user
                        from users_roles ur_1
                        where ur_1.slug = ?
                    )
                    group by ur_2.id_user
                ) as res
                where res.count_roles = 1
            "#
            )
            .bind(role_tmp.slug())
            .fetch_optional(&mut *trans)
            .await 
        {
            Ok(Some(row)) => {
                Err(
                    anyhow::anyhow!(
                            "You cannot delete role: {} because it is the only one for id_user: {}", 
                            role_tmp.slug(),
                            row.0
                    )
                )
            },
            Ok(None) => {
                sqlx::query(
                    r#"
                    delete from roles
                    where slug = ?
                    "#
                )
                .bind(role_tmp.slug())
                .execute(&mut *trans)
                .await? ;

                match Role::find_slug(
                            &mut *trans,
                            role_tmp.slug()
                    )   
                    .await? 
                {
                   Some(_) => Err(anyhow::anyhow!("The role: {} has not been deleted", role_tmp.slug())),
                   None => Ok(()),
                }
            },
            Err(err) => Err(err.into()),
        }
    }

    /// Создать роль
    pub async fn create_role(
                    db:             &Database,
                    slug:           &str,
                    name:           &str,
                    permissions:    &str,
                ) ->Result<Role> {
        // сформировать роль по умалчанию
        let mut new_role = Role::default() ;
        
        new_role.set_slug(slug)? ;

        new_role.set_name(name)? ;

        new_role.set_permissions(permissions)? ;

        let mut trans = 
                    db
                        .pool
                        // Устанавливает соединение и немедленно начинает новую транзакцию.
                        .begin()
                        .await? ;

        match Self::find_slug(
                    &mut *trans,
                    new_role.slug()
                )
                .await? {
            Some(r) => Ok(r),
            None => {
                Self::ins_role(
                    &mut *trans,
                    new_role.slug(),
                    new_role.name(),
                    new_role.permissions(),
                )
                .await? ;

                match Self::find_slug_raise(
                        &mut *trans,
                        new_role.slug()
                    )
                    .await {
                        Ok(r) => {
                            trans.commit().await? ;
                            Ok(r)
                        },
                        Err(err) => Err(err),
                }
            },
        }
    }   

    /// Получить все slugs
    pub async fn get_all_slugs(
                    trans: &mut sqlx::MySqlConnection,
                 ) ->Result<Vec<String>> {
        Ok(
            sqlx::query_scalar::<_, String>(r#"
                select slug
                from roles
                order by slug
                "#
            )
            .fetch_all(&mut *trans)
            .await?
        )
    }

}