use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::IResult;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Match<'a>(Span<'a>);
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Chain<'a>(Span<'a>);
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Defines<'a>(Span<'a>);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Inv,
    Or,
    And,
    Xor,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Range {
        inclusive: bool,
    },
    Shl,
    Shr,
}

pub fn match_(i: &str) -> IResult<&str, Match> {
    let (i, _) = tag("=")(i)?;
    Ok((i, Match))
}

pub fn chain(i: &str) -> IResult<&str, Chain> {
    let (i, _) = tag(".")(i)?;
    Ok((i, Chain))
}

pub fn defines(i: &str) -> IResult<&str, Defines> {
    let (i, _) = tag("->")(i)?;
    Ok((i, Defines))
}

pub fn op(i: &str) -> IResult<&str, Op> {
    let (i, m) = alt((
        tag("..."),
        tag(".."),
        tag("<="),
        tag(">="),
        tag("<<"),
        tag(">>"),
        take_while_m_n(1, 1, |c: char| "+-*/%!|&^<>".contains(c)),
    ))(i)?;

    let op = match m {
        "..." => Op::Range { inclusive: true },
        ".." => Op::Range { inclusive: false },
        ">=" => Op::GtEq,
        "<=" => Op::LtEq,
        ">>" => Op::Shr,
        "<<" => Op::Shl,
        "+" => Op::Add,
        "-" => Op::Sub,
        "*" => Op::Mul,
        "/" => Op::Div,
        "%" => Op::Mod,
        "!" => Op::Inv,
        "|" => Op::Or,
        "&" => Op::And,
        "^" => Op::Xor,
        ">" => Op::Gt,
        "<" => Op::Lt,
        _ => unreachable!(),
    };

    Ok((i, op))
}
