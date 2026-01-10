fn main() {
    // Implement EmailString 
    {
    use std::fmt ;
    use std::ops::Deref;

    #[derive(Clone, PartialEq, PartialOrd)]
    pub struct EmailString(String);

    impl EmailString {
        fn new(email: String) ->Result<Self, String> {
            if email.contains("@") && email.len() >= 5 {
                Ok(EmailString(email))
            } else {
                Err(format!("Invalid email: {}", email))
            }
        }
    }

    impl Deref for EmailString {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl fmt::Display for EmailString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl<'a> TryFrom<&'a str> for EmailString {
        type Error = String;

        fn try_from(value: &'a str) -> Result<Self, Self::Error> {
            Self::new(value.to_owned())
        }
    }

    let email_1 = EmailString::new("m@v.com".to_owned()).unwrap() ;
    println!("email_1: {}", email_1) ;

    let email_2 = EmailString::try_from("m2@v.com").unwrap() ;
    println!("email_2: {}", email_2) ;

    // need Deref
    let domain = email_1
        .split("@")
        .last()
        .unwrap()
         ;
    // need Deref 
    println!("domain: {} from email: {}, len email: {}", domain, email_1, email_1.len()) ;

    // need Clone
    let email_1_clone = email_1.clone() ;
    println!("email_1_clone: {}", email_1_clone) ;

    // need PartialEq
    if email_1_clone == email_1 {
        println!("email_1_clone == email_1") ;
    }

    // meed PartialEq, PartialOrd
    println!("email_1 > email_2 is {}", email_1 > email_2) ;

    let raw_email_3 = "m#v.com" ;
    let email_3 = EmailString::new("m#v.com".to_owned()) ;
    
    if email_3.is_err() {
        println!("email: {} is error", raw_email_3) ;
    }
    }

    // Implement Random<T>
    {
        use std::ops::Deref;
        use rand::Rng;

        pub struct Random3<T> {
            values: [T; 3],
        }

        impl<T> Random3<T> {
            fn new(v1: T, v2: T, v3: T) ->Self {
                Self {values: [v1, v2, v3]}
            }

            fn get_rand(&self) ->&T{
                let mut rnd = rand::rng() ;
                &self.values[rnd.random_range(0..3)]
            }
        }

        impl<T> From<(T, T, T)> for Random3<T> {
            fn from(tup: (T, T, T)) -> Self {
                Self::new(tup.0, tup.1, tup.2)
            }
        }

        impl<T> Deref for Random3<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                let mut rnd = rand::rng() ;
                &self.values[rnd.random_range(0..3)]
            }
        }

        let v3 = Random3::<u64>::new(10, 11, 12) ;
        let v_rand = v3.get_rand() ;
        println!("v_rand: {}", v_rand) ;

        for _ in 0..10 {
            println!("v_rand_tmp: {:?}", 
                        *v3 // need Deref
            ) ;
        }

        let v4: Random3<&str> = ("A", "AB", "ABC").into() ;
        println!("v4.len: {}", 
                    v4.len()    // need Deref
        ) ;

    }

    // Примеры из объяснений
    {
        let num: u32 = 5;
        let mut big_num: u64 = num
                            .into()
                            ;
        // или так:
        let big_num2 = Into::<u64>::into(num) ;

        let small_num: u16 = big_num
                                .try_into()
                                .expect("Value is too big")
                                ;

        let string: String = "some text"
                                    .into()
                                    ;
        // или так:
        let string2 = Into::<String>::into("some text") ;

        let bytes: &[u8] = string.as_ref();
    }
    
    {
        use std::mem;

        #[derive(Debug)]
        struct Id(u8);

        impl AsRef<u8> for Id {
            fn as_ref(&self) -> &u8 {
                &self.0
            }
        }

        impl AsRef<Id> for u8 {
            fn as_ref(&self) -> &Id {
                unsafe { mem::transmute(self) }
            }
        }

        let v = Id(10) ;
        let v1 = v.as_ref() ;

        println!("v1: {} / {:p}", v.as_ref(), v.as_ref()) ;

        println!("v2: {:?} / {:p}", 10_u8.as_ref(), 10_u8.as_ref()) ;
    }

}
