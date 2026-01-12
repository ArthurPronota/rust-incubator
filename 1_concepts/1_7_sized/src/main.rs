use std::borrow::Cow;
//use std::hash::Hash;
use std::rc::Rc ;
/*
// трейт функционала для репозитария (хранилища) пользователей
pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) ->Option<&V>;
    fn remove(&mut self, key: &K) ->Option<V>;
}
 */

// струткура пользователя
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u64,
    pub email: Cow<'static, str>,
    pub activated: bool,
}

// перечисление с ошибками
#[derive(Debug)]
pub enum UserError {
    AlreadyExists,
    NotFound,
    //StorageError,
}

// Команда создания пользователя
pub struct CreateUser {
    pub id: u64,
    pub email: String,
}

// Команда получения данных по пользователю
pub struct GetUser {
    pub id: u64,
}

// Команда удаления данных по пользователю
pub struct DelUser {
    pub id: u64,
}

// трейт функционала любой команды
pub trait Command {}

// реализация трейта команды для создания пользователя
impl Command for CreateUser {}

// реализация трейта команды для получения данных по пользователю
impl Command for GetUser {}

// реализация трейта команды для удаления данных по пользователю
impl Command for DelUser {}

// трейт функционала для пользовательского репозитария для Context
pub trait UserRepository {
    fn add(&self, user: User) -> Result<(), UserError>;// {Ok(())}
    fn get(&self, id: &u64) ->Option<User>; // {None}
    fn remove(&self, id: &u64) ->Option<User>; // {None}
    fn exists(&self, id: &u64) ->bool; // {false}
}

// трейт обработчика команд
pub trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository; // Используем трейт-объект для работы с разными типами
    type Result = Result<(), UserError>;

    fn handle_command(&self, cmd: &CreateUser, user_repo: &Self::Context) -> Self::Result {

        if user_repo.exists(&cmd.id) {
            return Err(UserError::AlreadyExists);
        }

        let new_user = User {
            id: cmd.id,
            email: Cow::Owned(cmd.email.clone()),
            activated: false,
        };

        user_repo.add(new_user)
    }
}

impl CommandHandler<DelUser> for User {
    type Context = dyn UserRepository; // Используем трейт-объект для работы с разными типами
    type Result = Option<User>;

    fn handle_command(&self, cmd: &DelUser, user_repo: &Self::Context) -> Self::Result {
        user_repo.remove(&cmd.id)
    }    
}

impl CommandHandler<GetUser> for User {
    type Context = dyn UserRepository; // Используем трейт-объект для работы с разными типами
    type Result = Option<User>;

    fn handle_command(&self, cmd: &GetUser, user_repo: &Self::Context) -> Self::Result {
        user_repo.get(&cmd.id)
    }
}

/*
// Оборачиваем Storage в RefCell, так как CommandHandler принимает &Context (неизменяемую ссылку),
// а Storage требует &mut self для записи.
use std::cell::RefCell;

pub struct UserRepository<T: Storage<u64, User>> {
    pub storage: RefCell<T>,
}

impl<T: Storage<u64, User>> IUserRepository for UserRepository<T> {

    fn add(&self, user: User) -> Result<(), UserError> {
        self.storage.borrow_mut().set(user.id, user);
        Ok(())
    }

    fn exists(&self, id: u64) -> bool {
        self.storage.borrow().get(&id).is_some()
    }
}
 */

 #[cfg(test)]
 mod test {
    use super::* ;
    use std::cell::RefCell ;
    use std::collections::HashMap ;

    struct MockRepo {
        users:  RefCell<HashMap<u64, User>>,
    }

    impl UserRepository for MockRepo {
        fn add(&self, user: User) -> Result<(), UserError> {
            self
                .users
                .borrow_mut()
                .insert(user.id, user)
                ;
            Ok(())
        }

        fn get(&self, id: &u64) ->Option<User> {
            self
                .users
                .borrow()
                .get(id)
                .map(|x| x.clone())
        }

        fn remove(&self, id: &u64) ->Option<User> {
            self
                .users
                .borrow_mut()
                .remove(id)
        }

        fn exists(&self, id: &u64) ->bool {
            self
                .users
                .borrow()
                .iter()
                .any(|(k, v)| k == id)
        }
    }


 }
/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, collections::HashMap};

    // Ручной Mock-тип для тестирования
    // Тестовый репозитарий для хранения пользователей
    struct MockRepo {
        users: RefCell<Vec<User>>,
        //users:   RefCell<HashMap<u64, User>>
    }

    impl UserRepository for MockRepo {

        fn add(&self, user: User) -> Result<(), UserError> {
            self.users.borrow_mut().push(user);
            Ok(())
        }

        fn exists(&self, id: u64) -> bool {
            self.users.borrow().iter().any(|u| u.id == id)
        }
    }

    #[test]
    fn test_create_user_success() {
        let mock_repo = MockRepo { users: RefCell::new(vec![]) };
        let cmd = CreateUser { id: 1, email: "test@example.com".into() };
        
        // Создаем "пустого" пользователя как исполнителя команды
        let actor = User { id: 0, email: "".into(), activated: false };

        let result = actor.handle_command(&cmd, &mock_repo);

        assert!(result.is_ok());
        assert_eq!(mock_repo.users.borrow().len(), 1);
        assert_eq!(mock_repo.users.borrow()[0].id, 1);
    }

    #[test]
    fn test_create_user_already_exists() {
        let existing_user = User { id: 1, email: "old@test.com".into(), activated: true };
        let mock_repo = MockRepo { users: RefCell::new(vec![existing_user]) };
        
        let cmd = CreateUser { id: 1, email: "new@test.com".into() };
        let actor = User { id: 0, email: "".into(), activated: false };

        let result = actor.handle_command(&cmd, &mock_repo);

        match result {
            Err(UserError::AlreadyExists) => (),
            _ => panic!("Should have failed with AlreadyExists"),
        }
    }
}
 */

fn main() {
    println!("Implement me!");
}
