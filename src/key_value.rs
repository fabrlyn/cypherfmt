use nom::{
    bytes::complete::tag, character::complete::space0, combinator::map, sequence::tuple, IResult,
};

use crate::{expression::Expression, token};

#[derive(Debug, PartialEq)]
pub struct KeyValue<'a> {
    pub key: &'a str,
    pub value: Expression<'a>,
}

impl<'a> KeyValue<'a> {
    pub fn format(&self) -> String {
        format!("{}: {}", self.key, self.value.format())
    }
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            tuple((
                token::parse,
                tuple((space0, tag(":"), space0)),
                Expression::parse,
            )),
            |(key, _, value)| KeyValue { key, value },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_key_value() {
        let expected = Ok((
            " some data",
            KeyValue {
                key: "some_key",
                value: Expression::decimal_int("10"),
            },
        ));

        let actual = KeyValue::parse("some_key: 10 some data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_key_value_with_space() {
        let expected = "a: [1, 2, 3]";
        let actual = KeyValue::parse("a : [ 1,2, 3 ]").unwrap().1.format();
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_key_value_without_space() {
        let expected = "a: [1, 2, 3]";
        let actual = KeyValue::parse("a:[1,2,3]").unwrap().1.format();
        assert_eq!(expected, actual);
    }
}
