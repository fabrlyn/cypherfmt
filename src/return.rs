use nom::{
    bytes::complete::tag_no_case, character::complete::space1, combinator::map, sequence::tuple,
    IResult,
};

use crate::projection_body::ProjectionBody;

#[derive(Debug, PartialEq)]
pub struct Return<'a>(ProjectionBody<'a>);

impl<'a> Return<'a> {
    pub fn format(&self) -> String {
        format!("RETURN {}", self.0.format())
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            tuple((tag_no_case("RETURN"), space1, ProjectionBody::parse)),
            |(_, _, result)| Return(result),
        )(input)
    }
}
