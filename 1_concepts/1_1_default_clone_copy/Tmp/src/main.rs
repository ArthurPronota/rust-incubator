/*
4293


// use of deprecated macro `r#try`: use the `?` operator instead

opt.transpose() // Transposes an Option of a Result into a Result of an Option.

and_then() преобразует  Result<Result<T, Err>, Err> -> Result<T, Err>
        fn multiply(f: &str, s: &str) ->AliasedResult<i32> {
            f
                .trim()
                .parse::<i32>()
                .and_then(|f_n| {
                    s
                        .trim()
                        .parse::<i32>()
                        .map(|s_n|{
                            f_n * s_n
                        })
                })
        }

    // У Option<T> есть встроенный метод map(), комбинатор для простого
    // сопоставления Some -> Some и None -> None.


                    
                    Если у вас есть коллекция Result<Vec<String>> и вы 
                    хотите объединить их в один Result<Vec<Vec<String>>>,
                    вы можете использовать возможности метода 
                    .collect::<Result<Vec<_>, _>>(). 
                    В Rust метод collect() может «перевернуть» коллекцию 
                    результатов в результат, содержащий коллекцию.                    
                    
                    .collect::<Result<Vec<_>, _>>()



        impl Iterator for Fibonacci {
            // Тип элементов, по которым производится итерация.
            type Item = u32;


        impl Drop for TempFile {
            fn drop(&mut self) {

        impl ops::Add<Foo> for Bar {
            type Output = BarFoo;
            fn add(self, rhs: Foo) -> Self::Output {


    // динамическое создания статических ссылок
            Box::leak(boxed)

        // Default - A trait for giving a type a useful default value.
        impl <'a> Default for Borrowed<'a> {
            fn default() -> Self {


        // ключевое слово `ref` можно использовать для получения ссылок на 
        // поля структуры/кортежа.


// RAII (Resource Acquisition Is Initialization)


    // Phantom type parameters
    // Типы данных могут использовать дополнительные параметры 
    // обобщенного типа в качестве маркеров или для проверки типов во 
    // время компиляции. Эти дополнительные параметры не хранят никаких 
    // значений и не имеют поведения во время выполнения.


    // Associated types of Trate - для читабельности


    // Attributes
    // #[outer_attribute] применяется к элементу немедленно следующему за ним.
    // #![inner_attribute] применяется к окружению элемента (обычно можулю или крейту).


    // Diverging functions
    // Расходящиеся функции никогда не возвращают значение.
    {
    fn foo() ->! {
        panic!("abc") ;
    }


    // Higher Order Functions (HOF) - функции  высшего порядка
    // Это функции, которые принимают одну или несколько других функций
    // и/или создают более полезную функцию.


    // Iterator::any — это функция, которая, получив на вход итератор, 
    // возвращает true, если хотя бы один элемент удовлетворяет предикату. 
    // В противном случае — false. 


    // Associated functions - асоциированы с типом данных
    // Methods - асоциированы с экземпляром типа

    // while let - подтвержающий шаблон для while

    // let-else - опровергаемый шаблон сопоставления

// if let - подтверждающий шаблон сопоставления

    // Binding - привязка значений к именам с помощью @


    struct Circle {
        radius: i32
    }
    // Converting to String
    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Circle of radius: {}", self.radius)
        }
    }


// TryFrom - Преобразование типов -> EvenNumber::try_from(10)
    #[derive(Debug)]
    struct EvenNumber(i32) ;
    impl TryFrom<i32> for EvenNumber {
        type Error = String;
        fn try_from(value: i32) -> Result<Self, Self::Error> {


// TryInto - Попытка преобразования, которая потребляет self -> (10.5f32).try_into()
    impl TryInto<EvenNumber> for f32 {
        type Error = String ;
        fn try_into(self) -> Result<EvenNumber, Self::Error> {

ok_or() -> Transforms the Option<T> into a Result<T, E>

.map_err(|e| DubleError) -> Maps a Result<T, E> to Result<T, F>

.ok_or_else(|| EmptyVec.into()) -> Transforms the Option<T> into a Result<T, E>


.partition - Предикат, передаваемый в функцию partition(), может 
    возвращать true или false. Функция partition() возвращает пару: 
    все элементы, для которых она вернула true, и все элементы, для 
    которых она вернула false.
        let strings = vec!["1","2","3","t"];
        let (errors, numbers): (Vec<_>, Vec<_>) = strings
                                    .into_iter()
                                    .map(|x| {
                                        x.parse::<i32>()
                                    })
                                    .partition(Result::is_err)
                                    ;
*/

// функция определения типа переменной
use std::any::type_name ;
fn type_of<T>(_:&T) ->&'static str {
    type_name::<T>()
}

// https://doc.rust-lang.org/stable/rust-by-example/mod/visibility.html
// Modules Visibility - видимость функцмй в модулях
mod my_mod {

    fn private_function() {
        println!("call my_mod::private_function()") ;
    }

    pub fn function() {
        println!("call my_mod::function()") ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/mod/visibility.html
    pub mod nested {
        /*
         `pub(in path)` Функция видна в данном пути.
          Путь может быть родителем или прародителем.
         */
        pub (in crate::my_mod) fn public_function_in_my_mod() {
            public_function_in_nested() ;
        }

        /*
         `pub(self)` Функция видна только в текущем модуле.
          Аналог privat.
         */
        pub(self) fn public_function_in_nested() {
            println!("call my_mod::nested::public_function_in_nested()") ;
        }

        /*
         `pub(super)` Функция видна только в родительском модуле.
         */
        pub(super) fn public_function_in_super_modul() {
            println!("call my_mod::public_function_in_super_modul()") ;
        }
    }

    /*
        `pub(crate)` Функция видимая в текущем crate
     */
    pub(crate) fn public_function_in_crate() {
        println!("call my_mod::public_function_in_crate()") ;
    }

}

// https://doc.rust-lang.org/stable/rust-by-example/mod/struct_visibility.html
// Struct visibility
// Видимость элементов структуры работает вне модуля.

mod my {

    use std::fmt::Debug;

    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct CloseBox<T> {
        contents: T,
    }

    impl<T: Debug> CloseBox<T> {
        pub fn new(contents: T) ->CloseBox<T> {
            CloseBox { contents: contents }
        }

        pub fn show(&self) {
            println!("CloseBox content: {:?}", self.contents) ;
        }
    }
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// Unit testing (Модульное тестирование)
// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// Unit testing - модульные тесты
// 
// Большинство модульных тестов помещаются в модуль tests с 
// атрибутом #[cfg(test)]. Тестовые функции помечаются атрибутом #[test].
// Тесты завершаются с ошибкой, когда что-то в тестовой функции вызывает
// панику. 
// Существуют некоторые вспомогательные макросы:
//      assert!(expression) — вызывает панику, если выражение 
//       оценивается как ложное.
//      assert_eq!(left, right) и assert_ne!(left, right) — проверяют 
//       выражения left и right на равенство и неравенство соответственно.
//
// cargo test panic
#[cfg(test)]
mod tests {
    use super::* ;

    fn par_sum(vec: Vec<i32>) ->i32 {
        use rayon::prelude::* ;

        vec
            .par_iter()
            .map(|x| x * x)
            .sum()
    }

    fn sqrt(num: f64) ->Result<f64, String> {
        if num >= 0. {
            Ok(num.powf(0.5))
        } else {
            Err(format!("Invalid num: {}", num))
        }
    }

    fn devide_non_zero_result(a: u32, b: u32) ->u32{
        if b == 0 {
            panic!("Devide-by-zero error.");
        } else if a < b {
            panic!("Devide result is zero.");
        }

        a / b
    }

    #[test]
    fn test_par_sum() {
        let vec = vec![0,1,2,3,4,5,6,7,8,9,10] ;
        assert_eq!(385, par_sum(vec)) ;
    }

    #[test]
    fn test_sqrt() ->Result<(), String> {
        let v = 4. ;
        Ok(assert_eq!(sqrt(v)?.powf(2.0), v))
    }

    #[test]
    fn test_devied() {
        assert_eq!(devide_non_zero_result(10, 3), 3) ;
    }

    // Для проверки функций, которые должны вызывать панику при 
    // определенных обстоятельствах, используйте атрибут #[should_panic].
    // Этот атрибут принимает необязательный параметр 
    // expected = с текстом сообщения о панике. Если ваша функция может 
    // вызывать панику несколькими способами, это поможет убедиться, что 
    // ваш тест проверяет правильную панику.
    // cargo test panic
    #[test]
    #[should_panic]
    fn test_any_panic() {
        assert_eq!(devide_non_zero_result(1, 0), 0) ;
    }

    // Rust также допускает сокращенную форму #[should_panic = "message"],
    // которая работает точно так же, 
    // как #[should_panic(expected = "message")]. Оба варианта допустимы;
    // последний используется чаще и считается более явным.  
    #[test]
    #[should_panic(expected = "Devide result is zero")]
    fn test_sprcific_panic() {
        assert_eq!(devide_non_zero_result(1, 10), 0) ;
    }

    // Тесты можно пометить атрибутом #[ignore], чтобы исключить 
    // некоторые из них. Или запустить их с помощью команды 
    // cargo test -- --ignored    
    #[test]
    #[should_panic = "Devide result is zero"]
    #[ignore]
    fn test_sprcific_panic2() {
        assert_eq!(devide_non_zero_result(1, 10), 0) ;
    }

}




fn main() {
    println!("Implement me!");

    /*
        Далее модифицированная реализация: Rust By Example
        https://doc.rust-lang.org/stable/rust-by-example/index.html
    */

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

    // https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display.html
    // Display
    // реализация трейта Display
    {
        use std::fmt ;

        struct Structure(i32) ;

        impl fmt::Display for Structure {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "Structure({})", self.0)
            }
        }

        let v = Structure(1) ;

        println!("v: {}", v) ;
    }

    {
        use std::fmt ;

        #[derive(Debug)]
        struct MinMax(i32, i32) ;

        impl fmt::Display for MinMax{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({}, {})", self.0, self.1)
            }
        }

        let min_max = MinMax(0, 100) ;
        println!("{:?} <-> {}", min_max, min_max) ;
    }

    {
        use std::fmt ;

        #[derive(Debug)]
        struct Point2D {
            x: f64,
            y: f64,
        }

        impl fmt::Display for Point2D {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "x: {}, y: {}", self.x, self.y)
            }
        }
        let point = Point2D {x: 3.3, y: 4.4} ;
        println!("Display Point2D: {}", point) ;
        println!("Debug Point2D: {:?}", point) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display/testcase_list.html
    // Testcase: List
    // реализация трейта fmt::Display для Vec
    {
    use std::fmt ;

    struct List(Vec<i32>) ;

    impl fmt::Display for List {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let vec = &self.0 ;

            write!(f, "[")? ;   // ? - возвращает ошибку

            for (index, &v) in vec
                                            .iter()
                                            .enumerate() {
                if index != 0 {
                    write!(f, ", ")? ;
                }
                write!(f, "{}:{}", index, v)? ;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]) ;

    println!("v: {}", v) ;

    }

    // https://doc.rust-lang.org/stable/rust-by-example/hello/print/fmt.html
    // Formatting
    // применеие форматирования в трейте fmt::Display

    {
    use std::fmt ;
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
    }

    // https://doc.rust-lang.org/stable/rust-by-example/primitives/tuples.html
    // Tuples
    {
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

    let v1 = (5u32) ;   // it's u32 !!!!!!!!!!!

    println!("Just an integer: {:?}", (5u32));

    let v = (5u32,) ;   // it's tuple !!!!!!!!!
    println!("v is {}", type_of(&v)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/primitives/array.html
    // Arrays and Slices

    {
    use std::mem ;

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

    println!("NOT true is {:?}", !true);

    let xs = [1,2,3,4,5] ;
    let ys = [0; 500];

    println!("Number elements of ws was: {}", xs.len()) ;
    println!("1) Size of [i32;500] bytes: {}", 
                mem::size_of::<[i32;500]>() // Returns the size of a type in bytes.
            ) ;
    println!("2) Size of xs bytes: {}", mem::size_of_val(&xs)) ;

    let empty_array: [i32;0] = [] ;
    assert_eq!(&empty_array, &[]) ;
    assert_eq!(&empty_array, &[][..]) ;

    for i in 0..xs.len() + 1 {
        match xs.get(i) { // Returns a reference to an element or subslice depending on the type of index.
           Some(&x_val) => println!("{}: {}", i, x_val),
           None => println!("Slow down, it's {} too far", i),
        }
    }

    //println!("{}", xs[5]) ; // Compile error: index out of bounds: the length is 5 but the index is 5
    //println!("{}", xs[..][5]) ; // Runtime error: index out of bounds: the len is 5 but the index is 5
    }

    // https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html
    // Structures
    {
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    let point = Point {x: 1.0, y: 2.0} ;

    let point2 = Point {x: 3.0, ..point} ;

    println!("point2: {:?}", point2) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum.html
    // Enums
    // https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum/c_like.html
    // C-like
    {
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
    }

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

    // https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum/testcase_linked_list.html
    // Testcase: linked-list
    {
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
    }
    // https://doc.rust-lang.org/stable/rust-by-example/custom_types/constants.html
    // constants
    {
    static MYSTAT: &str = "m" ;
    static mut MYSTAT_MUT: &str = "a" ;

    const MYCONST: i64 = 10 ;

    println!("{}, {}", MYSTAT, MYCONST) ;

    unsafe {
        MYSTAT_MUT = "a1" ;
    }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/mut.html
    // Mutability
    {
    let mut mutable_int = 10u64 ;
    {
        let mutable_int_2 = mutable_int ;

        // https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/freeze.html
        mutable_int = 50 ;  // mutable_int isn't frozen
    }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/types/cast.html
    // Casting
    // приведение типов
    {
    println!("{}, {}", -1i8 as u8, -100.00 as i8) ;

    unsafe {
        // Rounds toward zero and converts to any primitive integer type, assuming that the value is finite and fits in that type.
        // Окпугление к 0 и преоброзование к любоиу указанному примитивному целому типу.        
        println!("{}", 300.1f32.to_int_unchecked::<u8>()) ;
        println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>()) ;
    }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/types/literals.html
    // Literals
    // литералы и определение их размера
    {
    let f = 1.4f64 ;

    println!("size of f is: {}", 
        std::mem::size_of_val(&f)   // возвращает размер переменной по его указателю
    ) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/conversion/from_into.html
    // From and Into

    {
    let my_str = "hello" ;
    let my_string = String::from(my_str) ;

    #[derive(Debug)]
    struct Number3 {
        value: i32
    }

    // implement from i32 to Number3
    impl From<i32> for Number3 {
        fn from(num: i32) ->Self {
            Self { value: num }
        }
    }

    let numb = Number3::from(10) ;
    println!("numb: {:?}", numb) ;

    // Into автоматически реализуется при реализации From
    let numb2: Number3 = 10.into() ;
    println!("numb2: {:?}", numb2) ;


    #[derive(Debug)]
    struct Number4 {
        value: i32
    }

    // implement i32 into Number4
    impl Into<Number4> for i32 {
        fn into(self) -> Number4 {
            Number4 { value: self }
        }
    }

    let numb: Number4 = 10.into() ;

    println!("numb: {:?}", numb) ;
    
    }

    // https://doc.rust-lang.org/stable/rust-by-example/conversion/try_from_try_into.html
    // TryFrom and TryInto
    // TryFrom - Преобразование типов
    // TryInto - Попытка преобразования, которая потребляет self
    // 
    {
    use std::convert::{TryFrom, TryInto} ;

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32) ;

    impl TryFrom<i32> for EvenNumber {
        type Error = String;

        fn try_from(value: i32) -> Result<Self, Self::Error> {

            if value % 2 == 0 {
                Ok(Self(value))
            } else {
                Err(format!("Invalid value: {}", value))
            }
        }
    }

    impl TryInto<EvenNumber> for f32 {
        type Error = String ;

        fn try_into(self) -> Result<
                                EvenNumber,
                                //Self::Error
                                String
                                > {

            if (self as i32) % 2 == 0 {
                Ok(EvenNumber(self as i32))
            } else {
                Err(format!("Invalid value: {}", self))
            }
        }
    }

    let mut result = EvenNumber::try_from(10) ;
    println!("result: {:?}", result) ;

    result = EvenNumber::try_from(3) ;
    println!("result: {:?}", result) ;

    let mut result2 ;

    match <f32 as TryInto<EvenNumber>>::try_into(11.5f32) {
        Ok(v) => result2 = v,
        Err(err) => {println!("!!!!! error: {}", err)},
    }

    /*
    let mut result2: Result<EvenNumber, String> = 10.5f32
                                                    .try_into() // Это красное но работает
                                                    ;
    println!("result2: {:?}", result2) ;

    result2 = 9.01f32
                    .try_into() // Это красное но работает
                    ;
    println!("result2: {:?}", result2) ;
     */
    }

    // https://doc.rust-lang.org/stable/rust-by-example/conversion/string.html
    // To and from Strings

    {
    use std::fmt ;

    struct Circle {
        radius: i32
    }

    // Converting to String
    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Circle of radius: {}", self.radius)
        }
    }

    let circle = Circle{radius: 10} ;
    println!("circle: {}", circle) ;

    // Parsing a String
    let parsed: u32 = "5"
                        .parse()
                        .unwrap()
                        ;
    // или так:
    let parsed = "5"
                    .parse::<u32>()
                    .unwrap()
                    ;
    println!("parsed: {}", parsed) ;

    // преобразование строки в структуру
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
    }

    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/for.html
    // for and iterators
    {
    let names = vec!["1", "2", "3"] ;

    for name in names
                        .iter()
    {
        match name {
            &"3" => println!("1) It's: {}", name),
            _ => println!("{}", name),
        }
    }

    for name in names.into_iter() {
        match name {
            "3" => println!("2) It's: {}", name),
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
    }

    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/match.html
    // match
    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring/destructure_slice.html
    // arrays/slices
    // деструктуризация массивов и срезов
    {
    let array = [10, 1, 2, 3] ;

    match array {
        [10, tail @ .., last] => println!("1) Complex max {:?}, {}",
                tail, last
            ),
        [1, second, tail @ ..] => println!("2) Complex max {}, {:?}", 
                second, tail),
        _ => println!("Other."),
    }
    }
    
    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring/destructure_pointers.html
    // pointers/ref
    // Dereferencing (разименовывание) uses *
    // Destructuring (деструктуризация) uses &, ref, and ref mut

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

    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring/destructure_structures.html
    // structs
    // деструктуризация структур

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

    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring/destructure_enum.html
    // enums
    // деструктуризация enum и использование дополнительных if для анализа
    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/guard.html
    // Guards - условие if
    {
    enum Temperature {
        NoNo,
        Cels(i32),
        Fahr(i32),
    }

    let temp = Temperature::Cels(35) ;

    match temp {
        Temperature::NoNo => println!("It's NoNo"),
        Temperature::Cels(t) if t > 30 => println!("Big cel: {}", t),
        Temperature::Cels(t) => println!("Low cels: {}", t),
        Temperature::Fahr(t) if t > 86 => println!("Big fahr: {}", t),
        Temperature::Fahr(t) => println!("Low faht: {}", t),
    }

    let number = 10u8 ;
    match number {
        i if i == 0 => println!("Zero!"),
        i if i > 0 => println!("positive number: {}", i),
        _ => println!("Other number"),
    }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring/destructure_tuple.html
    // tuples
    // деструкиуризация кортежей

    {
        let my_tuple = (1, 2, 'A', 5.6, 'C') ;

        println!("my_tuple: {:?}", my_tuple) ;

        match my_tuple {
            // `two_mix @` is not allowed in a tuple !!!!!!!!!!!
            // (1, two_mix @ .., e) => println!("parse my_tuple: two_mix: {}, e: {}", two_mix, e),
            (fisrt_unit, .., last_char) => println!("fisrt_unit: {}, last_char: {}", fisrt_unit, last_char),
            _ => println!("Ërror parsing of my_tuple"),
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/binding.html
    // Binding - привязка значений к именам с помощью @
    {
    fn age() ->u32 {
        15
    }

    match age() {
        0 => println!("Zero!"),
        n @ 1..=12 => println!("Child: {}", n),
        n @ 13..=19 => println!("Teenager: {}", n),
        n => println!("Old man: {}", n),
    }

    fn some_number() ->Option<i32> {
        Some(42)
    }

    match some_number() {
        Some(n@42) => println!("It's: {}", n),
        Some(n) => println!("Other value: {}", n),
        None => println!("None"),
    }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/if_let.html
    // if let - подтверждающий шаблон сопоставления

    {

    let option = Some(7) ;

    match option {
        Some(n @7) => println!("It's 7: {}", n),
        Some(n) => println!("It's other value: {}", n),
        _ => {},
    }

    if let Some(n) = option {
        println!("n: {}", n)
    }

    enum Foo2 {
        Bar,
        Fizz,
        Qux(u32),
    }

    let c = Foo2::Qux(100) ;

    if let Foo2::Qux(n @ 100) = c {
        println!("n is: {}", n) ;
    }

    #[derive(PartialEq)]
    enum Foo3 {
        Bar
    }

    let a = Foo3::Bar ;

    if Foo3::Bar == a {
        println!("It's Foo3::Bar") ;
    }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/let_else.html
    // let-else - опровергаемый шаблон сопоставления
    {
    use std::str::FromStr ;

    fn get_count_item(s: &str) ->(u64, &str) {
        let mut it = s.split(' ') ;
        let (Some(number), Some(item)) = (it.next(), it.next()) else {
            panic!("Cannot segment: {}", s) ;
        } ;

        let Ok(number  ) = u64::from_str(number) else {
            panic!("Invalid number: {}", number) ;
        } ;

        (number, item)
    }

    assert_eq!(get_count_item("3 chairs"), (3, "chairs")) ;
    
    // альтернативный вариант без let-else VVV

    fn get_count_item_2(s: &str) ->(u64, &str) {
        let mut it = s.split(' ') ;

        let (count_str, item) = match (it.next(), it.next()) {
            (Some(n), Some(i)) => (n, i),
            _ => panic!("Cannit segment: {}", s)
        };

        let count = if let Ok(res) = u64::from_str(count_str) {
            res
        } else {
            panic!("Invalid count_str: {}", count_str) ;
        } ;

        (count, item)
    }

    assert_eq!(get_count_item_2("4 chairs"), (4, "chairs")) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/while_let.html
    // while let - подтвержающий шаблон для while
    {

    let mut optional = Some(0) ;

    loop {
        match optional {
            Some(n) => {
                if n > 9 {
                    println!("n is grater than 9") ;
                    optional = None ;
                } else {
                    println!("n is: {}", n) ;
                    optional = Some(n + 1) ;
                }
            },
            _ => break,
        }
    }


    println!("\n") ;

    let mut optional = Some(0) ;

    while let Some(n) = optional {
        if n > 9 {
            println!("n is grater than 9") ;
            optional = None ;
        } else {
            println!("n is: {}", n) ;
            optional = Some(n + 1) ;
        }
    }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/fn/methods.html
    // Associated functions & Methods
    // Associated functions - асоциированы с типом данных
    // Methods - асоциированы с экземпляром типа
    {
        struct Point {
            x: f64,
            y: f64,
        }

        impl Point {
            // Associated functions don't need to be called with an instance.
            fn origin() -> Point {
                Point { x: 0.0, y: 0.0 }
            }

            // Another associated function, taking two arguments:
            fn new(x: f64, y: f64) -> Point {
                Point { x: x, y: y }
            }
        }

        struct Rectangle {
            p1: Point,
            p2: Point,
        }

        impl Rectangle {
            // This is a method
            // `&self` is sugar for `self: &Self`, where `Self` is the type of the
            fn area(&self) -> f64 {
                // `self` gives access to the struct fields via the dot operator
                let Point { x: x1, y: y1 } = self.p1;
                let Point { x: x2, y: y2 } = self.p2;

                ((x1 - x2) * (y1 - y2)).abs()
            }

            fn perimeter(&self) -> f64 {
                let Point { x: x1, y: y1 } = self.p1;
                let Point { x: x2, y: y2 } = self.p2;

                2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
            }

            // This method requires the caller object to be mutable
            // `&mut self` desugars to `self: &mut Self`
            fn translate(&mut self, x: f64, y: f64) {
                self.p1.x += x;
                self.p2.x += x;

                self.p1.y += y;
                self.p2.y += y;
            }
        }

        // `Pair` owns resources: two heap allocated integers
        struct Pair(Box<i32>, Box<i32>);

        impl Pair {
            // This method "consumes" the resources of the caller object
            // `self` desugars to `self: Self`
            fn destroy(self) {
                // Destructure `self`
                let Pair(first, second) = self;

                println!("Destroying Pair({}, {})", first, second);

                // `first` and `second` go out of scope and get freed
            }
        }


        let rectangle = Rectangle {
            // Associated functions are called using double colons
            p1: Point::origin(),
            p2: Point::new(3.0, 4.0),
        };

        // Methods are called using the dot operator
        // Note that the first argument `&self` is implicitly passed, i.e.
        // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
        println!("Rectangle perimeter: {}", rectangle.perimeter());
        println!("Rectangle area: {}", rectangle.area());

        let mut square = Rectangle {
            p1: Point::origin(),
            p2: Point::new(1.0, 1.0),
        };

        // Error! `rectangle` is immutable, but this method requires a mutable
        // object
        // rectangle.translate(1.0, 0.0);

        // Okay! Mutable objects can call mutable methods
        square.translate(1.0, 1.0);

        let pair = Pair(Box::new(1), Box::new(2));

        pair.destroy();

        // Error! Previous `destroy` call "consumed" `pair`
        //pair.destroy();

    }

    // https://doc.rust-lang.org/stable/rust-by-example/fn/closures.html
    // Closures - захват окружающей среды
    // https://doc.rust-lang.org/stable/rust-by-example/fn/closures/capture.html
    // Capturing - Замыкания могут захватывать переменные:
    //      по ссылке: &T
    //      по изменяемой ссылке: &mut T
    //      по значению: T

    {
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    //println!("count: {}", count) ;
    inc();


    let tmp_count = &mut count ;

    // movable перемещается в замыкание
    use std::mem ;

    let movable = Box::new(3) ;

    let consume = || {
        println!("movable: {}", movable) ;
        mem::drop(movable);
    } ;

    consume() ;
    //consume() ;

    // явное перемещение haystack в замыкание по `move`
    let haystack = vec![1,2,3] ;

    let containts = move |needle| haystack.contains(needle) ;

    println!("{}", containts(&1)) ;
    println!("{}", containts(&10)) ;

    // заимствование по ищменяемой ссылке

    let mut my_str = String::from("abc") ;

    let mut my_closure = || {
        my_str = String::from("123") ;
    } ;

    my_closure() ;

    println!("my_str: {}", my_str) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_parameters.html
    // As input parameters
    // Замыкания как аргументы в порядке уменьшения ограничений:
    //    Fn: the closure uses the captured value by reference (&T)
    //    FnMut: the closure uses the captured value by mutable reference (&mut T)
    //    FnOnce: the closure uses the captured value by value (T)
    {
    fn apply<F>(f: F)
        where F: FnOnce()
    {
        f() ;
    }

    fn apply_to_3<F>(f: F) ->i32
        where F: Fn(i32) ->i32
    {
        f(3)
    }

    let greeting = "hello" ;

    let firewall = "abc".to_owned() ;

    let diary = || {
        println!("greeting: {}", greeting) ;
        println!("firewall: {}", firewall) ;
        std::mem::drop(firewall) ;
    } ;

    apply(diary);

    let double = |x| x * 2 ;
    println!("3 dubled: {}", apply_to_3(double)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/fn/closures/anonymity.html
    // Type anonymity - задание типов анонимности
    // https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_functions.html
    // Input functions - входные фугкции
    {
    fn apply_2<F>(f: F)
        where F: Fn()
    {
        f() ;
    }

    let x = 7 ;

    let print = || println!("{}", x) ;

    apply_2(print);
    apply_2(print);

    // или так:

    fn call_me<F: Fn()>(f: F) {
        f() ;
    }

    fn f1() {
        println!("I'm a function!") ;
    }

    let closure = || println!("I'm closure!") ;

    call_me(f1);
    call_me(closure);
    }
    
    // https://doc.rust-lang.org/stable/rust-by-example/fn/closures/output_parameters.html
    // As output parameters
    // The valid traits for returning a closure are:
    //      Fn
    //      FnMut
    //      FnOnce
    {
    fn create_fnmut() ->impl FnMut() {
        let text = "FnMut".to_owned() ;

        move || println!("This is: {}", text)
    }

    let mut fn_mut = create_fnmut() ;

    fn_mut() ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/fn/closures/closure_examples/iter_any.html
    // Iterator::any
    // Iterator::any — это функция, которая, получив на вход итератор, 
    // возвращает true, если хотя бы один элемент удовлетворяет предикату. 
    // В противном случае — false. 
    {
    let vec1 = vec![1, 2, 3] ;

    println!("2 in vec1: {}", vec1
                                .iter()
                                .any(|&x| x == 2)
                            ) ;


    println!("4 in vec1: {}", 
        vec1
            .iter()
            .any(|x| *x == 4)
    ) ;

    let vec2 = vec![4, 5, 6] ;

    println!("5 in vec2: {}", vec2
                                .into_iter()
                                .any(|x| x == 5)
                            ) ;
    }
    // https://doc.rust-lang.org/stable/rust-by-example/fn/closures/closure_examples/iter_find.html
    // Searching through iterators
    // возвращение первого значения по условию замыкания 
    {
    let vec1 = vec![1, 2, 3] ;

    println!("Find 2 in vec1: {:?}", vec1
                                    .iter()
                                    .find(|x| **x == 2)
                                ) ;
    
    println!("Find 10 in vec1: {:?}", vec1
                                        .iter()
                                        .find(|&&x| x == 10)
                                    ) ;

    println!("Find 1 in vec1: {:?}", vec1
                                    .into_iter()
                                    .find(|x| *x == 1)
                                ) ;

    let vec1 = vec![1, 2, 3] ;

    println!("Find 3 in vec1: {:?}", vec1
                                    .iter()
                                    .find(|&&x| x == 3)
                                ) ;

    // возвращение позиции найденного элемента

    let vec1 = vec![1, 2, 3] ;

    let v = vec1
                            .iter()
                            .position(|&x| x == 1) 
                            ;

    assert_eq!(v, Some(0)) ;

    let v = vec1
                                .iter()
                                .position(|x| *x == 10) 
                                ;

    assert_eq!(v, None) ;
    println!("v: {:?}", v) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/fn/hof.html
    // Higher Order Functions (HOF) - функции  высшего порядка
    // Это функции, которые принимают одну или несколько других функций
    // и/или создают более полезную функцию.
    {
    fn is_odd(n: u32) ->bool {
        n % 2 == 1
    }

    let upper = 1000 ;

    let my_sum = (0..)
                                    .map(|x| x * x)
                                    .take_while(|&x | x < upper)
                                    .filter(|&x| {
                                        println!("x: {}", x) ;
                                        is_odd(x)
                                    })
                                    .sum::<u32>()
                                    ;

    println!("my_sum: {}", my_sum) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/fn/diverging.html#diverging-functions
    // Diverging functions
    // Расходящиеся функции никогда не возвращают значение.

    {

    fn foo() ->! {
        panic!("abc") ;
    }

    // ------------------------

    fn sum_odd_numbers(up_to: u32) ->u32 {
        let mut acc = 0 ;

        for i in 0..up_to {
            let additional = match i%2 == 1 {
                true => i,                
                false => continue,  // continue не возвразает значение ии не нарушает требование к типу возвразаемому match 
            };
            acc += additional ;
        }

        acc
    }

    println!("Sum odd from 0 to 9: {}", sum_odd_numbers(9)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/mod/use.html
    // Modules - The use declaration

    // https://doc.rust-lang.org/stable/rust-by-example/mod/super.html
    // Modules - super and self

    // https://doc.rust-lang.org/stable/rust-by-example/mod/split.html
    // Modules - File hierarchy

    // https://doc.rust-lang.org/stable/rust-by-example/cargo/deps.html
    // Cargo - Dependencies

    // https://doc.rust-lang.org/stable/rust-by-example/cargo/conventions.html
    // Cargo -Conventions - дополнительные бинарники в суб. дир. bin
/*
foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs    
 */

    // https://doc.rust-lang.org/stable/rust-by-example/cargo/test.html
    // Cargo - Testing
    // модульные тесты можно размещать в модулях,
    // нтеграционные тесты — в отдельной директории tests/
    // Каждый файл в tests/ представляет собой отдельный интеграционный 
    // тест, то есть тест, предназначенный для проверки вашей библиотеки
    // так, как если бы она вызывалась из зависимого крейта.    
    /*
foo
├── Cargo.toml
├── src
│   └── main.rs
│   └── lib.rs
└── tests
    ├── my_test.rs
    └── my_other_test.rs    
    */
    
    // https://doc.rust-lang.org/stable/rust-by-example/cargo/build_scripts.html
    // Cargo - Build Scripts 
    // Скрипт сборки — это просто еще один файл Rust, который будет 
    // скомпилирован и вызван перед компиляцией чего-либо еще в пакете.
    
    // https://doc.rust-lang.org/stable/rust-by-example/attribute.html
    // Attributes
    // #[outer_attribute] применяется к элементу немедленно следующему за ним.
    // #![inner_attribute] применяется к окружению элемента (обычно можулю или крейту).

    // https://doc.rust-lang.org/stable/rust-by-example/attribute/unused.html
    // Attributes - dead_code - #[allow(dead_code)]
    {
    #[allow(dead_code)]     // разрешён мертвый код
    #[cfg(target_os = "windows")] // оценка флагов компиляции, возможно не включает код ниже (Удаляет код ниде если false).
    fn unused_function() {}

    if cfg!(target_os = "windows") { // Вычисляет логические комбинации конфигурационных флагов во время компиляции. Код не удаляется
        println!("It's windows.") ;
    }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/attribute/crate.html
    // Attributes - Crates - #![crate_type = "lib"] - #![crate_name = "rary"]
    // cargo не использует эти атрибуты

    // https://doc.rust-lang.org/stable/rust-by-example/attribute/cfg.html
    // Attributes - cfg - Configuration conditional
    //      the cfg attribute: #[cfg(...)] in attribute position
    //      the cfg! macro: cfg!(...) in boolean expressions    

    // https://doc.rust-lang.org/stable/rust-by-example/attribute/cfg/custom.html
    // Attributes - Custom
    // Some conditionals like target_os are implicitly provided by rustc,
    // but custom conditionals must be passed to rustc using the --cfg flag.

    // https://doc.rust-lang.org/stable/rust-by-example/mod/struct_visibility.html
    // Struct visibility
    // Видимость элементов структуры работает вне модуля.
    {
    let open_box = my::OpenBox {contents: "public data"} ;
    println!("open_box.contents: {}", open_box.contents) ;

    let close_book = my::CloseBox::new("private data") ;

    close_book.show();
    }

    // ------------------------

    use std::fs::OpenOptions ;
    use std::io::Write ;

    let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("text.txt")
                    .expect("Error to open file.")
                    ;

    file
        .write_all("äaaa".as_bytes())
        .expect("error to write file.")
        ;

    // https://doc.rust-lang.org/stable/rust-by-example/generics.html
    // Generics - обобщённые типы
    {
    struct SingheGeneric<T>(T) ;

    let str_char = SingheGeneric('c') ;
    let str_int = SingheGeneric(1) ;
    }


    // https://doc.rust-lang.org/stable/rust-by-example/generics/gen_fn.html
    // Generics - Functions - обобщённые типы функций
    // fn generic<T>(_s: SGen<T>) {}    // Define a function `generic`
    // generic::<char>(SGen('a'));  // Explicitly specified type parameter `char`
    // generic(SGen('c')); // Implicitly specified type parameter `char`

    // https://doc.rust-lang.org/stable/rust-by-example/generics/impl.html
    // Generics - Implementation
    {
    struct GenVal<T> {
        gen_val: T,
    }

    impl <T> GenVal<T> {
        fn value(&self) ->&T {
            &self.gen_val
        }
    }

    let y = GenVal{gen_val: 10} ;

    println!("y: {}", y.value()) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/generics/gen_trait.html
    // Generics - Traits (пример универсальных трейтов)
    {
    struct Empty;
    struct Null;

    // A trait generic over `T`.
    trait DoubleDrop<T> {
        // Define a method on the caller type which takes an
        // additional single parameter `T` and does nothing with it.
        fn double_drop(self, _: T);
    }

    // Implement `DoubleDrop<T>` for any generic parameter `T` and
    // caller `U`.
    impl<T, U> DoubleDrop<T> for U {
        // This method takes ownership of both passed arguments,
        // deallocating both.
        fn double_drop(self, _: T) {
            println!("Inside double_drop !!!!!!!!!!!!!!!!!!!!") ;
        }
    }

    let empty = Empty ;
    let null = Null ;
    empty.double_drop(null);
    //empty ; // use of moved value: `empty`
    //null;   // use of moved value: `null`
    }

    // https://doc.rust-lang.org/stable/rust-by-example/generics/bounds.html
    // Bounds
    {
    use std::fmt::Display ;
    use std::fmt::Debug;

    struct S<T: Display>(T) ;

    let s_int = S(1) ;
    let s_string = S("abc".to_string()) ;
    //let s_vec = S(vec![1]) ; // Error: `Vec<{integer}>` doesn't implement `std::fmt::Display`
   
    trait HashArea {
        fn area(&self) ->f64 ;
    }

    #[derive(Debug)]
    struct Rectangle {
        lenght: f64,
        height: f64,
    }

    impl HashArea for Rectangle {
        fn area(&self) ->f64 {
            self.lenght * self.height
        }
    }

    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t) ;
    }

    fn area<T: HashArea>(t: &T) ->f64 {
        t.area()
    }
    
    let rect = Rectangle {lenght: 10.0, height: 20.0} ;

    print_debug(&rect);

    println!("Space: {}", area(&rect)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/generics/bounds/testcase_empty.html
    // Testcase: empty bounds - трейты не включающие функциональность
    {
        struct Cardinal;
        struct BlueJay;

        trait Red {}
        trait Blue {}

        impl Red for Cardinal {}
        impl Blue for BlueJay {}

        fn red<T: Red>(_: &T) ->&'static str {"red"}
        fn blue<T: Blue>(_: &T) ->&'static str {"blue"}

        let cardinal = Cardinal ;
        let blue_jay = BlueJay ;

        println!("A cardinal is {}.", red(&cardinal)) ;
        println!("A blue jay is {}.", blue(&blue_jay)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/generics/multi_bounds.html
    // Multiple bounds - несколько границ для трейта
    {
    use std::fmt::Debug;
    use std::fmt::Display;

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: {:?}", t) ;
        println!("Display: {}", t) ;
    }

    fn compare_type<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: {:?}", t) ;
        println!("u: {:?}", u) ;
    }

    let str_1 = "abc" ;

    compare_prints(&str_1);
    let array = [1,2,3] ;
    let vec = vec![1,2,3] ;
    compare_type(&array, &vec);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/generics/where.html
    // Where clauses
    {
        use std::fmt::Debug ;

        trait PrintOption {
            fn print_in_option(self) ;
        }

        // для универсального трейта T с ограничениями
        impl <T> PrintOption for T
            where Option<T>: Debug  // для Option<T> должен быть реализован трейт Debug
        {
            fn print_in_option(self) {
                println!("print_in_option: {:?}", Some(self)) ;
            }
        }

        let vec = vec![1,2,3] ;
        vec.print_in_option();
    }

    // https://doc.rust-lang.org/stable/rust-by-example/generics/new_types.html
    // New Type Idiom - на этапе компиляции программе будет передано 
    // значение правильного типа.
    {
    struct Years(i64) ;
    struct Days(i64) ;

    impl Years {
        pub fn to_days(&self) ->Days {
            Days(self.0 * 365)
        }
    }

    fn is_adalt(age: &Years) ->bool {
        age.0 > 18
    }

    let year = Years(10) ;
    let age_days = year.to_days() ;
    //is_adalt(&age_days) ;   // mismatched types
    let Years(year_int) = year ;    // Destructuring !!!!!
    println!("year_int: {}", year_int) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/generics/assoc_items/the_problem.html
    // The Problem - пользователи трейта должны указывать все его 
    // обобщенные типы.
    {
    struct Conteiner(i32, i32) ;

    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) ->bool ;
        fn first(&self) ->i32 ;
        fn last(&self) ->i32 ;
    }

    impl Contains<i32, i32> for Conteiner {

        fn contains(&self, number_1: &i32, number_2: &i32) ->bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        fn first(&self) ->i32 {
            self.0
        }

        fn last(&self) ->i32 {
            self.1
        }
    }

    fn difference<A, B, C>(container: &C) ->i32
        where C: Contains<A, B>
    {
        container.first() - container.last()
    }

    let number_1 = 3 ;
    let number_2 = 10 ;

    let container = Conteiner(number_1, number_2) ;

    println!("Does it containt: {}", container.contains(&number_1, &number_2)) ;

    println!("first: {}, last: {}", container.first(), container.last()) ;
    println!("Difference: {}", difference(&container)) ;

    let a = 10 ;
    println!("{}", &a == &10) ; // происходит неявное разименовывание при сравнении
    }

    // https://doc.rust-lang.org/stable/rust-by-example/generics/assoc_items/types.html
    // Associated types of Trate - для читабельности

    {
    struct Container2(i32, i32) ;

    trait Contaits2  {
        // обобщённые типы
        type A  ;
        type B ;

        fn containts(&self, _: &Self::A, _: &Self::B) ->bool ;
        fn first(&self) ->i32 ;
        fn last(&self) ->i32 ;

    }

    impl Contaits2 for Container2 {
        // указатьчем типы явзяются
        type A = i32;
        type B = i32;

        fn containts(&self, number_1: &Self::A, number_2: &Self::B) ->bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        fn first(&self) ->i32 {
            self.0
        }

        fn last(&self) ->i32 {
            self.1
        }
    }

    fn difference2<C: Contaits2>(container: &C) ->i32 {
        container.first() - container.last()
    }

    let number_1 = 3 ;
    let number_2 = 10 ;

    let container = Container2(number_1, number_2) ;

    println!("Does it contain: {}", container.containts(&number_1, &number_2)) ;
    println!("first: {}, last: {}", container.first(), container.last()) ;
    println!("Difference: {}", difference2(&container)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/generics/phantom.html
    // Phantom type parameters
    // Типы данных могут использовать дополнительные параметры 
    // обобщенного типа в качестве маркеров или для проверки типов во 
    // время компиляции. Эти дополнительные параметры не хранят никаких 
    // значений и не имеют поведения во время выполнения.
    //
    // Пример изменён.
    {
    use std::marker::PhantomData ;

    struct Start ;
    struct Ongoing ;
    struct Finished ;

    struct Process<T> {
        _marker:    PhantomData<T>,
    }

    impl Process<Start> {
        pub fn new() ->Self {
            Process { _marker: PhantomData }
        }

        pub fn begin(&self) ->Process<Ongoing> {
            println!("Process started!") ;
            Process { _marker: PhantomData }
        }
    }

    impl Process<Ongoing> {
        pub fn complete(&self) ->Process<Finished> {
            println!("Process completed!") ;
            Process { _marker: PhantomData }
        }
    }

    let process = Process::new() ;
    let process = process.begin() ;
    let process = process.complete() ;
    // process.begin() ; // no method named `begin` found for struct `Process<Finished>`
    }


    // https://doc.rust-lang.org/stable/rust-by-example/generics/phantom/testcase_units.html
    // Testcase: unit clarification - юнитовое пояснение

    {
    use std::ops::Add ;
    use std::marker::PhantomData ;

    #[derive(Clone, Copy)]
    enum Inch {}
    
    #[derive(Clone, Copy)]
    enum Mm {}

    #[derive(Clone, Copy)]
    struct Length<T>(f64, PhantomData<T>) ;

    impl <T> Add for Length<T> {
        type Output = Length<T>;

        fn add(self, rhs: Self) -> Self::Output {
         Length(self.0 + rhs.0, PhantomData)
        }
    }

    let one_inch: Length<Inch> = Length(10.2, PhantomData) ;

    let one_meter: Length<Mm> = Length(1.6, PhantomData) ;

    let two_inches = one_inch + one_inch ;
    println!("two_inces: {}", two_inches.0) ;

    let two_metes = one_meter + one_meter ;
    println!("two meters: {}", two_metes.0) ;

    // let mix = one_inch + one_meter ; // mismatched types
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/raii.html
    // RAII (Resource Acquisition Is Initialization)
    // 
    {
    fn create_box() {
        let v = Box::new(3_i32) ;
    }

    // There's no need to manually free memory!
    for _ in 0_u32..1000_u32 {
        create_box();
    }

    // Destructor
    // Деструктор вызывается, когда ресурс выходит из области видимости.

    struct ToDrop ;

    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is beeing dropped") ;
        }
    }

    {
        let x = ToDrop ;
    }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/move/mut.html
    // Mutability
    // Mutability of data can be changed when ownership is transferred.
    {
    let imut_box = Box::new(5_u32) ;
    println!("imut_box: {}", imut_box) ;

    let mut mut_box = imut_box ;    // moved to mut_box

    *mut_box += 10 ;
    println!("mut_box: {}", mut_box) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/move/partial_move.html
    // Partial moves
    {

    #[derive(Debug)]
    struct Person {
        name:   String,
        age:    Box<u8>,
    }

    let person 
            = Person {
                name:   String::from("abc"),
                age:    Box::new(10),
            } ;
    
    // частичное перемещение name из person
    let Person { name, ref age } = person ;

    println!("name: {}, age: {}", name, age) ;

    //println!("{:?}", person) ;  // borrow of partially moved value: `person`

    // person.age можно использовать
    println!("person.age: {}", person.age) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/borrow.html
    // Borrowing
    {
    // This function takes ownership of a box and destroys it
    fn ate_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying Box<i32>: {}", boxed_i32) ;
    }

    // This function borrows an i32
    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int: {}", borrowed_i32) ;
    }

    let boxed_i32 = Box::new(10) ;
    let stacked_i32 = 6_i32 ;

    // Borrow the contents of the box. Ownership is not taken,
    borrow_i32(&boxed_i32);
    // Borrow i32
    borrow_i32(&stacked_i32);

    {
        let ref_to_i32/*: &i32  */ = &boxed_i32 ;

        println!("ref_to_i32: {}", ref_to_i32) ;

        // boxed_i32 moved to ate_box_i32
        ate_box_i32(boxed_i32);

        //println!("boxed_i32: {}", boxed_i32) ;  // borrow of moved value: `boxed_i32`


        //borrow_i32(ref_to_i32); // ссылка больше не действительна

        //println!("ref_to_i32: {}", ref_to_i32) ; // ссылка больше не действительна
    }

    //ate_box_i32(boxed_i32); // use of moved value: `boxed_i32`
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/mut.html
    // Mutability

    {
        #[derive(Debug)]
        #[derive(Clone, Copy)]
        struct Book {
            author: &'static str,
            title:  &'static str,
            year:   u32,
        }

        fn borrow_book(book: &Book) {
            println!("Immutable borrow: {:?}", book) ;
        }

        fn new_edition(book: &mut Book) {
            book.year = 2015 ;
            println!("Mutable borrow: {:?}", book) ;
        }

        let imm_book 
                = Book {author: "Author",
                        title:  "Title",
                        year:   2001, 
                } ;

        let mut mut_book = imm_book ;

        // Immutably borrow an immutable object
        borrow_book(&imm_book);

        // Immutably borrow a mutable object
        borrow_book(&mut_book);

        // Borrow a mutable object as mutable
        new_edition(&mut mut_book);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/alias.html
    // Aliasing - несколько ссылок на одно и то же

    {
        struct Point {
            x:  i32,
            y:  i32,
            z:  i32,
        }

        let mut point = Point {x: 1, y: 2, z: 3} ;
        let borrowing_point = &point ;
        let another_point = &point ;

        println!("Coordinate x: {}, y: {}, x: {}", point.x, borrowing_point.y, another_point.z) ;

        let mutable_borrowing = &mut point ;

        mutable_borrowing.x = 10 ;
        mutable_borrowing.y = 20 ;
        mutable_borrowing.z = 30 ;

        println!("Mutable x: {}, y: {}, z: {}", mutable_borrowing.x, mutable_borrowing.y, mutable_borrowing.z) ;

        println!("Origing x: {}", point.x) ;

        let new_borrowing_point = &point ;

        println!("New borrowing x: {}", new_borrowing_point.x) ;

        // https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/ref.html
        // The ref pattern
        // ключевое слово `ref` можно использовать для получения ссылок на 
        // поля структуры/кортежа.

        {
            let c = 'Q' ;
            let ref ref_c1 = c ;
            let ref_c2 = &c ;

            println!("ref_c1 equals ref_c2 {}", *ref_c1 == *ref_c2) ;

            #[derive(Clone, Copy)]
            struct Point {
                x: i32,
                y: i32,
            }

            let point = Point {
                x: 10,
                y: 20,
            } ;

            let copy_of_x = {
                let Point {x: ref ref_to_x, y: _} = point ; // destructuring
                *ref_to_x
            } ;

            // A mutable copy of `point`
            let mut point_mut = point ;

            {
                let Point {
                    x: ref mut ref_to_x, 
                    y: ref mut ref_to_y
                } = point_mut ; // destructuring

                *ref_to_x += 1 ;
                *ref_to_y += 1 ;
            }

            println!("point is ({}, {})", point.x, point.y) ;
            println!("point_mut is ({}, {})", point_mut.x, point_mut.y) ;

            let mut mutable_tuple = (Box::new(10),2_u32) ;

            {
                let (_, ref mut last) = mutable_tuple ; // destructuring
                *last *= 10 ;
            }

            println!("tuple: {:?}", mutable_tuple) ;
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html
    // Lifetimes

    {
    let i = 3; // Lifetime for `i` starts.────────────────┐
    //                                                         │
    { //                                                       │
      let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                    ││
        println!("borrow1: {}", borrow1); //                  ││
    } // `borrow1` ends.     ─────────────────────────────────┘│
    //                                                         │
    //                                                         │
    { //                                                       │
      let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                    ││
        println!("borrow2: {}", borrow2); //                  ││
    } // `borrow2` ends.     ─────────────────────────────────┘│
    //                                                         │
    }// Lifetime ends.    ─────────────────────────────────────┘

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/explicit.html
    // Lifetimes Explicit annotation
    //
    // foo<'a, 'b>
    // В этом случае время жизни foo не может превышать время жизни 'a' 
    // или 'b'.

    {
        // Оба этих времени жизни должны быть как минимум равны длительности 
        // функции `print_refs`.
        fn print_ref<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("x: {}, y: {}", x, y) ;
        }

        // время жижни failed_borrow не может превышать время жижни 'a
        fn failed_borrow<'a>() {
            let x = 12 ;
            //let y: &'a i32 = &x ;   // `x` does not live long enough
        }

        let (four, nine) = (4, 9) ;

        print_ref(&four, &nine);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/fn.html
    // Lifetimes Functions

    {
        // x must live at least as long as the function.
        fn print_one<'a>(x: &'a i32) {
            println!("print_one is {}", x) ;
        }

        // Mutable references are possible with lifetimes as well.
        fn add_ones<'a>(x: &'a mut i32) {
            *x += 1 ;
        }

        // Multiple elements with different lifetimes.
        fn print_mut<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("print_mut: x: {}, y: {}", x, y) ;
        }

        // Returning references that have been passed in is acceptable.
        fn pass_x<'a,'b>(x: &'a i32, y: &'b i32) ->&'a i32 {
            x
        }

        let (x, y) = (7, 9) ;

        print_one(&x);
        print_mut(&x, &y);

        let z = pass_x(&x, &y) ;
        println!("z: {}", z) ;

        let mut t = 3 ;
        add_ones(&mut t);
        println!("t: {} !!!!!!!!!!!!!!", t) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/methods.html
    // Lifetimes Methods

    {
        #[derive(Debug)]
        struct Owner(i32) ;

        impl Owner {
            // так:
            fn add_one<'a>(&'a mut self) {
                self.0 += 1 ;
            }
            // или так:
            fn add_two(&mut self) {
                self.0 += 2 ;
            }
        }

        let mut owner = Owner(10) ;
        owner.add_one();
        println!("1) owner: {:?}", owner) ;

        owner.add_two();
        println!("2) owner: {:?}", owner) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/struct.html
    // Lifetimes Structs

    {
        struct Borrowed<'a>(&'a i32) ;

        struct NameBorrowed<'a> {
            x: &'a i32,
            y: &'a i32,
        }

        #[derive(Debug)]
        enum Either<'a> {
            Num(i32),
            Ref(&'a i32),
        }

        let x = 10 ;
        let y = 20 ;

        let single = Borrowed(&x) ;
        let double = NameBorrowed{x: &x, y: &y} ;
        let refrence = Either::Num(x) ;
        let number = Either::Ref(&y) ;

        println!("number: {:?}, ref: {:?}", number, refrence) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/trait.html
    // Lifetimes Traits

    {
        #[derive(Debug)]
        struct Borrowed<'a> {
            x: &'a i32,
        }

        // Default - A trait for giving a type a useful default value.
        impl <'a> Default for Borrowed<'a> {
            fn default() -> Self {
                Self {
                    x: &10,
                }
            }
        }

        let b = Borrowed::default() ;
        println!("b: {:?}", b) ;
    }
    
    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/lifetime_bounds.html
    // Bounds
    //  1) T: 'a: All references in T must outlive lifetime 'a.
    //  2) T: Trait + 'a: Type T must implement trait Trait and all references in T must outlive 'a.
    {
        use std::fmt::Debug ;

        #[derive(Debug)]
        struct Ref<'a, T: 'a>(&'a T) ;

        fn print_ref<'a, T>(t: &'a T)
            where T: Debug //+ 'a
        {
            println!("ref t is: {:?}", t) ;
        }

        fn print<T>(t: T) 
            where T: Debug
        {
            println!("t is: {:?}", t) ;
        }

        let x = 10 ;
        let ref_x = Ref(&x) ;

        print_ref(&ref_x);
        print(ref_x) ;
        //print(ref_x) ; // use of moved value: `ref_x`

    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/lifetime_coercion.html
    // Coercion
    // Более длительное время жизни можно сократить, чтобы оно работало 
    // в области видимости, в которой обычно не работает. Это достигается
    // за счет автоматического преобразования компилятором Rust, а также 
    // путем объявления разницы во времени жизни:

    {
        // создаётся максимально короткое время жизни
        // обе ссылки приводятся к этому времени жизни
        fn multiply<'a>(first: &'a i32, second: &'a i32) ->i32 {
            first * second
        }

        // 'a: 'b -> lifetime `'a` is at least as long as `'b`
        fn coose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) ->&'b i32 {
            first
        }

        let first = 2 ;

        {
            let second = 3 ;
            println!("The product is {}", multiply(&first, &second)) ;
            println!("{} is first", coose_first(
                                        // так:
                                        &first,
                                        &second
                                        /*
                                        или так:
                                        &second,
                                        &first
                                         */
                                    )) ;
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/static_lifetime.html
    // Static

    {
        // Make a constant with `'static` lifetime.
        static NUM: i32 = 18 ;
        
        // Возвращает ссылку на `NUM`, где его `'static` время жизни 
        // преобразуется во время жизни входного аргумента.
        fn coerst_static<'a>(_: &'a i32) ->&'a i32 {
            &NUM
        }

        {
            let static_string = "abc" ;
            println!("static_string: {}", static_string) ;
        }

        {
            let life_time_num = 10 ;
            let coerced_stat = coerst_static(&life_time_num) ;
            println!("coerced_stat: {}", coerced_stat) ;
        }
        println!("NUM: {} !!!!!!!!!!!!!!!!!!!", NUM) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/static_lifetime.html
    // dynamically create 'static references - Box::leak
    // динамическое создания статических ссылок
    // исправленный вариант
    {
        //extern crate rand ;
        use rand::Rng ;

        fn random_vec() ->&'static [usize; 100] {

            let mut v_rand: [usize; 100] = [0_usize; 100] ;

            for v in &mut v_rand {
                //*v = rand::thread_rng().gen_range(0..=usize::MAX) ;
                *v = rand::rng()
                        .random_range(0..=usize::MAX) ;
            }

            let mut boxed = Box::new(v_rand) ;

            /*
            Эта функция в основном полезна для данных, которые сохраняются на 
            протяжении всего жизненного цикла программы. Удаление возвращаемой
            ссылки приведет к утечке памяти. Если это неприемлемо, ссылку следует
            сначала обернуть функцией Box::from_raw, создающей объект Box. Затем
            этот объект Box можно удалить, что корректно уничтожит объект T и 
            освободит выделенную память.
             */
            Box::leak(boxed)
        }

        let first = random_vec() ;
        let second = random_vec() ;

        println!("first: {:?}\nsecond:{:?}", first, second) ;

    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/static_lifetime.html
    // Static Trait bound
    {
        use std::fmt::Debug ;

        fn print_it(input: impl Debug + 'static) {
            println!("'static is: {:?}", input) ;
        }
        // Объект i является собственным и не содержит ссылок, 
        // поэтому он «статичен»:
        let i = 5  ;
        print_it(i);

        //print_it(&i);   // `i` does not live long enough

    }

    // https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/elision.html
    // Elision
    // Некоторые шаблоны времени жизни встречаются крайне часто, поэтому
    // проверка заимствований позволяет их опускать, чтобы сэкономить 
    // время на наборе текста и улучшить читаемость. 
    // Это называется элизией. Элизия существует в Rust исключительно 
    // потому, что эти шаблоны распространены.

    {
        // время жизни определяется компилятором при одной ссылке в аргументах
        fn elided_input(x: &i32) {
            println!("elided_input: x: {}", x) ;
        }
        // аналогичный код
        fn annotated_input<'a>(x: &'a i32) {
            println!("annotated_input: x: {}", x) ;
        }

        // время жизни определяется компилятором при одной ссылке в аргументах
        fn elided_pass(x: &i32) ->&i32 {
            x
        }
        // аналогичный код
        fn annotated_pass<'a>(x: &'a i32) ->&'a i32 {
            x
        }

        let x = 3;

        elided_input(&x);
        annotated_input(&x);

        println!("`elided_pass`: {}", elided_pass(&x));
        println!("`annotated_pass`: {}", annotated_pass(&x));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait.html
    // Traits
    
    {
        struct Sheep {
            naked: bool,
            name:  &'static str,
        }

        trait Animal {
            fn new(name: &'static str) ->Self ;

            fn name(&self) ->&'static str ;

            fn noise(&self) ->&'static str ;

            // default method definition
            fn talk(&self) {
                println!("{} says {}", self.name(), self.noise()) ;
            }
        }

        impl Sheep {
            fn  is_naked(&self) ->bool {
                self.naked
            }

            fn shear(&mut self) {
                if self.is_naked() {
                    println!("Already sheared.") ;
                } else {
                    {
                        println!("{} gets haircut", self.name) ;
                        self.naked = true ;
                    }
                }
            }
        }

        impl Animal for Sheep {

            // `Self` is the implementor type: `Sheep`.
            fn new(name: &'static str) ->Self {
                Self { naked: false, name }
            }
            
            fn name(&self) ->&'static str {
                self.name
            }

            fn noise(&self) ->&'static str {
                if self.is_naked() {
                    "b1"
                } else {
                    "b2"
                }
            }

            fn talk(&self) {
                println!("{} talks {}", self.name, self.noise()) ;
            }
        }

        let mut dolly = Sheep::new("Dolly") ;

        dolly.talk();
        dolly.shear();
        dolly.talk();

        // или  так:  !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        let mut dolly2: Sheep = Animal::new("Dolly2") ;
        dolly2.talk();
        dolly2.shear();
        dolly2.talk();

    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait/derive.html
    // Derive
    // Компилятор способен предоставлять базовые реализации для некоторых
    // трейтов с помощью атрибута #[derive].
    /*
    The following is a list of derivable traits:
        Comparison traits: Eq, PartialEq, Ord, PartialOrd.
        Clone, to create T from &T via a copy.
        Copy, to give a type 'copy semantics' instead of 'move semantics'.
        Hash, to compute a hash from &T.
        Default, to create an empty instance of a data type.
        Debug, to format a value using the {:?} formatter.    
    */
    {
        #[derive(Debug)]
        struct Seconts(i32) ;

        #[derive(PartialEq, PartialOrd)]
        struct Centimeters(f64) ;

        #[derive(Debug)]
        struct Inces(f64) ;

        impl Inces {
            fn to_centimeter(&self) ->Centimeters {
                // так:
                Centimeters(self.0 * 2.54)
                /* или так:
                let &Inces(inch_int) = self ;
                Centimeters(inch_int * 2.54)
                 */
            }
        }

        let one_sec = Seconts(1) ;

        println!("One sec: {:?}", one_sec) ;

        let foot = Inces(12_f64) ;
        
        println!("One foot is: {:?}", foot) ;

        let meter = Centimeters(100.0) ;

        let cmp = if foot.to_centimeter() > meter {
            "bigger"
        } else {
            "smaller"
        } ;

        println!("one foot is {} than one meter", cmp) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait/dyn.html
    // Returning Traits with dyn
    /*
    Компилятору Rust необходимо знать, сколько места занимает 
    возвращаемый тип каждой функции.
    Вы не можете написать функцию, которая возвращает Animal, потому что 
    для разных реализаций потребуется разное количество памяти.
    Вместо того чтобы напрямую возвращать объект трейта, наши функции 
    возвращают Box, который содержит объект Animal. 
    Box — это просто ссылка на некоторую область памяти в куче. 
    Поскольку ссылка имеет статически известный размер компилятор может 
    гарантировать, что она указывает на выделенный в куче объект Animal.
    Вам необходимо указать возвращаемый тип с помощью ключевого слова dyn,
    например, Box<dyn Animal>.
     */
    {
        struct Sheep {}
        struct Cow {}

        trait Animal {
            fn noise(&self) ->&'static str ;
        }

        impl Animal for Sheep {
            fn noise(&self) ->&'static str {
                "baaaaaah!"
            }
        }

        impl Animal for Cow {
            fn noise(&self) ->&'static str {
                "moooo!"
            }
        }

        // Returns some struct that implements Animal, but we don't know 
        // which one at compile time.
        fn random_amimal(random_number: f64) ->Box<dyn Animal> {
            if random_number < 0.5 {
                Box::new(Sheep{})
            } else {
                Box::new(Cow {})
            }
        }

        let random_number = 0.4 ;
        let animal = random_amimal(random_number) ;
        println!("This noise: {}", animal.noise()) ;
        let random_number2 = 0.6 ;
        let animal2 = random_amimal(random_number2) ;
        println!("This noise: {}", animal2.noise()) ;

    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait/ops.html
    // Operator Overloading
    // Перегрузка операторов

    {
        use std::ops ;  // Overloadable operators.

        struct Foo ;
        struct Bar ;

        #[derive(Debug)]
        struct FooBar ;

        #[derive(Debug)]
        struct BarFoo ;

        // The following block implements the operation: Foo + Bar = FooBar
        impl ops::Add<Bar> for Foo {
            type Output = FooBar;

            fn add(self, rhs: Bar) -> Self::Output {
                println!("> Foo.add(Bar)") ;
                FooBar
            }
        }

        // The following block implements the operation: Bar + Foo = BarFoo
        impl ops::Add<Foo> for Bar {
            type Output = BarFoo;

            fn add(self, rhs: Foo) -> Self::Output {
                println!("> Bar.add(Foo)") ;
                BarFoo
            }
        }

        let foo = Foo ;
        let bar = Bar ;
        let foo_bar = foo + bar ;
        println!("foo_var: {:?}", foo_bar) ;

        println!("Bar + Foo: {:?}", Bar + Foo) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait/drop.html
    // Drop
    // Трейт Drop имеет только один метод: drop, который вызывается 
    // автоматически, когда объект выходит из области видимости. 
    // Основное назначение трейта Drop — освобождение ресурсов, 
    // принадлежащих экземпляру, реализующему этот трейт.

    {
        struct Dropable {
            name: &'static str,
        }

        // This trivial implementation of `drop` adds a print to console.
        impl Drop for Dropable {
            fn drop(&mut self) {
                println!("> Dropping: {}", self.name)
            }
        }

        let a = Dropable {name: "a"} ;

        {
            let a = Dropable {name: "b"} ;

            println!("End B block.") ;
        }

        println!("After End B block.") ;

        // Variable can be manually dropped using the `drop` function
        drop(a);

        println!("End main block.") ;
    }

    // использованик свойство Drop для автоматической очистки временных 
    // файлов, когда они больше не нужны:

    {
        use std::fs::File ;
        use std::path::PathBuf ;

        struct TempFile {
            file:   File,
            path:   PathBuf,
        }

        impl TempFile {
            fn new(path: PathBuf) ->std::io::Result<Self> {
                let file = File::create(&path)?;
                Ok(Self { file, path })
            }
        }

        impl Drop for TempFile {
            fn drop(&mut self) {
                //let v = std::fs::remove_file(&self.path) ;
                if let Err(e) = std::fs::remove_file(&self.path) {
                    eprintln!("Err: {}, Failed to remove temporary file: {:?}", e, self.path) ;
                }
                println!("Drop temporary file: {:?}", self.path) ;
            }
        }

        {
            let f = match TempFile::new("test.txt".into()) {
                Ok(f) => f,
                Err(err) => 
                    panic!("Cannot create temporary file.")
            } ;

            println!("Temporary file created.") ;
        }

        let f = match TempFile::new("another_test.txt".into()) {
            Ok(f) => f,
            Err(e) => panic!("Cannot create temporary file."),
        } ;

        drop(f);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait/iter.html
    // Iterators
    // Трейт Iterator используется для реализации итераторов по 
    // коллекциям, таким как массивы.
    // Для реализации Iterator нужен только метод next.
    //
    // Для удобства в распространенных ситуациях конструкция for 
    // преобразует некоторые коллекции в итераторы с помощью 
    // метода .into_iter().

    {
        struct Fibonacci {
            curr:   u32,
            next:   u32,
        }

        impl Iterator for Fibonacci {
            // Тип элементов, по которым производится итерация.
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                let curr = self.curr ;
                self.curr = self.next ;
                self.next += curr ;
                Some(curr)
            }
        }


        fn fibonacci() ->Fibonacci {
            Fibonacci { curr: 0, next: 1 }
        }

        let mut seq = 0..3 ;

        println!("> {:?}", seq.next()) ;
        println!("> {:?}", seq.next()) ;
        println!("> {:?}", seq.next()) ;
        println!("> {:?}", seq.next()) ;

        for i in 0..3 {
            println!("> {}", i) ;
        }

        println!() ;

        for i in fibonacci()
                        .take(4) // Creates an iterator that yields the first n elements, or fewer if the underlying iterator ends sooner.

        {
            println!(">> {}", i) ;
        }

        println!() ;

        for i in fibonacci()
                        .skip(4) // Creates an iterator that skips the first n elements.
                        .take(4) // Creates an iterator that yields the first n elements, or fewer if the underlying iterator ends sooner.
        {
            println!(">>> {}", i) ;
        }

        println!() ;

        let array = [1u32, 2, 3, 4] ;

        for i in array.iter() {
            println!(">>>> {}", i) ;
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait/impl_trait.html
    // impl Trait

    {
        // как тип аргумента:
        fn parse_csv_document<R: std::io::BufRead>(src: R) ->std::io::Result<Vec<Vec<String>>> {
            src
                .lines() // Returns an iterator over the lines of this reader.
                .map(|line| {
                    line
                        .map(|line| {
                            line
                                .split(',')
                                .map(|entry| {
                                    String::from(entry.trim())
                                })
                                .collect()
                        })
                })
                .collect()
        }

        // как аргумент реализующий impl std::io::BufRead
        fn parse_cvs_documemt2(src: impl std::io::BufRead) ->std::io::Result<Vec<Vec<String>>> {
            src
                .lines()
                .map(|line| {
                    line
                        .map(|line| {
                            line
                                .split(',')
                                .map(|entry| {
                                    String::from(entry.trim())
                                })
                                .collect()
                        })
                })
                .collect()
        }

        // как аргумент реализующий impl std::io::BufRead во фразе where
        fn parse_cvs_documemt3<R>(src: R) ->std::io::Result<Vec<Vec<String>>> 
            where R: std::io::BufRead
        {
            src
                .lines()
                .map(|line| {
                    line
                        .map(|line| {
                            line
                                .split(',')
                                .map(|entry|{
                                    String::from(entry.trim())
                                })
                                .collect()
                        }) 
                })
                .collect()
        }

        // Явное пояснене преобразования коллекции результатов в результат
        // коллекции !!!!!!!!!!!!!!!!!!
        fn parse_cvs_documemt4<R>(src: R) ->std::io::Result<Vec<Vec<String>>>
            where R: std::io::BufRead
        {
            let v  = src
                    .lines()
                    .map(|line| {
                        line
                            .map(|line| {
                                line
                                    .split(',')
                                    .map(|s| {
                                        String::from(s.trim())
                                    })
                                    .collect::<Vec<_>>() // // Collect all strings in a row into a Vec<String>
                            })
                    })
                    /*
                    Если у вас есть коллекция Result<Vec<String>> и вы 
                    хотите объединить их в один Result<Vec<Vec<String>>>,
                    вы можете использовать возможности метода 
                    .collect::<Result<Vec<_>, _>>(). 
                    В Rust метод collect() может «перевернуть» коллекцию 
                    результатов в результат, содержащий коллекцию.                    
                    */
                    .collect::<Result<Vec<_>, _>>()
                    ;
                v
        }
    }
    
    // https://doc.rust-lang.org/stable/rust-by-example/trait/impl_trait.html
    // As a return type

    {
        use std::iter ;
        use std::vec::IntoIter ; // An iterator that moves out of a vector.

        // так:
        // Эта функция объединяет два `Vec<i32>` и возвращает итератор по ним.
        // Посмотрите, какой сложный у неё тип возвращаемого значения!        
        fn comb_vecs_explicit_return_type(
                v: Vec<i32>,
                u: Vec<i32>
            ) ->iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>>
        {
            v
                .into_iter()
                .chain( // Takes two iterators and creates a new iterator over both in sequence.
                    u
                        .into_iter()
                )
                .cycle()    // Repeats an iterator endlessly.
        }

        let v1 = vec![1,2,3] ;
        let v2 = vec![4, 5] ;

        /*
        let mut v3 = comb_vecs_explicit_return_type(v1, v2) ;

        for v in v3.take(10) {
            println!("{}",v) ;
        }
         */

        for v in comb_vecs_explicit_return_type(v1, v2)
                        .take(10) {
            println!("{}",v) ;
        }

        // или так:
        // Это та же самая функция, но в качестве возвращаемого типа 
        // используется `impl Trait`.
        // Посмотрите, насколько она проще!        
        fn comb_vecs_explicit_return_type2(
            v: Vec<i32>,
            u: Vec<i32>,
        ) ->impl Iterator<Item = i32>
        {
            v
                .into_iter()
                .chain(
                    u.into_iter()
                )
                .cycle()
        }

        println!() ;

        let v1 = vec![1,2,3] ;
        let v2 = vec![4,5] ;

        for v in comb_vecs_explicit_return_type2(v1, v2)
                        .take(10) {
            println!("{}", v) ;
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait/impl_trait.html
    // Например, у каждого замыкания есть свой собственный безымянный 
    // конкретный тип.

    {
        fn make_add_function(y: i32) ->impl Fn(i32) -> i32
        {
            /* для определения типа возвращаемого значения
            let v = move |x: i32| x + y ;
            v
             */
            move |x: i32| x + y
        }

        let plus_one = make_add_function(1) ;

        println!("result: {}", plus_one(10)) ;

    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait/impl_trait.html
    // Вы также можете использовать трейт `impl` для возврата итератора, 
    // использующего замыкания `map` или `filter`! Это упрощает 
    // использование `map` и `filter`.

    {
        fn double_positive(number: &Vec<i32>) ->impl Iterator<Item = i32>
        {
            /* определение типа возвращаемого значения
            let v = number
                .iter()
                .filter(|x| {
                    x > &&0
                })
                .map(|x| {
                    x * 2
                }) ;
             */
            number
                .iter()
                .filter(|x| {
                    x > &&0
                })
                .map(|x| {
                    x * 2
                })
        }

        let v = vec![-3,-2,2,3] ;
        let doubles = double_positive(&v) ;

        println!("{:?}", doubles.collect::<Vec<i32>>()) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait/clone.html
    // Clone
    // Иногда нам также необходимо создать копию ресурса.
    // Трейт Clone помогает нам сделать именно это. Чаще всего мы можем 
    // использовать метод .clone(), определенный трейтом Clone.
    {
        // A unit struct without resources
        #[derive(Debug, Clone, Copy)]
        struct Unit ;

        // A tuple struct with resources that implements the `Clone` trait
        #[derive(Debug, Clone)]
        struct Pair(Box<i32>, Box<i32>) ;

        let unit = Unit ;
        // Copy `Unit`, there are no resources to move
        let copy_unit = unit ;

        println!("unit: {:?}, copy_unit: {:?}", unit, copy_unit) ;

        let pair = Pair(Box::new(1),Box::new(2)) ;
        println!("pait: {:?}", pair) ;

        // Move `pair` into `moved_pair`, moves resources
        let moved_pair = pair ;
        println!("moved_pair: {:?}", moved_pair) ;
        //println!("pait: {:?}", pair) ; // borrow of moved value: `pair`

        // Clone `moved_pair` into `cloned_pair` (resources are included)
        let cloned_pair = moved_pair.clone() ;

        // Drop the moved original pair using std::mem::drop
        drop(moved_pair);

        println!("cloned_pait: {:?}", cloned_pair) ;

    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait/supertraits.html
    // Supertraits
    // В Rust нет «наследования», но вы можете определить трейт как 
    // надмножество другого трейта. Например:

    {
        use std::fmt::Display ;

        trait Person {
            fn name(&self) ->String ;
        }

        // Person является supertrait  Student.
        // Для реализации Student необходимо также реализовать Person.
        trait Student: Person {
            fn university(&self) ->String ;
        }

        trait Programmer {
            fn fav_language(&self) ->String ;
        }

        // CompSciStudent это subtrait как Programmer, так и Student. 
        // Для реализации CompSciStudent необходимо реализовать оба supertrate.
        trait CompSciStudent: Programmer + Student {
            fn git_username(&self) ->String ;
        }


        struct Stud {
            name:           String,
            univercity:     String,
            fav_language:   String,
            git_username:   String,
        }

        impl Person for Stud {
            fn name(&self) ->String {
                self.name.clone()
            }
        }

        impl Student for Stud {
            fn university(&self) ->String {
                self.univercity.clone()
            }
        }

        impl Programmer for Stud {
            fn fav_language(&self) ->String {
                self.fav_language.clone()
            }
        }

        impl CompSciStudent for Stud {
            fn git_username(&self) ->String {
                self.git_username.clone()
            }
        }

        impl Display for Stud {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "name: {}, univercity: {}, fav_language: {}, git_username: {}", 
                    self.name,
                    self.univercity,
                    self.fav_language,
                    self.git_username,
                )
            }
        }

        let stud = Stud {
            name:   String::from("name 1"),
            univercity: String::from("univercity 1"),
            fav_language:   String::from("lang 1"),
            git_username:   String::from("username 1"),
        } ;

        println!("Stud: {}", stud) ;

        // так:
        fn comp_sci_stud_greeting(s: &dyn CompSciStudent) {
            println!("1) stud: {}", s.git_username()) ;
        }
        // или так:
        fn comp_sci_stud_greeting2<T: CompSciStudent>(s: &T) {
            println!("2) stud: {}", s.git_username()) ;
        }
        // или так:
        fn comp_sci_stud_greeting3<T>(s: &T) 
            where T: CompSciStudent
        {
            println!("3) stud: {}", s.git_username()) ;
        }
        // или так:
        fn comp_sci_stud_greeting4(s: &impl CompSciStudent) {
            println!("4) stud: {}", s.git_username()) ;
        }

        comp_sci_stud_greeting(&stud);
        comp_sci_stud_greeting2(&stud) ;
        comp_sci_stud_greeting3(&stud) ;
        comp_sci_stud_greeting4(&stud) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/trait/disambiguating.html
    // Disambiguating overlapping traits
    // Тип может реализовывать множество различных трейтов. 
    // Что если два трейта требуют одинакового имени для функции?

    {
        trait UsernameWiget {
            fn get(&self) ->String ;
        }

        trait AgeWiget {
            fn get(&self) ->u8 ;
        }

        struct Form {
            username:   String,
            age:        u8,
        }

        impl UsernameWiget for Form {
            fn get(&self) ->String {
                self.username.clone()
            }
        }

        impl AgeWiget for Form {
            fn get(&self) ->u8 {
                self.age
            }
        }

        let form = Form{
            username:   "name".to_owned(),
            age:    10,
        } ;

        //form.get() ; // multiple applicable items in scope

        // так:
        let mut username = <Form as UsernameWiget>::get(&form) ;
        let mut age = <Form as AgeWiget>::get(&form) ;
        println!("1) username: {}, age: {}", username, age) ;
        // или так:
        username = UsernameWiget::get(&form) ;
        age = AgeWiget::get(&form) ;
        println!("2) username: {}, age: {}", username, age) ;

    }

    // https://doc.rust-lang.org/stable/rust-by-example/macros/designators.html
    // macro_rules!
    // Вместо генерации вызова функции макросы разворачиваются в исходный
    // код, который компилируется вместе с остальной частью программы.
    // Макросы создаются с помощью макроса macro_rules!.
    //
    // Почему макросы полезны?
    //      Избегает повторения кода
    //      Создание предметно ориентированных языков
    //      Переменное число аргументов
    //
    {
        macro_rules! say_hello {
            () => {
                println!("Hello!") ;
            };
        }

        say_hello!() ;

    }

    // https://doc.rust-lang.org/stable/rust-by-example/macros/designators.html
    // Designators - Обозначения
    // Аргументы макроса предваряются знаком доллара ($) и обозначаются 
    // идентификатором:
    // Вот некоторые из доступных обозначений:
    //  block
    //  expr is used for expressions
    //  ident is used for variable/function names
    //  item
    //  literal is used for literal constants
    //  pat (pattern)
    //  path
    //  stmt (statement)
    //  tt (token tree)
    //  ty (type)
    //  vis (visibility qualifier)    

    {
        macro_rules! create_function {
            ($func_name:ident) => {
                fn $func_name() {
                    println!("You called: {}()", 
                        stringify!($func_name)  // The `stringify!` macro converts an `ident` into a string.
                    ) ;
                }
            };
        }

        create_function!(foo) ;
        create_function!(bar) ;

        macro_rules! print_result {
            ($my_exp:expr) => {
                println!("{:?} = {:?}", 
                    stringify!($my_exp),
                    $my_exp,
                ) ;
            };
        }

        foo();
        bar();

        print_result!({
            let x = 10 ;
            x * x + 5
        }) ;

    }

    // https://doc.rust-lang.org/stable/rust-by-example/macros/overload.html
    // Overload
    // Макросы можно перегружать для приема различных комбинаций аргументов.
    {
        macro_rules! test {
            ($left:expr; and $right:expr) => {
                println!(
                    "{:?} and {:?} is {:?}",
                    stringify!($left),  // строковое представление всех токенов, переданных макросу
                    stringify!($right),
                    $left && $right
                    ) ;
            };
            ($left:expr; or $right:expr) => {
                println!(
                    "{:?} or {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left || $right
                ) ;
            };
        }

        test!(1_i32 + 1 == 2i32; and 2_i32 + 2 == 4_i32) ;
        test!(true; or false) ;

    }

    // https://doc.rust-lang.org/stable/rust-by-example/macros/repeat.html
    // Repeat - повторы
    // Макросы могут использовать символ "+" в списке аргументов, чтобы 
    // указать, что аргумент может повторяться хотя бы один раз,
    //  или символ "*", чтобы указать, что аргумент может повторяться 
    // ноль или более раз.
    //
    // $(...),+ позволит найти одно или несколько выражений, разделенных
    // запятыми

    {
        macro_rules! find_min {
            ($x:expr) => {$x};
            ($x:expr,$($y:expr),+) => {
                std::cmp::min($x, find_min!($($y),+))
            };
        }

        println!("1) min: {}", find_min!(1)) ;
        println!("2) min: {}", find_min!(1, 1-2, -2+4)) ;

    }

    // https://doc.rust-lang.org/stable/rust-by-example/macros/dry.html
    // DRY (Don't Repeat Yourself) - не повторяйте себя

    {
        use std::ops::{Add, Mul, Sub} ;

        macro_rules! assert_eq_len {
            ($a:expr, $b:expr, $func:ident, $op:tt) => {
                assert!(
                    $a.len() == $b.len(),
                    "{:?} dimenstion mismatch {:?} {:?} {:?}",
                    stringify!($func),
                    $a.len(),
                    stringify!($op),
                    $b.len(),
                );
            };
        }

        macro_rules! op {
            // `tt` (token tree)
            ($func:ident, $bound:ident, $op:tt, $method:ident) => {
                // The 'Output' associated type defines the result of Point + Point for Add trait
                fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
                    assert_eq_len!(xs, ys, $func, $op) ;

                    for (x, y) in xs.iter_mut().zip(ys.iter()) {
                        *x = $bound::$method(*x, *y) ;
                    }
                }
            };
        }

        op!(add_asing, Add, +=, add) ;
        op!(mul_asing, Mul, *=, mul) ;
        op!(sub_asing, Sub, -=, sub) ;

        let mut v1 = vec![1,2,3] ;
        let v2 = vec![4, 5, 6] ;

        add_asing(&mut v1, &v2);
        println!("add_asing v1: {:?}", v1) ;
        mul_asing(&mut v1, &v2);
        println!("mul_asing v1: {:?}", v1) ;
        sub_asing(&mut v1, &v2);
        println!("sub_asing v1: {:?}", v1) ;

        mod test {
            //use std::iter::repeat ; // Creates a new iterator that endlessly repeats a single element.
            use std::iter ; // итератор

            macro_rules! test {
                ($func:ident, $x:expr, $y:expr, $z:expr) => {
                    #[test]
                    fn $func() {
                        for size in 0_usize..10 {
                            let mut x: Vec<_> = iter::repeat($x) // Создает новый итератор, который бесконечно повторяет один и тот же элемент.
                                                    .take(size)  // Создает итератор, который возвращает первые n элементов, или меньше, если базовый итератор завершается раньше.
                                                    .collect();
                            let y: Vec<_> = iter::repeat($y).take(size).collect() ;
                            // z - не нужна !!!!!!!!!!!!!!!!
                            //let z: Vec<_> = iter::repeat($z).take(size).collect() ;

                            super::$func(&mut x, &y) ;
                            assert_eq!(x, y) ;
                        }
                    }
                };
            }
            
            /*
            test!(add_asing, 1u32, 2u32, 3u32) ;
            test!(mul_asing, 2u32, 3u32, 6u32) ;
            test!(sub_asing, 3u32, 2u32, 1u32) ;
             */
        }

    }
    
    // https://doc.rust-lang.org/stable/rust-by-example/macros/dsl.html
    // Domain Specific Languages (DSLs) - Предметно-ориентированные языки (DSL)
    // DSL — это мини-«язык», встроенный в макрос Rust. Он полностью 
    // соответствует стандарту Rust

    {
        // пример с двойными фигурными скобками с ';'
        macro_rules! calculate2 {
            (eval $e:expr) => {{
                let val: usize = $e ;
                println!("calculate2:   {:?} = {:?}", stringify!($e), val) ;
            }} ;
        }

        // пример без двойных фигурных скобок с ';'
        macro_rules! calculate {
            (eval $e:expr) => {
                let val: usize = $e ;
                println!("{} = {}", stringify!($e), val) ;
            };
        }

        calculate2!(eval 10 + 2) ;
        calculate2!(eval 10 * 2) ;

        calculate!(eval 10 + 2) ;
        calculate!(eval 20 * 2) ;
    }

    // --------------------------
    
    // https://doc.rust-lang.org/stable/rust-by-example/macros/variadics.html
    // Variadic Interfaces - Интерфейсы с переменным числом аргументов
    // можно так:
    {
        macro_rules! calculate {
            (eval $e:expr) => {{
                let v: usize = $e ;
                println!("eval {} = {}", stringify!($e), v) ;
            }};

            (eval $e:expr, $(eval $es:expr),+) => {{
                calculate!{eval $e}
                calculate!{$(eval $es),+}
            }};
        }

        calculate! {
            eval 1 + 1,
            eval 10 - 2,
            eval 10 * 2
        } ;
    }

    // или так:
    {
        macro_rules! calculate2 {
            (eval $e:expr) => {
                let v: usize = $e ;
                println!("eval2: {} = {}", stringify!($e), v) ;
            };

            (eval $e:expr, $(eval $es:expr),+) => {
                calculate2!(eval $e) ;
                calculate2!($(eval $es),+) ;
            } ;
        }

        calculate2! {
           eval 10 + 1,
           eval 10 * 2,
           eval 10 - 5
        } ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/panic.html
    // panic
    {
        fn drink(bevergase: &str) {
            if bevergase == "lemonade" {
                panic!("AAAAAAA it's: {}", bevergase) ;
            }

            println!("It's {}", bevergase) ;
        }

        drink("water");
        //drink("lemonade");    // panic
        drink("water");

    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/abort_unwind.html
    // abort and unwind - компиляционные флаги
    // использование  cfg!(...)
    {
        fn drink(beverage: &str) {
            // You shouldn't drink too much sugary beverages.
            if beverage == "lemonade" {
                if cfg!(panic = "abort") {
                    println!("This is not your party. Run!!!!");
                } else {
                    println!("Spit it out!!!!");
                }
            } else {
                println!("Some refreshing {} is all I need.", beverage);
            }
        }

        drink("water");
        drink("lemonade");
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/abort_unwind.html
    // abort and unwind - компиляционные флаги
    // использование  #[cfg(...)]
    {
        #[cfg(panic = "unwind")]
        fn ah() {
            println!("Spit it out!!!!");
        }

        #[cfg(not(panic = "unwind"))]
        fn ah() {
            println!("This is not your party. Run!!!!");
        }

        fn drink(beverage: &str) {
            if beverage == "lemonade" {
                ah();
            } else {
                println!("Some refreshing {} is all I need.", beverage);
            }
        }        

        drink("water");
        drink("lemonade");        
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap.html
    // Option & unwrap

    {
        fn give_adult(drink: Option<&str>) {
            match drink {
                Some("lemonade") => println!("It's lemonade."),
                Some(ins) => println!("It's: {}", ins),
                None => println!("No drink."),
            }
        }

        fn drink(drink: Option<&str>) {
            let ins = drink
                                .unwrap() ;
            if ins == "lemonade" {
                panic!("It is {} !!!!", ins) ;
            }

            println!("I like {}.", ins) ;
        }

        let water = Some("water") ;
        let lemonade = Some("lemonade") ;
        let void = None ;

        give_adult(water);
        give_adult(lemonade);
        give_adult(void);

        drink(water);
        //drink(void);    // panic
        //drink(lemonade); // panic
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/question_mark.html
    // Unpacking options with ?

    {
        fn next_birthday(curet_age: Option<u8>) ->Option<String> {
            let next_age = curet_age? + 1 ;

            Some(format!("Next year will be {}", next_age))
        }

        let mut curet_age = Some(10) ;

        println!("{:?}", next_birthday(curet_age)) ;

        curet_age = None ;

        println!("{:?}", next_birthday(curet_age)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/question_mark.html
    // Вы можете объединять множество знаков вопроса (?) в цепочку, чтобы
    // сделать ваш код гораздо более читаемым.

    {
        #[derive(Clone, Copy)]
        struct PhoneNumber {
            area_code:  Option<u8>,
            number:     u32,
        }

        #[derive(Clone, Copy)]
        struct Job {
            phone_number:   Option<PhoneNumber>
        }

        struct Person {
            job:    Option<Job>
        }

        impl Person {
            fn work_phone_area_code(&self) ->Option<u8> {
                self
                    .job?
                    .phone_number?
                    .area_code
            }
        }

        let mut p = 
        Person {
            job:    Some(
                        Job {
                            phone_number:   Some(
                                                PhoneNumber {
                                                    area_code:  Some(10_u8),
                                                    number: 124,
                                                }
                                            ) 
                            }
                    )
        } ;

        println!("1) area_code: {:?}", p.work_phone_area_code()) ;

        p = Person { job: None } ;

        println!("2) area_code: {:?}", p.work_phone_area_code()) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/map.html
    // Combinators: map
    // У Option<T> есть встроенный метод map(), комбинатор для простого
    // сопоставления Some -> Some и None -> None. Несколько вызовов map()
    // можно объединить в цепочку для еще большей гибкости.
    {
        #[derive(Debug)]
        enum Food {
            Apple,
            Carrot,
            Potato,
        }

        #[derive(Debug)]
        struct Peeled (Food) ;

        struct Chopped (Food) ;

        #[derive(Debug)]
        struct Cooked (Food) ;

        // так:
        fn peel(food: Option<Food>) ->Option<Peeled> {
            match food {
                Some(f) => Some(Peeled(f)),
                None => None,
            }
        }

        // или так:
        fn peel2(food: Option<Food>) ->Option<Peeled> {
            food
                .map(|f| Peeled(f))
        }

        // так:
        fn coop(peel: Option<Peeled>) ->Option<Chopped> {
            match peel {
                // под Some структура Peeled(а)
                Some(Peeled(f)) => Some(Chopped(f)),    // деструктуризация
                None => None,
            }
        }
        // или так:
        fn chop2(peel: Option<Peeled>) ->Option<Chopped>{
            peel
                .map(|Peeled(f)| Chopped(f))    // деструктуризация
        }

        // так:
        fn cook(chooped: Option<Chopped>) ->Option<Cooked> {
            match chooped {
                Some(Chopped(f)) => Some(Cooked(f)), // деструктуризация
                None => None,
            }
        }
        // или так:
        fn cook2(chopped: Option<Chopped>) -> Option<Cooked> {
            chopped
                .map(|Chopped(food)| Cooked(food))  // деструктуризация
        }

        fn preocess(food: Option<Food>) ->Option<Cooked> {
            food
                .map(|f| Peeled(f))
                .map(|Peeled(f)| Chopped(f)) // деструктуризация
                .map(|Chopped(f)| Cooked(f))
        }

        let apple = Some(Food::Apple) ;
        let carrot = Some(Food::Carrot) ;
        let potato = Some(Food::Potato) ;

        println!("{:?}", peel(apple)) ;
        println!("{:?}", peel2(carrot)) ;
        println!("{:?}", preocess(potato)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/and_then.html
    // .and_then() <- Some languages call this operation flatmap
    // Использование map(...) для функции, возвращающей Option<T>, приводит 
    // к вложенному Option<Option<T>>
    // Использование .and_then(...) предотврашает вложение Option<Option<T>>
    // или
    // Использование .and_then(...).flatten() предотврашает вложение Option<Option<T>>
    {
        enum Food {
            One,
            Two,
            Three,
        }

        fn have_1(food: Food) ->Option<Food> {
            match food {
                Food::One => None,
                _ => Some(food),
            }
        }

        fn have_2(food: Food) ->Option<Food> {
            match have_1(food) {
                Some(f) => Some(f),
                None => None,
            }
        }

        let my_food = Food::One ;
        let my_food_2 = Food::Two ;
        let my_foot_3 = Food::Three ;

        // Вложенный Option<Option<T>>
        let v = have_1(my_food_2)
                                .map(have_2)
                                ;

        // Option.map(Option<T>) -> Option<Option<T>>
        let v1 = have_1(my_food)
                                .map(have_2)
                                .flatten()  // Converts from Option<Option<T>> to Option<T>
                                ;

        let v3 = have_1(my_foot_3)
                                .and_then(have_2)
                                ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/defaults.html
    // Unpacking options and defaults
    // or() is chainable, evaluates eagerly, keeps empty value intact
    // v.or(v1) - возвращает v если v is Some(...) иначе v1
    {
        #[derive(Debug)]
        enum Fruit {
            Apple,
            Orange,
            Banana,
            Kivi,
            Lemon,
        }

        let mut apple = Some(Fruit::Apple) ;
        let mut orange = Some(Fruit::Orange) ;
        let mut no_fruit: Option<Fruit> = None ;

        // or() is chainable, evaluates eagerly, keeps empty value intact
        let fruit_available_fruit 
                = no_fruit // no_fruit moved in .or(orange)
                    .or(orange) // orange is moved
                    .or(apple)  // apple is moved
                    ;
        //println!("no_fruit: {:?}", no_fruit) ; // `no_fruit` moved due to this method call
        println!("fruit_available_fruit: {:?}", fruit_available_fruit) ;

        //println!("orange: {:?}", orange) ;  // borrow of moved value: `orange`
        //println!("apple: {:?}", apple) ;    // borrow of moved value: `apple`
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/defaults.html
    // or_else() is chainable, evaluates lazily, keeps empty value intact

    {
        #[derive(Debug)]
        enum Fruit {
            Apple,
            Orange,
            Banana,
            Kivi,
            Lemon,
        }

        let no_fruit: Option<Fruit> = None ;

        let get_kiwi = || {
            println!("get_kiwi()") ;
            Some(Fruit::Kivi)
        } ;

        let get_lemon = || {
            println!("get_lemon()") ;
            Some(Fruit::Lemon)
        } ;

        // or_else() is chainable, evaluates lazily, keeps empty value intact
        let first_awailable_fruit = no_fruit
            .or_else(get_kiwi)  // Returns the option if it contains a value, otherwise calls f and returns the result.
            .or_else(get_lemon) // Returns the option if it contains a value, otherwise calls f and returns the result.
            ;
        println!("first_awailable_fruit: {:?}", first_awailable_fruit) ;
        //println!("no_fruit: {:?}", no_fruit) ;  // borrow of moved value: `no_fruit`
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/defaults.html
    // get_or_insert() evaluates eagerly, modifies empty value in place

    {
        #[derive(Debug)]
        enum Fruit {
            Apple,
            Orange,
            Banana,
            Kivi,
            Lemon,
        }

        let mut my_fruit: Option<Fruit> = None ;
        let apple = Fruit::Apple ;

        // get_or_insert() evaluates eagerly, modifies empty value in place
        let fruit_available = my_fruit
                    .get_or_insert(apple)   // Inserts value into the option if it is None, then returns a mutable reference to the contained value.
                    ;
        println!("fruit_available: {:?}", fruit_available) ;
        println!("my_fruit: {:?}", my_fruit) ;
        //println!("apple: {:?}", apple) ;    // borrow of moved value: `apple`
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/defaults.html
    // get_or_insert_with() evaluates lazily, modifies empty value in place

    {
        #[derive(Debug)]
        enum Fruit {
            Apple,
            Orange,
            Banana,
            Kivi,
            Lemon,
        }

        let mut my_fruit: Option<Fruit> = None ;

        let get_lemon_as_fallback = || {
                println!("get_lemon_as_fallback()") ;
                Fruit::Lemon
        } ;

        let first_available_fruit = my_fruit
                .get_or_insert_with(get_lemon_as_fallback)
                ;
        println!("first_available_fruit: {:?}", first_available_fruit) ;
        println!("my_fruit: {:?}", my_fruit) ;

        my_fruit = Some(Fruit::Banana) ;

        // get_or_insert_with() evaluates lazily, modifies empty value in place
        let first_available_fruit = my_fruit
                .get_or_insert_with(get_lemon_as_fallback)
                ;
        println!("first_available_fruit: {:?}", first_available_fruit) ;
        println!("my_fruit: {:?}", my_fruit) ;
    }


    // https://doc.rust-lang.org/stable/rust-by-example/error/result.html
    // Result

    {
        fn multiply(f: &str, s: &str) ->i32 {
            let f_n = f
                            .trim()
                            .parse::<i32>()
                            .unwrap() ;
            let s_n = s
                            .trim()
                            .parse::<i32>()
                            .unwrap() ;
            f_n * s_n                        
        }

        println!("1) mul: {}", multiply("  25", "2")) ;
        //println!("2) mul: {}", multiply("t", "10")) ;   // panic: an `Err` value: ParseIntError { kind: InvalidDigit } 
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/result.html
    // Using Result in main
    // Однако функция main также может возвращать тип Result. Если в 
    // функции main возникает ошибка, она вернет код ошибки и выведет 
    // отладочное представление ошибки (используя трейт Debug). 
    /*
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}    
     */

    {
        use std::num::ParseIntError ;

        fn main_tmp(number_str: &str) ->Result<(), ParseIntError> {
            let num = match number_str
                                    .trim()
                                    .parse::<i32>() {
                Ok(v) => v,
                Err(e) => return Err(e),
            } ;
            println!("num: {}", num) ;
            Ok(())
        }

        let v_out = main_tmp(" 10 ") ;
        println!("v_out: {:?}", v_out) ;
    }


    // https://doc.rust-lang.org/stable/rust-by-example/error/result/result_map.html
    // map for Result

    {
        use std::num::ParseIntError ;

        fn multiply(f: &str, s: &str) ->Result<i32, ParseIntError> {
            match f
                    .trim()
                    .parse::<i32>() {
                Ok(f_n) => {
                    match s
                            .trim()
                            .parse::<i32>() {
                        Ok(s_n) => Ok(f_n * s_n),
                        Err(e) => Err(e),
                    }
                },
                Err(err) => Err(err),
            }
        }

        fn print(res: Result<i32, ParseIntError>) {
            match res {
                Ok(v) => println!("n: {}", v),
                Err(err) => println!("Error: {}", err),
            }
        }

        print(multiply("1", "2"));
        print(multiply("tt", "5"));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/result_map.html
    // Использование .and_then() для Result<...> для предотвращения Result<Result<>>
    {
        use std::num::ParseIntError ;
        // Result.map(Result<T, E>) ->Result<Result<T, E>>
        fn multiply(f: &str, s: &str) ->Result<i32, ParseIntError> {
            f
                .trim()
                .parse::<i32>()
                //.map(|f_n|      // Возвращает Result<Result<i32, ParseIntError>, ParseIntError>
                .and_then(|f_n| // Calls op if the result is Ok, otherwise returns the Err value of self.
                    s
                        .trim()
                        .parse::<i32>()
                        .map(|s_n| {
                            f_n * s_n
                        })
                )
        }

        fn print(res: Result<i32, ParseIntError>) {
            match res {
                Ok(n) => println!("2) n: {}", n),
                Err(err) => println!("2) Error: {}", err),
            }
        }

        print(multiply("1", "2"));
        print(multiply("tt", "5"));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/result_alias.html
    // aliases for Result
    {
        use std::num::ParseIntError ;

        type AliasedResult<T> = Result<T, ParseIntError> ;

        fn multiply(f: &str, s: &str) ->AliasedResult<i32> {
            f
                .trim()
                .parse::<i32>()
                .and_then(|f_n| {
                    s
                        .trim()
                        .parse::<i32>()
                        .map(|s_n|{
                            f_n * s_n
                        })
                })
        }

        fn print(res: AliasedResult<i32>) {
            match res {
                Ok(n) => println!("3) n: {}", n),
                Err(err) => println!("3) Error: {}", err),
            }
        }

        print(multiply(" 1 ", " 3"));
        print(multiply(" 3.2 ", "10.2 ")) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/early_returns.html
    // Early returns
    {
        use std::num::ParseIntError ;

        fn multiply(f: &str, s: &str) ->Result<i32, ParseIntError> {
            let f_n = match f
                                .trim()
                                .parse::<i32>() {
                Ok(f) => f,
                Err(err) => return Err(err),
            };

            let s_n = match s
                                .trim()
                                .parse::<i32>() {
                Ok(s) => s,
                Err(err) => return Err(err),
            };

            Ok(f_n * s_n)
        }

        fn print(res: Result<i32, ParseIntError>) {
            match res {
                Ok(n) => println!("4) n: {}", n),
                Err(err) => println!("4) Error: {}", err),
            }
        }

        print(multiply(" 1 ", " 2"));
        print(multiply(" tt "  , "10 "));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/enter_question_mark.html
    // Introducing ? для Result<...>
    // ? - возвращает ошибку Err а не вызывает панику

    {
        use std::num::ParseIntError ;

        fn multiply(f: &str, s: &str) ->Result<i32, ParseIntError> {
            let f_n = f
                            .trim()
                            .parse::<i32>()
                            ?
                            ;

            let s_n = s
                            .trim()
                            .parse::<i32>()
                            ?
                            ;

            Ok(f_n * s_n)
        }

        fn print(res: Result<i32, ParseIntError>) {
            match res {
                Ok(n) => println!("5) n: {}", n),
                Err(err) => println!("5) Error: {}", err),
            }
        }

        print(multiply(" 1 ", " 2 "));
        print(multiply(" tt ", " 10 " ));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/result/enter_question_mark.html
    // The try! macro в настоящее время используется ?
    // use of deprecated macro `r#try`: use the `?` operator instead
    {
        use std::num::ParseIntError ;

        fn multiply(f: &str, s: &str) ->Result<i32, ParseIntError> {
            let f_n = r#try!(f.trim().parse::<i32>()) ; // r#try!(...) Распаковывает результат или распространяет его ошибку.
            let s_n = r#try!(s.trim().parse::<i32>()) ;

            Ok(f_n * s_n)
        }

        fn print(res: Result<i32, ParseIntError>) {
            match res {
                Ok(n) => println!("6) n: {}", n),
                Err(err) => println!("6) Error: {}", err),
            }
        }

        print(multiply(" 1 ", "2")) ;
        print(multiply("t", "10"));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types.html
    // Multiple error types

    {
        fn duble_first(vec: Vec<&str>) ->i32 {
            let first = vec
                                .first()
                                .unwrap()
                                ;
            2 * first
                    .trim()
                    .parse::<i32>()
                    .unwrap() 
        }

        let numbers = vec!["23", "2", "3"] ;
        println!("1) first val * 2: {}", duble_first(numbers)) ;

        let empty: Vec<&str> = vec![] ;
        //println!("2) first val * 2: {}", duble_first(empty)) ;  // called `Option::unwrap()` on a `None` value

        let strings = vec!["t", "1"] ;
        //println!("3) first val * 2: {}", duble_first(strings)) ;    // called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/option_result.html
    // Pulling Results out of Options

    {
        use std::num::ParseIntError ;

        fn double_first(vec: Vec<&str>) ->Option<Result<i32, ParseIntError>> {
            vec
                .first()
                .map(|first|{
                    first
                        .trim()
                        .parse::<i32>()
                        .map(|n | {
                            n * 2
                        })
                })
        }

        let numbers = vec!["1", "2", "3"] ;
        println!("for numbers: {:?}", double_first(numbers)) ;

        let empty = vec![] ;
        println!("for empty: {:?}", double_first(empty)) ;

        let strings = vec!["t", "1", "2"] ;
        println!("for strings: {:?}", double_first(strings)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html
    // Defining an error type
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // Функция transpose() пригодится для замены местами Result и Option.
    {
        use std::num::ParseIntError ;

        fn double_first(vec: Vec<&str>) ->Result<Option<i32>, ParseIntError> {
            let opt = vec
                                .first()
                                .map(|v|{
                                    v
                                    .trim()
                                    .parse::<i32>()
                                    .map(|v| {
                                        v * 2
                                    })
                                })
                                ;
            println!("Before transpose opt: {:?}", opt) ;
            // транспонирование !!!!!!
            // Option<Result<i32, ParseIntError>> в Result<Option<i32>, ParseIntError>
            opt.transpose() // Transposes an Option of a Result into a Result of an Option.
        }

        let numbers = vec!["1", "2", "3"] ;
        println!("2) for numbers: {:?}", double_first(numbers)) ;

        let empty =vec![] ;
        println!("2) for empty: {:?}", double_first(empty)) ;

        let strings = vec!["t", "1", "2"] ;
        println!("2) strings: {:?}", double_first(strings)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html
    // Defining an error type

    {
        use std::fmt ;

        // Определяем типы ошибок. Их можно настроить для обработки ошибок.
        // Теперь мы сможем писать собственные ошибки, использовать базовую 
        // реализацию обработки ошибок или делать что-то среднее.
        struct DubleError ;

        impl fmt::Display for DubleError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "invalid first item to double")
            }
        }

        type Result<T> = std::result::Result<T, DubleError> ;

        fn double_first(vec: Vec<&str>) ->Result<i32> {
            vec
                .first()
                // .ok_or Transforms the Option<T> into a Result<T, E>,
                // mapping [Some(v)] to [Ok(v)] and None to [Err(err)].
                .ok_or(DubleError) 
                .and_then(|v|{
                    v
                        .trim()
                        .parse::<i32>()
                        // .map_err Maps a Result<T, E> to Result<T, F>
                        // by applying a function to a contained Err value, 
                        // leaving an Ok value untouched.
                        .map_err(|e| DubleError)
                        // Maps a Result<T, E> to Result<U, E> 
                        // by applying a function to a contained Ok value, 
                        // leaving an Err value untouched.
                        .map(|x| {
                            x * 2
                        })
                })
        }
        
        fn print(res: Result<i32>) {
            match res {
                Ok(v) => println!("double first number: {}", v),
                Err(err) => println!("Error: {}", err),
            }
        }
        
        let numbers = vec!["1", "2", "3"] ;
        print(double_first(numbers));

        let empty = vec![] ;
        print(double_first(empty));

        let strings = vec!["t","1","2"] ;
        print(double_first(strings));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html
    // Boxing errors - упаковка ошибок в Box с динамической диспетчеризацией
    // Стандартная библиотека помогает упаковывать наши ошибки, позволяя 
    // Box реализовывать преобразование из любого типа, реализующего 
    // трейт Error, в объект трейта Box<Error> с помощью From.

    {
        use std::error ;
        use std::fmt::{self, Display} ;

        type Result<T> = std::result::Result<T, Box<dyn error::Error>> ;

        #[derive(Debug)]
        struct EmptyVec ;

        impl Display for EmptyVec {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "invalid first item to double.")
            }
        }

        impl error::Error for EmptyVec {

        }

        fn double_first(vec: Vec<&str>) ->Result<i32> {
            vec
                .first()
                .ok_or_else(|| EmptyVec.into())
                .and_then(|v| {
                    v
                        .trim()
                        .parse::<i32>()
                        .map_err(|err|{
                            err.into()
                        })
                        .map(|x|{
                            x * 2
                        })
                })
        }

        fn print(res: Result<i32>) {
            match res {
                Ok(v) => println!("The first doubles is {}", v),
                Err(err) => println!("Error: {}", err),
            }
        }

        let numbers = vec!["1","2","3"] ;
        print(double_first(numbers));

        let empty = vec![] ;
        print(double_first(empty));

        let strings = vec!["t","1","2"] ;
        print(double_first(strings));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html
    // Other uses of ?

    {
        use std::error ;
        use std::fmt ;

        type Result<T> = std::result::Result<T, Box<dyn error::Error>> ;

        #[derive(Debug)]
        struct EmptyVec ;

        impl fmt::Display for EmptyVec {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "invalid first item to double.")
            }
        }

        impl error::Error for EmptyVec {
            
        }

        fn double_first(vec: Vec<&str>) ->Result<i32>{
            let val = vec
                 .first()
                 .ok_or(EmptyVec) // Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err)
                 ?  // автоматическое преобразование ошибки к нужному типу: Box<dyn error::Error>
                 ;
            let v_out = val
                    .trim()
                    .parse::<i32>()
                    ?  // автоматическое преобразование ошибки к нужному типу: Box<dyn error::Error>
                    ;

            Ok(2 * v_out)   
        }

        fn print(res: Result<i32>) {
            match res {
                Ok(n) => println!("The first doubled is {}", n),
                Err(err) => println!("Error: {}", err),
            }
        }

        let numbers = vec!["1","2","3"] ;
        print(double_first(numbers));
        
        let empty = vec![] ;
        print(double_first(empty));

        let strings = vec!["t", "1", "2"] ;
        print(double_first(strings));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html
    // Wrapping errors
    //

    {
        use std::fmt ;
        use std::error ;
        use std::error::Error ;
        use std::num::ParseIntError ;

        #[derive(Debug)]
        enum DoubleError {
            EmptyVec,
            Parse(ParseIntError),
        }

        type Result<T> = std::result::Result<T, DoubleError> ;

        impl fmt::Display for DoubleError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match *self {
                    DoubleError::EmptyVec => write!(f, "vector is empty"),
                    // The wrapped error contains additional information and is available
                    // via the source() method.                    
                    DoubleError::Parse(..) => write!(f, "parse string error"),
                }
            }
        }

        impl error::Error for DoubleError {
            // Returns the lower-level source of this error, if any.
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                match *self {
                    DoubleError::EmptyVec => None,
                    DoubleError::Parse(ref e) => Some(e),
                }                
            }
        }

        // Implement the conversion from `ParseIntError` to `DoubleError`.
        // This will be automatically called by `?` if a `ParseIntError`
        // needs to be converted into a `DoubleError`.
        impl From<ParseIntError> for DoubleError {
            fn from(value: ParseIntError) -> Self {
                DoubleError::Parse(value)
            }
        }

        fn double_first(vec: Vec<&str>) ->Result<i32>{
            let v = vec
                                .first()
                                .ok_or(DoubleError::EmptyVec)
                                ?
                                ;
            let n = v
                            .trim()
                            .parse::<i32>()
                            ?
                            ;
            Ok(2 * n)
        }

        fn print(rec: Result<i32>) {
            match rec {
                Ok(n) => println!("doubled n: {}", n),
                Err(e) => {
                    println!("Error: {}", e) ;
                    if let Some(source) = e.source() {
                        println!("   Caused: {}", source) ;
                    }
                }
            }
        }

        let numbers = vec!["1","2","3"] ;
        print(double_first(numbers));

        let empty = vec![] ;
        print(double_first(empty));

        let strings = vec!["t","1","2"] ;
        print(double_first(strings));

    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/iter_result.html
    // Iterating over Results
    {
        let strings = vec!["t", "1","2","3"] ;

        let numbers: Vec<_> = strings
                                .into_iter()
                                .map(|x| {
                                    x.parse::<i32>()
                                })
                                .collect()
                                 ;

        println!("numbers: {:?}", numbers) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/iter_result.html
    // Ignore the failed items with filter_map()
    // Creates an iterator that both filters and maps.
    // The returned iterator yields only the values for which 
    // the supplied closure returns Some(value).
    {
        let strings = vec!["t","1","2","3"] ;
        let numbers: Vec<_> = strings
                        //.into_iter()
                        .iter()
                        .filter_map(|x| {
                            x
                                .parse::<i32>()
                                .ok()   // Converts from Result<T, E> to Option<T>.
                        })
                        .collect()
                        ;
        println!("1) numbers: {:?}", numbers) ;
        // это аналог
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map
        let numbers: Vec<_> = strings
            .iter()
            .map(|x| x.parse::<i32>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect() 
            ;
        println!("2) numbers: {:?}", numbers) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/iter_result.html
    // Collect the failed items with map_err() and filter_map()
    // Функция map_err вызывает функцию с ошибкой, поэтому, добавив её к 
    // предыдущему решению с filter_map, мы можем сохранять их в стороне 
    // во время итерации.
    {
        let strings = vec!["t","1", "2", "10000"] ;
        let mut errors = vec![] ;
        let numbers: Vec<_> = strings
                                    .into_iter()
                                    .map(|x| {
                                        x.parse::<u8>()
                                    })
                                    .filter_map(|x|{
                                        x
                                            .map_err(|e| {
                                                errors
                                                    .push(e);
                                            })
                                            .ok()
                                    })
                                    .collect()
                                    ;
        println!("numbers: {:?}", numbers) ;
        println!("errors: {:?}", errors) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/iter_result.html
    // Fail the entire operation with collect()
    //
    // вектор результатов (Vec<Result<T, E>>) может быть преобразован в 
    // результат с вектором (Result<Vec<T>, E>). 
    // Как только будет обнаружена ошибка Result::Err, итерация 
    // завершится.
    {
        let strings = vec!["1","2","3","t"] ;
        // создание вектора результатов        
        let numbers: Vec<_> = strings
                            .iter()
                            .map(|x|{
                                x.parse::<i32>()
                            })
                            .collect()
                            ;

        // создание результата векторов
        // Как только будет обнаружена ошибка Result::Err, 
        // итерация завершится с ошибкой и будет возвращена ошибка.
        // преобразование коллекции результатов в результат  коллекции
        let numbers: Result<Vec<_>, _> = strings
                            //.into_iter()
                            .iter()
                            .map(|x|{
                                x.parse::<i32>()
                            })
                            .collect()
                            ;
        println!("2) numbers: {:?}", numbers) ;
        // или так:
        let numbers = strings
                            .iter()
                            .map(|x|{
                                x.parse::<i32>()
                            })
                            .collect::<Result<Vec<_>, _>>()
                            ;
        println!("3) numbers: {:?}", numbers) ;                        
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/iter_result.html
    // Collect all valid values and failures with partition()
    //
    {
        let strings = vec!["1","2","3","t"] ;

        let (numbers, errors): (Vec<_>, Vec<_>) = strings
                            .into_iter()
                            .map(|x|{
                                x.parse::<i32>()
                            })
                            .partition(Result::is_ok) // возвращает пару, состоящую из всех элементов, для которых она вернула значение true, и всех элементов, для которых она вернула значение false.
                            ;
        println!("numbers: {:?}", numbers) ;
        println!("errors: {:?}", errors) ;

        let numbers_out = numbers
            .into_iter()
            .filter_map(|x| {
                x.ok()
            })
            .map(|x|{
                x * 2
            })
            .collect::<Vec<_>>()
            ;
        println!("numbers_out: {:?}", numbers_out) ;

        let errors_out /*: Vec<_> */ = errors
                .into_iter()
                .map(|x|{
                    x.err() // Converts from Result<T, E> to Option<E>
                })
                .collect::<Vec<_>>()
                ;
        println!("error_out: {:?}", errors_out) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/error/iter_result.html
    // Collect all valid values and failures with partition()
    // ещё один вариант
    {
        let strings = vec!["1","2","3","t"];
        
        let (errors, numbers): (Vec<_>, Vec<_>) = strings
                                    .into_iter()
                                    .map(|x| {
                                        x.parse::<i32>()
                                    })
                                    .partition(Result::is_err)
                                    ;
        let numbers = numbers
                                        .into_iter()
                                        .map(Result::unwrap)
                                        //.collect::<Vec<i32>>()
                                        .collect::<Vec<_>>()
                                        ;
        let errors: Vec<_> = errors
                                    .into_iter()
                                    .map(Result::unwrap_err)
                                    .collect()
                                    ;
        println!("2) numbers: {:?}", numbers) ;
        println!("2) errors: {:?}", errors) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/box.html
    // Box, stack and heap
    // В Rust все значения по умолчанию выделяются в стеке. Значения 
    // можно упаковать (выделить в куче), создав объект Box<T>.
    // Упакованные значения можно разыменовывать с помощью оператора *; 
    // это устраняет один уровень косвенной адресации.
    {
        use std::mem ;

        struct Point {
            x:  f64,
            y:  f64,
        }

        struct Rectangle {
            top_left:   Point,
            bottom_right: Point
        }

        fn origin() ->Point {
            Point { x: 0_f64, y: 0_f64 }
        }
    
        fn box_origin() ->Box<Point> {
            //Box::new(Point { x: 0., y: 0. })
            Box::new(origin())
        }

        // Stack allocated variables
        let point = origin() ;
        let rectangle = Rectangle {
            top_left:   origin(),
            bottom_right:   Point { x: 10., y: 20. },
        } ;

        // Heap allocated variables
        let box_point = Box::new(origin()) ;
        let box_rectangle = Box::new(Rectangle {
            top_left:   origin(),
            bottom_right:   Point { x: 10., y: 20. }
        }) ;

        // Double inderection
        let box_in_a_box = Box::new(box_origin()) ;

        println!("point occupies {} in stack", 
                mem::size_of_val(   // Returns the size of the pointed-to value in bytes.
                    &point
                )
        ) ;

        println!("box_point occupies {} in stack", 
                mem::size_of_val(&box_point)
        ) ;

        let unbox_point = *box_point ;
        println!("unbox_point occypies {} in stack", 
            mem::size_of_val(&unbox_point)
        ) ;

        println!("size of Point: {}", 
            mem::size_of::<Point>() // Returns the size of a type in bytes.
        ) ;

        println!("box_in_a_box occupies {} in stack", 
            mem::size_of_val(&box_in_a_box)
        ) ;

        let unboxed_box_in_a_box = *box_in_a_box ;
        println!("unboxed_box_in_a_box occupies {} in stack", 
            mem::size_of_val(&unboxed_box_in_a_box)
        ) ;

        let unboxed_unboxed_box_in_a_box = *unboxed_box_in_a_box ;
        println!("unboxed_unboxed_box_in_a_box ocypies {} in stack",
            mem::size_of_val(&unboxed_unboxed_box_in_a_box)
        ) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/vec.html
    // Vectors
    // Вектор представляется с помощью 3 параметров:
    //      указатель на данные
    //      длина
    //      емкость

    {
        let collected_iterator = (0..10)
                                    .collect::<Vec<_>>()
                                    ;
        let mut xs = vec![1i32, 20, 30, 127, 538] ;
        xs.push(4);
        println!("xs: {:?}", xs) ;

        //collected_iterator.push(10);    // cannot borrow `collected_iterator` as mutable, as it is not declared as mutable

        println!("xs.len: {}", xs.len()) ;

        println!("pop last element: {:?}", 
            xs
                .pop()  // Removes the last element from a vector and returns it, or None if it is empty.
        ) ;

        let index = 4 ;
        if xs
            .get(index)
            .is_some() {
            println!("Fourh element: {}", xs[3]) ;  // index out of bounds: the len is 3 but the index is 3
        } else {
            println!("Not found index: xs[{}]", index) ;
        }

        /*
        for x in xs {   // `xs` moved due to this implicit call to `.into_iter()`rustcE0382
            println!("> {}", x) ;
        }

        println!() ;
        */

        for x in xs.iter() {
            println!("> {}", x) ;
        }

        println!() ;

        for x in &xs {
            println!("> {}", x) ;
        }

        println!() ;

        for (i, x) in xs.iter().enumerate() {
            println!("> xs[{}] = {}", i, x) ;
        }

        println!() ;

        for x in xs.iter_mut() {
            *x *= 10 ;
            println!("> {}", x) ;
        }

        println!() ;

        for (i, x) in xs.iter_mut().enumerate() {
            *x *= 10 ;
            println!("> {}", x) ;
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/str.html
    // Strings
    // Строка хранится как вектор байтов (Vec<u8>), но гарантируется, 
    // что она всегда будет допустимой последовательностью UTF-8. Строка
    // выделяется в куче, может расширяться и не завершается нулевым символом.
    //
    // &str — это срез (&[u8]), который всегда указывает на допустимую 
    // последовательность UTF-8 и может использоваться для просмотра 
    // строки, подобно тому как &[T] — это просмотр Vec<T>.
    {
        let pangram = "the quick brown fox jumps over the lazy dog" ;
        println!("\npangram: {}", pangram) ;

        for word in pangram
                            .split_ascii_whitespace()
                            .rev()  // Reverses an iterator's direction.
        {
            println!("> {}", word) ;
        }

        let mut chars = pangram
                        .chars()    // Returns an iterator over the [char]s of a string slice.
                        .collect::<Vec<_>>()
                        ;
        chars
            .sort() // Sorts the slice, preserving initial order of equal elements.
            ;
        chars
            .dedup()    // Removes consecutive repeated elements in the vector 
            ;

        /*
        chars
            .remove(0)
             ;
         */

        println!("chars: {:?}\nlen: {}", chars, chars.len()) ;

        let mut string = String::new() ;
        for c in chars {
            string
                .push(c) // Appends the given char to the end of this String.
                ;
            string
                .push_str(", ") // Appends a given string slice onto the end of this String.
            ;
        }

        println!("string: >{}<", string) ;

        let char_to_trims = &[';', ' ', ','] 
                                      //&[' ', ','] 
                                      ;
        let trimed_string = string
                                    // Возвращает фрагмент: Возвращает новый объект &str (представление/фрагмент), указывающий на потенциально меньший участок исходной строки. Не выделяет новую память.
                                    .trim_matches(char_to_trims)
                                    ;
        println!("trimed_string >{}<", trimed_string) ;

        let alice = String::from("I like dogs.") ;
        let bob = alice.replace("dog", "cat") ;
        println!("alice: {}, bob: {}", alice, bob) ;

        let long_string = "abc
                            tyu
                            dfg->\
                            $-123
                        " ;
/* Выглядит так:
long_string: abc
                            tyu
                            dfg->$-123
*/                        
        println!("long_string: {}", long_string) ;

        println!("Mix chars: \x3F \u{211D}") ;

        let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        println!("raw_str: {}", raw_str);

        let longer_delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", longer_delimiter);

        let bytestring = b"this is a byte string \x52\x75\x73\x74" ;
        println!("bytestring: {:?} -> as string: {:?}", bytestring, str::from_utf8(bytestring)) ;

        let raw_bytestring = br"\u{211D} is not escaped here" ;
        println!("raw_bytestring: {:?}", raw_bytestring) ;

        if let Ok(s) = str::from_utf8(raw_bytestring) {
            println!("raw_bytes as string: {}", s) ;
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/option.html
    // Option
    // Перечисление Option<T> имеет два варианта:
    //      None, указывающий на неудачу или отсутствие значения, и
    //      Some(value), структура кортежа, которая оборачивает значение
    //           типа T.
    //
    {
        let optional_float = Some(0_f32) ;
        let rquive_option_float: Option<f32> = Some(0.) ;
        println!("{:?} -> {}", 
            optional_float,
            optional_float.unwrap()
        ) ;

        let none: Option<i32> = None ;
        let equivalent_none: Option<i32> = None::<i32> ;

        println!("none: {:?}", none) ;
        // println!("none.unwrap(): {}", none.unwrap()) ; // <- panic
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/result.html
    // Result
    // Перечисление Result<T, E> имеет два варианта:
    //      Ok(value), указывающий на успешное выполнение операции и 
    //          содержащий возвращаемое значение (value имеет тип T).
    //      Err(why), указывающий на неудачное выполнение операции и 
    //          содержащий значение why, которое (надеемся) объясняет 
    //          причину сбоя (why имеет тип E).
    {
        mod checked {
            // Mathematical "errors" we want to catch
            #[derive(Debug)]
            pub enum MathError {
                DivisionByZero,
                NonPositiveLogarithm,
                NegativeSquareRoot,
            }

            pub type MathResult = Result<f64, MathError>;

            pub fn div(x: f64, y: f64) -> MathResult {
                if y == 0.0 {
                    // This operation would `fail`, instead let's return the reason of
                    // the failure wrapped in `Err`
                    Err(MathError::DivisionByZero)
                } else {
                    // This operation is valid, return the result wrapped in `Ok`
                    Ok(x / y)
                }
            }

            pub fn sqrt(x: f64) -> MathResult {
                if x < 0.0 {
                    Err(MathError::NegativeSquareRoot)
                } else {
                    Ok(x.sqrt())
                }
            }

            pub fn ln(x: f64) -> MathResult {
                if x <= 0.0 {
                    Err(MathError::NonPositiveLogarithm)
                } else {
                    Ok(x.ln())
                }
            }
        }

        // `op(x, y)` === `sqrt(ln(x / y))`
        fn op(x: f64, y: f64) -> f64 {
            // This is a three level match pyramid!
            match checked::div(x, y) {
                Err(why) => panic!("{:?}", why),
                Ok(ratio) => match checked::ln(ratio) {
                    Err(why) => panic!("{:?}", why),
                    Ok(ln) => match checked::sqrt(ln) {
                        Err(why) => panic!("{:?}", why),
                        Ok(sqrt) => sqrt,
                    },
                },
            }
        }

        //println!("{}", op(1.0, 10.0));  // panic: NegativeSquareRoot
        println!("op: {}", op(100., 10.)) ;

        let v: Result<i32, String> = Err("error".to_string()) ;
        println!("v: {:?}", v) ;
        /*
        println!("v: {:?}", 
            v.unwrap()  // panic:    called `Result::unwrap()` on an `Err` value: "error"
        ) ;
          */
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/result/question_mark.html
    // ?

    {
        mod checked {
            #[derive(Debug)]
            enum MathError {
                DivisionByZero,
                NonPositiveLogarithm,
                NegativeSquareRoot,
            }

            type MathResult = Result<f64, MathError>;

            fn div(x: f64, y: f64) -> MathResult {
                if y == 0.0 {
                    Err(MathError::DivisionByZero)
                } else {
                    Ok(x / y)
                }
            }

            fn sqrt(x: f64) -> MathResult {
                if x < 0.0 {
                    Err(MathError::NegativeSquareRoot)
                } else {
                    Ok(x.sqrt())
                }
            }

            fn ln(x: f64) -> MathResult {
                if x <= 0.0 {
                    Err(MathError::NonPositiveLogarithm)
                } else {
                    Ok(x.ln())
                }
            }

            // Intermediate function
            fn op_(x: f64, y: f64) -> MathResult {
                // if `div` "fails", then `DivisionByZero` will be `return`ed
                let ratio = div(x, y)?;

                // if `ln` "fails", then `NonPositiveLogarithm` will be `return`ed
                let ln = ln(ratio)?;

                sqrt(ln)
            }

            pub fn op(x: f64, y: f64) {
                match op_(x, y) {
                    Err(why) => panic!("{}", 
                      match why {
                        MathError::NonPositiveLogarithm => "logarithm of non-positive number",
                        MathError::DivisionByZero => "division by zero",
                        MathError::NegativeSquareRoot => "square root of negative number",
                      }),
                    Ok(value) => println!("{}", value),
                }
            }
        }

        checked::op(100.0, 10.0); // square root
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/panic.html
    // panic!

    {
        fn division(divident: i32, divisor: i32) ->i32 {
            if divisor == 0 {
                panic!("division by zero !!!") ;
            } else {
                divident / divisor
            }
        }

        // division(10, 0) ;   // division by zero !!!
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/hash.html
    // HashMap
    // HashMap хранят значения по ключу. Ключами HashMap могут быть 
    // логические значения, целые числа, строки или любой другой тип,
    // реализующий трейты Eq и Hash.
    {
        use std::collections::HashMap ;

        fn call(number: &str) ->&str {
            match number {
                "1" => "Hello ms. Smith.",
                "2" => "Hello Adam.",
                _ => "Who is it?",
            }
        }

        let mut contacts = HashMap::new() ;

        let name = "Smith" ;

        contacts
            .insert(name, //"Smith", 
                    "1") ;
        contacts
            .insert("Adam", "2") ;
        contacts
            .insert("N", "0") ;

        println!("name: {}", name) ;

        println!("contacts: {:?}", contacts) ;

        match contacts.get(&"Smith")    // Returns a reference to the value corresponding to the key.
        {
            Some(&num) => println!("num: {}, name: {}", num, call(num)),
            _ => println!("It isn't Smith"),
        }

        let name = "Smith" ;
        let res = contacts.insert(name, "10") ;
        println!("{} old number: {:?}, new number: {}", name, res, contacts[name]) ;

        let res = contacts.remove(&name) ;
        println!("{} was removed, old value: {:?}", name, res) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/hash/alt_key_types.html
    // Alternate/custom key types
    // В качестве ключа в HashMap может выступать любой тип, реализующий
    // трейты Eq и Hash. Сюда входят:
    //      bool (хотя это не очень полезно, поскольку возможны только два ключа)
    //      int, uint и все их вариации
    //      String и &str (подсказка: можно создать HashMap с ключом типа
    //           String и вызвать метод .get() с ключом &str)    
    //
    // Все классы коллекций реализуют интерфейсы Eq и Hash, если их 
    //      содержащий тип также реализует интерфейсы Eq и Hash 
    //      соответственно. Например, Vec<T> будет реализовывать 
    //      интерфейс Hash, если T реализует интерфейс Hash.
    // Вы можете легко реализовать интерфейсы Eq и Hash для 
    // пользовательского типа всего одной строкой кода: 
    // #[derive(PartialEq, Eq, Hash)]    
    {
        use std::collections::HashMap ;

        #[derive(Eq, Hash, PartialEq, Debug)]
        struct Account<'a> {
            username:   &'a str,
            password:   &'a str,
        }

        #[derive(Debug)]
        struct AccountInfo<'a> {
            name:   &'a str,
            email:  &'a str,
        }

        type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>> ;

        fn try_login<'a>(
                accounts:   &'a Accounts,
                username:   &'a str,
                password:   &'a str,
            ) 
        {
            println!("un: {}, pwd: {}, trying login ...", 
                    username,
                    password,
                ) ;

            let login = Account {
                username,
                password,
            } ;

            accounts.get(&login) ;


            match accounts.get(&login) {
                Some(account_info) => 
                    println!("Successful login.\nname: {}\nemail: {}", account_info.name, account_info.email),
                None => println!("Login filed."),
            }
        }

        let mut accounts = HashMap::new() ;

        let (username, password) = ("username", "password") ;
        let account = Account {
            username:   username, //"username",
            password:   password, // "password",
        } ;

        let account_info = AccountInfo {
            name:   "name",
            email:  "name@name.gov",
        } ;

        accounts.insert(account, account_info) ;

        println!("accpunts: {:#?}", accounts) ;
        //println!("account: {:?}", account) ;    // borrow of moved value: `account`
        //println!("account_info: {:?}", account_info) ;  // borrow of moved value: `account_info`

        try_login(&accounts, username, password);

        try_login(&accounts, 
                  username, 
                  format!("{}{}", password, "10")
                    .as_str()
                );

        //let v  = ("1".to_owned() + "2").as_str() ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/hash/hashset.html
    // HashSet
    // Уникальная особенность HashSet заключается в том, что он 
    // гарантированно не содержит повторяющихся элементов. 
    // Если вы вставляете значение, которое уже присутствует в HashSet 
    // (то есть новое значение равно существующему, и оба имеют 
    // одинаковый хеш), то новое значение заменит старое.
    //
    // Множества имеют 4 основные операции (все следующие вызовы возвращают итератор):
    //      union: получить все уникальные элементы в обоих множествах.
    //      difference: получить все элементы, которые есть в первом множестве, но не во втором.
    //      intersection: получить все элементы, которые есть только в обоих множествах.
    //      symmetric_difference: получить все элементы, которые есть либо в одном множестве, либо в другом, но не в обоих.
    {
        use std::collections::HashSet ; // A [hash set] implemented as a HashMap where the value is ().

        // содержит только уникальные значения
        let mut a = vec![1u32, 2, 3, 3, 4]
                                .into_iter()
                                .collect::<HashSet<_>>()
                                ;
        println!("a: {:?}", a) ;    // вывод в произвольном порядке

        println!("10 is inserted: {}", a.insert(10)) ;
        println!("10 is inserted: {}", a.insert(10)) ;

        let v = 10 ;
        println!("a containts {} -> {}", v, a.contains(&v)) ;

        let v = 11 ;
        assert!(a.insert(v),
                // формирование ошибочного выражения об ошибке
                // если a.insert(v) не вставляетданые
                "{}",
                format!("{} was inserted earlier", v)
            ) ;

        let mut b = vec![2_u32, 3, 4]
                            .into_iter()
                            .collect::<HashSet<u32>>()
                            ;
        println!("a: {:?}\nb: {:?}", a, b) ;

        // union
        let un = a
                                    .union(&b)
                                    ;
        println!("un: {:?}", un) ;

        let mut array_un = a
                        .union(&b)
                        .collect::<Vec<_>>()
                        ;
        array_un
            .sort();

        println!("array_un: {:?}", array_un) ;

        // difference
        let diff = a
                .difference(&b)
                ;
        println!("diff: {:?}, type: {}", diff, type_of(&diff)) ;

        let mut array_diff = a
                .difference(&b)
                .collect::<Vec<_>>()
                //.sort_by(|a, b| a.cmp(b))
                ;
        array_diff.sort_by(|a, b| b.cmp(a));
        println!("array_diff: {:?}", array_diff) ;  // reverse sort

        /*
        array_diff
            .sort()
            ;
        println!("array_diff: {:?}", array_diff) ;
        */
        /* или так:
        let array_diff = {
                    array_diff
                    .sort() ;
                    array_diff
                }
                .into_iter()
                .rev()
                .map(|&x| x)
                .collect::<Vec<_>>()
                ;
        println!("array_diff: {:?}", array_diff) ;
         */
        
        // intersection
        let mut array_int = a
                .intersection(&b)
                .into_iter()
                .map(|x| *x)
                .collect::<Vec<_>>()
                ;
        array_int
            .sort_by(|a, b| b.cmp(a)) // reverse sort
                        ;
        println!("array_int: {:?}", array_int) ;

        // symmetric difference
        let mut array_sym_diff = a
                .symmetric_difference(&b)
                .into_iter()
                .map(|x| *x)
                .collect::<Vec<_>>()
                ;
        array_sym_diff
            .sort_by(|a, b| b.cmp(a));

        println!("array_sym_diff: {:?}", array_sym_diff) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/rc.html
    // Rc
    // Когда требуется множественное владение, можно использовать 
    // Rc (счетчик ссылок). Rc отслеживает количество ссылок, то есть 
    // количество владельцев значения, заключенного внутри Rc.
    // Клонирование Rc никогда не выполняет глубокого копирования. 
    //
    // Клонирование создает просто еще один указатель на заключенное 
    // значение и увеличивает счетчик.
    {
        use std::rc::Rc ;

        let rc_val = "Rc example".to_owned() ;

        {
            let rc_a = Rc::new(rc_val) ;

            println!("rc_a: {}", rc_a) ;

            println!("rc_a count: {}", Rc::strong_count(&rc_a)) ; // Gets the number of strong (Rc) pointers to this allocation.

            println!("-----------------------------") ;

            {
                let rc_b = Rc::clone(&rc_a) ;
                println!("rc_b count: {}", Rc::strong_count(&rc_b)) ;
                println!("rc_a count: {}", Rc::strong_count(&rc_a)) ;

                println!("rc_a = rc_b -> {}", rc_a.eq(&rc_b)) ;
                println!("rc_a len value = {} in bytes.", rc_a.len()) ;   // Returns the length of this String, in bytes
                println!("-----------------------------") ;
            }

            println!("rc_a count: {}\n", Rc::strong_count(&rc_a)) ;
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std/arc.html
    // Arc
    // Когда требуется совместное владение данными между потоками, можно
    // использовать структуру Arc (Atomically Reference Counted). Эта 
    // структура, благодаря реализации Clone, может создавать указатель 
    // ссылки на местоположение значения в куче памяти, одновременно 
    // увеличивая счетчик ссылок.
    {
        use std::time::Duration ;
        use std::sync::Arc ;
        use std::thread ;

        let apple = Arc::new("the same apple.") ;
        println!("apple: {}, apple count: {}", apple, Arc::strong_count(&apple)) ;

        for _ in 0..10 {
            let apple = Arc::clone(&apple) ;

            thread::spawn(move ||{
                println!("Inside apple: {}, count: {}", apple, Arc::strong_count(&apple)) ;
            }) ;
        }
        thread::sleep(Duration::from_secs(1));
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/threads.html
    // Threads
    // В Rust предусмотрен механизм для запуска нативных потоков 
    // операционной системы с помощью функции `spawn`, аргументом 
    // которой является замыкание с перемещением.
    {
        use std::thread ;

        const NTHREADS: u8 = 10 ;

        let mut children = vec![] ;

        println!() ;

        for i in 0..NTHREADS {
            children.push(
                thread::spawn(move || ->_ { // можно так
                    println!("This is thread: {}", i) ;
                    i * 10
                })
            );
        }

        for child in children {
            let v = child.join() ;  // Waits for the associated thread to finish
            println!("v: {:?}", v) ;
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/threads/testcase_mapreduce.html
    // Testcase: map-reduce

    {
            use std::thread ;

            let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

            let mut children = vec![] ;

            for (i, data_segment) in data
                        .split_whitespace()
                        .enumerate()
            {
                println!("data segment {} is {}", i, data_segment) ;
                children.push(
                    thread::spawn(move || ->u32 {
                        data_segment
                            .chars()
                            .map(|c| c
                                .to_digit(10)
                                .expect(format!("Invalid char: {}", c).as_str())
                            )
                            .sum()
                    })
                );
            }
            
            let final_result = children
                    .into_iter()
                    .map(|n| 
                        n
                            .join()
                            .unwrap()
                    )
                    .sum::<u32>()
                    ;
        println!("final_result: {}", final_result) ;

        // или так (последовательный подчсёт):
        let final_result_2 = data
                    .split_whitespace()
                    .map(|s_in| {
                        s_in
                            .chars()
                            .map(|ch| ->u32{
                                ch
                                    .to_digit(10)
                                    .unwrap()
                            })
                            .sum::<u32>()
                    })
                    .sum::<u32>()
                    ;
        println!("final_result_2: {}", final_result_2) ;

        // или так:
        let final_result_3 = data
                    .split_whitespace()
                    .map(|s| {
                        thread::spawn(|| ->_{
                            s
                            .chars()
                            .map(|ch| ->_{
                                ch
                                .to_digit(10)
                                .unwrap()
                            })
                            .sum::<u32>()
                        })
                    })
                    .map(|un| {
                        un
                        .join()
                        .unwrap()
                    })
                    .sum::<u32>()
                    ;
        println!("final_result_3: {}", final_result_3) ;
    }

    // функция на тестах:
    {
        use std::thread ;

        fn par_sum(v: Vec<i32>) ->i32 {
            let mut child_proces = vec![] ;

            for v_tmp in v.chunks(3) {
                let v_unit = v_tmp.to_vec() ;
                child_proces.push(
                    thread::spawn(move || ->i32 {
                            v_unit.to_vec()
                                .iter()
                                .map(|x| x * x)
                                .sum()
                    })
                ) ;
            }

            /*
            let mut sum_out = 0 ;

            for p in child_proces {
                sum_out += p
                .join()
                .unwrap()
                ;
            }
            sum_out
             */
            child_proces
                .into_iter()
                .map(|x| {
                    x
                        .join()
                        .unwrap()
                })
                .sum/*::<i32> */()
        }

        let vec = vec![0,1,2,3,4,5,6,7,8,9,10] ;
        /*
        vec
            .iter()
            .max()
            .map(|v| *v)
            ;
         */
        println!("1) sum: {}", par_sum(vec)) ;
    }

    // или так

    {
        use std::thread ;

        fn par_sum(vec: Vec<i32>) ->i32 {
            let child_proces = 
                    vec
                        .chunks(3)
                        .map(|v| {
                            let v_tmp = v.to_vec() ;
                            thread::spawn(move || ->i32{
                                v_tmp
                                    .iter()
                                    .map(|x| x * x)
                                    .sum()
                            })
                        })
                        .collect::<Vec<_>>()
                        ;
            /*
            let mut sum_out = 0 ;
            for p in child_proces {
                sum_out += p
                    .join()
                    .unwrap()
                    ;
            }
            sum_out
             */
            child_proces
                .into_iter()
                .map(|p| {
                    p
                        .join()
                        .unwrap()
                })
                .sum()
        }
        
        let vec = vec![0,1,2,3,4,5,6,7,8,9,10] ;

        println!("2) sum: {}", par_sum(vec)) ;
    }

    // или так

    {
        use std::thread ;

        fn par_sum(vec: Vec<i32>) ->i32 {
            vec
                .chunks(3)
                .map(|v_u| {
                    let v_tmp = v_u.to_vec() ;
                    thread::spawn(move || ->i32{
                        v_tmp
                            .iter()
                            .map(|x| x * x)
                            .sum()
                    })
                })
                //.collect::<Vec<_>>()
                //.into_iter()
                .map(|p|{
                    p
                        .join()
                        .unwrap()
                })
                .sum()
        }

        let vec = vec![0,1,2,3,4,5,6,7,8,9,10] ;

        println!("3) sum: {}", par_sum(vec)) ;
    }

    // или так

    {
        use rayon::prelude::* ;

        fn par_sum(vec: Vec<i32>) ->i32 {
            vec
                .par_iter()
                .map(|x| x * x)
                .sum()
        }

        let vec = vec![0,1,2,3,4,5,6,7,8,9,10] ;

        println!("4) sum: {}", par_sum(vec)) ;

        assert_eq!(385, par_sum(vec![0,1,2,3,4,5,6,7,8,9,10])) ;
    }

    // или так:
    {
        use std::thread ;

        const SIZE_PART: u32 = 3 ;

        fn par_sum(v: Vec<i32>) ->i32 {
            v
                .chunks(SIZE_PART as usize)
                .map(|x| {
                    let v_p = x.to_vec() ;
                    thread::spawn(move || ->_{
                        v_p
                            .iter()
                            .map(|x| x * x)
                            .sum::<i32>()
                    })
                })
                .map(|u| ->_{
                    u
                    .join()
                    .unwrap()
                })
                .sum()
        }

        let vec = vec![0,1,2,3,4,5,6,7,8,9,10] ;

        println!("5) sum: {}", par_sum(vec)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/channels.html
    // Channels
    // Rust предоставляет асинхронные каналы для связи между потоками. 
    // Каналы позволяют осуществлять однонаправленный поток информации 
    // между двумя конечными точками: отправителем и получателем.
    {
        //use std::sync::mpsc::{Sender,Receiver} ;
        use std::sync::mpsc ;
        use std::thread ;

        static NTHREADS: i32 = 3 ;

        let (tr, rc) = mpsc::channel/*::<i32> */() ;

        let ids = (0..NTHREADS)
            .map(|id|{
                let tr_clone = tr.clone() ;
                thread::spawn(move ||{
                    tr_clone
                            .send(id)
                            .unwrap()
                })
            })
            //.collect::<Vec<_>>()
            //.into_iter()
            .map(|p|{
                println!("Make join!") ;
                p
                    .join()
                    .expect("1) child thread panicked")
            })
            .map(|x| {
                rc
                 .recv()
                 .unwrap()
            })
            .collect::<Vec<_>>()
            ;

            println!("After call! {:?}", ids) ;

            /*
            let mut ids = Vec::with_capacity(NTHREADS as usize) ;

            for _ in (0..1) {
                let id = rc.recv().unwrap() ;
                println!("Receive: {:?}", id) ;
                ids.push(id);
            }

            println!("ids: {:?}", ids) ;
             */
    }

    // или так:

    {
        static THCOUNT: i32 = 3 ;

        //use std::sync::mpsc::{self, Sender, Receiver} ;
        use std::sync::mpsc ;
        use std::thread ;

        let (tr, rc) = mpsc::channel() ;

        let mut  v_out = (0..THCOUNT)
                .map(|id| {
                    let tr_clone = tr.clone() ;
                    thread::spawn(move ||{
                        tr_clone
                            .send(id)
                            .unwrap()
                    })
                })
                //.collect::<Vec<_>>()
                //.into_iter()
                .map(|j| {
                    j
                    .join()
                    .unwrap()
                })
                //.collect::<Vec<_>>()
                //.iter()
                .map(|_| {
                    rc.recv().unwrap()
                })
                .collect::<Vec<_>>()
                ;
        println!("v_out: {:?}", {v_out.sort() ; v_out}) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/path.html
    // Path
    // Структура Path представляет пути к файлам в базовой файловой 
    // системе. Существует два варианта Path: posix::Path для 
    // UNIX-подобных систем и windows::Path для Windows.
    // Строка Path является неизменяемой. Принадлежащая версия Path — это
    // PathBuf. Связь между Path и PathBuf аналогична связи между str и 
    // String: PathBuf может изменяться на месте и может быть 
    // разыменована в Path.
    {
        use std::path::Path ;

        let path = Path::new(".") ;
        let displ = path
                                    .display() // Returns an object that implements [Display] for safely printing paths
                                     ;
        println!("displ: {}", displ) ;  // .

        let mut new_path = path
                                        .join("")
                                        .join("a") // Creates an owned PathBuf with path adjoined to self.
                                        .join("b")
                                        ;

        println!("{:?}", new_path) ;    // ".\\a\\b"

        new_path
            .push("c")  // Extends self with path.
            ;

        println!("{:?}", new_path) ;    // ".\\a\\b\\c"

        new_path
            .push("myfile.tar.gz")
            ;

        println!("{:?}", new_path) ;    // ".\\a\\b\\c\\myfile.tar.gz"

        new_path
            .set_file_name("package.tgz") // Updates [self.file_name] to file_name.
            ;

        println!("{:?}", new_path) ;    // ".\\a\\b\\c\\package.tgz"

        new_path
            .set_extension("BIG_EXT")
            ;

        println!("{:?}", new_path) ; // ".\\a\\b\\c\\package.BIG_EXT"

        match new_path
                .to_str() // Yields a [&str] slice if the Path is valid unicode.
        {
            Some(s) => println!("new_path is: {}", s),
            None => panic!("invalid utf8 in new_path"),
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/open.html
    // open
    // Функция open позволяет открыть файл в режиме только для чтения.
    {
        use std::fs::File ;
        use std::path::Path ;
        use std::io::Read ;

        let path = Path::new("Cargo.toml") ;
        let displ = path
                                    //Returns an object that implements [Display] 
                                    // for safely printing paths that may contain non-Unicode data.
                                    .display()
                                    ;

        println!("displ: {}", displ) ;

        // Функция open позволяет открыть файл в режиме только для чтения.
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(err) => panic!("Cannot open file: {}", displ),
        } ;

        let mut s = String::new() ;
        // Reads all bytes until EOF in this source, appending them to buf.
        match file.read_to_string(&mut s) 
        {
            Ok(_) => println!("{} content:\n{}", displ, s),
            Err(err) => panic!("Error: {} to open file: {}", err, displ),
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/create.html
    // create
    // Функция create открывает файл в режиме только для записи (write-only ). 
    // Если файл уже существовал, его старое содержимое уничтожается. 
    // В противном случае создается новый файл.
    {
        use std::fs::File ;
        use std::path::Path ;
        use std::io::Write ;

        static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
" ;

        let path = Path::new("text2.txt") ;
        let displ = path.display() ;

        let mut file = match File::create(path) { // Opens a file in write-only mode.
            Ok(f) => f,
            Err(err) => panic!("Cannot create file: {}, error: {}", displ, err),
        } ;

        // Attempts to write an entire buffer into this writer.
        match file.write_all(
                    LOREM_IPSUM.as_bytes()  // Converts a string slice to a byte slice.
                ) {
            Ok(_) => println!(),
            Err(err) => panic!("Error: {} write to file: {}", err, displ),
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
    // read_lines

    {
        use std::fs::read_to_string ; // Reads the entire contents of a file into a string.

        // так:
        fn read_lines(filename: &str) ->Vec<String> {
            let mut result = Vec::new() ;

            for line in read_to_string(filename)
                                    .unwrap()
                                    .lines() // Returns an iterator over the lines of a string, as string slices.
            {
                result.push(line.to_string());
            }

            result
        }

        // или так:
        fn read_lines2(file_name: &str) ->Vec<String> {
            read_to_string(file_name)
                .unwrap()
                .lines() // Line terminators are not included in the lines returned by the iterator.
                .map(|l|{l.to_owned()})
                .collect()
        }

        // или так
        fn read_lines3(file_name: &str) ->Vec<String> {
            read_to_string(file_name)
                .unwrap()
                .lines()
                .map(|l| l.to_string())
                .collect()
        }

        // или так:
        fn read_lines4(file_name: &str) ->Vec<String> {
            read_to_string(file_name)
                .unwrap()
                .lines()
                .map(String::from)
                .collect()
        }

        let name_file = "text2.txt" ;
        println!("1) {} Content from vector:\n{:#?}\n", name_file, read_lines(name_file)) ;
        println!("2) {} Content from vector:\n{:#?}\n", name_file, read_lines2(name_file)) ;
        println!("3) {} Content from vector:\n{:#?}\n", name_file, read_lines3(name_file)) ;
        println!("4) {} Content from vector:\n{:#?}\n", name_file, read_lines4(name_file)) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
    // создание итератора для чтения строк из файла
    {
        use std::fs::File ;
        use std::path::Path ;
        use std::io::{self, BufReader} ;
        use std::io::BufRead ;


        fn read_lines<P>(file_name: P) ->io::Result<io::Lines<io::BufReader<File>>>
            where P: AsRef<Path>,
        {
            let file = File::open(file_name) // Attempts to open a file in read-only mode.
                                ?
                                ;
            Ok(io::BufReader::new(  // Creates a new BufReader<R> with a default buffer capacity.
                    file
                )
                .lines()    // Returns an iterator over the lines of this reader.
            )
        }

        let file_name = "text2.txt" ;

        if let Ok(lines) = read_lines(file_name) {
            for line in lines
                                    .map_while( // возвразает элементы пока замыкание возвращает Some(_)
                                        // |x| // x: Result<String, Error>
                                        Result::ok  // Converts from Result<T, E> to Option<T>
                                    ) 
            {
                println!("{}", line) ;
            }
        }

    }


    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/process.html
    // Child processes
    // Структура process::Output представляет собой выходные данные 
    // завершенного дочернего процесса, 
    // а структура process::Command — это построитель процессов.
    {
        println!("\n") ;

        use std::process::Command ;

        let output = Command::new("rustc") // Constructs a new Command for launching the program at path program
                                .arg("--version")
                                .output()   // Executes the command as a child process, waiting for it to finish and collecting all of its output.
                                // Returns the contained Ok value or computes it from a closure.
                                .unwrap_or_else(|err| panic!("faild to execute process: {}", err))
                                ;

        if output
            .status
            .success() {
            println!("successful, stdout: {}", 
                String::from_utf8_lossy( //Converts a slice of bytes to a string, including invalid characters.
                        &output.stdout
                    )
            ) ;
        } else {
            println!("failed,stderr: {}",
                String::from_utf8_lossy(&output.stderr)
            )
        }

    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/process/pipe.html
    // Pipes
    // Структура std::process::Child представляет дочерний процесс и 
    // предоставляет доступ к дескрипторам stdin, stdout и stderr для 
    // взаимодействия с нижележащим процессом через каналы.
    //
    {
        use std::process::{Command, Stdio} ;
        //use std::io::prelude::* ;
        use std::io::{Write, Read} ;
        

        static PANGRAM: &str = "the quick brown fox jumped ober the lazy dog" ; 

        let mut cmd = if cfg!(target_os = "windows") // Evaluates boolean combinations of configuration flags at compile-time.
        {
            let mut cmd = Command::new("powershell") ;
            cmd
                .arg("-Command")
                .arg("$input | Measure-Object -Line -Word -Character") 
                ;
            cmd
        } else {
            Command::new("wc") // Constructs a new Command for launching the program at path program
        } ;

        // конфигурирование и порождение дочернего процесса
        let process = match cmd
                .stdin( // Configuration for the child process's standard input (stdin) handle.
                    Stdio::piped()  // A new pipe should be arranged to connect the parent and child processes.
                )
                .stdout( // Configuration for the child process's standard output (stdout) handle.
                    Stdio::piped()  // Ноывй pip должен быть устроенным чтобы соединить родительский и дочерний процессы.
                )
                .spawn() // Executes the command as a child process, returning a handle to it.
        {
            Ok(p) => p,
            Err(err) => panic!("couldn't spawn proceess: {}", err),
        } ;

        match process
                .stdin //The handle for writing to the child's standard input (stdin)
                .unwrap()
                .write_all(PANGRAM.as_bytes())
        {
          Ok(_) => println!("Send pamgram to process"),
          Err(err) => panic!("couldn't write to process stdin: {}", err),
        }

        let mut s = String::new() ;

        match process
                    .stdout // The handle for reading from the child's standard output (stdout)
                    .unwrap()
                    .read_to_string(&mut s) // Reads all bytes until EOF in this source, appending them to buf.
        {
            Ok(readed) => println!("Readed {} bytes string from process stdout:\n{}", readed, s),
            Err(err) => println!("Error read from process stdout: {}", err),
        }

    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/process/wait.html
    // Wait
    // Если вы хотите дождаться завершения процесса process::Child, 
    // необходимо вызвать метод Child::wait, который вернет значение 
    // process::ExitStatus.
    {
        use std::process::Command ;

        println!("Start process.") ;

        let mut child = if cfg!(target_os = "windows") {
                Command::new("powershell")
            .arg("Start-Sleep -Seconds 5")
            .spawn()
            .unwrap()
        }
        else {
            Command::new("sleep")
                .arg("5")
                .spawn()
                .unwrap()
        }
        ;

        let exit_status = child
                    .wait()
                    .unwrap()
                    ;
        println!("Exist status: {}", exit_status) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/fs.html
    // Filesystem Operations
    // Модуль std::fs содержит несколько функций, работающих с файловой 
    // системой.
    {
        use std::fs::{self,
                      File, 
                      OpenOptions   // Options and flags which can be used to configure how a file is opened.
                    } ;
        use std::path::Path ;
        use std::io::{self, Read, Write} ;
        
        #[cfg(target_family = "unix")]
        use std::os::unix ;

        #[cfg(target_family = "windows")]
        use std::os::windows ;

        // Simple release of: $ cat path
        // так:
        fn cat(path: &Path) ->io::Result<String> {
            let mut f = File::open(path)? ;
            let mut s = String::new() ;

            match f.read_to_string(&mut s) { // Reads all bytes until EOF in this source, appending them to buf.
                Ok(_) => Ok(s),
                Err(err) => Err(err),
            }
        }
        // или так:
        fn cat2(path: &Path) -> io::Result<String>{
            let mut f = File::open(path)?;
            let mut s = String::new() ;
            f.read_to_string(&mut s)?;
            Ok(s)
        }

        // Simple release: echo s > path (запись в контента в указанный файл)
        fn echo(s: &str, path: &Path) ->io::Result<()> {
            let mut f = File::create(path)? ;   // Opens a file in write-only mode.
            match f.write_all(s.as_bytes()) { // Attempts to write an entire buffer into this writer.
                Ok(v) => Ok(v),
                Err(err) => Err(err),
            }
        }

        // Simple release: $ touch path (ignores existing files)
        // Struct OpenOptions: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
        fn touch(path: &Path) ->io::Result<()> {
            match OpenOptions::new() // Creates a blank new set of options ready for configuration.
                        .create(true) // Sets the option to create a new file, or open it if it already exists.
                        .write(true) // Sets the option for write access.
                        .open(path) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }

        println!("mkdir a") ;

        let dir_name = Path::new("a") ;

        // let dir_name = "a" ;
        if ! dir_name.is_dir() {
            match fs::create_dir(dir_name) {
                Ok(v) => {},
                Err(err) => println!("cannot make dir: {:?}, error: {}", dir_name, err),
            }
        }

        let mut file_name = "a/b.txt" ;
       
        println!("echo hello > {}", file_name) ;
        echo("hello", Path::new(file_name))
            .unwrap_or_else(|err| {
                println!("Error create file: {}, err: {}", 
                    file_name, 
                    err.kind()
                ) ;
            }) ;

        file_name = "a/c/d" ;
        if ! Path::new(file_name).is_dir() {
            println!("mkdir -p {}", file_name) ;
            fs::create_dir_all(file_name)
                .unwrap_or_else(|err|{
                    println!("Cannot make dir: {}, error lind: {}",
                        file_name, err.kind()
                    ) ;
                }) ;
        }

        /*
        use std::io::{Error, ErrorKind};
        println!("Example Error Kind: {}",
            Error::new(ErrorKind::AddrInUse, "oh no!")
            .kind()
        ) ;
        */

        file_name = "a/c/e.txt" ;
        println!("touch({})", file_name) ;
        touch(Path::new(file_name))
            .unwrap_or_else(|x|{
                println!("rouch({}) error kind: {}", file_name, x.kind()) ;
            }) ;

        let file_orig = "a/b.txt" ;

        println!("Contol file: {} is {}",
            file_orig, Path::new(file_orig).is_file()
        ) ;

        file_name = "a/c/b.txt" ;
        //file_name = "a/b1.txt" ;
        println!("ln -s {} {}", file_orig, file_name) ;
        #[cfg(target_family = "windows")]
        windows::fs::symlink_file(file_orig, file_name)
            .unwrap_or_else(|err| {
                println!("ln -s {} {} , error kind: {}, error: {}",
                    file_orig, file_name, err.kind(), err
                ) ;
            }) ;

        #[cfg(target_family = "unix")]
        unix::fs::symlink_file(file_orig, file_name)
            .unwrap_or_else(|err| {
                println!("ln -s {} {} , error kind: {}. error: {}",
                    file_orig, file_name, err.kind(), err
                ) ;
            }) ;

        file_name = "a/b.txt" ;
        println!("cat {}", file_name) ;
        match cat(Path::new(file_name)) {
            Ok(v) => println!("{}", v),
            Err(err) => println!("cat {}, error kind: {},error: {}",
                file_name, err.kind(), err),
        }
        
        file_name = "a" ;
        println!("ls {}", file_name) ;
        match fs::read_dir(Path::new(file_name)) {
            Ok(paths) => {
                println!("{:?}", paths) ;

                for path in paths {
                    let dir_entry_tmp = path.unwrap() ;

                    println!("> path: {:?}, file_name: {:?}, file_type: {:#?}, metadata: {:#?}",
                        dir_entry_tmp.path(),
                        dir_entry_tmp.file_name(),
                        dir_entry_tmp.file_type().unwrap(),
                        dir_entry_tmp.metadata().unwrap(),
                    ) ;
                }
            },
            Err(err) => println!("ls {}, error kind: {}, error: {}",
                file_name, err.kind(), err
            ),
        }

        fn visit_dir(path: &Path, 
                     del_file: bool,
                     del_dir: bool,
                     del_symb_link: bool,
                    ) ->io::Result<()>{
            if path.is_dir() {
                for entry in path.read_dir()? {
                    let entry = entry? ;
                    let path = entry.path() ;
                    if path.is_dir() {
                        println!("dir: {:?}", path) ;
                        visit_dir(&path, del_file, del_dir, del_symb_link)? ;
                        /*
                        if del_dir {
                            println!("rm dir: {:?}", path) ;
                            fs::remove_dir(path)?;
                        }
                         */
                    } else if path.is_file() {
                        println!("file: {:?}", path) ;
                        if del_file {
                            println!("rm file: {:?}", path) ;
                            fs::remove_file(path)? ;
                        }
                    } else if path.is_symlink() {
                       println!("symbol link: {:?}", path) ;
                       if del_symb_link {
                            println!("rm symbol link: {:?}", path) ;
                            fs::remove_file(path)?;
                       }
                    }
                }

                if del_dir {
                    println!("rm dir: {:?}", path) ;
                    fs::remove_dir(path) ?
                }
            }
            Ok(())
        }
        // или так:
        fn visit_dir2(path: &Path) ->io::Result<()>{
            if path.is_dir() {
                for entry in path.read_dir()? {
                    let dir_entry = entry? ;
                    let path_unit = dir_entry.path() ;
                    if path_unit.is_dir() {
                        println!("dir: {:?}", path_unit) ;
                        visit_dir2(&path_unit)? ;
                    } else if path_unit.is_file() {
                        println!("file: {:?}", path_unit) ;
                    } else if path.is_symlink() {
                        println!("symbol link: {:?}", path_unit) ;
                    }
                }
            }
            Ok(())
        }
        // или так:
        fn visit_dir3(path: &Path) ->io::Result<()> {
            if path.is_dir() {
                let dir_reader = path.read_dir()? ;
                for entry in dir_reader {
                    let entry_dir = entry? ;
                    let path_unit = entry_dir.path() ;
                    if path_unit.is_file() {
                        println!("file: {:?}", path_unit) ;
                    } else if path_unit.is_dir() {
                      println!("dir: {:?}", path_unit) ;
                      visit_dir3(&path_unit)?;
                    }
                }
            }
            Ok(())
        }

        visit_dir3(&Path::new("a")) ;
        println!("\n\n") ;

        visit_dir2(&Path::new("a")) ;
        println!("\n\n") ;

        visit_dir(Path::new("a"), true, false, false)
            .unwrap() ;

        visit_dir(Path::new("a"), true, false, true)
            .unwrap() ;

        visit_dir(Path::new("a"), true, true, true)
            .unwrap() ;            
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/arg.html
    // Program arguments
    // Command line apps in Rust: https://rust-cli.github.io/book/index.html

    {
        use std::env ;

        let args = env::args()
                .collect::<Vec<_>>() 
            ;

        println!("args: {:?}, count real args: {}", args, args.len() - 1) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/arg/matching.html
    // Argument parsing
    // Run as:
    // C:\Users\user>cd C:\Users\user\work\MyWorks\Rust\rust-incubator\1_concepts\1_1_default_clone_copy\
    // так:
    // cargo run
    // или так:
    // cargo run 42
    // или так:
    // cargo run increase 4
    // или так:
    // cargo run decrease 42 ???
    // 

    {
        use std::env ;
        
        fn increment(num: i32) {
            println!("Increased: {}", num + 1) ;
        }

        fn decrement(num: i32) {
            println!("Decreased: {}", num - 1) ;
        }

        fn help(name: &str) {
            println!("usage:
{} <string>
{} {{increase|decrease}} <string>", name, name) ;
        }

        println!("\n\n") ;

        let args = env::args()
                            .collect::<Vec<_>>()
                            ;
        match args.len() {
            // no arguments passed
            1 => println!("My Name is {}", args[0]),
            // one argument passed
            2 => {
                match args[1].parse() {
                    Ok(42) => println!("This is the answer."),
                    Err(err) => println!("This isn't the answer, it's error: {}", err),
                    _ => println!("The invalid answer: {}", args[1]),
                }
            },
            // one command and one argument passed
            3 => {
                let number = match args[2].parse::<i32>() {
                    Ok(n) => n,
                    Err(err) => {
                        eprintln!("Invalid number: {}, error: {}", args[2], err) ;
                        return;
                    },
                };

                // получить &str из String
                // так:
                let v = &args[1][..] ;
                // или так:
                let v = args[1].as_str() ;
                
                match &args[1][..] {
                    "increase" => increment(number),
                    "decrease" => decrement(number),
                    _ => {
                        eprintln!("Invalid command: {}", &args[1][..]) ;
                        help(&args[1][..]);
                    }
                }
            }
            // all the other cases
            _ => help(&args[0]),
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/ffi.html
    // Foreign Function Interface
    // Rust предоставляет Foreign Function Interface (FFI) для 
    // библиотек C. Внешние функции должны быть объявлены внутри блока 
    // extern, аннотированного атрибутом #[link], содержащим имя внешней 
    // библиотеки.
    {
        use std::fmt ;

        #[derive(Clone, Copy)]
        #[repr(C)]
        struct Complex {
            re: f32,    // real
            im: f32,    // imaginary
        }

        impl fmt::Debug for Complex{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.im < 0. {
                    write!(f, "{}{}i", self.re, self.im)
                }
                //else if self.im == 0. {
                //    write!(f, "{}+{}i", self.re, self.im.abs())
                //}
                else {
                    write!(f, "{}+{}i", self.re, self.im.abs())
                }
            }
        }

        // The msvcrt library (Microsoft Visual C++ Runtime) 
        #[link(name="msvcrt")]
        unsafe extern { // Link to or import external code.
            fn csqrtf(z: Complex) ->Complex ;   // complex sqrt
            fn ccosf(z: Complex) ->Complex ;    // complex cos
        }

        fn my_sqrt(z: Complex) ->Complex {
            unsafe {
                csqrtf(z)
            }
        }

        let mut z = Complex {re: 1., im: 0.} ;

        println!("z: {:?}", z) ;

        println!("my_sqrt({:?}) = {:?}", z, my_sqrt(z)) ;
        
        z = Complex { re: 0., im: 0. } ;    // угол в радианах.
        println!("cos({:?}) = {:?}", z, unsafe {ccosf(z)}) ;
    }


    {
        // функция получения суммы квадратов из вектора целых чисел
        fn sum_of_squares(v: Vec<i32>) -> i32 {
            v.iter().map(|x| x * x).sum()
        }

        #[test]
        fn test_sum_of_squares() {
            let vec = vec![0,1,2,3,4,5,6,7,8,9,10] ;
            assert_eq!(385, sum_of_squares(vec)) ;
        }
    }

    {
        // функция обхода директории и вывод всех файлов и папок
        use std::fs ;
        use std::path::Path ;   // Срез a path (подобный str).

        fn visit_dir(path: &Path) {
            if path.is_dir() {
                for entry in path.read_dir().unwrap() {
                    let entry = entry.unwrap() ;
                    let path = entry.path() ;
                    if path.is_dir() {
                        println!("dir: {:?}", path) ;
                        visit_dir(&path) ;
                    } else if path.is_file() {
                        println!("file: {:?}", path) ;
                    } else if path.is_symlink() {
                        println!("symbol link: {:?}", path) ;
                    }
                }
            }
        }
    }

    // тест чтения директория
    {
        use std::path::Path ;
        use std::io ;

        fn visit_dir(path_in: &Path) ->io::Result<()>{

            if path_in.is_dir() {
                let dir_reader = path_in.read_dir()? ;
                for dir_unit in dir_reader {
                    let dir_entry = dir_unit? ;
                    let dir_en_path = dir_entry.path() ;
                    if dir_en_path.is_dir() {
                        println!("dir: {:?}", dir_en_path) ;
                        visit_dir(&dir_en_path)? ;                        
                    } else if dir_en_path.is_file() {
                        println!("file: {:?}", dir_en_path) ;
                    } else if dir_en_path.is_symlink() {
                        println!("symb.link: {:?}", dir_en_path) ;
                    }
                }
            }

            Ok(())
        }

        visit_dir(&Path::new(
                            //    "..\\..\\"
                            "."
                            )) ;
    }

    // https://doc.rust-lang.org/stable/rust-by-example/testing.html
    // Testing
    // Rust — это язык программирования, который уделяет большое внимание корректности и включает в себя поддержку написания программных тестов непосредственно в языке.
    // 
    // Тестирование бывает трёх типов:
    //      Модульное тестирование (Unit testing.).
    //      Документальное тестирование (Doc testing.).
    //      Интегрированное тестирование (Integration testing.).

    // https://doc.rust-lang.org/stable/rust-by-example/testing/doc_testing.html
    // Documentation testing
    // See file: lib.rs
    
    // https://doc.rust-lang.org/stable/rust-by-example/testing/integration_testing.html
    // Integration testing
    // Модульные тесты проверяют один модуль изолированно за раз: 
    //      они небольшие и могут проверять закрытый код. 
    // Интеграционные тесты находятся вне вашего крейта и используют 
    // только его публичный интерфейс так же, как и любой другой код. 
    // Их цель — проверить, что многие части вашей библиотеки корректно 
    // работают вместе.
    // Cargo ищет интеграционные тесты в каталоге tests рядом с src.
    // See tests\integration_test.rs, Cargo.toml !!!!!!!!!!!!!!!!!!

    // https://doc.rust-lang.org/stable/rust-by-example/testing/dev_dependencies.html
    // Development dependencies - Зависимости для разработки
    // Иногда возникает необходимость в зависимостях только для тестов 
    // (или примеров, или бенчмарков). Такие зависимости добавляются 
    // в Cargo.toml в раздел [dev-dependencies]. Эти зависимости не 
    // распространяются на другие пакеты, которые зависят от этого пакета.
    // Одним из таких примеров является pretty_assertions, который 
    // расширяет стандартные макросы assert_eq! и assert_ne!, 
    // обеспечивая цветное отображение различий. Файл Cargo.toml:
    /*
# standard crate data is left out
[dev-dependencies]
pretty_assertions = "1"
File src/lib.rs:

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}    
     */


    // https://doc.rust-lang.org/stable/rust-by-example/unsafe.html
    // Unsafe Operations
    // `unsafe` используется для четырех основных целей:
    //      разыменование сырых указателей; (dereferencing raw pointers)
    //      вызов небезопасных функций или методов (calling functions or methods which are unsafe )
    //          (включая вызов функции через FFI, см. предыдущую главу книги);
    //      доступ к статическим изменяемым переменным или их изменение; (accessing or modifying static mutable variables)
    //      реализация небезопасных трейтов. (implementing unsafe traits)

    // Raw Pointers
    {
        let my_row: *const u32 = &10 ;

        unsafe {
            assert_eq!(*my_row, 10) ;
        }
    }
    
    // Calling Unsafe Functions
    {
        use std::slice ; // Utilities for the slice primitive type.

        let some_vector = vec![1,2,3,4] ;

        let pointer = some_vector
                                    .as_ptr() // Returns a raw pointer to the vector's buffer
                                    ;
        let length = some_vector.len() ; // Returns the number of elements in the vector

        unsafe {
                                   // unsafe fn VVV
            let my_slice = slice::from_raw_parts( // Forms a slice from a pointer and a length.
                                pointer,
                                 length // The len argument is the number of elements, not the number of bytes.
                                )
                                ;
            assert_eq!(
                some_vector.as_slice(), // Extracts a slice containing the entire vector.
                my_slice,
            ) ;
        }
    }

    // https://doc.rust-lang.org/stable/rust-by-example/unsafe/asm.html
    // Inline assembly - Встроенный ассемблер
    // Rust поддерживает встроенный ассемблер с помощью макроса asm!.

    {
        use std::arch::asm ;    // Inline assembly.

        unsafe {
            asm!("nop") ;
        }

        let x: u64 ;
        unsafe {
            asm!("mov {}, 5", out(reg) x) ;
        }
        println!("x: {}", x) ;

        let i: u64 = 3 ;
        let o: u64 ;
        unsafe {
            asm!(
                "mov {0}, {1}",
                "add {0}, 5",
                out(reg) o,
                in(reg) i,
            ) ;
        }
        println!("o: {}", o) ;

        let mut x: u64 = 3 ;
        unsafe {
            asm!(
                "add {0}, 5",
                inout(reg) x,
            ) ;
        }
        println!("x: {}", x) ;

        let x: u64 = 3 ;
        let y: u64 ;
        unsafe {
            asm!(
                "add {0}, 5",
                inout(reg) x => y,
            ) ;
        }
        println!("y: {}", y) ;

        // продолжить после ознакомления с asm.
    }

    // https://doc.rust-lang.org/stable/rust-by-example/compatibility/raw_identifiers.html
    // Raw identifiers
    // Использование «сырых» идентификаторов позволяет использовать 
    // ключевые слова там, где они обычно не допускаются. Это особенно 
    // полезно, когда в Rust вводятся новые ключевые слова, и библиотека,
    // использующая более старую версию Rust, имеет переменную или 
    // функцию с тем же именем, что и ключевое слово, введенное в более 
    // новой версии.    
    /*
    You can write this with a raw identifier: r#try

extern crate foo;

fn main() {
    foo::r#try();
}
     */

    // https://doc.rust-lang.org/stable/rust-by-example/meta/doc.html
    // Documentation
    // Используйте cargo doc для сборки документации в target/doc, 
    // cargo doc --open автоматически откроет её в вашем веб-браузере.
    //
    // Используйте cargo test для запуска всех тестов (включая тесты 
    // документации), а cargo test --doc — только для запуска тестов 
    // документации.


    // https://doc.rust-lang.org/stable/rust-by-example/meta/playground.html
    // Playground
    // Rust Playground — это платформа для экспериментирования с кодом 
    // на Rust через веб-интерфейс.

    println!("\nAll Ok.") ;
}