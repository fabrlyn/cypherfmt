use nom::{bytes::complete::tag, combinator::map, IResult};

use crate::token;

#[derive(Debug, PartialEq)]
pub struct Key<'a>(pub &'a str);

impl<'a> Key<'a> {
    pub fn format(&self) -> String {
        self.0.to_string()
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, key) = map(token::parse, Key)(input)?;
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
        let actual = Key::parse("some_Key_1 data");
        assert_eq!(expected, actual);
    }
}
