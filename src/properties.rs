use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::key_value::KeyValue;

#[derive(Clone, Debug, PartialEq)]
pub struct Properties<'a>(pub Vec<KeyValue<'a>>);

impl<'a> Properties<'a> {
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
                tag("{"),
                separated_list0(tag(","), Self::parse_key_value),
                tag("}"),
            ),
            Properties,
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{key::Key, value::Value};

    use super::*;

    #[test]
    fn parse_properties_single() {
        let expected = Ok((
            " some data",
            Properties(vec![KeyValue {
                key: Key("some_key"),
                value: Value("10"),
            }]),
        ));

        let actual = Properties::parse("{ some_key: 10 } some data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_properties_multiple() {
        let expected = Ok((
            " some data",
            Properties(vec![
                KeyValue {
                    key: Key("some_key"),
                    value: Value("10"),
                },
                KeyValue {
                    key: Key("some_other"),
                    value: Value("false"),
                },
            ]),
        ));

        let actual = Properties::parse("{ some_key: 10, some_other: false } some data");
        assert_eq!(expected, actual);
    }
}
