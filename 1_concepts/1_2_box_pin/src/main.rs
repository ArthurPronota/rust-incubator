use std::fmt ;
use std::ops::Deref;
use std::pin::Pin ;

/*
use std::pin::pin;

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&mut Self>) {
        println!("Hi from {:?}", self)
    }
}


let pinned_person = pin!(person) ;
 */

#[derive(Debug)]
struct Person {
    name:   String
}

fn main() {

    // 1. Реализация trait SayHi: fmt::Debug

    {
        trait SayHi: fmt::Debug {
            fn say_hi(self: Pin<&Self>) {
                println!("Hi from {:?}", self) ;
            }
        }

        impl<T: fmt::Debug> SayHi for Box<T> {}

        let person = 
                Box::new(
                    Person {
                        name: "Ann".to_owned(),
                    }
                );
    
   
        let pinned_person = Pin::new(&person);
        pinned_person.say_hi();
    }

    {
        use std::rc::Rc;

        trait SayHi: fmt::Debug {
            fn say_hi(self: Pin<&Self>) {
                println!("Hi from {:?}", self) ;
            }
        }

        impl<T: fmt::Debug> SayHi for Rc<T> {}

        let person = 
                Rc::new(
                    Person {
                        name: "Ann".to_owned(),
                    }
                );

        let person_pinned = Pin::new(&person) ;
        person_pinned.say_hi() ;
    }

    {
        trait SayHi: fmt::Debug {
            fn say_hi(self: Pin<&Self>) {
                println!("Hi from {:?}", self) ;
            }
        }

        impl<T: fmt::Debug> SayHi for Vec<T> {}

        let persons = vec![
                        Person {
                            name: "Ann".to_owned(),
                        }
        ] ;

        let persons_pinned = Pin::new(&persons) ;
        persons_pinned.say_hi() ;
    }

    {
        trait SayHi: fmt::Debug {
            fn say_hi(self: Pin<&Self>) {
                println!("Hi from {:?}", self) ;
            }
        }

        impl SayHi for String {}

        let str_1 = "Ann".to_owned() ;
        let str_pinned = Pin::new(&str_1) ;
        str_pinned.say_hi();
    }

    {
        trait SayHi: fmt::Debug {
            fn say_hi(self: Pin<&Self>) {
                println!("Hi from {:?}", self) ;
            }
        }

        impl SayHi for &[u8] {}

        let u8_slice = &[1 as u8, 2, 3][..] ;
        let u8_slice_pinned = Pin::new(&u8_slice) ;
        u8_slice_pinned.say_hi() ;
    }

    {
         trait SayHi: fmt::Debug {
            fn say_hi(self: Pin<&Self>) {
                println!("Hi from {:?}", self) ;
            }
        }

        impl<T: fmt::Debug> SayHi for T {}
        
        let person = 
                    Person {
                        name: "Ann".to_owned(),
                    };
        let person_pinned = Pin::new(&person) ;
        person_pinned.say_hi() ;         
    }

    // 2. Реализация trait MutMeSomehow

    {
        trait MutMeSomehow {
            fn mut_me_somehow(self: Pin<&mut Self>) {}
        }

        #[derive(Default, Debug)]
        struct Person {
            name:   String
        }

        impl<T: Default> MutMeSomehow for Box<T> {
            fn mut_me_somehow(self: Pin<&mut Self>) {
                unsafe {
                    let this: &mut Self = self.get_unchecked_mut();

                    *this = Default::default() ;
                }   
            }
        }

        let mut p = Box::new(Person{name: "Abc".to_owned()}) ;

        println!("Befire p: {:?}", p) ;

        let p_pinned = Pin::new(&mut p) ;

        p_pinned.mut_me_somehow() ;

        println!("After p: {:?}", p) ;
    }

    {
        use std::rc::Rc ;

        trait MutMeSomehow {
            fn mut_me_somehow(self: Pin<&mut Self>) {}
        }
        
        #[derive(Default, Debug)]
        struct Person {
            name:   String
        }

        impl<T: Default> MutMeSomehow for Rc<T> {
            fn mut_me_somehow(self: Pin<&mut Self>) {
                unsafe {
                    let this: &mut Self = self.get_unchecked_mut();

                    *this = Default::default() ;
                }   
            }
        }

        let mut rc_bef = Rc::new(Person{name: "Abc".to_owned()}) ;
        println!("rc before: {:?}", rc_bef) ;

        let rc_pinned = Pin::new(&mut rc_bef) ;
        rc_pinned.mut_me_somehow() ;
        println!("rc after: {:?}", rc_bef) ;
    }

    {
        trait MutMeSomehow {
            fn mut_me_somehow(self: Pin<&mut Self>) {}
        }

        impl<T> MutMeSomehow for Vec<T> {

            fn mut_me_somehow(self: Pin<&mut Self>) {
                unsafe {
                    let this: &mut Self = self.get_unchecked_mut();
                    this.clear();
                }
            }

            // https://internals.rust-lang.org/t/pin-from-mut-refs/9294
        }
        
        let mut v = vec!["1","2","3"] ;
        println!("Before v: {:?}", v) ;

        let v_pinned = Pin::new(&mut v) ;
        v_pinned.mut_me_somehow() ;
        println!("After v: {:?}", v) ;
    }

    {
        trait MutMeSomehow {
            fn mut_me_somehow(self: Pin<&mut Self>) {}
        }

        impl MutMeSomehow for String {
            fn mut_me_somehow(self: Pin<&mut Self>) {
                unsafe {
                    let this: &mut Self = self.get_unchecked_mut();
                    *this = "qrt".to_owned() ;
                }                
            }
        }

        let mut str_base = "abc".to_owned() ;
        println!("Before str_base: {}", str_base) ;

        let str_pinned = Pin::new(&mut str_base) ;
        str_pinned.mut_me_somehow() ;
        println!("After str_base: {}", str_base) ;
    }

    {
        trait MutMeSomehow {
            fn mut_me_somehow(self: Pin<&mut Self>) {}
        }

        impl MutMeSomehow for &[u8] {
            fn mut_me_somehow(self: Pin<&mut Self>) {
                let this: &mut &[u8] = self.get_mut();
                if this.len() > 1 {
                    *this = &this[1..]; 
                }
            }
        }

        let mut u8_slice = &[1 as u8, 2, 3][..] ;

        println!("Before u8_slice: {:?}", u8_slice) ;

        let u8_slice_pinned = Pin::new(&mut u8_slice) ;
        u8_slice_pinned.mut_me_somehow() ;
        println!("Afetr u8_slice: {:?}", u8_slice) ;
    }

    {
        trait MutMeSomehow {
            fn mut_me_somehow(self: Pin<&mut Self>) {}
        }

        impl<T: Default> MutMeSomehow for T {   
            fn mut_me_somehow(self: Pin<&mut Self>) {
                unsafe {
                    let this: &mut Self = self.get_unchecked_mut();
                    *this = Default::default() ;
                }                 
            }
        }

        let mut v = 10 ;
        println!("Before v: {:?}", v) ;

        let v_pinned = Pin::new(&mut v) ;
        v_pinned.mut_me_somehow() ;
        println!("After v: {:?}", v) ;

        #[derive(Default, Debug)]
        struct Person {
            name:   String
        }

        let mut p = Person{name: "abc".to_owned()} ;
        println!("Before p: {:?}", p) ;

        let p_pinned = Pin::new(&mut p) ;
        p_pinned.mut_me_somehow() ;
        println!("After p: {:?}", p) ;

    }

    // Provide a Future trait implementation
    {
        use std::future::Future;
        use std::pin::Pin;
        use std::task::{Context, Poll};
        use tokio ;

        struct MeasurableFuture<Fut> {
            inner_future: Fut,
            started_at: Option<std::time::Instant>, // Время начала выполнения future
        }

        impl<Fut> MeasurableFuture<Fut> {
            pub fn new(inner_future: Fut) -> Self {
                Self {
                    inner_future,
                    started_at:     None,   // Начальная инициализация таймера
                }
            }
        }

        impl<Fut: Future> Future for MeasurableFuture<Fut> {
            type Output = Fut::Output;  // тип значения произведённого по завершению

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let (inner_future, started_at) = unsafe {
                    let this = self.get_unchecked_mut(); // Получаем &mut Self из Pin<&mut Self>
                    (&mut this.inner_future, &mut this.started_at) // получаем 2-а поля из сруктуры
                };

                // Инициализация таймера при первом опросе
                if started_at.is_none() {
                    *started_at = Some(std::time::Instant::now()); // установка начального значения таймера
                }

                // конструируем новый не проверенный Pin<&mut Fut>
                let inner_pin = unsafe { Pin::new_unchecked(inner_future) };

                match inner_pin
                        .poll(cx) // Попытка опроса будущего
                {
                    Poll::Ready(output) => { // значение готово
                        if let Some(start) = started_at {
                            let elapsed = start
                                                    .elapsed() // Возвращает количество времени, прошедшего с этого Instant.
                                                    .as_nanos()
                                                    ;
                            println!("Future finished in {} ns", elapsed);
                        }
                        Poll::Ready(output)
                    },
                    Poll::Pending => Poll::Pending, // значение ещё не готово
                }
            }
        }

        let rt = tokio::runtime::Runtime::new()
                    .expect("tokio::runtime::Runtime::new() error.");

        rt.block_on(async {
            let my_async_task = async {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            };

            let measurable = MeasurableFuture::new(my_async_task);
            measurable.await;
        }) ;
    }
    
}
