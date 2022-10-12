use crate::{
    datatype_parsers::{boolean_parser::parse_boolean, number_parser::parse_number},
    function_parser::{parse_function, Function},
    table_parser::{parse_table, Table},
};

use super::datatype_parsers::string_parser::parse_string;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::{map, not, opt, recognize},
    multi::many0_count,
    sequence::{pair, tuple},
    IResult,
};

#[derive(PartialEq, Debug, Clone)]
pub enum IdentifierValues {
    Number(f32),
    String(String),
    Bool(bool),
    Table(Table),
    Function(Function),
    End,
    If,
    Then,
    While,
    Do,
    Return(String),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub value: IdentifierValues,
}

pub fn parse_equals(input: &str) -> IResult<&str, &str> {
    recognize(many0_count(alt((tag(" "), tag("=")))))(input)
}

pub fn parse_return(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        tuple((tag("return"), multispace0)),
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        )),
    ))(input)
}

pub fn parse_name(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        not(tag("function")),
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        )),
    ))(input)
}

pub fn parse_local(input: &str) -> IResult<&str, (&str, &str)> {
    pair(tag("local"), tag(" "))(input)
}

fn identifier_name(input: &str) -> IResult<&str, &str> {
    let (remainder, (identifier_name, _)) = tuple((parse_name, parse_equals))(input)?;

    Ok((remainder, identifier_name))
}

pub fn parse_identifier_value(input: &str) -> IResult<&str, IdentifierValues> {
    alt((
        map(parse_string, |s| IdentifierValues::String(s)),
        map(parse_number, |n| IdentifierValues::Number(n)),
        map(parse_boolean, |b| IdentifierValues::Bool(b)),
        map(parse_table, |t| IdentifierValues::Table(t)),
        map(parse_function, |f| IdentifierValues::Function(f)),
        map(parse_return, |ret_str| {
            IdentifierValues::Return(ret_str.to_string())
        }),
    ))(input)
}

pub fn parse_identifier(input: &str) -> IResult<&str, Identifier> {
    let (remainder, (_, name, value, _)) = tuple((
        opt(parse_local),
        opt(identifier_name),
        parse_identifier_value,
        opt(multispace0),
    ))(input)?;

    match value {
        IdentifierValues::Table(mut t) => {
            t.name = name.unwrap().to_string();
            Ok((
                remainder,
                Identifier {
                    name: name.unwrap().to_string(),
                    value: IdentifierValues::Table(t),
                },
            ))
        }
        IdentifierValues::Function(func) => Ok((
            remainder,
            Identifier {
                name: func.name.clone(),
                value: IdentifierValues::Function(func),
            },
        )),
        IdentifierValues::Return(str) => Ok((
            remainder,
            Identifier {
                name: String::from("Return"),
                value: IdentifierValues::Return(str),
            },
        )),
        _ => Ok((
            remainder,
            Identifier {
                name: name.unwrap().to_string(),
                value,
            },
        )),
    }
}
