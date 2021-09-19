use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, take_while1},
    character::complete::one_of,
    combinator::{map, recognize},
    sequence::delimited,
    IResult,
};

use crate::shared::{double_qoute, is_alphanumeric, single_qoute};

#[derive(Debug, PartialEq)]
pub struct Value<'a>(pub &'a str);

fn allowed_for_other(c: char) -> bool {
    match c {
        '-' | '.' => true,
        c => is_alphanumeric(c),
    }
}

impl<'a> Value<'a> {
    pub fn format(&self) -> String {
        self.0.to_string()
    }

    fn parse_other(input: &'a str) -> IResult<&str, Self> {
        map(take_while1(allowed_for_other), Value)(input)
    }

    fn parse_double_qoute_string(input: &'a str) -> IResult<&str, Self> {
        map(
            recognize(delimited(
                double_qoute,
                escaped(is_not(r#"\""#), '\\', one_of(r#"""#)),
                double_qoute,
            )),
            Value,
        )(input)
    }

    fn parse_single_qoute_string(input: &'a str) -> IResult<&str, Self> {
        map(
            recognize(delimited(
                single_qoute,
                escaped(is_not(r#"\'"#), '\\', one_of(r#"'"#)),
                single_qoute,
            )),
            Value,
        )(input)
    }

    fn parse_string(input: &'a str) -> IResult<&str, Self> {
        alt((
            Self::parse_double_qoute_string,
            Self::parse_single_qoute_string,
        ))(input)
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((Self::parse_other, Self::parse_string))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_decimal_integer() {
        let expected = Ok((" data", Value("5")));
        let actual = Value::parse("5 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_signed_decimal_integer() {
        let expected = Ok((" data", Value("-5")));
        let actual = Value::parse("-5 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_decimal_float() {
        let expected = Ok((" data", Value("3.14")));
        let actual = Value::parse("3.14 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_signed_decimal_float() {
        let expected = Ok((" data", Value("-3.14")));
        let actual = Value::parse("-3.14 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_scientific_decimal_float() {
        let expected = Ok((" data", Value("6.022E23")));
        let actual = Value::parse("6.022E23 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_signed_scientific_decimal_float() {
        let expected = Ok((" data", Value("-6.022E23")));
        let actual = Value::parse("-6.022E23 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_hex_integer() {
        let expected = Ok((" data", Value("0x13af")));
        let actual = Value::parse("0x13af data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_signed_hex_integer() {
        let expected = Ok((" data", Value("-0x13af")));
        let actual = Value::parse("-0x13af data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_octal_integer() {
        let expected = Ok((" data", Value("0o1372")));
        let actual = Value::parse("0o1372 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_signed_octal_integer() {
        let expected = Ok((" data", Value("-0o1372")));
        let actual = Value::parse("-0o1372 data");
        assert_eq!(expected, actual);
    }
}
