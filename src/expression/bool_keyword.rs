use nom::{
    branch::alt, bytes::complete::tag_no_case, character::complete::space1, combinator::map,
    sequence::tuple, IResult,
};

#[derive(Debug, PartialEq)]
pub enum BoolKeyword {
    StartsWith,
    EndsWith,
    Contains,
    In,
}

impl BoolKeyword {
    fn parse_starts_with(input: &str) -> IResult<&str, Self> {
        map(
            tuple((tag_no_case("STARTS"), space1, tag_no_case("WITH"))),
            |_| Self::StartsWith,
        )(input)
    }

    fn parse_ends_with(input: &str) -> IResult<&str, Self> {
        map(
            tuple((tag_no_case("ENDS"), space1, tag_no_case("WITH"))),
            |_| Self::EndsWith,
        )(input)
    }

    fn parse_contains(input: &str) -> IResult<&str, Self> {
        map(tag_no_case("CONTAINS"), |_| Self::Contains)(input)
    }

    fn parse_in(input: &str) -> IResult<&str, Self> {
        map(tag_no_case("IN"), |_| Self::In)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            Self::parse_starts_with,
            Self::parse_ends_with,
            Self::parse_contains,
            Self::parse_in,
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bool_keyword_starts_with() {
        let expected = Ok((" data", BoolKeyword::StartsWith));
        let actual = BoolKeyword::parse("Starts With data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_bool_keyword_ends_with() {
        let expected = Ok((" data", BoolKeyword::EndsWith));
        let actual = BoolKeyword::parse("Ends With data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_bool_keyword_contains() {
        let expected = Ok((" data", BoolKeyword::Contains));
        let actual = BoolKeyword::parse("Contains data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_bool_keyword_in() {
        let expected = Ok((" data", BoolKeyword::In));
        let actual = BoolKeyword::parse("In data");
        assert_eq!(expected, actual);
    }
}
