use nom::{combinator::map, IResult};

use crate::pattern::Pattern;

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Pattern(Pattern<'a>),
}

impl<'a> Expression<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(Pattern::parse, Expression::Pattern)(input)
    }
}
