/*
   Реализация парзера на основе regex.

   Документация по winnow:
   https://docs.rs/winnow/latest/winnow/_tutorial/chapter_1/index.html

   Запуск тестов:   cargo test 
 */

// Скомпилированное регулярное выражение для поиска в Unicode стогах.
use regex::Regex ;
use winnow::Parser;
// Примитив синхронизации, в который номинально можно записать данные только один раз.
use std::sync::OnceLock ;

fn main() {
    //println!("Implement me!");

    let res = 
            parse(
                //" +10",
                //" +10$",
                //" +10.",
                //" +10.10",
                // " +10.*",
                //" +10.15$"

                //" -10"
                // "abc"
                //"-8x?",
                //"-8X?",
                //"-8r#",
                "-8_"
            ) ;
    println!("res: {:?}", res) ;

    {   // Глава 1: https://docs.rs/winnow/latest/winnow/_tutorial/chapter_1/index.html

        // pub type Result<O, E = ContextError> = core::result::Result<O, E>
        use winnow::Result;
        // корневой trait для parsing
        use winnow::Parser ;
        pub fn do_nothing_parser<'s>(input: &mut &'s str) -> Result<&'s str> {
            Ok("")
        }

        let mut input = "0x1a2b Hello";

        let output = 
                do_nothing_parser
        /*
        Извлекать токены из потока, преобразуя их в выходные данные.
        Это включает в себя продвижение входного потока к следующему местоположению.
        В случае ошибки входные данные будут указывать на место ошибки.                    
        */
                    .parse_next(&mut input)
                    .unwrap()
                    ;

        assert_eq!(input, "0x1a2b Hello");
        assert_eq!(output, "");
    }

    // Глава 2: https://docs.rs/winnow/latest/winnow/_tutorial/chapter_2/index.html
    {
        use winnow::stream::Stream;
        use winnow::error::ParserError;
        use winnow::Result;

        fn parse_prefix(input: &mut &str) -> Result<char> {

            let c = input
                            // Откалывает следующий токен от input
                            .next_token()
                            // Трансформирует Option<T> в Result<T, E>, отображая Some(v) в Ok(v) и None в Err(err())
                            .ok_or_else(|| {
                                ParserError::from_input(input)  // Создаёт ошибку от input position
                            })?;

            if c != '0' {
                return Err(ParserError::from_input(input));
            }
            Ok(c)
        }        

        let mut input = "0x1a2b Hello";

        let output = parse_prefix

                            /* 
                            Извлекать токены из потока, преобразуя их в выходные данные.
                            Это включает в себя продвижение входного потока к следующему местоположению.            
                            */
                            .parse_next(&mut input)
                            .unwrap()
                            ;

        assert_eq!(input, "x1a2b Hello");
        assert_eq!(output, '0');

        assert!(parse_prefix.parse_next(&mut "d").is_err());

    }

    {
        use winnow::Parser;
        use winnow::token::any; // Соответствует одному токену
        use winnow::error::ParserError;
        use winnow::Result;

        fn parse_prefix(input: &mut &str) -> Result<char> {
            let c = any
                /* 
                 Извлекать токены из потока, преобразуя их в выходные данные.
                 Это включает в себя продвижение входного потока к следующему местоположению.
                 */
                .parse_next(input)?;
            if c != '0' {
                return Err(ParserError::from_input(input));
            }
            Ok(c)
        }        
    }

    {
        use winnow::Parser;
        use winnow::token::any;
        use winnow::Result;

        fn parse_prefix(input: &mut &str) -> Result<char> {
            let c = any
                    /*
                     Возвращает результат работы дочернего парсера, если он удовлетворяет 
                     функции проверки.
                      */
                    .verify(|c| *c == '0')
                    /* 
                     Извлекать токены из потока, преобразуя их в выходные данные.
                     Это включает в себя продвижение входного потока к следующему местоположению.
                     */
                    .parse_next(input)
                    ?;
            Ok(c)
        }
    }

    {
        use winnow::Parser;
        use winnow::Result;

        fn parse_prefix(input: &mut &str) -> Result<char> {
            let c = '0'
                    /* 
                     Извлекать токены из потока, преобразуя их в выходные данные.
                     Это включает в себя продвижение входного потока к следующему местоположению.
                     */            
                            .parse_next(input)?;
            Ok(c)
        }        
    }

    {
        use winnow::stream::Stream;
        use winnow::error::ParserError;
        use winnow::Result;

        fn parse_prefix<'s>(input: &mut &'s str) -> Result<&'s str> {
            let expected = "0x";
            if input.len() < expected.len() {
                return Err(ParserError::from_input(input));
            }
            let actual = input
                                .next_slice(expected.len())
                                ;
            if actual != expected {
                return Err(ParserError::from_input(input));
            }
            Ok(actual)
        }


        let mut input = "0x1a2b Hello";

        let output = parse_prefix
                            /* 
                            Извлекать токены из потока, преобразуя их в выходные данные.
                            Это включает в себя продвижение входного потока к следующему местоположению.
                            */
                            .parse_next(&mut input)
                            .unwrap();
        assert_eq!(input, "1a2b Hello");
        assert_eq!(output, "0x");

        assert!(parse_prefix.parse_next(&mut "0o123").is_err());
    }

    {
        use winnow::token::literal;
        use winnow::Result;

        fn parse_prefix<'s>(input: &mut &'s str) -> Result<&'s str> {
            let expected = "0x";
            let actual = literal(expected) // Распознаёт литерал
                                /* 
                                Извлекать токены из потока, преобразуя их в выходные данные.
                                Это включает в себя продвижение входного потока к следующему местоположению.
                                */
                                .parse_next(input)
                                ?;
            Ok(actual)
        }        
    }

    {
        use winnow::Parser;
        use winnow::Result;

        fn parse_prefix<'s>(input: &mut &'s str) -> Result<&'s str> {
            let actual = "0x"
                                /* 
                                Извлекать токены из потока, преобразуя их в выходные данные.
                                Это включает в себя продвижение входного потока к следующему местоположению.
                                */            
                                .parse_next(input)
                                ?;
            Ok(actual)
        }        
    }

    {
        // Распознать токен, соответствующий набору токенов.
        use winnow::token::one_of;
        use winnow::Result;

        fn parse_digits(input: &mut &str) -> Result<char> {
            one_of(('0'..='9', 'a'..='f', 'A'..='F'))
                /* 
                 Извлекать токены из потока, преобразуя их в выходные данные.
                 Это включает в себя продвижение входного потока к следующему местоположению.
                 */
                .parse_next(input)
        }

        let mut input = "1a2b Hello";

        let output = parse_digits.parse_next(&mut input).unwrap();
        assert_eq!(input, "a2b Hello");
        assert_eq!(output, '1');

        assert!(parse_digits.parse_next(&mut "Z").is_err());
    }

    {
        use winnow::token::take_while;
        use winnow::Result;

        fn parse_digits<'s>(input: &mut &'s str) -> Result<&'s str> {
            /* 
             Распознать самый длинный (m <= len <= n) входной фрагмент, 
             соответствующий набору токенов.
            */
            take_while(1.., ('0'..='9', 'a'..='f', 'A'..='F'))
                .parse_next(input)
        }

        let mut input = "1a2b Hello";

        let output = parse_digits.parse_next(&mut input).unwrap();
        assert_eq!(input, " Hello");
        assert_eq!(output, "1a2b");

        assert!(parse_digits.parse_next(&mut "Z").is_err());
        
    }

    // Глава 3: https://docs.rs/winnow/latest/winnow/_tutorial/chapter_3/index.html

    {
        use winnow::token::take_while;
        use winnow::Result;

        fn parse_prefix<'s>(input: &mut &'s str) -> Result<&'s str> {
            "0x"
                .parse_next(input)
        }

        fn parse_digits<'s>(input: &mut &'s str) -> Result<&'s str> {
            take_while(
                1.., // 1 или более (как +)
                (
                    ('0'..='9'),
                    ('A'..='F'),
                    ('a'..='f'),
                )
            ).parse_next(input)
        }

        // -------------------------------

        let mut input = "0x1a2b Hello";

        let prefix = parse_prefix
                            .parse_next(&mut input)
                            .unwrap();
        let digits = parse_digits
                            .parse_next(&mut input)
                            .unwrap();

        assert_eq!(prefix, "0x");
        assert_eq!(digits, "1a2b");
        assert_eq!(input, " Hello");


        let mut input = "0x1a2b Hello";

        let (prefix, digits) = (
                parse_prefix,
                parse_digits
            )
            .parse_next(&mut input)
            .unwrap();

        assert_eq!(prefix, "0x");
        assert_eq!(digits, "1a2b");
        assert_eq!(input, " Hello");

        // -------------------------------

        use winnow::combinator::preceded;

        let mut input = "0x1a2b Hello";

        let digits = preceded(
                    parse_prefix,
                     parse_digits
                    )
                    .parse_next(&mut input)
                    .unwrap();

        assert_eq!(digits, "1a2b");
        assert_eq!(input, " Hello");
    }

    // Глава 4. https://docs.rs/winnow/latest/winnow/_tutorial/chapter_4/index.html
    {
        use winnow::Result;
        use winnow::ascii::digit1;

        fn parse_digits(input: &mut &str) -> Result<usize> {
            digit1
                .parse_to::<usize>()
                .parse_next(input)
        }

        let mut input = "1024 Hello";

        let output = parse_digits.parse_next(&mut input).unwrap();
        assert_eq!(input, " Hello");
        assert_eq!(output, 1024);

        assert!(parse_digits(&mut "Z").is_err());

    }

    {
        use winnow::Result;
        use winnow::combinator::opt;
        use winnow::combinator::terminated;
        use winnow::ascii::digit1;

        fn parse_digits(input: &mut &str) -> Result<usize> {
            digit1
                .parse_to::<usize>()
                .parse_next(input)
        }

        fn parse_list(input: &mut &str) -> Result<Vec<usize>> {
            let mut list = Vec::new();
            while let Some(output) = opt(
                                terminated(
                                        parse_digits,
                                       opt(',')
                                    )
                                )
                                .parse_next(input)? {
                list.push(output);
            }
            Ok(list)
        }

        let mut input = "1,2,3,4,5 Hello" ;

        let digits = parse_list.parse_next(&mut input).unwrap();

        assert_eq!(input, " Hello");
        assert_eq!(digits, vec![1usize, 2, 3, 4, 5]);

        assert!(parse_digits(&mut "ghiWorld").is_err());
    }

    
}

/// разбор входной строки `format_spec`
fn parse(input: &str) -> (
                            Option<Sign>,       // sign
                            Option<usize>,      // width
                            Option<Precision>   // precision 
                        ) {
    static RE: OnceLock<Regex> = OnceLock::new() ;

    // Получает содержимое ячейки, инициализируя его значением f(), если ячейка не была инициализирована.
    let re = RE.get_or_init(|| {
        Regex::new(
            r"(?x)      # Игнорировать пробелы и комментарии
        (?:[^<>\^]?[<>\^])? # [[fill]align], Незахватывающая группа,
                            # [fill] > [^<>\^]? любой символ кроме <>^ символ не обязательный
                            # align > [<>\^] любой из символов <>^ ,символа может не быть
        (?P<sign>[\+-])?   # [sign] > [+-] Именованная группа sign, символ + или - ,символа может не быть
        \#?                 # ['#'] Флаг, захват не нужен
        0?                  # ['0'] Флаг, захват не нужен
        (?P<width>\d+)?     # [width]  Ширина
        (?:\.(?P<precision>(\*|(?P<raw_width>\d+)(?P<end_width>\$)?)))?    # ['.' precision] Незахватывающая группа, Точность
        (?:(\?|x\?|X\?|_|r\#))?  # type, Незахватывающая группа
            "
        )
        .unwrap()
    }) ;

    let mut sign_out = Option::<Sign>::None ;
    let mut width_out= Option::<usize>::None ;
    let mut precision_out = Option::<Precision>::None ;

    // прощедура ищет первое совпадение с данным регулярным выражением
    if let Some(data) = re.captures(input) {
        // Возвращает объект Match, связанный с группой захвата с именем "sign".
        if let Some(m) = data.name("sign") {
           sign_out = match m.as_str() {
               "+" => Some(Sign::Plus),
               "-" => Some(Sign::Minus),
               _ => Option::<Sign>::None,
           } ;
        }

        // Возвращает объект Match, связанный с группой захвата с именем "width".
        if let Some(m) = data.name("width") {
            width_out = match m.as_str().parse::<usize>() {
             Ok(w)   => Some(w),
             Err(err) => panic!("cannot convert {} to usize, error: {}", m.as_str(), err),
            }
        }

        // Возвращает объект Match, связанный с группой захвата с именем "precision".
        if let Some(m) = data.name("precision") {
            precision_out = match m.as_str() {
               "*" => Some(Precision::Asterisk),
               _ => {
                    // получаем raw width
                    let w = match data.name("raw_width") {
                        Some(m) => {
                            match m.as_str().parse::<usize>() {
                                Ok(w) => w,
                                Err(err) => panic!("Invalid raw_width: {}, error: {}", m.as_str(), err),
                            }
                        },
                        None => panic!("Not found raw_width"),
                    } ;

                    // определеяем width
                    match data.name("end_width") {
                      Some(_) => Some(Precision::Argument(w)),
                      None => Some(Precision::Integer(w)),
                    }
                }
            } ;
        }
    }

    (sign_out, width_out, precision_out)
}

/// знак (+-)
#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

/// точность
#[derive(Debug, PartialEq)]
enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse(input);
            assert_eq!(width, expected);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse(input);
            assert_eq!(precision, expected);
        }
    }
}
