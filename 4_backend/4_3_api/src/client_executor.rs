use anyhow::Result ;

use ureq::{
        self,
        http::StatusCode
    } ;

use crate::args::Command ;

use crate::roles::Role;
use crate::users::User;
use crate::common ;

const CONTENT_TYPE: &str = "Content-Type" ;
const JSON_TYPE: &str =  "application/json" ;

/// Выполнить любую команду
pub fn any_command(
        args:   &Command,
        host:   &str,
        port:   u32,
       ) ->Result<()> {

    // Выполнение команд на основе аргументов CLI
    let mut resp = match args {
        // Создание в DB необходимых объектов
        Command::InitDb => {
            // документация https://docs.rs/ureq/latest/ureq/
            ureq::get(&common::get_initdb_url(host, port))
                // Отправляет запрос и блокирует вызывающего до получения ответа.
                .call()?
        },
        // Создание пользователя
        Command::CreateUser(arg_unit)  => {
            let mut tmp_user = User::default() ;
            // проверка имени пользователя
            tmp_user.set_name(&arg_unit.name)? ;
            // проверка email пользователя
            tmp_user.set_email(&arg_unit.email)? ;

            ureq::post(common::get_create_user_url(host, port))
                .header(CONTENT_TYPE, JSON_TYPE)
                .send_json(&arg_unit)? 
        },
        // Удаление пользователя
        Command::DeleteUser(arg_unit) => {
            let mut tmp_user = User::default() ;

            tmp_user.set_id_user(arg_unit.id_user)? ;

            ureq::delete(
                common::get_delete_user_url(host, port, tmp_user.id_user())
            )
            .call()?
        },
        // Модифицировать имя у пользователя
        Command::UpdateNameUser(arg_unit) => {
            let mut tmp_user = User::default() ;

            tmp_user.set_id_user(arg_unit.id_user)? ;

            tmp_user.set_name(&arg_unit.new_name)? ;

            ureq::put(common::get_update_username_url(host, port))
                .header(CONTENT_TYPE, JSON_TYPE)
                .send_json(&arg_unit)?
        },
        // Модифицировать email у пользователя
        Command::UpdateEmailUser(arg_unit) => {
            let mut tmp_user = User::default() ;

            tmp_user.set_id_user(arg_unit.id_user)? ;

            tmp_user.set_email(&arg_unit.new_email)? ;

            ureq::put(common::get_update_useremail_url(host, port))
                .header(CONTENT_TYPE, JSON_TYPE)
                .send_json(&arg_unit)?            
        },
        // Показать пользователей и их роли
        Command::ShowUsersRoles(arg_unit) => {
            match arg_unit.id_user {
                // показ одиносного пользователя
                Some(id_user) => {
                    let mut tmp_user = User::default() ;

                    tmp_user.set_id_user(id_user)? ;
                    ureq::get(common::get_show_user_url(host, port, id_user))
                        .call()?
                },
                // показ всех пользователей
                None => ureq::get(common::get_show_users_url(host, port))
                            .call()?
            }
        },
        // Создать роль
        Command::CreateRole(arg_unit) => {
            let mut tmp_role = Role::default() ;
            
            tmp_role.set_name(&arg_unit.name)? ;

            tmp_role.set_permissions(
                        &arg_unit.permissions.join(",")
                    )? ;

            tmp_role.set_slug(&arg_unit.slug)? ;

            ureq::post(common::get_create_role_url(host, port))
                .header(CONTENT_TYPE, JSON_TYPE)
                .send_json(&arg_unit)?
        },
        // Удалить роль
        Command::DeleteRole(arg_unit) => {
            let mut tmp_role = Role::default() ;

            tmp_role.set_slug(&arg_unit.slug)? ;

            ureq::delete(common::get_delete_role_url(host, port, &tmp_role.slug()))
                .call()?
        },
        // Модифицировать имя у роли
        Command::UpdateNameRole(arg_unit) => {
            let mut tmp_role = Role::default() ;

            tmp_role.set_name(&arg_unit.new_name)? ;

            tmp_role.set_slug(&arg_unit.slug)? ;

            ureq::put(common::get_update_rolename_url(host, port))
                .header(CONTENT_TYPE, JSON_TYPE)
                .send_json(&arg_unit)?
        },
        // Модифицировать разрешения у роли
        Command::UpdatePermissionsRole(arg_unit) => {
            let mut tmp_role = Role::default() ;

            tmp_role.set_permissions(&arg_unit.slug)? ;

            tmp_role.set_slug(&arg_unit.slug)? ;

            ureq::put(common::get_update_rolepermissions_url(host, port))
                .header(CONTENT_TYPE, JSON_TYPE)
                .send_json(&arg_unit)?            
        }
        // Показать роли
        Command::ShowRoles(arg_unit) => {
            match &arg_unit.slug {
                // Показ одной роли
                Some(sl) => {
                        let mut tmp_role = Role::default() ;
                        tmp_role.set_slug(&sl)? ;
                        ureq::get(
                            common::get_show_role_url(host, port, sl)
                        )
                        .call()?
                },
                // Показ всех ролей
                None => ureq::get(common::get_show_roles_url(host,port))
                                .call()?,
            }
        },
        // Добавить роль к пользователю
        Command::AddRoleToUser(arg_unit) => {
            let mut tmp_user = User::default() ;

            tmp_user.set_id_user(arg_unit.id_user)? ;

            let mut tmp_role = Role::default() ;

            tmp_role.set_slug(&arg_unit.slug)? ;

            ureq::post(&common::get_add_role_to_user_url(host, port))
                .header(CONTENT_TYPE, JSON_TYPE)
                .send_json(&arg_unit)?
        },
        // Удалить роль у пользователя
        Command::RemoveRoleFromUser(arg_unit) => {
            let mut tmp_user = User::default() ;

            tmp_user.set_id_user(arg_unit.id_user)? ;

            let mut tmp_role = Role::default() ;

            tmp_role.set_slug(&arg_unit.slug)? ;

            ureq::post(&common::get_remove_role_from_user_url(host, port))
                .header(CONTENT_TYPE, JSON_TYPE)
                .send_json(&arg_unit)?            
        },
    } ;

    // Проверка кода возврата
    if !matches!(resp.status(), StatusCode::OK | StatusCode::CREATED) {
        return Err(anyhow::anyhow!("Server error: {}", resp.status())) ;
    }

    // Разбор ответа сервера
    match resp
            .body_mut()
            .read_json::<common::Responce>()? 
    {
        // Команда выполнена успешно
        common::Responce::Success(mess) => println!("{}", mess),
        // Возникла ошибка при выполнении команды
        common::Responce::Error(err) => println!("{}", err),
        // Получены данные по пользователю и его ролям
        common::Responce::UserWithRole(ur) => println!("{}", ur),
        // Получены данные по пользователям и их ролям
        common::Responce::UsersRoles(list_ur) => {
            for u_r in &list_ur {
              println!("{}", u_r) ;
              println!("--------------------------------------------") ;
            }
        },
        // Получены данные по роли
        common::Responce::Role(rl) => println!("{}", rl),
        // Получены данные по ролям
        common::Responce::ListRoles(list_roles) => {
            for rl in &list_roles {
                println!("{}", rl) ;
                println!("--------------------------------------------") ;
            }
        },
    }

    Ok(())
}