use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0, combinator::map, multi::many0,
    sequence::tuple, IResult,
};

use crate::{
    atom::Atom, label::Label, math_op::MathOp, property_lookup::PropertyLookup, shared::optional,
};

use super::bool_or_list_expression::BoolOrListExpression;

#[derive(Debug, PartialEq)]
pub enum AddOrSub {
    Add,
    Sub,
}

impl AddOrSub {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("+"), |_| AddOrSub::Add),
            map(tag("-"), |_| AddOrSub::Sub),
        ))(input)
    }
}

#[derive(Debug, PartialEq)]
pub struct CalculableExpression<'a> {
    pub add_or_subs: Vec<AddOrSub>,
    pub atom: Atom<'a>,
    pub property_lookups: Vec<PropertyLookup<'a>>,
    pub labels: Vec<Label<'a>>,
    pub bool_or_list_expressions: Vec<BoolOrListExpression<'a>>,
    pub math_op: Option<MathOp>,
}

impl<'a> CalculableExpression<'a> {
    pub fn format(&self) -> String {
        format!("{}", self.atom.format())
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, add_or_subs) =
            many0(map(tuple((AddOrSub::parse, space0)), |(result, _)| result))(input)?;

        let (input, atom) = Atom::parse(input)?;
        let (input, _) = space0(input)?;
        let (input, property_lookups) = many0(PropertyLookup::parse)(input)?;
        let (input, labels) = many0(Label::parse)(input)?;
        let (input, bool_or_list_expressions) = many0(map(
            tuple((BoolOrListExpression::parse, space0)),
            |(result, _)| result,
        ))(input)?;
        let (input, math_op) = optional(MathOp::parse)(input)?;

        Ok((
            input,
            CalculableExpression {
                add_or_subs,
                atom,
                property_lookups,
                labels,
                bool_or_list_expressions,
                math_op,
            },
        ))
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
            bool_or_list_expressions: vec![],
        }
    }
}
