use crate::single_part_query::SinglePartQuery;

#[derive(Debug, PartialEq)]
pub enum PartQuery<'a> {
    Single(SinglePartQuery<'a>),
    Multi,
}

#[derive(Debug, PartialEq)]
pub enum Combinator {
    Union,
    UnionAll,
}

#[derive(Debug, PartialEq)]
pub struct CombinableSinglePartQuery<'a> {
    query: SinglePartQuery<'a>,
    combinator: Option<Combinator>,
}

#[derive(Debug, PartialEq)]
pub struct Cypher<'a> {
    queries: Vec<PartQuery<'a>>,
    semicolon: bool,
}
