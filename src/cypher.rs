use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::space1,
    combinator::{map, recognize},
    multi::many1,
    sequence::tuple,
    IResult,
};

use crate::{shared::optional, single_part_query::SinglePartQuery};

#[derive(Debug, PartialEq)]
pub enum PartQuery<'a> {
    Single(SinglePartQuery<'a>),
    Multi,
}

impl<'a> PartQuery<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(SinglePartQuery::parse, PartQuery::Single)(input)
    }
}

#[derive(Debug, PartialEq)]
pub struct CombinablePartQuery<'a> {
    combinator: Option<&'a str>,
    part_query: PartQuery<'a>,
}

fn parse_union(input: &str) -> IResult<&str, &str> {
    tag_no_case("UNION")(input)
}

fn parse_union_all(input: &str) -> IResult<&str, &str> {
    recognize(tuple((parse_union, space1, tag_no_case("ALL"))))(input)
}

impl<'a> CombinablePartQuery<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            tuple((
                optional(alt((parse_union_all, parse_union))),
                PartQuery::parse,
            )),
            |(combinator, part_query)| CombinablePartQuery {
                combinator,
                part_query,
            },
        )(input)
    }
}

#[derive(Debug, PartialEq)]
pub struct Cypher<'a> {
    queries: Vec<PartQuery<'a>>,
    semicolon: bool,
}

impl<'a> Cypher<'a> {
    pub fn parse(query: &'a str) -> IResult<&str, Self> {
        map(
            tuple((many1(PartQuery::parse), optional(tag(";")))),
            |(queries, semicolon)| Cypher {
                queries,
                semicolon: semicolon.is_some(),
            },
        )(query)
    }

    pub fn format(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_union() {
        let expected = Ok((" data", "union"));
        let actual = super::parse_union("union data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_union_all() {
        let expected = Ok((" data", "union all"));
        let actual = super::parse_union_all("union all data");
        assert_eq!(expected, actual);
    }
}
