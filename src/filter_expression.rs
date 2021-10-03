use nom::bytes::complete::tag_no_case;
use nom::character::complete::{space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

use crate::expression::Expression;
use crate::shared::optional;
use crate::{parameter, symbolic_name};

#[derive(Debug, PartialEq)]
pub struct FilterExpression<'a> {
    pub variable: &'a str,
    pub expression: Expression<'a>,
    pub where_expression: Option<Expression<'a>>,
}

impl<'a> FilterExpression<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, (variable, _, _, _, expression)) = tuple((
            symbolic_name::parse,
            space1,
            tag_no_case("IN"),
            space1,
            Expression::parse,
        ))(input)?;

        let (input, _) = space0(input)?;

        let (input, where_expression) = optional(map(
            tuple((tag_no_case("WHERE"), space1, Expression::parse)),
            |(_, _, expr)| expr,
        ))(input)?;

        Ok((
            input,
            FilterExpression {
                variable,
                expression,
                where_expression,
            },
        ))
    }

    pub fn format(&self) -> String {
        let where_format = match &self.where_expression {
            Some(exp) => format!(" WHERE {}", exp.format()),
            None => "".to_string(),
        };
        format!(
            "{} IN {}{}",
            self.variable,
            self.expression.format(),
            where_format
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_filter_expression() {
        let expected = Ok((
            "data",
            FilterExpression {
                variable: "someVar",
                expression: Expression::list_of_decimal_ints(&["10", "11", "12"]),
                where_expression: Some(Expression::bool(true)),
            },
        ));

        let actual = FilterExpression::parse("someVar in [10, 11, 12] where true data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_filter_expression() {
        let expected = "someVar IN [10, 11, 12] WHERE TRUE";
        let actual = FilterExpression::parse("someVar in   [10,   11, 12  ] where true").unwrap().1.format();
        assert_eq!(expected, actual);
    }
}
