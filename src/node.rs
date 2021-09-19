use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{delimited, tuple},
    IResult,
};

use crate::{label::Label, properties::Properties, token};

#[derive(Debug, PartialEq)]
pub struct Node<'a> {
    pub variable: Option<&'a str>,
    pub labels: Vec<Label<'a>>,
    pub properties: Option<Properties<'a>>,
}

impl<'a> Node<'a> {
    fn variable_str(&self) -> String {
        self.variable
            .map(|v| v.to_string())
            .unwrap_or("".to_string())
    }

    fn labels_str(&self) -> String {
        if self.labels.len() == 0 {
            return "".to_string();
        }

        format!(
            ":{}",
            self.labels
                .iter()
                .map(|l| l.format())
                .collect::<Vec<_>>()
                .join(":")
        )
    }

    fn properties_str(&self) -> String {
        self.properties
            .as_ref()
            .map(|p| p.format())
            .unwrap_or("".to_string())
    }

    pub fn format(&self) -> String {
        format!(
            "({}{}{})",
            self.variable_str(),
            self.labels_str(),
            self.properties_str()
        )
    }
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("("),
                tuple((opt(token::parse), opt(Label::parse), opt(Properties::parse))),
                tag(")"),
            ),
            |(variable, labels, properties)| Node {
                variable,
                labels: labels.unwrap_or(vec![]),
                properties,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{key::Key, key_value::KeyValue, value::Value};

    use super::*;

    #[test]
    fn parse_node_empty() {
        let expected = Ok((
            " data",
            Node {
                variable: None,
                labels: vec![],
                properties: None,
            },
        ));

        let actual = Node::parse("() data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_node_single_label() {
        let expected = Ok((
            " data",
            Node {
                variable: None,
                labels: vec![Label("ALabel")],
                properties: None,
            },
        ));

        let actual = Node::parse("(:ALabel) data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_node_multiple_label() {
        let expected = Ok((
            " data",
            Node {
                variable: None,
                labels: vec![Label("ALabel"), Label("BLabel")],
                properties: None,
            },
        ));

        let actual = Node::parse("(:ALabel:BLabel) data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_node_properties() {
        let expected = Ok((
            " data",
            Node {
                variable: None,
                labels: vec![],
                properties: Some(Properties(vec![KeyValue {
                    key: Key("some_key"),
                    value: Value("10"),
                }])),
            },
        ));

        let actual = Node::parse("({some_key: 10}) data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_node_variable() {
        let expected = Ok((
            " data",
            Node {
                variable: Some("myVar"),
                labels: vec![],
                properties: None,
            },
        ));

        let actual = Node::parse("(myVar) data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_node_variable_single_label() {
        let expected = Ok((
            " data",
            Node {
                variable: Some("myVar"),
                labels: vec![Label("ALabel")],
                properties: None,
            },
        ));

        let actual = Node::parse("(myVar:ALabel) data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_node_variable_single_label_properties() {
        let expected = Ok((
            " data",
            Node {
                variable: Some("myVar"),
                labels: vec![Label("ALabel")],
                properties: Some(Properties(vec![KeyValue {
                    key: Key("some_key"),
                    value: Value("10"),
                }])),
            },
        ));

        let actual = Node::parse("(myVar:ALabel{some_key: 10}) data");
        assert_eq!(expected, actual);
    }
}
