use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

use crate::shared::optional;

use super::Expression;

#[derive(Debug, PartialEq)]
pub enum ListExpression<'a> {
    Single(Expression<'a>),
    Dotted((Option<Expression<'a>>, Option<Expression<'a>>)),
}

impl<'a> ListExpression<'a> {
    fn parse_single(input: &'a str) -> IResult<&str, Self> {
        map(
            tuple((space0, delimited(tag("["), Expression::parse, tag("]")))),
            |(_, result)| ListExpression::Single(result),
        )(input)
    }

    fn parse_dotted(input: &'a str) -> IResult<&str, Self> {
        let (input, _) = space0(input)?;
        let (input, _) = tag("[")(input)?;

        let (input, first) = optional(Expression::parse)(input)?;
        let (input, _) = tag("..")(input)?;
        let (input, second) = optional(Expression::parse)(input)?;

        let (input, _) = tag("]")(input)?;

        Ok((input, ListExpression::Dotted((first, second))))
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((Self::parse_single, Self::parse_dotted))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_list_expression_single() {
        let expected = Ok((
            " data",
            ListExpression::Single(Expression::decimal_int("10")),
        ));

        let actual = ListExpression::parse("[10] data");
        assert_eq!(expected, actual);
    }
}
