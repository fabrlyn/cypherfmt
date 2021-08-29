use nom::{
    bytes::complete::{take_while, take_while_m_n},
    combinator::recognize,
    sequence::tuple,
    IResult,
};

use crate::shared::{is_alphabetic, is_alphanumeric};

fn parse_first_character(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 1, is_alphabetic)(input)
}

fn parse_rest(input: &str) -> IResult<&str, &str> {
    take_while(|c| is_alphanumeric(c) || c == '_')(input)
}

pub fn parse(input: &str) -> IResult<&str, &str> {
    recognize(tuple((parse_first_character, parse_rest)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_key() {
        let expected = Ok((" data", "some_Key_1"));
        let actual = parse("some_Key_1 data");
        assert_eq!(expected, actual);
    }
}
