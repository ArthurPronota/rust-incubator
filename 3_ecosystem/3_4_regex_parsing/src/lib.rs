/* 
   Реализация парзера на основе winnow являющейсяs парзерной 
    комбинаторной библиотекой.
   
   Поск документации по любому элементу winnow 
    на странице https://docs.rs/winnow/latest/winnow/combinator/fn.peek.html

   В файле lib.rs.old старый вариант парзера.

   Запуск тестов:   cargo test 
 */
use winnow::prelude::*; // концепции ядра

use winnow::token::{
                any,        // совпадение с одним токеном
                one_of      // распознаёт токен что совпадает с набором токенов
            };

use winnow::combinator::{
                    opt,    // комбинатор делает парсер опциональным
                    peek,   // комбинатор, который позволяет посмотреть вперед (peek) на следующие данные, не потребляя их. 
                    preceded, // комбинатор который парсит два элемента последовательно, но возвращает только результат второго парсера
                    alt, // комбинатор, который реализует альтернативу (ИЛИ логику) между несколькими парсерами, выбирает первый успешный парзер
                };

use winnow::ascii::{
                digit1, // это парсер, который распознает одну или более ASCII цифр (символы '0'-'9')
                Caseless, // Делает значение как регистро не чуйствительное для ASCII символов
            };

use winnow::error::{
            ContextError, // Накапливайте контекст при возврате к ошибкам.
            ErrMode // Добавить состояние ошибки синтаксического анализа в ParserErrors
        };

/// Определение возвращаемого типа для функции парзера
pub type PResult<O, E = ContextError> = Result<O, ErrMode<E>>;

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

// так:
fn parse_format_spec<'a>(input: &mut &'a str) ->
        PResult<(
                    Option<Sign>,       // sign
                    Option<usize>,      // width
                    Option<Precision>   // precision         
        )> 
{
    //let mut spec = FormatSpec::default();
    let mut out_data = (
                    Option::<Sign>::None,       // sign
                    Option::<usize>::None,      // width
                    Option::<Precision>::None,  // precision         
        ) ;

    // 1. [[fill]align]
    // Пробуем заглянуть вперед: если второй символ это <, >, ^, значит первый - это fill
    let align_chars = ['<','>','^'] ;
    if let Some(_) = opt(
                peek( // это комбинатор, который позволяет посмотреть вперед (peek) на следующие данные, не потребляя их. 
                        preceded( // этот комбинатор который парсит два элемента последовательно, но возвращает только результат второго парсера
                                any, // это парсер, который распознает любой один символ из входных данных.
                                one_of( // это парсер который распознает один символ из заданного набора символов.
                                    align_chars
                                    )
                                )
                            )
                        ).parse_next(input)? {
        
        Some(any // это парсер, который распознает любой один символ из входных данных.
                            .parse_next(input)?
                        );

        Some(one_of( // это парсер который распознает один символ из заданного набора символов.
                        align_chars
                        ).parse_next(input)?
                    );
    } else {
        // Если первого символа нет, возможно есть только align
        opt(   // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
                one_of( // это парсер который распознает один символ из заданного набора символов.
                                align_chars
                            )).parse_next(input)?;
    }

    // 2. [sign] (+ или -)
    if let Some(sign) = opt(    // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
            one_of( // это парсер который распознает один символ из заданного набора символов.
                        ['+', '-']
                    )
                )
                .parse_next(input)
                ? {
        match sign {
            '+' => out_data.0 = Some(Sign::Plus),
            '-' => out_data.0 = Some(Sign::Minus),
            s => panic!("Invalid sign: {}", s),
        }
    }

    // 3. [#] (alternate)
    opt(   // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
            "#"
        )
        .parse_next(input)?
        ;

    // 4. [0] (zero padding)
    opt(    // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
            "0"
        )
        .parse_next(input)? // это основной метод трейта Parser, который выполняет парсинг входных данных.
        ;

    // 5. [width] (цифры)
    match opt(   // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
                digit1  // это парсер, который распознает одну или более ASCII цифр (символы '0'-'9')
                )
                .parse_next(input)  // это основной метод трейта Parser, который выполняет парсинг входных данных.
                {
        Ok(w) => {
            if let Some(w) = w {
                match w.parse::<usize>() {
                    Ok(w) => {
                       out_data.1 = Some(w) ;
                    },
                    Err(err) => panic!("Invalid width: {}, error: {}", w, err),
                } 
            }
        },
        Err(err) => return Err(err),
    }

    // 6. [.precision]
    match opt(   // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
                preceded( // этот комбинатор который парсит два элемента последовательно, но возвращает только результат второго парсера
                ".", // игнорирекм
                 alt( // этот комбинатор, который реализует альтернативу (ИЛИ логику) между несколькими парсерами, выбирает первый успешный парзер
                    ( // кортеж альтернатив
                                ("*", opt("")),
                                // raw_width и опциональный $
                                (digit1, opt("$")),
                            )
                        )
                )
            )
            .parse_next(input)  // это основной метод трейта Parser, который выполняет парсинг входных данных.
            ? {
        Some((prec_one, prec_two) ) => {
            match prec_one {
                "*" => out_data.2 = Some(Precision::Asterisk),
                pr => {
                    match pr.parse::<usize>() {
                        Ok(pr) => {
                            if prec_two.is_some() {
                                out_data.2 = Some(Precision::Argument(pr)) ;
                            }
                            else {
                                out_data.2 = Some(Precision::Integer(pr)) ;
                            }
                        },
                        Err(err) => panic!("Invalid precision: {}, error: {}", pr, err),
                    }
                },
            }
        },
        None => {},
    }

    // 7. [type] ( ?, x?, X?, _, r# )
    opt( // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
        alt( // этот комбинатор, который реализует альтернативу (ИЛИ логику) между несколькими парсерами, выбирает первый успешный парзер
        (   // кортеж альтернатив
                        "?",
                        Caseless(  // Делает значение как регистро не чуйствительное для ASCII символов
                            "x?"
                        ),
                        "_",
                        "r#"
                     )
                   )
                )
                .parse_next(input)  // это основной метод трейта Parser, который выполняет парсинг входных данных.
                ?;

    Ok(out_data)
}

/// разбор входной строки `format_spec`
#[allow(dead_code)]
fn parse(input: &str) -> (
                            Option<Sign>,       // sign
                            Option<usize>,      // width
                            Option<Precision>   // precision 
                        ) {
    let v = input.to_string() ;
    let mut v2 = v.as_str() ;
    match parse_format_spec.parse_next(&mut v2) {
        Ok(spec) => spec,
        Err(e) => panic!("Error: {}", e),
    } 
}

#[cfg(test)]
mod tests {
    use super::* ;

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