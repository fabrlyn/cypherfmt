use nom::{character::complete::space1, combinator::map, multi::many1, sequence::tuple, IResult};

use crate::{
    clause::{in_query_call::InQueryCall, r#match::Match, unwind::Unwind},
    r#return::Return,
};

#[derive(Debug, PartialEq)]
pub enum ReadingClause<'a> {
    Match(Match<'a>),
    Unwind(Unwind<'a>),
    InQueryCall(InQueryCall),
}

impl<'a> ReadingClause<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(Match::parse, ReadingClause::Match)(input)
    }
}

#[derive(Debug, PartialEq)]
pub enum UpdatingClause {
    Create,
    Merge,
    Delete,
    Set,
    Remove,
}

#[derive(Debug, PartialEq)]
pub enum ReturnOrMutate<'a> {
    Return(Return<'a>),
    Mutate(MutationPart<'a>),
}

impl<'a> ReturnOrMutate<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(Return::parse, ReturnOrMutate::Return)(input)
    }
}

#[derive(Debug, PartialEq)]
pub struct MutationPart<'a> {
    mutation_parts: Vec<UpdatingClause>,
    r#return: Option<Return<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct SinglePartQuery<'a> {
    read_parts: Vec<ReadingClause<'a>>,
    return_or_mutate: ReturnOrMutate<'a>,
}

impl<'a> SinglePartQuery<'a> {
    fn parse_read_parts(input: &'a str) -> IResult<&str, Vec<ReadingClause<'a>>> {
        map(many1(tuple((ReadingClause::parse, space1))), |result| {
            result.into_iter().map(|(r, _)| r).collect()
        })(input)
    }
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        todo!()
    }
}
