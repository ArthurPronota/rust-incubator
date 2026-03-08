//use std::f64::consts::E;

use anyhow::Result ;

use sqlx::FromRow ;

use validator::{Validate, ValidateLength} ;

use crate::db::Database ;


pub const MIN_LENGTH_SLUG: u64 = 1 ;

pub const MAX_LENGTH_SLUG: u64 = 50 ;

pub const MIN_LENGTH_NAME: u64 = 1 ;

pub const MAX_LENGTH_NAME: u64 = 255 ;

pub const MIN_LENGTH_PERMISSIONS: u64 = 1 ;

pub const MAX_LENGTH_PERMISSIONS: u64 = 255 ;

// slug по умолчанию
const SLUG_DEFAULT: &str = "default" ;

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

impl Role {

    /// Установить slug
    pub fn set_slug(&mut self, slug: &str) ->Result<()> {

        self.slug = match slug.trim() {
            sl if sl.is_empty() => return Err(anyhow::anyhow!("slug is empty")),
            sl if sl.validate_length(
                            Some(MIN_LENGTH_SLUG), 
                            Some(MAX_LENGTH_SLUG),
                            None
                        ) => sl.to_owned(),
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

    /// Создать роль
    pub async fn create_role(
                    db:             &Database,
                    slug:           &str,
                    name:           &str,
                    permissions:    &str,
                ) ->Result<Role> {
        
        let mut new_role = Role::default() ;
        
        new_role.set_slug(slug)? ;

        new_role.set_name(name)? ;

        new_role.set_permissions(permissions)? ;

        let mut trans = 
                    db
                        .pool
                        .begin()
                        .await? ;

        sqlx::query(
            r#"
            insert into roles (slug, name, permissions)
            values (?, ?, ?)
            "#
        )
        .bind(new_role.slug())
        .bind(new_role.name())
        .bind(new_role.permissions())
        .execute(&mut *trans)
        .await? ;

        new_role = sqlx::query_as::<_, Role>(
            r#"
                select *
                from roles
                where slug = ?
            "#
        )
        .bind(new_role.slug())
        .fetch_one(&mut *trans)
        .await? ;

        new_role
            .validate()
            .map_err(|err| 
                anyhow::anyhow!("{}", err)
            )? ;

        trans
            .commit()
            .await? ;

        Ok(new_role)
    }

    pub async fn create_default_role(db: &Database) ->Result<Role> {
        Self::create_role(
                db,
                SLUG_DEFAULT,
                "Default role",
                "access,read,write",
            )
            .await
    }
}