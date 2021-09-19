use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not},
    character::complete::one_of,
    combinator::{map, recognize},
    sequence::delimited,
    IResult,
};

use crate::shared::{double_qoute, single_qoute};

#[derive(Debug, PartialEq)]
pub struct String<'a>(pub &'a str);

impl<'a> String<'a> {
    pub fn format(&self) -> std::string::String {
        self.0.to_string()
    }

    fn parse_double_qoute_string(input: &'a str) -> IResult<&str, &str> {
        recognize(delimited(
            double_qoute,
            escaped(is_not(r#"\""#), '\\', one_of(r#"""#)),
            double_qoute,
        ))(input)
    }

    fn parse_single_qoute_string(input: &'a str) -> IResult<&str, &str> {
        recognize(delimited(
            single_qoute,
            escaped(is_not(r#"\'"#), '\\', one_of(r#"'"#)),
            single_qoute,
        ))(input)
    }
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            alt((
                Self::parse_double_qoute_string,
                Self::parse_single_qoute_string,
            )),
            String,
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string_double_qoute() {
        let expected = Ok((" abc", String(r#""a\"bc . a 123%!@# ""#)));
        let actual = String::parse(r#""a\"bc . a 123%!@# " abc"#);
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_string_single_qoute() {
        let expected = Ok((" abc", String(r#"'a\'bc . a 123%!@# '"#)));
        let actual = String::parse(r#"'a\'bc . a 123%!@# ' abc"#);
        assert_eq!(expected, actual);
    }
}
