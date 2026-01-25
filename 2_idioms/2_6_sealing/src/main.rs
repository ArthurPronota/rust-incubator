use step_2_6::MyIteratorExt;

 fn main() {
    {
        use std::{any::Any, fmt};

        use step_2_6::MyError;

        #[derive(Debug)]
        struct SuperErrorSideKick;

        #[derive(Debug)]
            struct SuperError {
                source: SuperErrorSideKick,
        }

        impl fmt::Display for SuperError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "SuperError is here!")
            }
        }

        impl fmt::Display for SuperErrorSideKick {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "SuperErrorSideKick is here!")
            }
        }

        impl MyError for SuperError {

            fn source(&self) -> Option<&(dyn MyError + 'static)> {
                Some(&self.source)
            }
        }

    impl MyError for SuperErrorSideKick {}

    fn get_super_error() -> Result<(), SuperError> {
        Err(SuperError { source: SuperErrorSideKick })
    }

    match get_super_error() {
        Err(e) => {
            println!("Error: {e}, type_id: {:?}", e.type_id());
            println!("Caused by: {}, type_id: {:?}", e.source().unwrap(), e.source().unwrap().type_id());
            println!("SuperErrorSideKick: {:?}", e.source.source()) ;
        }
        _ => println!("No error"),
    }

    }

    {
    use step_2_6::MyIteratorExt as _ ;
    
    /*
    struct MyIterator(u32) ;

    impl Iterator for MyIterator {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 < 10 {
                self.0 += 1 ;
                Some(self.0)
            }
            else {
                None
            }
        }
    }

    impl <I: Iterator> MyIteratorExt for MyIterator<T> {}
    

    let mi = MyIterator(0) ;
    println!("{}", mi.into_iter().format(" x ")) ;
    */

    let data = [1.1, 2.71828, -3.];
    assert_eq!(
         format!("{:.2}", data
                            .iter()
                            .format(", ")
                        ),
                "1.10, 2.72, -3.00");
    }

    // ----------------------
    {
    use step_2_6::MyIteratorExt as _;
    
    let data = [1.1, 2.71828, -3.];
    let data_formatter = data.iter().format_with(", ", |elt, f| f(&format_args!("{:.2}", elt)));
    assert_eq!(format!("{}", data_formatter),
                "1.10, 2.72, -3.00");
    
    // .format_with() is recursively composable
    let matrix = [[1., 2., 3.],
                   [4., 5., 6.]];
    let matrix_formatter = matrix.iter().format_with("\n", |row, f| {
         f(&row.iter().format_with(", ", |elt, g| g(&elt)))
    });
    assert_eq!(matrix_formatter.to_string(), "1, 2, 3\n4, 5, 6");
    }
 }