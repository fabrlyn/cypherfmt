use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::{space0, space1},
    combinator::{map, recognize},
    multi::{many0, separated_list0},
    sequence::tuple,
    IResult,
};

use crate::{expression::Expression, shared::optional, symbolic_name};

#[derive(Debug, PartialEq)]
pub enum Order {
    Ascending,
    Asc,
    Descending,
    Desc,
}

#[derive(Debug, PartialEq)]
pub struct ProjectionItem<'a> {
    pub expression: Expression<'a>,
    pub variable: Option<&'a str>,
}

impl<'a> ProjectionItem<'a> {
    fn variable_str(&self) -> String {
        self.variable
            .map(|v| format!(" AS {}", v))
            .unwrap_or("".to_string())
    }

    pub fn format(&self) -> String {
        format!("{}{}", self.expression.format(), self.variable_str())
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, expression) = Expression::parse(input)?;
        let (input, variable) = optional(map(
            tuple((space1, tag_no_case("AS"), space1, symbolic_name::parse)),
            |(_, _, _, variable)| variable,
        ))(input)?;
        Ok((
            input,
            ProjectionItem {
                expression,
                variable,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct SortItem<'a> {
    expression: Expression<'a>,
    order: Option<Order>,
}

impl<'a> SortItem<'a> {
    pub fn parse(_input: &'a str) -> IResult<&str, Self> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct ProjectionBody<'a> {
    pub distinct: bool,
    pub wild_card: bool,
    pub projection_items: Vec<ProjectionItem<'a>>,
    pub sort_expressions: Vec<SortItem<'a>>,
    pub skip_expression: Option<Expression<'a>>,
    pub limit_expression: Option<Expression<'a>>,
}

fn parse_distinct<'a>(input: &'a str) -> IResult<&str, Option<&str>> {
    optional(tag_no_case("DISTINCT"))(input)
}

fn parse_wild_card<'a>(input: &'a str) -> IResult<&str, Option<&str>> {
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
    separated_list0(tag(", "), ProjectionItem::parse)(input)
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
    fn distinct_str(&self) -> String {
        if self.distinct {
            return "DISTINCT".to_string();
        }
        "".to_string()
    }

    fn wild_card_str(&self) -> String {
        if self.wild_card {
            return "*".to_string();
        }
        "".to_string()
    }

    pub fn format(&self) -> String {
        format!(
            "{}{}{}",
            self.distinct_str(),
            self.wild_card_str(),
            self.projection_items
                .iter()
                .map(|p| p.format())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, distinct) = parse_distinct(input)?;
        let (input, wild_card) = parse_wild_card(input)?;
        let (input, projection_items) = parse_projection_items(input)?;
        //let (input, sort_items) = parse_sort_items(input)?;
        //let (input, skip_item) = parse_skip_item(input)?;
        //let (input, limit_item) = parse_limit_item(input)?;

        Ok((
            input,
            ProjectionBody {
                distinct: distinct.is_some(),
                wild_card: wild_card.is_some(),
                projection_items,
                sort_expressions: vec![],
                limit_expression: None,
                skip_expression: None,
            },
        ))
    }
}
