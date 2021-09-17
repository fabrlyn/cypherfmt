use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::space1,
    combinator::{map, recognize},
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
pub struct SortItem<'a> {
    expression: Expression<'a>,
    order: Option<Order>,
}

impl<'a> SortItem<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct ProjectionBody<'a> {
    distinct: bool,
    wild_card: bool,
    projection_items: Vec<ProjectionItem<'a>>,
    sort_expressions: Vec<SortItem<'a>>,
    skip_expression: Option<Expression<'a>>,
    limit_expression: Option<Expression<'a>>,
}

fn parse_distinct<'a>(input: &'a str) -> IResult<&str, Option<&str>> {
    optional(tag_no_case("DISTINCT"))(input)
}

fn parse_wildcard<'a>(input: &'a str) -> IResult<&str, Option<&str>> {
    optional(tag("*"))(input)
}

fn parse_order_by<'a>(input: &'a str) -> IResult<&str, Option<&str>> {
    optional(recognize(tuple((
        tag_no_case("ORDER"),
        space1,
        tag_no_case("BY"),
    ))))(input)
}

fn parse_projection_items<'a>(input: &'a str) -> IResult<&str, Vec<ProjectionItem<'a>>> {
    map(many0(tuple((ProjectionItem::parse, space1))), |result| {
        result.into_iter().map(|(item, _)| item).collect()
    })(input)
}

fn parse_sort_items<'a>(input: &'a str) -> IResult<&str, Vec<SortItem<'a>>> {
    if let Ok((input, None)) = parse_order_by(input) {
        return Ok((input, vec![]));
    }

    map(many0(tuple((SortItem::parse, space1))), |result| {
        result.into_iter().map(|(item, _)| item).collect()
    })(input)
}

fn parse_skip_item<'a>(input: &'a str) -> IResult<&'a str, Option<Expression>> {
    optional(map(
        tuple((tag_no_case("SKIP"), space1, Expression::parse)),
        |(_, _, result)| result,
    ))(input)
}

fn parse_limit_item<'a>(input: &'a str) -> IResult<&'a str, Option<Expression>> {
    optional(map(
        tuple((tag_no_case("LIMIT"), space1, Expression::parse)),
        |(_, _, result)| result,
    ))(input)
}

impl<'a> ProjectionBody<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, distinct) = parse_distinct(input)?;
        let (input, wildcard) = parse_wildcard(input)?;
        let (input, projection_items) = parse_projection_items(input)?;
        let (input, sort_items) = parse_sort_items(input)?;
        let (input, skip_item) = parse_skip_item(input)?;
        let (input, limit_item) = parse_limit_item(input)?;
        todo!()
    }
}
