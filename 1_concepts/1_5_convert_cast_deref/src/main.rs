fn main() {
    println!("Implement me!");

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
