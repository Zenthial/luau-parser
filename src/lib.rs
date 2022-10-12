#![allow(unused)]

mod identifiers;
mod line_parser;
mod number_parser;
mod string_parser;

use identifiers::identifier_name;
use number_parser::parse_number;
use std::sync::mpsc::RecvError;
use string_parser::parse_string;

use nom::character::complete::{multispace1, newline};
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

fn newline_test(input: &str) -> IResult<&str, char> {
    newline(input)
}

#[cfg(test)]
mod tests {
    use nom::character::complete::newline;

    use crate::{
        identifier_name,
        identifiers::{parse_identifier, IdentifierValues},
        line_parser, newline_test, parse_number, parse_string,
    };

    #[test]
    fn test_multiline_identifiers() {
        let lines = "local test = 1\nlocal other_test = \"2\"\n";
        let (remaining_lines, line_1) = line_parser::parse_line(lines).unwrap();
        let (remainder, identifier_1) = parse_identifier(line_1).unwrap();
        let (_, line_2) = line_parser::parse_line(remaining_lines).unwrap();
        let (remainder, identifier_2) = parse_identifier(line_2).unwrap();
        assert_eq!(identifier_1.name, "test");
        assert_eq!(identifier_1.value, IdentifierValues::Number(1 as f32));
        assert_eq!(identifier_2.name, "other_test");
        assert_eq!(
            identifier_2.value,
            IdentifierValues::String(String::from("2"))
        );
    }

    #[test]
    fn test_line() {
        let lines = "local test = 1\nlocal other_test = 2\n";
        let (remaining_lines, line_1) = line_parser::parse_line(lines).unwrap();
        let (_, line_2) = line_parser::parse_line(remaining_lines).unwrap();
        assert_eq!(line_1, "local test = 1");
        assert_eq!(line_2, "local other_test = 2");
    }

    #[test]
    fn test_identifier_number() {
        let identifier_string = "local test = 1";
        let (remainder, identifier) = parse_identifier(identifier_string).unwrap();
        assert_eq!(identifier.name, "test");
        assert_eq!(identifier.value, IdentifierValues::Number(1 as f32));
    }

    #[test]
    fn test_identifier_string() {
        let identifier_string = "local test = \"tom\"";
        let (remainder, identifier) = parse_identifier(identifier_string).unwrap();
        assert_eq!(identifier.name, "test");
        assert_eq!(
            identifier.value,
            IdentifierValues::String(String::from("tom"))
        );
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
