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
            //self.get(key)
            // или так
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
            // self.remove(key)
            // или так
            HashMap::remove(self, key)
        }

    }

    let mut repo = UserRepository::new(HashMap::new());

    let user = User {
        id: 42,
        email: Cow::Borrowed("dev@example.com"),
        activated: true,
    };    

    repo.set(user.id, user);

    println!("{:?}", repo.get(42)) ;

    repo.remove(&42) ;

    println!("{:?}", repo.get(42)) ;
    }

}
