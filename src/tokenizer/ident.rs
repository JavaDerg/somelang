use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::combinator::recognize;
use nom::sequence::tuple;
use nom::IResult;

pub struct Literal(String);

pub fn ident(i: &str) -> IResult<&str, Literal> {
    let (i, l) = recognize(tuple((
        take_while1(|c: char| c == '_' || c.is_alphabetic()),
        take_while(|c: char| c == '_' || c.is_alphanumeric()),
    )))(i)?;
    Ok((i, Literal(l.to_string())))
}
