/*

Документация:
https://docs.rs/im/latest/im/hashmap/struct.HashMap.html

Запуск тестов:  cargo test

*/

use im::HashMap ;
use std::collections::HashSet ;

/// Пользователь
#[derive(
        Clone, 
        Debug,
        PartialEq,
    )
]
pub struct User {
    /// id польльзователя
    id:         usize,
    /// nickname польльзователя
    nickname:   String,
}

/// Функционал репозитария пользователей
pub trait UsersRepository {
    
    /// Получить пользователя по его id
    fn get_user_by_id(&self, id: usize) ->Option<&User> ;

    /// Получить пользователей по их ids
    fn get_users_by_ids(&self, ids: &[usize]) ->Vec<&User> ;

    /// Получить ids пользователей по поисковой фразе
    fn get_users_by_nickname(&self, phrase: &str) ->HashSet<usize> ;
}

/// Репозитарий пользователей
struct Users(HashMap<usize, User>) ;

// Реализация функционала репозитария пользователей
impl Users {
    /// Создать новоый репозитарий пользователя
    pub fn new(list_users: Vec<User>) ->Self {
        Self(
            list_users
            .into_iter()
            .map(|u| (u.id, u))
            .collect()
        )
    }
}

// Реализация trait UsersRepository для репозитария пользователей
impl UsersRepository for Users {

    /// Получить пользователя по его id
    fn get_user_by_id(&self, id: usize) ->Option<&User> {
        match self.0.get_key_value(&id) {
            Some((_, u)) => Some(u),
            None => None
        }
    }

    /// Получить пользователей по их ids
    fn get_users_by_ids(&self, ids: &[usize]) ->Vec<&User> {
        ids
            .iter()
            .filter_map(|i|
                self.0
                    .get(i)
            )
            .collect()
    }

    /// Получить ids пользователей по поисковой фразе
    fn get_users_by_nickname(&self, phrase: &str) ->HashSet<usize> {
        // привести поисковую фразу к нижнему регистру (тольуко одно преобразование)
        let phrase_to_lowercase = phrase.to_lowercase() ;

        self.0
            .iter()
            .filter(|(_, u)|    // нужен только User
                u
                    .nickname
                    .to_lowercase() // привести nickname к нижнему регистру
                    .contains(  // проверка поисковой фразы с nickname
                        &phrase_to_lowercase
                    )
            )
            .map(|(i, _)| *i) // нужен usize
            .collect()
    }
}


fn main() {

    let users = Users::new(vec![
                            User{id: 1, nickname: "user1".to_owned()},
                            User{id: 2, nickname: "user2".to_owned()},
                            User{id: 3, nickname: "user23".to_owned()},
                        ]) ;

    let id = 1;
    println!("user for id: {} -> {:#?}", id, users.get_user_by_id(id)) ;

    let list_ids = &[1, 2] ;
    let list_users = users.get_users_by_ids(list_ids) ;
    println!("\nList users id: {:#?}", list_users) ;

    let phrase = "er2" ;
    let phrase_ids = users.get_users_by_nickname(phrase) ;
    println!("\nphrase: {} -> {:?}", phrase, phrase_ids) ;

}

#[cfg(test)]
mod tests {
    use super::* ;

    fn make_users() ->Users {
       Users::new(vec![
                                User{id: 1, nickname: "user1".to_owned()},
                                User{id: 2, nickname: "user2".to_owned()},
                                User{id: 3, nickname: "user23".to_owned()}
                            ]
                ) 
    }

    /// Проверка поиска по User's id
    #[test]
    fn check_users_by_ids() {

        // получить список пользователей
        let users = make_users() ;

        let id = 1 ;
        let user_name = "user1" ;

        let user = User{
                            id: id,
                            nickname: user_name.to_owned()
                        } ;

        assert_eq!(
            users.get_user_by_id(id),
            Some(&user)
        ) ;

        assert_eq!(
            users.get_user_by_id(0),
            None
        ) ;        
    }

    /// Проверка поиска пользователя по списку ids
    #[test]
    fn test_users_by_ids() {
        // получить список пользователей
        let users = make_users() ;

        // перечень пользователей для поиска
        let list_id_user = &[3, 2] ;
        
        // поиск пользователей
        let found_ids = users.get_users_by_ids(list_id_user) ;
        
        // сравнение количества из list_id_user и found_ids
        assert_eq!(found_ids.len(), list_id_user.len()) ;

        // проверка id User в обеих местах: list_id_user и found_ids
        for u in users.get_users_by_ids(list_id_user) {
            assert!(list_id_user.contains(&u.id)) ;
        }
    }

    /// Проверка поиска пользователя по nickname
    #[test]
    fn check_users_by_nickname() {
        // получить список пользователей
        let users = make_users() ;

        let phrase = "er2" ;

        let list_id_user = users.get_users_by_nickname(phrase) ;

        // проверка количества найденных элементов
        assert_eq!(list_id_user.len(), 2) ;

        // проверка nickname на содержание phrase
        for id in list_id_user {
            match users.get_user_by_id(id) {
                Some(u) => {
                    assert!(
                        u.nickname.to_lowercase().contains(&phrase.to_lowercase())
                    ) ;
                },
                None => panic!("Not found user for id: {}", id),
            }    
        }
    }

}