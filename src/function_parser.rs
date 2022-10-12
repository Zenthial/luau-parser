use std::{collections::HashMap, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::{map, opt, recognize},
    multi::{many0, many0_count},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use crate::{
    identifier_parser::{parse_identifier, Identifier},
    types::Types,
};

#[derive(Debug, PartialEq)]
pub struct FunctionArguments {
    pub name: String,
    pub function_type: Types,
}

#[derive(PartialEq, Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Types,
    pub arguments: Vec<FunctionArguments>,
    pub identifiers: HashMap<String, Identifier>,
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
                    function_type: func_type,
                }),
                Err(_) => function_arguments.push(FunctionArguments {
                    name: arg_name,
                    function_type: Types::Any,
                }),
            };
        } else {
            function_arguments.push(FunctionArguments {
                name: arg.trim().to_string(),
                function_type: Types::Any,
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
    let (remainder, ((_, _), identifier_name, args, return_type, _end_of_line)) = tuple((
        pair(tag("function"), tag(" ")),
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        )),
        preceded(tag("("), argument_possibilities),
        opt(tuple((alt((tag(":"), tag(": "))), alpha1))),
        opt(multispace0),
    ))(input)?;

    let mut func = Function {
        name: identifier_name.to_string(),
        return_type: Types::Any,
        arguments: parse_arguments(args),
        identifiers: HashMap::new(),
    };

    match return_type {
        Some((_, s)) => {
            func.return_type = match Types::from_str(s) {
                Ok(t) => t,
                Err(_) => Types::Any,
            };
        }
        None => {}
    };

    Ok((remainder, func))
}

pub fn parse_function(input: &str) -> IResult<&str, Function> {
    let (remainder, mut function) = parse_function_definition(input)?;
    let (remainder, identifiers) = many0(parse_identifier)(remainder)?;

    for identifier in identifiers {
        function
            .identifiers
            .insert(identifier.name.clone(), identifier);
    }

    Ok((remainder, function))
}
