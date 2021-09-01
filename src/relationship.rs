use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{delimited, tuple},
    IResult,
};

use crate::{label::Label, line::Line, properties::Properties, token};

#[derive(Debug, PartialEq)]
pub struct Relationship<'a> {
    pub variable: Option<&'a str>,
    pub labels: Vec<Label<'a>>,
    pub properties: Option<Properties<'a>>,
    pub right_line: Line<'a>,
    pub left_line: Line<'a>,
}

impl<'a> Relationship<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, left_line) = Line::parse(input)?;
        let (input, (variable, labels, properties)) = delimited(
            tag("["),
            tuple((opt(token::parse), opt(Label::parse), opt(Properties::parse))),
            tag("]"),
        )(input)?;
        let (input, right_line) = Line::parse(input)?;

        Ok((
            input,
            Relationship {
                variable,
                labels: labels.unwrap_or(vec![]),
                properties,
                right_line,
                left_line,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{key::Key, key_value::KeyValue, value::Value};

    use super::*;

    #[test]
    fn parse_relationship_empty() {
        let expected = Ok((
            " data",
            Relationship {
                variable: None,
                labels: vec![],
                properties: None,
                right_line: Line("-"),
                left_line: Line("-"),
            },
        ));

        let actual = Relationship::parse("-[]- data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_relationship_single_label() {
        let expected = Ok((
            " data",
            Relationship {
                variable: None,
                labels: vec![Label("ALabel")],
                properties: None,
                right_line: Line("-"),
                left_line: Line("-"),
            },
        ));

        let actual = Relationship::parse("-[:ALabel]- data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_relationship_multiple_label() {
        let expected = Ok((
            " data",
            Relationship {
                variable: None,
                labels: vec![Label("ALabel"), Label("BLabel")],
                properties: None,
                right_line: Line("-"),
                left_line: Line("-"),
            },
        ));

        let actual = Relationship::parse("-[:ALabel:BLabel]- data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_relationship_properties() {
        let expected = Ok((
            " data",
            Relationship {
                variable: None,
                labels: vec![],
                properties: Some(Properties(vec![KeyValue {
                    key: Key("some_key"),
                    value: Value("10"),
                }])),
                right_line: Line("-"),
                left_line: Line("-"),
            },
        ));

        let actual = Relationship::parse("-[{some_key: 10}]- data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_relationship_variable() {
        let expected = Ok((
            " data",
            Relationship {
                variable: Some("myVar"),
                labels: vec![],
                properties: None,
                right_line: Line("-"),
                left_line: Line("-"),
            },
        ));

        let actual = Relationship::parse("-[myVar]- data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_relationship_variable_single_label() {
        let expected = Ok((
            " data",
            Relationship {
                variable: Some("myVar"),
                labels: vec![Label("ALabel")],
                properties: None,
                right_line: Line("-"),
                left_line: Line("-"),
            },
        ));

        let actual = Relationship::parse("-[myVar:ALabel]- data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_relationship_variable_single_label_properties() {
        let expected = Ok((
            " data",
            Relationship {
                variable: Some("myVar"),
                labels: vec![Label("ALabel")],
                properties: Some(Properties(vec![KeyValue {
                    key: Key("some_key"),
                    value: Value("10"),
                }])),
                right_line: Line("-"),
                left_line: Line("-"),
            },
        ));

        let actual = Relationship::parse("-[myVar:ALabel{some_key: 10}]- data");
        assert_eq!(expected, actual);
    }
}
