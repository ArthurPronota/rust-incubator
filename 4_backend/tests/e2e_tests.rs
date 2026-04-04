// Позволяет создавать сериализованные тесты Rust.

// Подклбчение модуля common из родительского дирректория
#[path = "../src/common.rs"]
mod common;

// Подклбчение модуля db из родительского дирректория
#[path = "../src/db.rs"]
mod db ;

// Подклбчение модуля users из родительского дирректория
#[path = "../src/users.rs"]
mod users ;

// Подклбчение модуля graphql_client из родительского дирректория
#[path = "../src/graphql_client.rs"]
mod graphql_client ;

// данные по первому тестовому пользователю
const USERNAME_1: &str = "test_user_1" ;
const PASSWORD_1: &str = "password_1" ;

// данные по второму тестовому пользователю
const USERNAME_2: &str = "test_user_2" ;
const PASSWORD_2: &str = "password_2" ;

// Удаление тестовых пользователей перед началом тестов
async fn test_before_del_test_users() {

    let (_, // port_http, 
         _, // host_http, 
         db_path,
         ..
    ) = common::get_all_env_vars().unwrap() ;

    // Создание нового пула соединений с базой
    let db_res = db::Database::new(&db_path).await.unwrap() ;

    // сформировать новую транзакцию
    let mut trans = 
                db_res
                    .pool
                    .begin()
                    .await
                    .unwrap() 
                    ;

    let list_user_names = vec![
            USERNAME_1,
            USERNAME_2
        ] ;

    for name in list_user_names {
        if let Some(usr) = users::Users::find_for_name(
                &mut *trans,
                name,
                true,
            )
            .await
            .unwrap() {
                users::Users::del_user_by_id(
                    &mut *trans,
                    usr.id_user(),
                )
                .await
                .unwrap()
                ;
        }
    }

    assert!(trans.commit().await.is_ok()) ;
}

// Тест регистрации нового пользователя
async fn test_user_register() {
    // получить параметры сервера
    let (port_http, 
         host_http, 
         ..
        ) = common::get_all_env_vars().unwrap() ;

    // создание GraphQLClient 
    let mut gr_client = 
                graphql_client::GraphQLClient::new(&host_http, port_http).unwrap() ;

    for (nm, pass) in [(USERNAME_1, PASSWORD_1), (USERNAME_2, PASSWORD_2)] {
        assert!(gr_client.register(nm, pass).is_ok()) ;
    }
}

// Тест логина пользователя
async fn test_login() {

    // получить параметры сервера
    let (port_http, 
         host_http, 
         ..
        ) = common::get_all_env_vars().unwrap() ;

    // создание GraphQLClient 
    let mut gr_client = 
                graphql_client::GraphQLClient::new(&host_http, port_http).unwrap() ;

    for (nm, pass) in [(USERNAME_1, PASSWORD_1), (USERNAME_2, PASSWORD_2)] {
        assert!(gr_client.login(nm, pass).is_ok()) ;
    }
}

// добавить друга
async fn add_friend() {

    // получить параметры сервера
    let (port_http, 
         host_http, 
         ..
        ) = common::get_all_env_vars().unwrap() ;

    // создание GraphQLClient 
    let mut gr_client = 
                graphql_client::GraphQLClient::new(&host_http, port_http).unwrap() ;

    let (
            _, // user_info_1, 
            jwt_raw
        ) = gr_client.login(USERNAME_1, PASSWORD_1).unwrap() ;

    let jwt = jwt_raw.to_string() ;

    let (user_info_2, _) = gr_client.login(USERNAME_2, PASSWORD_2).unwrap() ;
    
    gr_client.add_friend(user_info_2.id, &jwt).unwrap() ;
}

// Печать себя, своих друзей и друзей друзей
async fn show_friends() {

    // получить параметры сервера
    let (port_http, 
         host_http, 
         ..
        ) = common::get_all_env_vars().unwrap() ;

    // создание GraphQLClient 
    let mut gr_client = 
                graphql_client::GraphQLClient::new(&host_http, port_http).unwrap() ;

    let (
            _,  // user_info_1, 
            jwt_raw
        ) = gr_client.login(USERNAME_1, PASSWORD_1).unwrap() ;

    let jwt = jwt_raw.to_owned() ;

    assert!(gr_client.show_friend(&jwt).is_ok()) ;
}

// Удалить друга
async fn del_friend() {

    // получить параметры сервера
    let (port_http, 
         host_http, 
         ..
        ) = common::get_all_env_vars().unwrap() ;

    // создание GraphQLClient 
    let mut gr_client = 
                graphql_client::GraphQLClient::new(&host_http, port_http).unwrap() ;

    let (_, jwt_raw) = gr_client.login(USERNAME_1, PASSWORD_1).unwrap() ;

    let jwt = jwt_raw.to_string() ;

    let (user_info_2, _) = gr_client.login(USERNAME_2, PASSWORD_2).unwrap() ;

    assert!(gr_client.del_friend(user_info_2.id, &jwt).is_ok()) ;
}

// Тест полного жизненного цикла
#[tokio::test]
async fn test_e2e() {
    
    test_before_del_test_users().await ;

    test_user_register().await ;

    test_login().await ;
    
    add_friend().await ;

    show_friends().await ;

    del_friend().await ;
}