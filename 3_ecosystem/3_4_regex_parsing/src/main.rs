// Скомпилированное регулярное выражение для поиска в Unicode стогах.
use regex::Regex ;
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

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

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
