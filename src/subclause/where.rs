use nom::{bytes::complete::tag, character::complete::space1, IResult};

use crate::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct Where<'a>(Vec<Expression<'a>>);

impl<'a> Where<'a> {
    pub fn format(&self) -> String {
        format!(
            "{}",
            self.0
                .iter()
                .map(|e| e.format())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, _) = tag("WHERE")(input)?;
        let (input, _) = space1(input)?;
        let (input, expression) = Expression::parse(input)?;
        Ok((input, Where(vec![expression])))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        atom::Atom,
        expression::{
            calculable_expression::CalculableExpression,
            combinable_expression::CombinableExpression, Expression,
        },
        literal::{bool::Bool, Literal},
    };

    use super::Where;

    #[test]
    fn parse_where() {
        let expected = Ok((
            "data",
            Where(vec![Expression {
                expressions: vec![CombinableExpression {
                    calculables: vec![CalculableExpression {
                        atom: Atom::Literal(Literal::Bool(Bool(true))),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
            }]),
        ));
        let actual = Where::parse("WHERE TRUE data");
        assert_eq!(expected, actual);
    }
}
