use nom::{combinator::map, multi::many1, IResult};

use crate::{
    atom::Atom,
    combinator::Combinator,
    literal::{
        bool::Bool,
        integer::{Decimal, Integer},
        list::List,
        number::Number,
        Literal,
    },
    math_op::MathOp,
    property_lookup::PropertyLookup,
};

#[derive(Debug, PartialEq)]
pub struct Not;

#[derive(Debug, PartialEq)]
pub enum AddOrSub {
    Add,
    Sub,
}

#[derive(Debug, PartialEq)]
pub enum BoolKeyword {
    StartsWith,
    EndsWith,
    Contains,
    In,
}

#[derive(Debug, PartialEq)]
pub enum NullExpression {
    IsNull,
    IsNotNull,
}

#[derive(Debug, PartialEq)]
pub struct BoolExpression<'a> {
    pub keyword: BoolKeyword,
    pub atom: Atom<'a>,
    pub property_lookups: Vec<PropertyLookup<'a>>,
    pub labels: Vec<&'a str>,
}
#[derive(Debug, PartialEq)]
pub enum ListExpression<'a> {
    Single(Expression<'a>),
    Dotted((Option<Expression<'a>>, Option<Expression<'a>>)),
}

#[derive(Debug, PartialEq)]
pub enum BoolOrListExpression<'a> {
    Null(NullExpression),
    Bool(BoolExpression<'a>),
    List(ListExpression<'a>),
}

#[derive(Debug, PartialEq)]
pub struct CalculableExpression<'a> {
    pub add_or_subs: Vec<AddOrSub>,
    pub atom: Atom<'a>,
    pub property_lookups: Vec<PropertyLookup<'a>>,
    pub labels: Vec<&'a str>,
    pub math_op: Option<MathOp>,
}

impl<'a> Default for CalculableExpression<'a> {
    fn default() -> Self {
        CalculableExpression {
            add_or_subs: vec![],
            atom: Atom::Variable(""),
            labels: vec![],
            property_lookups: vec![],
            math_op: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CombinableExpression<'a> {
    pub not_count: usize,
    pub combinator: Option<Combinator>,
    pub calculables: Vec<CalculableExpression<'a>>,
}

impl<'a> Default for CombinableExpression<'a> {
    fn default() -> Self {
        CombinableExpression {
            not_count: 0,
            combinator: None,
            calculables: vec![],
        }
    }
}

impl<'a> CombinableExpression<'a> {
    pub fn format(&self) -> String {
        todo!()
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        todo!()
    }
}

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
        map(many1(CombinableExpression::parse), |expressions| {
            Expression { expressions }
        })(input)
        //map(Atom::parse, Expression)(input)
    }
}

#[cfg(test)]
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
