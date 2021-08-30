use nom::{
    combinator::{map, opt},
    multi::many1,
    IResult,
};

use crate::{line::Line, node::Node, relationship::Relationship};

#[derive(Debug, PartialEq)]
pub enum Component<'a> {
    Node(Node<'a>),
    Line(Line<'a>),
    Relationship(Relationship<'a>),
}

impl<'a> Component<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        match opt(Node::parse)(input)? {
            (input, Some(node)) => return Ok((input, Component::Node(node))),
            _ => {}
        }

        match opt(Line::parse)(input)? {
            (input, Some(line)) => return Ok((input, Component::Line(line))),
            _ => {}
        }

        map(Relationship::parse, Component::Relationship)(input)
    }
}

#[cfg(test)]
mod component_tests {
    use super::*;

    #[test]
    fn parse_component_node() {
        let expected = Ok((
            " data",
            Component::Node(Node {
                variable: None,
                labels: vec![],
                properties: None,
            }),
        ));
        let actual = Component::parse("() data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_component_line() {
        let expected = Ok((" data", Component::Line(Line("-"))));
        let actual = Component::parse("- data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_component_relationship() {
        let expected = Ok((
            " data",
            Component::Relationship(Relationship {
                variable: None,
                labels: vec![],
                properties: None,
            }),
        ));
        let actual = Component::parse("[] data");
        assert_eq!(expected, actual);
    }
}

#[derive(Debug, PartialEq)]
pub struct Pattern<'a>(pub Vec<Component<'a>>);

impl<'a> Pattern<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(many1(Component::parse), Pattern)(input)
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
                Component::Node(Node {
                    variable: Some("var1"),
                    labels: vec![Label("ALabel")],
                    properties: Some(Properties(vec![KeyValue {
                        key: Key("a"),
                        value: Value("10"),
                    }])),
                }),
                Component::Line(Line("<-")),
                Component::Relationship(Relationship {
                    variable: Some("rel1"),
                    labels: vec![Label("ARelationship")],
                    properties: Some(Properties(vec![KeyValue {
                        key: Key("r"),
                        value: Value("15"),
                    }])),
                }),
                Component::Line(Line("-")),
                Component::Node(Node {
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
