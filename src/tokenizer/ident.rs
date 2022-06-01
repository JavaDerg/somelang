use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::combinator::recognize;
use nom::sequence::tuple;
use nom::IResult;
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

pub fn ident(i: Span) -> IResult<Span, Span> {
    recognize(tuple((
        take_while1(|c: char| c == '_' || c.is_alphabetic()),
        take_while(|c: char| c == '_' || c.is_alphanumeric()),
    )))(i)
}

pub fn generic(i: Span) -> IResult<Span, Span> {
    let (i, _) = tag("'")(i)?;
    ident(i)
}
