use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, oct_digit1},
    combinator::{map, recognize},
    sequence::tuple,
    IResult,
};

use crate::shared::optional_signed;

#[derive(Debug, PartialEq)]
pub enum Integer<'a> {
    Decimal(Decimal<'a>),
    Octal(Octal<'a>),
    Hex(Hex<'a>),
}

impl<'a> Integer<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        if let Ok((input, hex)) = Hex::parse(input) {
            return Ok((input, Integer::Hex(hex)));
        }

        if let Ok((input, octal)) = Octal::parse(input) {
            return Ok((input, Integer::Octal(octal)));
        }

        map(Decimal::parse, Integer::Decimal)(input)
    }
}

#[cfg(test)]
mod integer_tests {
    use super::*;

    #[test]
    fn parse_integer_decimal() {
        let expected = Ok((" data", Integer::Decimal(Decimal("1234"))));
        let actual = Integer::parse("1234 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_integer_octal() {
        let expected = Ok((" data", Integer::Octal(Octal("01234"))));
        let actual = Integer::parse("01234 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_integer_hex() {
        let expected = Ok((" data", Integer::Hex(Hex("0x1234"))));
        let actual = Integer::parse("0x1234 data");
        assert_eq!(expected, actual);
    }
}

#[derive(Debug, PartialEq)]
pub struct Decimal<'a>(pub &'a str);

impl<'a> Decimal<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            recognize(tuple((optional_signed, alt((tag("0"), digit1))))),
            Decimal,
        )(input)
    }
}

#[cfg(test)]
mod decimal_tests {
    use super::*;

    #[test]
    fn parse_decimal_zero() {
        let expected = Ok((" data", Decimal("0")));
        let actual = Decimal::parse("0 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_decimal_digits() {
        let expected = Ok((" data", Decimal("12304")));
        let actual = Decimal::parse("12304 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_decimal_signed_digits() {
        let expected = Ok((" data", Decimal("-12304")));
        let actual = Decimal::parse("-12304 data");
        assert_eq!(expected, actual);
    }
}

#[derive(Debug, PartialEq)]
pub struct Octal<'a>(&'a str);

impl<'a> Octal<'a> {
    fn parse_prefix(input: &str) -> IResult<&str, &str> {
        alt((tag("0o"), tag("0")))(input)
    }
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            recognize(tuple((optional_signed, Self::parse_prefix, oct_digit1))),
            Octal,
        )(input)
    }
}

#[cfg(test)]
mod octal_tests {
    use super::*;

    #[test]
    fn parse_octal_zero() {
        let expected = Ok((" data", Octal("00")));
        let actual = Octal::parse("00 data");
        assert_eq!(expected, actual);

        let expected = Ok((" data", Octal("0o0")));
        let actual = Octal::parse("0o0 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_octal_digits() {
        let expected = Ok((" data", Octal("012304")));
        let actual = Octal::parse("012304 data");
        assert_eq!(expected, actual);

        let expected = Ok((" data", Octal("0o12304")));
        let actual = Octal::parse("0o12304 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_octal_signed_digits() {
        let expected = Ok((" data", Octal("-012304")));
        let actual = Octal::parse("-012304 data");
        assert_eq!(expected, actual);

        let expected = Ok((" data", Octal("-0o12304")));
        let actual = Octal::parse("-0o12304 data");
        assert_eq!(expected, actual);
    }
}

#[derive(Debug, PartialEq)]
pub struct Hex<'a>(&'a str);

impl<'a> Hex<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            recognize(tuple((optional_signed, tag("0x"), hex_digit1))),
            Hex,
        )(input)
    }
}

#[cfg(test)]
mod hex_tests {
    use super::*;

    #[test]
    fn parse_hex_zero() {
        let expected = Ok((" data", Hex("0x0")));
        let actual = Hex::parse("0x0 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_hex_digits() {
        let expected = Ok((" data", Hex("0xFe10ab7")));
        let actual = Hex::parse("0xFe10ab7 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_hex_signed_digits() {
        let expected = Ok((" data", Hex("-0xFe10ab7")));
        let actual = Hex::parse("-0xFe10ab7 data");
        assert_eq!(expected, actual);
    }
}
