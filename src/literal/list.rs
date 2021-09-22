use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct List<'a>(pub Vec<Expression<'a>>);

impl<'a> List<'a> {
    pub fn format(&self) -> String {
        format!(
            "[{}]",
            self.0
                .iter()
                .map(|e| e.format())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            delimited(
                tuple((tag("["), space0)),
                separated_list0(tuple((space0, tag(","), space0)), Expression::parse),
                tuple((space0, tag("]"))),
            ),
            List,
        )(input)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        atom::Atom,
        expression::CombinableExpression,
        literal::{
            integer::{Decimal, Integer},
            number::Number,
            Literal,
        },
    };

    use super::*;

    #[test]
    fn parse_list_of_decimals() {
        let expected = Ok((
            " data",
            List(vec![
                Expression {
                    expressions: vec![CombinableExpression {
                        atom: Atom::Literal(Literal::Number(Number::Integer(Integer::Decimal(
                            Decimal("10"),
                        )))),
                        ..Default::default()
                    }],
                },
                Expression {
                    expressions: vec![CombinableExpression {
                        atom: Atom::Literal(Literal::Number(Number::Integer(Integer::Decimal(
                            Decimal("11"),
                        )))),
                        ..Default::default()
                    }],
                },
                Expression {
                    expressions: vec![CombinableExpression {
                        atom: Atom::Literal(Literal::Number(Number::Integer(Integer::Decimal(
                            Decimal("12"),
                        )))),
                        ..Default::default()
                    }],
                },
            ]),
        ));

        let actual = List::parse("[10, 11, 12] data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_list_of_decimals() {
        let expected = "[10, 11, 12]";
        let actual = List::parse("[ 10  ,  11, 12]").unwrap().1.format();
        assert_eq!(expected, actual);
    }
}
