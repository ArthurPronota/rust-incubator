fn main() {
    // Статическая диспетчеризация
    {
    use std::borrow::Cow ;
    use std::collections::HashMap ;
    use std::hash::Hash;

    trait Storage<K, V> {
        fn set(&mut self, key: K, val: V);
        fn get(&self, key: &K) ->Option<&V>;
        fn remove(&mut self, key: &K) -> Option<V>;
    }

    #[derive(Debug)]
    struct User {
        id: u64,
        email: Cow<'static, str>,
        activated: bool,
    }

    struct UserRepository<T: Storage<u64, User>> {
        storage:   T
    }

    impl<T> UserRepository<T> 
        where T: Storage<u64, User>
    {
        fn new(storage: T) ->Self {
            Self { storage }
        }

        fn set(&mut self, key: u64, val: User) {
            self.storage.set(key, val);
        }

        fn get(&self, key: u64) ->Option<&User> {
            self
                .storage
                .get(&key)
        }

        fn remove(&mut self, key: &u64) ->Option<User> {
            self
                .storage
                .remove(key)
        }
    }

    impl<K, V> Storage<K, V> for HashMap<K, V> 
        where K: Hash + Eq
    {

        fn get(&self, key: &K) ->Option<&V> {
            // обращается к стандартному методу HashMap, а не к методу трейта, 
            // который вы реализуете.
            // Когда компилятор видит self.get(key):
            // Он сначала ищет метод get среди "родных" методов HashMap.
            // Находит стандартный метод HashMap::get.
            // Использует его.
            // так:
            //self.get(key)
            // или так:
            HashMap::get(&self, key)
        }

        fn set(&mut self, key: K, val: V) {
            self.insert(key, val) ;
        }

        fn remove(&mut self, key: &K) ->Option<V> {
            // обращается к стандартному методу HashMap, а не к методу трейта, 
            // который вы реализуете.
            // Когда компилятор видит self.remove(key):
            // Он сначала ищет метод get среди "родных" методов HashMap.
            // Находит стандартный метод HashMap::remove.
            // Использует его.
            // так:
            // self.remove(key)
            // или так:
            HashMap::remove(self, key)
        }

    }

    let mut repo = UserRepository::new(HashMap::new());

    let id_user = 42u64 ;

    let user = User {
        id: id_user,
        email: Cow::Borrowed("m@n.c"),
        activated: true,
    };    

    repo.set(user.id, user);

    println!("{:?}", repo.get(id_user)) ;

    repo.remove(&id_user) ;

    println!("{:?}", repo.get(id_user)) ;
    }

    // Динамическая диспетчеризация
    {
        use std::borrow::Cow;
        use std::collections::HashMap;

        trait Storage<K, V> {
            fn set(&mut self, key: K, val: V);
            fn get(&self, key: &K) ->Option<&V>;
            fn remove(&mut self, key: &K) ->Option<V>;
        }

        #[derive(Debug)]
        struct User {
            id: u64,
            email: Cow<'static, str>,
            activated: bool,
        }        

        struct UserRepository {
            storage:    Box<dyn Storage<u64, User>>
        }

        impl UserRepository {

            fn new(storage: Box<dyn Storage<u64, User>>) ->Self {
                Self { storage }
            }

            fn set(&mut self, key: u64, val: User) {
                self
                    .storage
                    .set(key, val)
                    ;
            }

            fn get(&self, key: &u64) ->Option<&User> {
                self
                    .storage
                    .get(key)
            }

            fn remove(&mut self, key: &u64) ->Option<User> {
                self
                    .storage
                    .remove(key)
            }

        }

        impl<K, V> Storage<K, V> for HashMap<K, V> 
            where K: std::hash::Hash + Eq
        {
            fn get(&self, key: &K) -> Option<&V> {
                // так:
                // self.get(key)
                // или так:
                HashMap::get(&self, key)
            }

            fn remove(&mut self, key: &K) -> Option<V> {
                // так:
                // self.remove(key)
                // или так:
                HashMap::remove(self, key)
            }

            fn set(&mut self, key: K, val: V) {
                self.insert(key, val) ;
            }
        }

        let mut repo = UserRepository::new(Box::new(HashMap::new())) ;

        let id_user = 42u64 ;

        let user = User {
            id:     id_user,
            email: Cow::Borrowed("m@m.c"),
            activated:  true
        } ;

        repo.set(user.id, user) ;

        println!("{:?}", repo.get(&id_user)) ;

        repo.remove(&id_user) ;

        println!("{:?}", repo.get(&id_user)) ;
    }
}
