use nom::{
    combinator::{map, opt},
    multi::many1,
    IResult,
};

use crate::{line::Line, node::Node, relationship::Relationship};

#[derive(Debug, PartialEq)]
pub enum Entity<'a> {
    Node(Node<'a>),
    Relationship(Relationship<'a>),
}

impl<'a> Entity<'a> {

    pub fn format(&self) -> String {
        match self {
            Entity::Node(n) => n.format(),
            Entity::Relationship(r) => r.format()
        }
    }
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        match opt(Node::parse)(input)? {
            (input, Some(node)) => return Ok((input, Entity::Node(node))),
            _ => {}
        }

        map(Relationship::parse, Entity::Relationship)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_component_node() {
        let expected = Ok((
            " data",
            Entity::Node(Node {
                variable: None,
                labels: vec![],
                properties: None,
            }),
        ));
        let actual = Entity::parse("() data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_component_relationship() {
        let expected = Ok((
            " data",
            Entity::Relationship(Relationship {
                variable: None,
                labels: vec![],
                properties: None,
                right_line: Line("-"),
                left_line: Line("-"),
            }),
        ));
        let actual = Entity::parse("-[]- data");
        assert_eq!(expected, actual);
    }
}
