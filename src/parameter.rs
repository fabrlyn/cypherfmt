use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric0};
use nom::combinator::recognize;
use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};

use crate::literal::integer::Decimal;
use crate::symbolic_name;

#[derive(Debug, PartialEq)]
pub struct Parameter<'a>(pub &'a str);

impl<'a> Parameter<'a> {
    fn parse_symbolic_name(input: &str) -> IResult<&str, &str> {
        recognize(tuple((tag("$"), symbolic_name::parse)))(input)
    }

    fn parse_decimal(input: &'a str) -> IResult<&str, &str> {
        recognize(tuple((tag("$"), map(Decimal::parse, |d| d.0))))(input)
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            alt((Self::parse_decimal, Self::parse_symbolic_name)),
            Parameter,
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_parameter_decimal() {
        let expected = Ok((" data", Parameter("$2")));
        let actual = Parameter::parse("$2 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_parameter_symbolic_name() {
        let expected = Ok((" data", Parameter("$someParameter1a")));
        let actual = Parameter::parse("$someParameter1a data");
        assert_eq!(expected, actual);
    }
}
