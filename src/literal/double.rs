use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit0, digit1},
    combinator::{map, recognize},
    sequence::tuple,
    IResult,
};

use crate::shared::optional_signed;

#[derive(Debug, PartialEq)]
pub enum Double<'a> {
    Exponent(Exponent<'a>),
    Regular(Regular<'a>),
}

impl<'a> Double<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        if let Ok((input, exponent)) = Exponent::parse(input) {
            return Ok((input, Double::Exponent(exponent)));
        }

        map(Regular::parse, Double::Regular)(input)
    }
}

#[cfg(test)]
mod double_tests {
    use super::*;

    #[test]
    fn parse_double_regular() {
        let expected = Ok((" data", Double::Regular(Regular("123.321"))));
        let actual = Double::parse("123.321 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_double_exponent() {
        let expected = Ok((" data", Double::Exponent(Exponent("6.022E23"))));
        let actual = Double::parse("6.022E23 data");
        assert_eq!(expected, actual);
    }
}

#[derive(Debug, PartialEq)]
pub struct Exponent<'a>(&'a str);

impl<'a> Exponent<'a> {
    pub fn parse_segment_0(input: &str) -> IResult<&str, &str> {
        alt((tag("."), recognize(tuple((digit0, tag("."))))))(input)
    }

    pub fn parse_segment_1(input: &str) -> IResult<&str, &str> {
        recognize(tuple((digit1, tag("E"))))(input)
    }

    pub fn parse_segment_2(input: &str) -> IResult<&str, &str> {
        recognize(tuple((optional_signed, digit1)))(input)
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            recognize(tuple((
                Self::parse_segment_0,
                Self::parse_segment_1,
                Self::parse_segment_2,
            ))),
            Exponent,
        )(input)
    }
}

#[cfg(test)]
mod exponent_tests {

    use super::*;

    #[test]
    fn parse_exponent() {
        let expected = Ok((" data", Exponent("6.022E23")));
        let actual = Exponent::parse("6.022E23 data");
        assert_eq!(expected, actual);
    }
}

#[derive(Debug, PartialEq)]
pub struct Regular<'a>(&'a str);

impl<'a> Regular<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            recognize(tuple((optional_signed, digit0, tag("."), digit1))),
            Regular,
        )(input)
    }
}

#[cfg(test)]
mod regular_tests {

    use super::*;

    #[test]
    fn parse_double_regular() {
        let expected = Ok((" data", Regular("123.4321")));
        let actual = Regular::parse("123.4321 data");
        assert_eq!(expected, actual);

        let expected = Ok((" data", Regular(".4321")));
        let actual = Regular::parse(".4321 data");
        assert_eq!(expected, actual);

        let expected = Ok((" data", Regular("-1.4321")));
        let actual = Regular::parse("-1.4321 data");
        assert_eq!(expected, actual);
    }
}
