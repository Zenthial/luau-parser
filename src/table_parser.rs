use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{
    function_parser::Function,
    identifier_parser::{parse_equals, parse_identifier_value, parse_name, IdentifierValues},
};
#[derive(PartialEq, Debug, Clone)]
pub enum TableMemberType {
    RawType(IdentifierValues),
    NestedTable(Table),
    Function(Function),
    Method(Function),
}
#[derive(PartialEq, Debug, Clone)]
pub struct TableMember {
    pub name: String,
    pub is_a: TableMemberType,
}
#[derive(PartialEq, Debug, Clone)]
pub struct Table {
    pub name: String,
    pub members: Vec<TableMember>,
}

fn make_table_member(name: &str, value: IdentifierValues) -> TableMember {
    TableMember {
        name: name.to_string(),
        is_a: match value {
            IdentifierValues::Table(t) => TableMemberType::NestedTable(t),
            _ => TableMemberType::RawType(value),
        },
    }
}

fn parse_table_member(input: &str) -> IResult<&str, (&str, &str, IdentifierValues, Option<&str>)> {
    tuple((
        parse_name,
        parse_equals,
        parse_identifier_value,
        opt(tag(",")),
    ))(input)
}

pub fn parse_table(input: &str) -> IResult<&str, Table> {
    let (remainder, values) = delimited(
        tag("{"),
        many0(map(
            delimited(multispace0, parse_table_member, multispace0),
            |(s, _, i, _)| make_table_member(s, i),
        )),
        tag("}"),
    )(input)?;

    Ok((
        remainder,
        Table {
            name: String::new(),
            members: values,
        },
    ))
}
