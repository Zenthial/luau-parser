use crate::number_parser::parse_number;

use super::string_parser::parse_string;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, recognize},
    multi::many0_count,
    sequence::{pair, tuple},
    IResult,
};

#[derive(PartialEq, Debug)]
pub enum IdentifierValues {
    Number(f32),
    String(String),
}

pub struct Identifier {
    pub name: String,
    pub value: IdentifierValues,
}

pub fn identifier_name(input: &str) -> IResult<&str, &str> {
    let (remainder, ((_, _), identifier_name, _)) = tuple((
        pair(tag("local"), tag(" ")),
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        )),
        recognize(many0_count(alt((tag(" "), tag("="))))),
    ))(input)?;

    Ok((remainder, identifier_name))
}

pub fn parse_identifier(input: &str) -> IResult<&str, Identifier> {
    let (remainder, name) = identifier_name(input)?;
    let (remainder_2, value) = alt((
        map(parse_string, |s| IdentifierValues::String(s)),
        map(parse_number, |n| IdentifierValues::Number(n)),
    ))(remainder)?;

    Ok((
        remainder_2,
        Identifier {
            name: name.to_string(),
            value,
        },
    ))
}
