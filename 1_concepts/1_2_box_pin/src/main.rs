use std::fmt ;
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
            fn mut_me_somehow(self: Pin<&mut Self>) {
                /*
                let v = unsafe {
                 self
                    .get_unchecked_mut()
                } ;
                 */
            }
        }
        
    }

    
}
