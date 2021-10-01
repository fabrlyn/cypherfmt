use nom::{combinator::map, IResult};

use self::{bool::Bool, list::List, map::Map, null::Null, number::Number, string::String};

pub mod bool;
pub mod double;
pub mod integer;
pub mod list;
pub mod map;
pub mod null;
pub mod number;
pub mod string;

#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    Null(Null),
    Bool(Bool),
    Number(Number<'a>),
    List(List<'a>),
    Map(Map<'a>),
    String(String<'a>),
}

impl<'a> Literal<'a> {
    pub fn format(&self) -> std::string::String {
        match self {
            Literal::Null(n) => n.format(),
            Literal::Bool(b) => b.format(),
            Literal::Number(n) => n.format(),
            Literal::String(s) => s.format(),
            Literal::List(l) => l.format(),
            Literal::Map(p) => p.format(),
        }
    }

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

        if let Ok((input, list)) = List::parse(input) {
            return Ok((input, Literal::List(list)));
        }

        if let Ok((input, map)) = Map::parse(input) {
            return Ok((input, Literal::Map(map)));
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
        let expected = Ok((" data", Literal::Null(Null)));
        let actual = Literal::parse("NULL data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_literal_bool() {
        let expected = Ok((" data", Literal::Bool(Bool(true))));
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
