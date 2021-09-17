use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::space1,
    combinator::map,
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::{expression::Expression, shared::optional};

#[derive(Debug, PartialEq)]
pub enum Order {
    Ascending,
    Asc,
    Descending,
    Desc,
}

#[derive(Debug, PartialEq)]
pub struct ProjectionItem<'a> {
    expression: Expression<'a>,
    variable: Option<&'a str>,
}

impl<'a> ProjectionItem<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        todo!()
    }
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
    projection_items: Vec<ProjectionItem<'a>>,
    order_expressions: Vec<MaybeOrderedExpression<'a>>,
    skip_expression: Option<Expression<'a>>,
    limit_expression: Option<Expression<'a>>,
}

fn parse_distinct<'a>(input: &'a str) -> IResult<&str, Option<&str>> {
    optional(tag_no_case("DISTINCT"))(input)
}

fn parse_wildcard<'a>(input: &'a str) -> IResult<&str, Option<&str>> {
    optional(tag("*"))(input)
}

fn parse_projection_items<'a>(input: &'a str) -> IResult<&str, Vec<ProjectionItem<'a>>> {
    map(many0(tuple((ProjectionItem::parse, space1))), |result| {
        result.into_iter().map(|(item, _)| item).collect()
    })(input)
}

impl<'a> ProjectionBody<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, distinct) = parse_distinct(input)?;
        let (input, wildcard) = parse_wildcard(input)?;
        let (input, projection_items) = parse_projection_items(input)?;
        todo!()
    }
}
