use nom::{combinator::map, IResult};

use self::{bool::Bool, null::Null, number::Number, string::String};

mod bool;
mod double;
pub mod integer;
mod null;
mod number;
mod string;

#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    Null(Null<'a>),
    Bool(Bool<'a>),
    Number(Number<'a>),
    String(String<'a>),
}

impl<'a> Literal<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        if let Ok((input, null)) = Null::parse(input) {
            return Ok((input, Literal::Null(null)));
        }

        if let Ok((input, r#bool)) = Bool::parse(input) {
            return Ok((input, Literal::Bool(r#bool)));
        }

        if let Ok((input, number)) = Number::parse(input) {
            return Ok((input, Literal::Number(number)));
        }

        map(String::parse, Literal::String)(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::literal::integer::{Decimal, Integer};

    use super::*;

    #[test]
    fn parse_literal_null() {
        let expected = Ok((" data", Literal::Null(Null("NULL"))));
        let actual = Literal::parse("NULL data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_literal_bool() {
        let expected = Ok((" data", Literal::Bool(Bool("TRUE"))));
        let actual = Literal::parse("TRUE data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_literal_number() {
        let expected = Ok((
            " data",
            Literal::Number(Number::Integer(Integer::Decimal(Decimal("13")))),
        ));
        let actual = Literal::parse("13 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_literal_string() {
        let expected = Ok((" data", Literal::String(String("\"someString\""))));
        let actual = Literal::parse("\"someString\" data");
        assert_eq!(expected, actual);
    }
}
