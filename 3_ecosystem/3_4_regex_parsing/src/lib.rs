/*
use winnow::prelude::*;

use winnow::token::{
                any,
                //alt, 
                take_while,
                one_of
            };
use winnow::combinator::{
                    opt,
                    peek,
                    dispatch,
                    fail,
                    preceded,
                    terminated,
                    alt,            // add
                };
use winnow::ascii::{
                digit1,
                //caseless,
                Caseless,       // add
            };

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
    let align_chars = "<>^";
    if let Some(_) = opt(peek(preceded(any, one_of(align_chars)))).parse_next(input)? {
        spec.fill = Some(any.parse_next(input)?);
        spec.align = Some(one_of(align_chars).parse_next(input)?);
    } else {
        // Если первого символа нет, возможно есть только align
        spec.align = opt(one_of(align_chars)).parse_next(input)?;
    }

    // 2. [sign] (+ или -)
    spec.sign = opt(one_of("+-")).parse_next(input)?;

    // 3. [#] (alternate)
    spec.alternate = opt("#").parse_next(input)?.is_some();

    // 4. [0] (zero padding)
    spec.zero_pad = opt("0").parse_next(input)?.is_some();

    // 5. [width] (цифры)
    spec.width = opt(digit1).parse_next(input)?;

    // 6. [.precision]
    spec.precision = opt(preceded(
        ".",
        alt((
            "*",
            // raw_width и опциональный $
            terminated(digit1, opt("$"))
        ))
    )).parse_next(input)?;

    // 7. [type] ( ?, x?, X?, _, r# )
    spec.ty = opt(alt((
        "?",
        caseless("x?"),
        "_",
        "r#"
    ))).parse_next(input)?;

    Ok(spec)
}
*/

/*
fn main() {
    let mut input = "0>+#010.5r#";
    match parse_format_spec.parse_next(&mut input) {
        Ok(spec) => println!("{:#?}", spec),
        Err(e) => eprintln!("Error: {}", e),
    }
}
 */