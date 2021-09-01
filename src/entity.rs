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
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        match opt(Node::parse)(input)? {
            (input, Some(node)) => return Ok((input, Entity::Node(node))),
            _ => {}
        }

        map(Relationship::parse, Entity::Relationship)(input)
    }
}

#[cfg(test)]
mod component_tests {
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

#[derive(Debug, PartialEq)]
pub struct Pattern<'a>(pub Vec<Entity<'a>>);

impl<'a> Pattern<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(many1(Entity::parse), Pattern)(input)
    }
}

#[cfg(test)]
mod pattern_test {
    use crate::{
        key::Key, key_value::KeyValue, label::Label, properties::Properties, value::Value,
    };

    use super::*;

    #[test]
    fn parse_pattern_node_relationship_node() {
        let expected = Ok((
            " data",
            Pattern(vec![
                Entity::Node(Node {
                    variable: Some("var1"),
                    labels: vec![Label("ALabel")],
                    properties: Some(Properties(vec![KeyValue {
                        key: Key("a"),
                        value: Value("10"),
                    }])),
                }),
                Entity::Relationship(Relationship {
                    variable: Some("rel1"),
                    labels: vec![Label("ARelationship")],
                    properties: Some(Properties(vec![KeyValue {
                        key: Key("r"),
                        value: Value("15"),
                    }])),
                    right_line: Line("-"),
                    left_line: Line("<-"),
                }),
                Entity::Node(Node {
                    variable: Some("var2"),
                    labels: vec![Label("BLabel")],
                    properties: Some(Properties(vec![KeyValue {
                        key: Key("b"),
                        value: Value("20"),
                    }])),
                }),
            ]),
        ));

        let actual = Pattern::parse(
            "(var1:ALabel{a:10})<-[rel1:ARelationship{r:15}]-(var2:BLabel{b:20}) data",
        );
        assert_eq!(expected, actual);
    }
}
