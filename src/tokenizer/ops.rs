use nom::bytes::complete::tag;
use nom::IResult;

pub struct Defines;
pub enum Ops {

}

pub fn defines(i: &str) -> IResult<&str, Defines> {
    let (i, _) = tag("->")(i)?;
    Ok((i, Defines))
}
