use nom::{
    bytes::complete::tag_no_case,
    character::complete::{space0, space1},
    combinator::map,
    multi::{many0, many1},
    sequence::tuple,
    IResult,
};

use crate::{combinator::Combinator, shared::optional};

use super::CalculableExpression;

#[derive(Debug, PartialEq)]
pub struct CombinableExpression<'a> {
    pub not_count: usize,
    pub calculables: Vec<CalculableExpression<'a>>,
    pub combinator: Option<Combinator>,
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
    fn parse_nots(input: &'a str) -> IResult<&str, usize> {
        map(many0(tuple((tag_no_case("NOT"), space1))), |nots| {
            nots.len()
        })(input)
    }

    fn parse_calculable(input: &'a str) -> IResult<&str, CalculableExpression<'a>> {
        map(
            tuple((CalculableExpression::parse, space0)),
            |(result, _)| result,
        )(input)
    }

    fn parse_calcualables(input: &'a str) -> IResult<&str, Vec<CalculableExpression<'a>>> {
        let (input, calculable) = Self::parse_calculable(input)?;
        if calculable.math_op.is_none() {
            return Ok((input, vec![calculable]));
        }

        let (input, mut calculables_rest) = many1(Self::parse_calculable)(input)?;
        let mut calculables = vec![calculable];
        calculables.append(&mut calculables_rest);

        Ok((input, calculables))
    }

    pub fn format(&self) -> String {
        format!(
            "{}",
            self.calculables
                .iter()
                .map(|c| c.format())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, not_count) = Self::parse_nots(input)?;
        let (input, calculables) = Self::parse_calcualables(input)?;
        let (input, combinator) = optional(Combinator::parse)(input)?;

        Ok((
            input,
            CombinableExpression {
                not_count,
                calculables,
                combinator,
            },
        ))
    }
}
