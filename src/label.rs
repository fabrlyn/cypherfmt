use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::token;

#[derive(Debug, PartialEq)]
pub struct Label<'a>(pub &'a str);

impl<'a> Label<'a> {
    pub fn parse_one(input: &'a str) -> IResult<&str, Self> {
        map(token::parse, Label)(input)
    }

    pub fn parse(input: &'a str) -> IResult<&'a str, Vec<Label<'a>>> {
        many1(preceded(tag(":"), Self::parse_one))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_label() {
        let expected = Ok((" data", vec![Label("ALabel")]));
        let actual = Label::parse(":ALabel data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_labels() {
        let expected = Ok((" data", vec![Label("ALabel"), Label("BLabel")]));
        let actual = Label::parse(":ALabel:BLabel data");
        assert_eq!(expected, actual);
    }
}
