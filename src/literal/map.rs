use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::key_value::KeyValue;

#[derive(Debug, PartialEq)]
pub struct Map<'a>(pub Vec<KeyValue<'a>>);

impl<'a> Map<'a> {
    pub fn format(&self) -> String {
        if self.0.len() == 0 {
            return "".to_string();
        }

        format!(
            "{{{}}}",
            self.0
                .iter()
                .map(|kv| kv.format())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    fn parse_key_value(input: &str) -> IResult<&str, KeyValue> {
        map(tuple((space0, KeyValue::parse, space0)), |(_, kv, _)| kv)(input)
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            delimited(
                tuple((tag("{"), space0)),
                separated_list0(tuple((space0, tag(","), space0)), Self::parse_key_value),
                tuple((space0, tag("}"))),
            ),
            Map,
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::Expression;

    use super::*;

    #[test]
    fn format_map() {
        let expected = "{a: 'abc', b: 10, c: [1, 2, 3]}";
        let actual = Map::parse("{     a: 'abc', b: 10, c: [1,2,3]    }")
            .unwrap()
            .1
            .format();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_map_single() {
        let expected = Ok((
            " some data",
            Map(vec![KeyValue {
                key: "some_key",
                value: Expression::decimal_int("10"),
            }]),
        ));

        let actual = Map::parse("{ some_key: 10 } some data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_map_multiple() {
        let expected = Ok((
            " some data",
            Map(vec![
                KeyValue {
                    key: "some_key",
                    value: Expression::decimal_int("10"),
                },
                KeyValue {
                    key: "some_other",
                    value: Expression::bool(false),
                },
            ]),
        ));

        let actual = Map::parse("{ some_key: 10, some_other: false } some data");
        assert_eq!(expected, actual);
    }
}
