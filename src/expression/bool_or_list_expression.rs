use nom::{branch::alt, combinator::map, IResult};

use super::{
    bool_expression::BoolExpression, list_expression::ListExpression,
    null_expression::NullExpression,
};

#[derive(Debug, PartialEq)]
pub enum BoolOrListExpression<'a> {
    Null(NullExpression),
    Bool(BoolExpression<'a>),
    List(ListExpression<'a>),
}

impl<'a> BoolOrListExpression<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(NullExpression::parse, BoolOrListExpression::Null),
            map(BoolExpression::parse, BoolOrListExpression::Bool),
            map(ListExpression::parse, BoolOrListExpression::List),
        ))(input)
    }
}
