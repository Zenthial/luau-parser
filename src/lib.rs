#![allow(dead_code)]

mod comment_parser;
mod datatype_parsers;
mod function_parser;
mod identifier_parser;
mod line_parser;
mod table_parser;
mod types;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::{
        comment_parser,
        datatype_parsers::number_parser::parse_number,
        datatype_parsers::string_parser::parse_string,
        function_parser,
        function_parser::{parse_function, Function, FunctionArguments},
        identifier_parser::{parse_identifier, Identifier, IdentifierValues},
        line_parser,
        table_parser::{parse_table, Table, TableMember, TableMemberType},
        types::Types,
    };

    #[test]
    fn test_parse_function() {
        let contents = fs::read_to_string("test.lua").unwrap();
        let (_remainder, function) = parse_function(&contents).unwrap();
        println!("{:?}", function);
        assert_eq!(
            function,
            Function {
                name: "test".to_string(),
                return_type: Types::Any,
                arguments: vec![
                    FunctionArguments {
                        name: "a".to_string(),
                        function_type: Types::Any
                    },
                    FunctionArguments {
                        name: "b".to_string(),
                        function_type: Types::Any
                    }
                ],
                identifiers: HashMap::<String, Identifier>::from([
                    (
                        "two".to_string(),
                        Identifier {
                            name: "two".to_string(),
                            value: IdentifierValues::String("2".to_string())
                        }
                    ),
                    (
                        "three".to_string(),
                        Identifier {
                            name: "three".to_string(),
                            value: IdentifierValues::Bool(true)
                        }
                    ),
                    (
                        "one".to_string(),
                        Identifier {
                            name: "one".to_string(),
                            value: IdentifierValues::Number(1.0)
                        }
                    ),
                    (
                        "four".to_string(),
                        Identifier {
                            name: "four".to_string(),
                            value: IdentifierValues::Table(Table {
                                name: "four".to_string(),
                                members: vec![TableMember {
                                    name: "one".to_string(),
                                    is_a: TableMemberType::RawType(IdentifierValues::Number(1.0))
                                }]
                            })
                        }
                    )
                ])
            }
        );
    }

    #[test]
    fn test_parse_table() {
        let input = "{a=1,\nb = 3\nc=\"tom\",d=true\ne={a=2.}}";
        let (_, table) = parse_table(input).unwrap();
        assert_eq!(
            table.members,
            vec![
                TableMember {
                    name: "a".to_string(),
                    is_a: TableMemberType::RawType(IdentifierValues::Number(1.0))
                },
                TableMember {
                    name: "b".to_string(),
                    is_a: TableMemberType::RawType(IdentifierValues::Number(3.0))
                },
                TableMember {
                    name: "c".to_string(),
                    is_a: TableMemberType::RawType(IdentifierValues::String("tom".to_string()))
                },
                TableMember {
                    name: "d".to_string(),
                    is_a: TableMemberType::RawType(IdentifierValues::Bool(true))
                },
                TableMember {
                    name: "e".to_string(),
                    is_a: TableMemberType::NestedTable(Table {
                        name: "".to_string(),
                        members: vec![TableMember {
                            name: "a".to_string(),
                            is_a: TableMemberType::RawType(IdentifierValues::Number(2.0))
                        },]
                    })
                }
            ]
        );
    }

    #[test]
    fn test_parse_function_definition_types() {
        let line = "function tester_function(one: boolean, two: number)";
        let (_, function) = function_parser::parse_function_definition(line).unwrap();
        assert_eq!(function.name, "tester_function");
        assert_eq!(
            function.arguments,
            vec![
                FunctionArguments {
                    name: String::from("one"),
                    function_type: Types::Boolean,
                },
                FunctionArguments {
                    name: String::from("two"),
                    function_type: Types::Number,
                }
            ]
        )
    }

    #[test]
    fn test_parse_function_definition_any_types() {
        let line = "function tester_function(one, two)";
        let (_, function) = function_parser::parse_function_definition(line).unwrap();
        assert_eq!(function.name, "tester_function");
        assert_eq!(
            function.arguments,
            vec![
                FunctionArguments {
                    name: String::from("one"),
                    function_type: Types::Any,
                },
                FunctionArguments {
                    name: String::from("two"),
                    function_type: Types::Any,
                }
            ]
        )
    }

    #[test]
    fn test_comment_line() {
        let line = "-- this is a comment\n";
        let (_, comment) = comment_parser::parse_comment_line(line).unwrap();
        assert_eq!(comment, " this is a comment");
    }

    #[test]
    fn test_comment_block() {
        let block = "--[[ block comment block comment\nblock comment block\ncomment]]";
        let (_, comment) = comment_parser::parse_comment_block(block).unwrap();
        assert_eq!(
            comment,
            " block comment block comment\nblock comment block\ncomment"
        );
    }

    #[test]
    fn test_multiline_identifiers() {
        let lines = "local test = 1\nlocal other_test = \"2\"\n";
        let (remaining_lines, line_1) = line_parser::parse_line(lines).unwrap();
        let (_remainder, identifier_1) = parse_identifier(line_1).unwrap();
        let (_, line_2) = line_parser::parse_line(remaining_lines).unwrap();
        let (_remainder, identifier_2) = parse_identifier(line_2).unwrap();
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
        let (_remainder, identifier) = parse_identifier(identifier_string).unwrap();
        assert_eq!(identifier.name, "test");
        assert_eq!(identifier.value, IdentifierValues::Number(1 as f32));
    }

    #[test]
    fn test_identifier_string() {
        let identifier_string = "local test = \"tom\"";
        let (_remainder, identifier) = parse_identifier(identifier_string).unwrap();
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
        assert_eq!(parsed_str, "string thing   thing");
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
