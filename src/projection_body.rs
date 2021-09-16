use crate::expression::Expression;

#[derive(Debug, PartialEq)]
pub enum Order {
    Ascending,
    Asc,
    Descending,
    Desc,
}

#[derive(Debug, PartialEq)]
pub struct MaybeNamedExpression<'a> {
    expression: Expression<'a>,
    variable: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct MaybeOrderedExpression<'a> {
    expression: Expression<'a>,
    order: Option<Order>,
}

#[derive(Debug, PartialEq)]
pub struct ProjectionBody<'a> {
    distinct: bool,
    wild_card: bool,
    expressions: Vec<MaybeNamedExpression<'a>>,
    order_expressions: Vec<MaybeOrderedExpression<'a>>,
    skip_expression: Option<Expression<'a>>,
    limit_expression: Option<Expression<'a>>,
}
