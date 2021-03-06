use nom::{
    bytes::complete::tag,
    combinator::opt,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{label::Label, line::Line, literal::map::Map, token};

#[derive(Debug, PartialEq)]
pub struct Relationship<'a> {
    pub variable: Option<&'a str>,
    pub labels: Vec<Label<'a>>,
    pub properties: Option<Map<'a>>,
    pub right_line: Line<'a>,
    pub left_line: Line<'a>,
}

impl<'a> Relationship<'a> {
    fn variable_str(&self) -> String {
        self.variable
            .map(|v| v.to_string())
            .unwrap_or("".to_string())
    }

    fn labels_str(&self) -> String {
        if self.labels.len() == 0 {
            return "".to_string();
        }

        self.labels.iter().map(|l| l.format()).collect()
    }

    fn properties_str(&self) -> String {
        self.properties
            .as_ref()
            .clone()
            .map(|p| p.format())
            .unwrap_or("".to_string())
    }

    pub fn format(&self) -> String {
        format!(
            "{}[{}{}{}]{}",
            self.right_line.format(),
            self.variable_str(),
            self.labels_str(),
            self.properties_str(),
            self.left_line.format()
        )
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, left_line) = Line::parse(input)?;
        let (input, (variable, labels, properties)) = delimited(
            tag("["),
            tuple((opt(token::parse), opt(Label::parse_many1), opt(Map::parse))),
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
    use crate::{expression::Expression, key_value::KeyValue, value::Value};

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
                properties: Some(Map(vec![KeyValue {
                    key: "some_key",
                    value: Expression::decimal_int("10"),
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
                properties: Some(Map(vec![KeyValue {
                    key: "some_key",
                    value: Expression::decimal_int("10"),
                }])),
                right_line: Line("-"),
                left_line: Line("-"),
            },
        ));

        let actual = Relationship::parse("-[myVar:ALabel{some_key: 10}]- data");
        assert_eq!(expected, actual);
    }
}
