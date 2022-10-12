#![allow(unused)]

mod identifiers;
mod number_parser;
mod string_parser;

use identifiers::identifier_name;
use number_parser::parse_number;
use std::sync::mpsc::RecvError;
use string_parser::parse_string;

use nom::character::complete::multispace1;
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, one_of},
    combinator::recognize,
    multi::many0_count,
    multi::{many0, many1},
    sequence::{pair, tuple},
    sequence::{preceded, terminated},
    IResult,
};

#[cfg(test)]
mod tests {
    use crate::{
        identifier_name,
        identifiers::{parse_identifier, IdentifierValues},
        parse_number, parse_string,
    };

    #[test]
    fn test_identifier_number() {
        let identifier_string = "local test = 1";
        let (remainder, identifier) = parse_identifier(identifier_string).unwrap();
        assert_eq!(identifier.name, "test");
        assert_eq!(identifier.value, IdentifierValues::Number(1 as f32));
    }

    #[test]
    fn test_string() {
        let str = "\"string thing   thing\"";
        let (_, parsed_str) = parse_string::<()>(str).unwrap();
        println!("{}", parsed_str);
        assert_eq!(parsed_str, "string thing   thing");
    }

    #[test]
    fn test_identifier_name() {
        let line = "local test = 1";
        let (value, name) = identifier_name(line).unwrap();
        assert_eq!(name, "test");
        assert_eq!(value, "1");
    }

    #[test]
    fn test_integer() {
        let int = "6";
        let (_, int_value) = parse_number(int).unwrap();
        assert_eq!(int_value, 6 as f32);
    }

    #[test]
    fn test_float_with_dot() {
        let int = "6.";
        let (_, int_value) = parse_number(int).unwrap();
        assert_eq!(int_value, 6.);
    }

    #[test]
    fn test_float_with_decimal() {
        let int = "6.160";
        let (_, int_value) = parse_number(int).unwrap();
        assert_eq!(int_value, 6.160);
    }

    #[test]
    fn test_decimal() {
        let int = ".160";
        let (_, int_value) = parse_number(int).unwrap();
        assert_eq!(int_value, 0.160);
    }

    #[test]
    fn test_decimal_leading_zero() {
        let int = "0.160";
        let (_, int_value) = parse_number(int).unwrap();
        assert_eq!(int_value, 0.160);
    }
}
