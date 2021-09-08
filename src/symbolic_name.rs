use nom::{
    character::complete::{alpha1, alphanumeric0},
    combinator::recognize,
    sequence::tuple,
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, &str> {
    recognize(tuple((alpha1, alphanumeric0)))(input)
}
