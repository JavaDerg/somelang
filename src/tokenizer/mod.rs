use nom::IResult;
use nom_locate::LocatedSpan;
use crate::tokenizer::numeric::Numeric;
use crate::tokenizer::ops::{Defines, Match, Op};

mod ident;
mod numeric;
mod ops;

pub fn tokenize(source: &str) -> Vec<LexicalToken> {
    todo!()
}

pub enum LexicalToken<'a> {
    Ident(LocatedSpan<&'a str>),
    Generic(LocatedSpan<&'a str>),
    Numeric(Numeric),
    Match(Match),
    Defines(Defines),
    Op(Op),
}
