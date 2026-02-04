
use winnow::prelude::*;

use winnow::token::{
                any,
                one_of
            };

use winnow::combinator::{
                    opt,
                    peek,
                    preceded,
                    terminated,
                    alt,
                };

use winnow::ascii::{
                digit1,
                Caseless,
            };

use winnow::error::{ContextError, ErrMode};

pub type PResult<O, E = ContextError> = Result<O, ErrMode<E>>;

#[derive(Debug, Default)]
pub struct FormatSpec<'a> {
    pub fill: Option<char>,
    pub align: Option<char>,
    pub sign: Option<char>,
    pub alternate: bool,
    pub zero_pad: bool,
    pub width: Option<&'a str>,
    pub precision: Option<&'a str>,
    pub ty: Option<&'a str>,
}

fn parse_format_spec<'a>(input: &mut &'a str) -> PResult<FormatSpec<'a>> {
    let mut spec = FormatSpec::default();

    // 1. [[fill]align]
    // Пробуем заглянуть вперед: если второй символ это <, >, ^, значит первый - это fill
    let align_chars = ['<','>','^'] ;
    if let Some(_) = opt(
                peek(
                        preceded(
                                any, 
                                one_of(
                                    align_chars
                                    )
                                )
                            )
                        ).parse_next(input)? {
        spec.fill = Some(any.parse_next(input)?);
        spec.align = Some(one_of(
                        align_chars
                        ).parse_next(input)?);
    } else {
        // Если первого символа нет, возможно есть только align
        spec.align = opt(one_of(
                                //align_chars
                                ['<','>','^']
                            )).parse_next(input)?;
    }

    // 2. [sign] (+ или -)
    spec.sign = opt(
            one_of(
                        //"+-"
                        ['+', '-']
                    )
                )
                .parse_next(input)
                ?;

    // 3. [#] (alternate)
    spec.alternate = opt(   // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
                "#"
                    )
                    .parse_next(input)?
                    .is_some();

    // 4. [0] (zero padding)
    spec.zero_pad = opt(    // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
                "0"
                    )
                    .parse_next(input)? // это основной метод трейта Parser, который выполняет парсинг входных данных.
                    .is_some();

    // 5. [width] (цифры)
    spec.width = opt(   // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
                digit1  // это парсер, который распознает одну или более ASCII цифр (символы '0'-'9')
                )
                .parse_next(input)  // это основной метод трейта Parser, который выполняет парсинг входных данных.
                ?;

    // 6. [.precision]
    spec.precision = opt(   // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
                preceded( //  этот комбинатор который парсит два элемента последовательно, но возвращает только результат второго парсера
                ".", // игнорирекм
                alt( // этот комбинатор, который реализует альтернативу (ИЛИ логику) между несколькими парсерами, выбирает первый успешный парзер
                    ( // кортеж альтернатив
                                "*",
                                // raw_width и опциональный $
                                terminated( // этот комбинатор который парсит два элемента последовательно, но возвращает только результат первого парсера, игнорируя результат второго. Это противоположность preceded.
                                    digit1, // это парсер, который распознает одну или более ASCII цифр (символы '0'-'9')
                                    opt("$")
                                )
                            )
                        )
                )
            )
            .parse_next(input)  // это основной метод трейта Parser, который выполняет парсинг входных данных.
            ?;

    // 7. [type] ( ?, x?, X?, _, r# )
    spec.ty = opt( // этот комбинатор делает парсер опциональным, если парсер не срабатывает возвращает None
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

    Ok(spec)
}


/*
fn main() {
    let mut input = "0>+#010.5r#";
    match parse_format_spec.parse_next(&mut input) {
        Ok(spec) => println!("{:#?}", spec),
        Err(e) => eprintln!("Error: {}", e),
    }
}
*/

