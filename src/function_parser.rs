use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::{map, recognize},
    multi::{many0, many0_count},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use crate::{identifier_parser::Identifier, types::Types};

#[derive(Debug, PartialEq)]
pub struct FunctionArguments {
    pub name: String,
    pub function_type: Option<Types>,
}

#[derive(PartialEq, Debug)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<FunctionArguments>,
    pub identifiers: Vec<Identifier>,
}

fn parse_arguments(args: Vec<String>) -> Vec<FunctionArguments> {
    let mut function_arguments = Vec::new();

    for arg in args {
        if arg.contains(":") {
            let split: Vec<String> = arg.split(":").map(|s| s.to_string()).collect();
            let arg_name = split.get(0).unwrap().trim().to_string();
            let arg_type = split.get(1).unwrap();
            let trimmed_type = arg_type.trim().to_lowercase();
            let function_argument = Types::from_str(&trimmed_type);

            match function_argument {
                Ok(func_type) => function_arguments.push(FunctionArguments {
                    name: arg_name,
                    function_type: Some(func_type),
                }),
                Err(_) => function_arguments.push(FunctionArguments {
                    name: arg_name,
                    function_type: Some(Types::Any),
                }),
            };
        } else {
            function_arguments.push(FunctionArguments {
                name: arg.trim().to_string(),
                function_type: Some(Types::Any),
            });
        }
    }

    return function_arguments;
}

fn argument_parser(input: &str) -> IResult<&str, &str> {
    alt((alphanumeric1, tag("_"), tag(": "), tag(":")))(input)
}

fn argument_possibilities(input: &str) -> IResult<&str, Vec<String>> {
    many0(alt((
        terminated(
            delimited(
                multispace0,
                map(many0(argument_parser), |s: Vec<&str>| {
                    s.iter().map(|e| e.to_string()).collect::<String>()
                }),
                multispace0,
            ),
            tag(","),
        ),
        terminated(
            delimited(
                multispace0,
                map(many0(argument_parser), |s: Vec<&str>| {
                    s.iter().map(|e| e.to_string()).collect::<String>()
                }),
                multispace0,
            ),
            tag(")"),
        ),
    )))(input)
}

pub fn parse_function_definition(input: &str) -> IResult<&str, Function> {
    let (remainder, ((_, _), identifier_name, args)) = tuple((
        pair(tag("function"), tag(" ")),
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        )),
        preceded(tag("("), argument_possibilities),
    ))(input)?;

    Ok((
        remainder,
        Function {
            name: identifier_name.to_string(),
            arguments: parse_arguments(args),
            identifiers: Vec::new(),
        },
    ))
}
