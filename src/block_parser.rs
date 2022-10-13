// parses any type of blocks (if, do, while, repeat)

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::{
        complete::{alphanumeric1, multispace0},
        streaming::space1,
    },
    combinator::{map, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

use crate::identifier_parser::{parse_identifier, Identifier, IdentifierValues};

#[derive(Debug, PartialEq, Clone)]
pub enum BlockType {
    Do,
    If,
    While,
    Repeat,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub block_type: BlockType,
    pub identifiers: HashMap<String, Identifier>,
}

fn parse_repeat(input: &str) -> IResult<&str, Block> {
    let (remainder, _repeat_tag) = delimited(multispace0, tag("repeat"), multispace0)(input)?;
    let (more_remainder, (bytes, _last)) = pair(
        take_until("until"),
        terminated(many1(alt((alphanumeric1, space1, tag("_")))), multispace0),
    )(remainder)?;
    let (_, identifiers) = many0(parse_identifier)(bytes)?;

    let mut block = Block {
        block_type: BlockType::Repeat,
        identifiers: HashMap::new(),
    };

    for ident in identifiers {
        block.identifiers.insert(ident.name.clone(), ident);
    }

    Ok((more_remainder, block))
}

fn parse_while(input: &str) -> IResult<&str, &str> {
    let (remainder, _while_tag) = delimited(multispace0, tag("while"), multispace0)(input)?;
    let (more_remainder, bytes) = take_until("do")(remainder)?;
    Ok((more_remainder, bytes))
}

fn parse_end(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((take_until("end"), terminated(tag("end"), multispace0)))(input)
}

fn parse_do(input: &str) -> IResult<&str, Block> {
    let (remainder, _) = recognize(pair(tag("do"), multispace0))(input)?;
    let (remainder_2, (bytes, _end)) = parse_end(remainder)?;
    let (_, identifiers) = many0(parse_identifier)(bytes)?;

    let mut block = Block {
        block_type: BlockType::Do,
        identifiers: HashMap::new(),
    };

    for ident in identifiers {
        if ident.value != IdentifierValues::End {
            block.identifiers.insert(ident.name.clone(), ident);
        }
    }

    Ok((remainder_2, block))
}

fn parse_if(input: &str) -> IResult<&str, Block> {
    let (remainder, _if_tag) = delimited(multispace0, tag("if"), multispace0)(input)?;
    let (remainder_2, _rest_of_line) =
        pair(take_until("then"), terminated(tag("then"), multispace0))(remainder)?;

    let (real_remainder, (bytes, _end_tag)) = parse_end(remainder_2)?;
    let (_, identifiers) = many0(parse_identifier)(bytes)?;
    let mut block = Block {
        block_type: BlockType::If,
        identifiers: HashMap::new(),
    };

    for ident in identifiers {
        block.identifiers.insert(ident.name.clone(), ident);
    }

    Ok((real_remainder, block))
}

pub fn parse_block(input: &str) -> IResult<&str, Block> {
    alt((
        map(pair(parse_while, parse_do), |(_, mut b)| {
            b.block_type = BlockType::While;
            b
        }),
        map(parse_do, |b| b),
        map(parse_repeat, |b| b),
        map(parse_if, |b| b),
    ))(input)
}
