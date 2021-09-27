use nom::{
    branch::alt, bytes::complete::tag_no_case, character::complete::space1, combinator::map,
    sequence::tuple, IResult,
};

#[derive(Debug, PartialEq)]
pub enum NullExpression {
    IsNull,
    IsNotNull,
}

impl NullExpression {
    fn parse_is_null(input: &str) -> IResult<&str, Self> {
        map(
            tuple((tag_no_case("IS"), space1, tag_no_case("NULL"))),
            |_| Self::IsNull,
        )(input)
    }

    fn parse_is_not_null(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                tag_no_case("IS"),
                space1,
                tag_no_case("NOT"),
                space1,
                tag_no_case("NULL"),
            )),
            |_| Self::IsNotNull,
        )(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_is_null, Self::parse_is_not_null))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_null_expression_is_null() {
        let expected = Ok((" data", NullExpression::IsNull));
        let actual = NullExpression::parse("Is null data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_null_expression_is_not_null() {
        let expected = Ok((" data", NullExpression::IsNotNull));
        let actual = NullExpression::parse("Is NOT null data");
        assert_eq!(expected, actual);
    }
}
