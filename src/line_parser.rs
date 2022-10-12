use nom::{
    bytes::complete::{take, take_until},
    character::complete::newline,
    sequence::tuple,
    IResult,
};

pub fn parse_line(input: &str) -> IResult<&str, &str> {
    let (remainder, (line, newline_char)) = tuple((take_until("\n"), take(1u16)))(input)?;
    Ok((remainder, line))
}
