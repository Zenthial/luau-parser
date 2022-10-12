use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, one_of},
    combinator::{map, opt, recognize},
    multi::many0_count,
    multi::{many0, many1},
    sequence::{pair, tuple},
    sequence::{preceded, terminated},
    IResult,
};

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

pub fn parse_number(input: &str) -> IResult<&str, f32> {
    map(
        alt((
            // Case one: .42
            recognize(tuple((
                char('.'),
                decimal,
                opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
            ))), // Case two: 42e42 and 42.42e42
            recognize(tuple((
                decimal,
                opt(preceded(char('.'), decimal)),
                one_of("eE"),
                opt(one_of("+-")),
                decimal,
            ))), // Case three: 42. and 42.42
            recognize(tuple((decimal, char('.'), opt(decimal)))),
            recognize(decimal),
        )),
        |number| (number.parse::<f32>().unwrap()),
    )(input)
}
