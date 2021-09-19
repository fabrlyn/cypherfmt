use nom::{branch::alt, combinator::map, IResult};

use super::{double::Double, integer::Integer};

#[derive(Debug, PartialEq)]
pub enum Number<'a> {
    Double(Double<'a>),
    Integer(Integer<'a>),
}

impl<'a> Number<'a> {
    pub fn format(&self) -> String {
        match self {
            Number::Double(d) => d.format(),
            Number::Integer(i) => i.format(),
        }
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        if let Ok((input, double)) = Double::parse(input) {
            return Ok((input, Number::Double(double)));
        }

        map(Integer::parse, Number::Integer)(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::literal::{double::Regular, integer::Decimal};

    use super::*;

    #[test]
    fn parse_number_double() {
        let expected = Ok((" data", Number::Double(Double::Regular(Regular("13.23")))));
        let actual = Number::parse("13.23 data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_number_integer() {
        let expected = Ok((" data", Number::Integer(Integer::Decimal(Decimal("13")))));
        let actual = Number::parse("13 data");
        assert_eq!(expected, actual);
    }
}
