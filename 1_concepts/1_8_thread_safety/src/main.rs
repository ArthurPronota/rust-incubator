// Make type Sync/Send/!Sync/!Send
// https://users.rust-lang.org/t/make-type-sync-send-sync-send/97344/8
/*
struct OnlySync(PhantomData<MutexGuard<'static, ()>>);
    impl OnlySync {
        fn print(&self, val: i32) {
            println!("OnlySync - {val}")
        }
    }

    struct OnlySend(PhantomData<UnsafeCell<i32>>);
    impl OnlySend {
        fn print(&self, val: i32) {
            println!("OnlySend - {val}")
        }
    }

    struct SyncAndSend(PhantomData<i32>);
    impl SyncAndSend {
        fn print(&self, val: i32) {
            println!("SyncAndSend - {val}")
        }
    }

    struct NotSyncNotSend(PhantomData<Rc<i32>>);
    impl NotSyncNotSend {
        fn print(&self, val: i32) {
            println!("NotSyncNotSend - {val}")
        }
    }
*/
fn main() {
    // 1.1 Реализация OnlySync is Sync, but !Send.
    {
    use std::marker::PhantomData;

    // A type that is Sync but not Send.
    // Тип данных у которого нет Send
    #[derive(Debug)]
    pub struct OnlySync {
        data:       i32,
        // ключ который реализует !Send
        _marker:    PhantomData<*mut i32>,   // сырой изменяемый указатель
    }

    // небезопасная реализация Sync для типа OnlySync
    unsafe impl Sync for OnlySync {}

    let only_sync = OnlySync {
        data:       1,
        _marker:    PhantomData,
    };

    // Создание ссылки
    let only_sync_ref = &only_sync ;

    // Создаём область для прождаемых потоков.
    // Внутри созданной области можно работать с нестатическими ссылками.
    std::thread::scope(|s| {
        s.spawn(|| {
            // допуск к ссылка разрешёт т.к. для OnlySync реализован Sync
            println!("1) borrow OnlySync: {:?}", only_sync_ref);
        });

        s.spawn(|| {
            // допуск к ссылка разрешёт т.к. для OnlySync реализован Sync            
            println!("2) borrow OnlySync: {:?}", only_sync_ref);
        });
        
        s.spawn(|| {
            // допуск к ссылка разрешёт т.к. для OnlySync реализован Sync            
            println!("3) borrow OnlySync: {:?}", only_sync_ref);
        });          
    });

    // Следующий код будет причиной ошибки компиляции
    // Текс ошибки: the trait `Send` is not implemented for `*mut i32`
    /*
    std::thread::spawn(move || {
        println!("move OnlySync: {:?}", only_sync);

    });
     */
    }

    // 1.2 Реализация OnlySync is Sync, but !Send.
    {
        use std::marker::PhantomData;
        use std::sync::MutexGuard ;

        #[derive(Debug)]
        pub struct OnlySync {
            data:       i32,
            // ключ который реализует !Send
            _marker:    PhantomData<MutexGuard<'static, ()>>,   // сырой изменяемый указатель
        }

        let only_sync = OnlySync {
            data:       1,
            _marker:    PhantomData,
        };

        // Создание ссылки
        let only_sync_ref = &only_sync ;

        // Создаём область для прождаемых потоков.
        // Внутри созданной области можно работать с нестатическими ссылками.
        std::thread::scope(|s| {
            s.spawn(|| {
                // допуск к ссылка разрешёт т.к. для OnlySync реализован Sync
                println!("1) borrow OnlySync: {:?}", only_sync_ref);
            });

            s.spawn(|| {
                // допуск к ссылка разрешёт т.к. для OnlySync реализован Sync            
                println!("2) borrow OnlySync: {:?}", only_sync_ref);
            });
        
            s.spawn(|| {
                // допуск к ссылка разрешёт т.к. для OnlySync реализован Sync            
                println!("3) borrow OnlySync: {:?}", only_sync_ref);
            });          
        });

        // Следующий код будет причиной ошибки компиляции
        // Текс ошибки: the trait `Send` is not implemented for `std::sync::MutexGuard<'static, ()>`
        /*
        std::thread::spawn(move || {
            println!("move OnlySync: {:?}", only_sync);
        });
        */
    }

    // 2.1 Реализация is Send but !Sync
    {
        use std::marker::PhantomData;
        use std::cell::Cell ;
        use std::thread ;

        #[derive(Debug)]
        struct OnlySend {
            data:       i32,
            _marker:    PhantomData<Cell<()>>,
        }

        // Создаём небезопасную реализацию трейта Send для типа OnlySend
        unsafe impl Send for OnlySend {}

        let only_send = OnlySend {data: 1, _marker: PhantomData} ;

        let only_send_ref = &only_send ;

        // Следующий код будет причиной ошибки компиляции
        // Текс ошибки: the trait `Sync` is not implemented for `Cell<()>`
        /*
        thread::scope(|s| {
            s.spawn(|| {
                println!("borrow OnlySend: {:?}", only_send_ref) ;
            })
        }) ;
         */

        let _ = thread::spawn(move || {
            println!("moved OnlySend: {:?}", only_send) ;
        }).join() ;
    }

    // 2.2 Реализация is Send but !Sync
    {
        use std::marker::PhantomData;
        use std::cell::UnsafeCell ;
        use std::thread ;

        #[derive(Debug)]
        struct OnlySend {
            data:       i32,
            _marker:    PhantomData<UnsafeCell<i32>>,
        }

        let only_send = OnlySend {data: 1, _marker: PhantomData} ;

        let only_send_ref = &only_send ;

        // Следующий код будет причиной ошибки компиляции
        // Текс ошибки: the trait `Sync` is not implemented for `UnsafeCell<i32>`
        /*
        thread::scope(|s| {
            s.spawn(|| {
                println!("borrow OnlySend: {:?}", only_send_ref) ;
            })
        }) ;
         */

        let _ = thread::spawn(move || {
            println!("moved OnlySend: {:?}", only_send) ;
        }).join() ;        
    }

    // 3 Реализация is Send but Sync
    {
        use std::marker::PhantomData;
        use std::thread ;

        #[derive(Debug)]
        struct SendSync {
            data:       i32,
            _marker:    PhantomData<i32>,
        }

        let send_sync = SendSync {data: 1, _marker: PhantomData} ;
        
        let send_sync_ref = &send_sync ;

        thread::scope(|s| {
            s.spawn(|| {
                println!("borrow SendSync: {:?}", send_sync_ref) ;
            }) ;
        }) ;

        let _ = thread::spawn(move || {
            println!("move SendSync: {:?}", send_sync) ;
        }).join() ;

    }

    // 4.1 Реализация is !Send but !Sync
    {
        use std::marker::PhantomData;
        use std::thread ;

        #[derive(Debug)]
        struct NotSendNotSync {
            data:       i32,
            _marker:    PhantomData<*mut i32>,
        }

        let notsend_notsync = NotSendNotSync {
            data:    1,
            _marker: PhantomData,
        };

        // Создание ссылки
        let _notsend_notsync_ref = &notsend_notsync ;

        // Следующий код будет причиной ошибки компиляции
        // Ошибка компиляции: the trait `Sync` is not implemented for `*mut i32`
        /*
        thread::scope(|s| {
            s.spawn(|| {
                println!("borrow NotSendNotSync: {:?}", _notsend_notsync_ref) ;
            }) ;
        }) ;
        */

        // Следующий код будет причиной ошибки компиляции
        // Ошибка компиляции: the trait `Send` is not implemented for `*mut i32`
        /*
        thread::spawn(move || {
            println!("move NotSendNotSync: {:?}", notsend_notsync) ;
        }) ;
         */
    }

    // 4.2 Реализация is !Send but !Sync
    {
        use std::marker::PhantomData;
        use std::rc::Rc ;
        use std::thread ;

        #[derive(Debug)]
        struct NotSendNotSync {
            data:       i32,
            _marker:    PhantomData<Rc<i32>>,
        }

        let notsend_notsync = NotSendNotSync {
            data:    1,
            _marker: PhantomData,
        };

        // Создание ссылки
        let _notsend_notsync_ref = &notsend_notsync ;

        // Следующий код будет причиной ошибки компиляции
        // Ошибка компиляции: the trait `Sync` is not implemented for `Rc<i32>`
        /*
        thread::scope(|s| {
            s.spawn(|| {
                println!("borrow NotSendNotSync: {:?}", _notsend_notsync_ref) ;
            }) ;
        }) ;
        */
        // Следующий код будет причиной ошибки компиляции
        // Ошибка компиляции: the trait `Send` is not implemented for `Rc<i32>`
        /*
        thread::spawn(move || {
            println!("move NotSendNotSync: {:?}", notsend_notsync) ;
        }).join() ;
         */
    }
}