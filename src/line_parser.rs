use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::multispace0,
    multi::many0_count,
    sequence::{delimited, tuple},
    IResult,
};

pub fn parse_line(input: &str) -> IResult<&str, &str> {
    let (remainder, (line, _newline_char)) = delimited(
        multispace0,
        tuple((take_until("\n"), take(1u16))),
        multispace0,
    )(input)?;

    Ok((remainder, line))
}

pub fn blank_line(input: &str) -> IResult<&str, usize> {
    many0_count(alt((multispace0, tag("\n"))))(input)
}
