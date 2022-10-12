use crate::{
    datatype_parsers::{boolean_parser::parse_boolean, number_parser::parse_number},
    table_parser::{parse_table, Table},
};

use super::datatype_parsers::string_parser::parse_string;

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
    Bool(bool),
    Table(Table),
}

#[derive(PartialEq, Debug)]
pub struct Identifier {
    pub name: String,
    pub value: IdentifierValues,
}

pub fn parse_equals(input: &str) -> IResult<&str, &str> {
    recognize(many0_count(alt((tag(" "), tag("=")))))(input)
}

pub fn parse_name(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

pub fn parse_local(input: &str) -> IResult<&str, (&str, &str)> {
    pair(tag("local"), tag(" "))(input)
}

fn identifier_name(input: &str) -> IResult<&str, &str> {
    let (remainder, ((_, _), identifier_name, _)) =
        tuple((parse_local, parse_name, parse_equals))(input)?;

    Ok((remainder, identifier_name))
}

pub fn parse_identifier_value(input: &str) -> IResult<&str, IdentifierValues> {
    alt((
        map(parse_string, |s| IdentifierValues::String(s)),
        map(parse_number, |n| IdentifierValues::Number(n)),
        map(parse_boolean, |b| IdentifierValues::Bool(b)),
        map(parse_table, |t| IdentifierValues::Table(t)),
    ))(input)
}

pub fn parse_identifier(input: &str) -> IResult<&str, Identifier> {
    let (remainder, name) = identifier_name(input)?;
    let (remainder_2, value) = parse_identifier_value(remainder)?;

    match value {
        IdentifierValues::Table(mut t) => {
            t.name = name.to_string();
            Ok((
                remainder_2,
                Identifier {
                    name: name.to_string(),
                    value: IdentifierValues::Table(t),
                },
            ))
        }
        _ => Ok((
            remainder_2,
            Identifier {
                name: name.to_string(),
                value,
            },
        )),
    }
}
