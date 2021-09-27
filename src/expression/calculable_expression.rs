use nom::IResult;

use crate::{atom::Atom, math_op::MathOp, property_lookup::PropertyLookup};

#[derive(Debug, PartialEq)]
pub enum AddOrSub {
    Add,
    Sub,
}

#[derive(Debug, PartialEq)]
pub struct CalculableExpression<'a> {
    pub add_or_subs: Vec<AddOrSub>,
    pub atom: Atom<'a>,
    pub property_lookups: Vec<PropertyLookup<'a>>,
    pub labels: Vec<&'a str>,
    pub math_op: Option<MathOp>,
}

impl<'a> CalculableExpression<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        todo!()
    }
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
