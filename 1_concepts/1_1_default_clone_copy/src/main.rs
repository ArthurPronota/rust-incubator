//use std::intrinsics::const_eval_select;

use std::{error::Error, fmt::Display};

fn main() {
    println!("Implement me!");

    // ---------------------------

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            // This comparison works because we constrained T to require the 'PartialOrd' trait
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    // This works for integers:
    let number_list = vec![34, 50, 25, 100, 65];
    let f = &number_list ;
    let result = largest(&number_list); // T becomes i32

    // And it works for characters:
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list); // T becomes char

    println!("{number:0>5}", number=1);
    println!("{number:0>width$}", number=1, width=7);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // --------------------------

    use std::fmt ;

    struct Structure(i32) ;

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Structure({})", self.0)
        }
    }

    let v = Structure(1) ;

    println!("v: {}", v) ;

    // -------------------

    #[derive(Debug)]
    struct MinMax(i32, i32) ;

    impl fmt::Display for MinMax{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    let min_max = MinMax(0, 100) ;
    println!("{:?} <-> {}", min_max, min_max) ;

    // -------------------

    struct List(Vec<i32>) ;

    impl fmt::Display for List {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let vec = &self.0 ;

            write!(f, "[") ;

            for (index, &v) in vec
                                            .iter()
                                            .enumerate() {
                if index != 0 {
                    write!(f, ", ") ;
                }
                write!(f, "{}:{}", index, v) ;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]) ;

    println!("v: {}", v) ;

    // ---------------------------

    struct City {
        name:   &'static str,
        lat:    f32,
        lon:    f32,
    }

    impl fmt::Display for City {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let lat_c = if self.lat >= 0.0 {
                'N'
            } else {
                'S'
            } ;

            let lon_c = if self.lon >= 0.0 {
                'E'
            } else {
                'W'
            } ;

            write!(f, "{} {:.3}°{} {:.3}°{}", 
                    self.name, 
                    self.lat.abs(),
                    lat_c,
                    self.lon.abs(),
                    lon_c,
                )
        }
    }

    for city in [
            City {name: "A", lat: 10.2, lon: -10.4},
            City {name: "B", lat: 180.2, lon: 140.4},
            City {name: "C", lat: -80.18, lon: 40.24},
        ] {
            println!("{}", city) ;
    }

    // ------------------------------

    use std::any::type_name ;
    fn type_of<T>(_:&T) ->&'static str {
        type_name::<T>()
    }

    /* An array of integers on the stack
    1) Fixed Size: The size of the array is fixed at 5 at compile time and cannot change.
    2) Stack Allocation: The entire array (20 bytes total: 5 integers * 4 bytes each) is allocated on the stack, which is very fast and efficient.
    3) Immutability: Because mut was not used, you cannot change the values within my_array after this initialization.
     */
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    let my_array2 =  vec![1,2,3] ;
    let my_array3 = [1, 2, 3, 4, 5] ;

    
    println!("my_array: {}, my_array2: {}, my_array3: {}", 
                type_of(&my_array),
                type_of(&my_array2),
                type_of(&my_array3),
            ) ;

    let my_tuple = (5u32, 1u8, true, -5.04f32);

    println!("my_tuple: {:#?}", my_tuple) ;

    println!("NOT true is {}", !true);

    // --------------------------

        // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}, type of :{}", 
                tuple_of_tuples,
                type_of(&tuple_of_tuples),
            );

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));

    let v1 = (5u32) ;

    println!("Just an integer: {:?}", (5u32));

    let v = (5u32,) ;
    println!("v is {}", type_of(&v)) ;

    // -------------------------

    use std::mem ;

    let xs = [1,2,3,4,5] ;
    let ys = [0; 500];

    println!("Number elements of ws was: {}", xs.len()) ;
    println!("1) Size of [i32;500] bytes: {}", mem::size_of::<[i32;500]>()) ;
    println!("2) Size of xs bytes: {}", mem::size_of_val(&xs)) ;

    let empty_array: [i32;0] = [] ;
    assert_eq!(&empty_array, &[]) ;
    assert_eq!(&empty_array, &[][..]) ;

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
           Some(&x_val) => println!("{}: {}", i, x_val),
           None => println!("Slow down, it's {} too far", i),
        }
    }

    //println!("{}", xs[5]) ; // Compile error: index out of bounds: the length is 5 but the index is 5
    //println!("{}", xs[..][5]) ; // Runtime error: index out of bounds: the len is 5 but the index is 5

    // ---------------------------

    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    let point = Point {x: 1.0, y: 2.0} ;

    let point2 = Point {x: 3.0, ..point} ;

    println!("point2: {:?}", point2) ;

    // -------------------------

    enum Number {
        One,
        Two,
        Three,
    }

    println!("{}, {}, {}", 
                Number::One as i32,
                Number::Two as i32,
                Number::Three as i32,
            ) ;

    enum Number2 {
        One = 1,
        Two = 2,
        Three = 3,
    }

    println!("{}, {}, {}", 
                Number2::One as i32,
                Number2::Two as i32,
                Number2::Three as i32,
            ) ;

    // ---------------------

    /*
    enum List2 {
        Cons(u32, Box<List2>),
        Nil,
    }

    use List2::* ;

    impl List2 {
        fn new() ->List2 {
            List2::Nil
        }

        fn prepend(self, elem: u32) -> List2 {
            Cons(elem, Box::new(self))
        }

        fn len(&self) ->u32 {
            match *self {
                Cons(_, ref tile) => 1 + tile.len(),
                Nil => 0,
            }
        }

        fn stringify(&self) ->String {
            match *self {
                Cons(head, ref title) => {
                    format!("{}, {}", head, title.stringify())
                },
                Nil => format!("Nil"),
            }
        }
    }

    let mut list = List2::new() ;
    list = list.prepend(1) ;
    list = list.prepend(2) ;
    list = list.prepend(3) ;

    println!("len: {}",list.len()) ;

    println!("{}", list.stringify()) ;
     */

    enum List2 {
        Cons(u32, Box<List2>),
        Nil,
    }

    use List2::* ;

    impl List2 {
        fn new() ->List2 {
            Nil
        }
        
        fn prepend(self, num: u32) ->List2 {
            Cons(num, Box::new(self))
        }

        fn len(&self) ->u32 {
            match *self {
                Cons(_, ref tile) => 1 + tile.len(),
                Nil => 0,
            }
        }

        fn stringify(&self) ->String {
            match *self {
                Cons(head, ref title ) => format!("{}, {}", head, title.stringify()),
                Nil => format!("Nil"),
            }
        }
    }

    let mut list = List2::new() ;

    list = list.prepend(1) ;
    list = list.prepend(2) ;
    list = list.prepend(3) ;

    println!("len of list: {}", list.len()) ;
    println!("strigify: {}", list.stringify()) ;

    // ------------------

    static MYSTAT: &str = "m" ;
    static mut MYSTAT_MUT: &str = "a" ;

    const MYCONST: i64 = 10 ;

    println!("{}, {}", MYSTAT, MYCONST) ;

    unsafe {
        MYSTAT_MUT = "a1" ;
    }

    // -----------------------

    let mut mutable_int = 10u64 ;
    {
        let mutable_int_2 = mutable_int ;

        // https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/freeze.html
        mutable_int = 50 ;  // mutable_int isn't frozen
    }

    // ------------------------

    println!("{}, {}", -1i8 as u8, -100.00 as i8) ;

    unsafe {
        println!("{}", 300.1f32.to_int_unchecked::<u8>()) ;
        println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>()) ;
    }

    // ---------------------

    let f = 1.4f64 ;

    println!("size of f is: {}", std::mem::size_of_val(&f)) ;

    // -----------------------

    #[derive(Debug)]
    struct Number3 {
        value: i32
    }

    impl From<i32> for Number3 {
        fn from(num: i32) ->Self {
            Self { value: num }
        }
    }

    let numb = Number3::from(10) ;
    println!("numb: {:?}", numb) ;

    // -------------------

    #[derive(Debug)]
    struct Number4 {
        value: i32
    }

    impl Into<Number4> for i32 {
        fn into(self) -> Number4 {
            Number4 { value: self }
        }
    }

    let numb: Number4 = 10.into() ;

    println!("numb: {:?}", numb) ;

    // --------------------

    use std::convert::{TryFrom, TryInto} ;

    #[derive(Debug)]
    struct EvenNumber(i32) ;

    impl TryFrom<i32> for EvenNumber {
        type Error = () ;

        fn try_from(value: i32) -> Result<Self, Self::Error> {

            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    impl TryInto<EvenNumber> for f32 {
        type Error = () ;

        fn try_into(self) -> Result<EvenNumber, Self::Error> {
            if (self as i32) % 2 == 0 {
                Ok(EvenNumber(self as i32))
            } else {
                Err(())
            }
        }
    }

    let mut result = EvenNumber::try_from(10) ;
    println!("result: {:?}", result) ;

    result = EvenNumber::try_from(3) ;
    println!("result: {:?}", result) ;

    let mut result2: Result<EvenNumber, ()>  = (10.5f32).try_into() ;
    println!("result2: {:?}", result2) ;

    result2 = (9.0f32).try_into() ;
    println!("result2: {:?}", result2) ;

    // -------------------------------

    //use std::fmt ;

    struct Circle {
        radius: i32
    }

    impl Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Circle of radius: {}", self.radius)
        }
    }

    let circle = Circle{radius: 10} ;
    println!("circle: {}", circle) ;

    let parsed: u32 = "5"
                        .parse()
                        .unwrap()
                        ;
    println!("parsed: {}", parsed) ;

    use std::num::ParseIntError ;
    use std::str::FromStr ;

    impl FromStr for Circle {
        type Err = ParseIntError ;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim().parse() {
                Ok(num) => Ok(Circle{radius: num}),
                Err(e) => Err(e),
            }
        }
    }

    let circle = "    5 ".parse::<Circle>()
                                    .unwrap() ;
    println!("circle: {}", circle) ;

    let circle: Circle = "    6 ".parse().unwrap() ;
    println!("circle: {}", circle) ;
    
    // -----------------------

    let names = vec!["1", "2", "3"] ;

    for name in names
                        .iter()
                         {
        match name {
            &"3" => println!("It's: {}", name),
            _ => println!("{}", name),
        }
    }

    for name in names.into_iter() {
        match name {
            "3" => println!("It's: 3"),
            _ => println!("{}", name),
        }
    }

    //println!("{:?}", names) ; // Error: borrow of moved value: `names`

    let mut names = vec!["1", "2", "3"] ;

    for name in names.iter_mut() {
        *name = match name {
            &mut "3" => "30",
            _ => name,
        }
    }

    println!("{:?}", names) ;

    // ---------------------

    let array = [10, 1, 2, 3] ;

    match array {
        [10, tail @ .., last] => println!("{:?}, {}",
                tail, last
            ),
        [1, second, tail @ ..] => println!("{}, {:?}", 
                second, tail),
        _ => println!("Other."),
    }

    // ------------------------

    let refrence = &5 ;

    match refrence {
        &v => println!("v value: {}", v),
    }

    match *refrence {
        v => println!("v value: {}", v),
    }

    let not_a_refrence = 4 ;
    let ref a_refrence = 4 ;

    let val = 5 ;

    match val {
        ref v => println!("is refrence: {:?}", v),
    }

    let mut mut_val = 6 ;

    match mut_val {
        ref mut v => {
            *v += 10 ;
            println!("m: {}", v) ;
        }
    }

    // ---------------------------

    struct Foo {
        x:  (u32, u32),
        y:  u32,
    }

    let foo = Foo {x: (1, 2), y: 3} ;

    match foo {
        Foo { x: (1, b), y } => println!("b: {}, y: {}", b, y),
        Foo { y: 2, x: i } => println!("i: {:?}", i),
        Foo { y, .. } => println!("y: {}", y),
    }

    let Foo { x: x0, y: y0 } = foo;
    println!("x0: {:?}, y0: {}", x0, y0) ;

    struct Bar {
        foo: Foo,
    }

    let bar = Bar {foo: foo} ;

    let Bar { foo: Foo { x: nested_x, y: nested_y } } = bar;
    println!("nested_x: {:?}, nested_y: {}", nested_x, nested_y) ;

    // ------------------------

    
}
