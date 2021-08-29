use nom::{
    bytes::complete::{tag, take_till, take_while, take_while_m_n},
    combinator::{map, recognize},
    sequence::tuple,
    IResult,
};

use crate::shared::{is_alphabetic, is_alphanumeric};

#[derive(Debug, PartialEq)]
pub struct Key<'a>(&'a str);

impl<'a> Key<'a> {
    fn parse_first_character(input: &str) -> IResult<&str, &str> {
        take_while_m_n(1, 1, is_alphabetic)(input)
    }

    fn parse_rest(input: &str) -> IResult<&str, &str> {
        take_while(|c| is_alphanumeric(c) || c == '_')(input)
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, key) = map(
            recognize(tuple((Self::parse_first_character, Self::parse_rest))),
            Key,
        )(input)?;

        let (input, _) = tag(":")(input)?;
        Ok((input, key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_key() {
        let expected = Ok((" data", Key("some_Key_1")));
        let actual = Key::parse("some_Key_1: data");
        assert_eq!(expected, actual);
    }
}
