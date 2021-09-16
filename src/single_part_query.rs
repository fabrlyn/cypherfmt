use crate::{
    clause::{in_query_call::InQueryCall, r#match::Match, unwind::Unwind},
    r#return::Return,
};

#[derive(Debug, PartialEq)]
pub enum ReadPart<'a> {
    Match(Match<'a>),
    Unwind(Unwind<'a>),
    InQueryCall(InQueryCall),
}

#[derive(Debug, PartialEq)]
pub enum Mutation {
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

#[derive(Debug, PartialEq)]
pub struct MutationPart<'a> {
    mutation_parts: Vec<Mutation>,
    r#return: Option<Return<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct SinglePartQuery<'a> {
    read_parts: Vec<ReadPart<'a>>,
    return_or_mutate: ReturnOrMutate<'a>,
}
