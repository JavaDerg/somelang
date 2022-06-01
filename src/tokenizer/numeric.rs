use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{opt, recognize};
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[derive(Eq, PartialEq, Debug)]
pub struct Numeric {
    pub sign: Option<Sign>,
    pub digits: String,
    pub suffix: Option<String>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Sign {
    Positive,
    Negative,
}

pub fn numeric(i: &str) -> IResult<&str, Numeric> {
    let (i, sign) = opt(sign)(i)?;
    let (i, digits) = digits(i)?;
    let (i, suffix) = opt(suffix)(i)?;

    Ok((
        i,
        Numeric {
            sign,
            digits: digits.to_string(),
            suffix: suffix.map(|s| s.to_string()),
        },
    ))
}

fn suffix(i: &str) -> IResult<&str, &str> {
    recognize(alt((
        tuple((
            alt((tag("u"), tag("i"))),
            alt((tag("8"), tag("16"), tag("32"), tag("64"))),
        )),
        tuple((tag("f"), alt((tag("32"), tag("64"))))),
    )))(i)
}

fn digits(i: &str) -> IResult<&str, &str> {
    recognize(tuple((
        take_while1(char::is_numeric),
        opt(preceded(tag("."), take_while1(char::is_numeric))),
    )))(i)
}

fn sign(i: &str) -> IResult<&str, Sign> {
    let (i, s) = alt((tag("+"), tag("-")))(i)?;
    Ok((
        i,
        if s == "+" {
            Sign::Positive
        } else {
            Sign::Negative
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::numeric::{numeric, Numeric, Sign};
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn sign() {
        assert_eq!(
            numeric("+123"),
            Ok((
                "",
                Numeric {
                    sign: Some(Sign::Positive),
                    digits: "123".to_string(),
                    suffix: None
                }
            ))
        );
        assert_eq!(
            numeric("-123"),
            Ok((
                "",
                Numeric {
                    sign: Some(Sign::Negative),
                    digits: "123".to_string(),
                    suffix: None
                }
            ))
        );
    }

    #[test]
    fn normal() {
        assert_eq!(
            numeric("123"),
            Ok((
                "",
                Numeric {
                    sign: None,
                    digits: "123".to_string(),
                    suffix: None
                }
            ))
        );
        assert_eq!(
            numeric("123.321"),
            Ok((
                "",
                Numeric {
                    sign: None,
                    digits: "123.321".to_string(),
                    suffix: None
                }
            ))
        );
    }
}
