use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

use crate::expression::Expression;
use crate::filter_expression::FilterExpression;
use crate::shared::optional;

#[derive(Debug, PartialEq)]
pub struct ListComprehension<'a> {
    pub filter_expression: FilterExpression<'a>,
    pub expression: Option<Expression<'a>>,
}

impl<'a> ListComprehension<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, (_, _, filter_expression)) =
            tuple((tag("["), space0, FilterExpression::parse))(input)?;

        let (input, expression) = optional(map(
            tuple((space0, tag("|"), space0, Expression::parse)),
            |(_, _, _, expr)| expr,
        ))(input)?;

        let (input, _) = tag("]")(input)?;

        Ok((
            input,
            ListComprehension {
                filter_expression,
                expression,
            },
        ))
    }

    pub fn format(&self) -> String {
        let expression = match &self.expression {
            Some(exp) => format!(" | {}", exp.format()),
            None => "".to_string(),
        };
        format!("[{}{}]", self.filter_expression.format(), expression)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn parse_list_comprehension() {
        let expected = Ok((
            " data",
            ListComprehension {
                filter_expression: FilterExpression {
                    variable: "someVar",
                    expression: Expression::list_of_decimal_ints(&["10", "11", "12"]),
                    where_expression: None,
                },
                expression: Some(Expression::bool(true)),
            },
        ));

        let actual = ListComprehension::parse("[  someVar  in [  10, 11,12]  | True] data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_list_comprehension() {
        let expected = "[someVar IN [10, 11, 12] | TRUE]";
        let actual = ListComprehension::parse("[  someVar   in [  10,11, 12  ] |   true]")
            .unwrap()
            .1
            .format();
        assert_eq!(expected, actual);
    }
}
