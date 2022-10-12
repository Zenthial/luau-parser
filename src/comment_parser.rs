use nom::{
    bytes::complete::{tag, take, take_until},
    sequence::{preceded, tuple},
    IResult,
};

pub fn parse_comment_line(input: &str) -> IResult<&str, &str> {
    let (remainder, (comment_line, _)) =
        preceded(tag("--"), tuple((take_until("\n"), take(1u16))))(input)?;

    Ok((remainder, comment_line))
}

pub fn parse_comment_block(input: &str) -> IResult<&str, &str> {
    let (remainder, (comment_line, _)) =
        preceded(tag("--[["), tuple((take_until("]]"), take(2u16))))(input)?;

    Ok((remainder, comment_line))
}
