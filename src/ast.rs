use std::collections::HashMap;

use nom::{character::complete::multispace0, multi::many0, sequence::delimited, IResult};

use crate::identifier_parser::{parse_identifier, Identifier};

pub fn make_ast(input: &str) -> IResult<&str, HashMap<String, Identifier>> {
    let (remainder, identifiers) =
        many0(delimited(multispace0, parse_identifier, multispace0))(input)?;
    let mut ast = HashMap::new();
    for ident in identifiers {
        // if let Some(ident) = ident_option {
        //     ast.insert(ident.name.clone(), ident);
        // }
        ast.insert(ident.name.clone(), ident);
    }

    Ok((remainder, ast))
}
