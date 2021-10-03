use nom::{
    bytes::complete::tag_no_case,
    character::complete::{space0, space1},
    multi::separated_list1,
    IResult,
};

use crate::{expression::Expression, shared::optional};

#[derive(Debug, PartialEq)]
pub struct Case<'a> {
    pub case_expression: Option<Expression<'a>>,
    pub cases: Vec<(Expression<'a>, Expression<'a>)>,
    pub else_expression: Option<Expression<'a>>,
}

impl<'a> Case<'a> {
    fn parse_when_then(input: &'a str) -> IResult<&str, (Expression<'a>, Expression<'a>)> {
        let (input, _) = tag_no_case("WHEN")(input)?;
        let (input, _) = space1(input)?;
        let (input, when_expression) = Expression::parse(input)?;
        let (input, _) = tag_no_case("THEN")(input)?;
        let (input, _) = space1(input)?;
        let (input, then_expression) = Expression::parse(input)?;
        let (input, _) = space0(input)?;

        Ok((input, (when_expression, then_expression)))
    }

    fn parse_else(input: &'a str) -> IResult<&str, Expression> {
        let (input, _) = tag_no_case("ELSE")(input)?;
        let (input, _) = space1(input)?;
        Expression::parse(input)
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, _) = tag_no_case("CASE")(input)?;
        let (input, _) = space0(input)?;
        let (input, case_expression) = optional(Expression::parse)(input)?;
        let (input, _) = space0(input)?;

        let (input, cases) = separated_list1(space1, Self::parse_when_then)(input)?;

        let (input, else_expression) = optional(Self::parse_else)(input)?;

        let (input, _) = space0(input)?;
        let (input, _) = tag_no_case("END")(input)?;

        Ok((
            input,
            Case {
                case_expression,
                cases,
                else_expression,
            },
        ))
    }

    pub fn format(&self) -> String {
        let case_expression = match &self.case_expression {
            Some(expression) => format!("CASE {}\n", expression.format()),
            None => format!("CASE\n"),
        };

        let cases = self
            .cases
            .iter()
            .map(|(when_exp, then_exp)| {
                format!("WHEN {} THEN {}\n", when_exp.format(), then_exp.format())
            })
            .collect::<String>();

        let end_case = match &self.else_expression {
            Some(exp) => format!("ELSE {}\n", exp.format()),
            None => "".to_string(),
        };

        format!("{}{}{}END\n", case_expression, cases, end_case)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_case() {
        let expected = Ok((
            " data",
            Case {
                case_expression: Some(Expression::variable("someVar")),
                cases: vec![(Expression::string("'abc'"), Expression::bool(true))],
                else_expression: None,
            },
        ));

        let actual = Case::parse("CASE someVar WHEN 'abc' THEN TRUE END data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_case() {
        let formatted = Case::parse("CASE $someVar WHEN true THEN 10 ELSE 20 END").unwrap().1.format();
        println!("{}", formatted);
    }
}
