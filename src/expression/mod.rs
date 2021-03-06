pub mod bool_expression;
pub mod bool_keyword;
pub mod bool_or_list_expression;
pub mod calculable_expression;
pub mod combinable_expression;
pub mod list_expression;
pub mod null_expression;

use nom::{combinator::map, multi::many1, IResult};

use crate::{
    atom::Atom,
    expression::calculable_expression::CalculableExpression,
    literal::{
        bool::Bool,
        integer::{Decimal, Integer},
        list::List,
        number::Number,
        string, Literal,
    },
};

use self::{
    bool_expression::BoolExpression, combinable_expression::CombinableExpression,
    list_expression::ListExpression, null_expression::NullExpression,
};

#[derive(Debug, PartialEq)]
pub struct Not;

#[derive(Debug, PartialEq)]
pub struct Expression<'a> {
    pub expressions: Vec<CombinableExpression<'a>>,
}

impl<'a> Expression<'a> {
    pub fn format(&self) -> String {
        self.expressions
            .iter()
            .map(|e| e.format())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, expression) = CombinableExpression::parse(input)?;
        if expression.combinator.is_none() {
            return Ok((
                input,
                Expression {
                    expressions: vec![expression],
                },
            ));
        }

        let mut expressions = vec![expression];
        let (input, mut expressions_rest) = many1(CombinableExpression::parse)(input)?;
        expressions.append(&mut expressions_rest);
        Ok((input, Expression { expressions }))
    }
}

impl<'a> Expression<'a> {
    pub fn decimal_int(i: &'a str) -> Self {
        Expression {
            expressions: vec![CombinableExpression {
                calculables: vec![CalculableExpression {
                    atom: Atom::Literal(Literal::Number(Number::Integer(Integer::Decimal(
                        Decimal(i),
                    )))),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        }
    }

    pub fn string(i: &'a str) -> Self {
        Expression {
            expressions: vec![CombinableExpression {
                calculables: vec![CalculableExpression {
                    atom: Atom::Literal(Literal::String(string::String(i))),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        }
    }

    pub fn variable(i: &'a str) -> Self {
        Expression {
            expressions: vec![CombinableExpression {
                calculables: vec![CalculableExpression {
                    atom: Atom::Variable(i),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        }
    }

    pub fn bool(b: bool) -> Self {
        Expression {
            expressions: vec![CombinableExpression {
                calculables: vec![CalculableExpression {
                    atom: Atom::Literal(Literal::Bool(Bool(b))),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        }
    }

    pub fn list_of_decimal_ints(ints: &[&'a str]) -> Self {
        let ints = ints.iter().map(|s| Self::decimal_int(s)).collect();
        Expression {
            expressions: vec![CombinableExpression {
                calculables: vec![CalculableExpression {
                    atom: Atom::Literal(Literal::List(List(ints))),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        }
    }
}
