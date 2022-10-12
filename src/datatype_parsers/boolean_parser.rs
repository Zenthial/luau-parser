use nom::{branch::alt, bytes::complete::tag_no_case, combinator::map, IResult};

pub fn parse_boolean(input: &str) -> IResult<&str, bool> {
    alt((
        map(tag_no_case("true"), |_| true),
        map(tag_no_case("false"), |_| false),
    ))(input)
}
