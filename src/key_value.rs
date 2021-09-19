use nom::{character::complete::space0, combinator::map, sequence::tuple, IResult};

use crate::{key::Key, value::Value};

#[derive(Clone, Debug, PartialEq)]
pub struct KeyValue<'a> {
    pub key: Key<'a>,
    pub value: Value<'a>,
}

impl<'a> KeyValue<'a> {
    pub fn format(&self) -> String {
        format!("{}: {}", self.key.format(), self.value.format())
    }
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            tuple((Key::parse, space0, Value::parse)),
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
                key: Key("some_key"),
                value: Value("10"),
            },
        ));

        let actual = KeyValue::parse("some_key: 10 some data");
        assert_eq!(expected, actual);
    }
}
