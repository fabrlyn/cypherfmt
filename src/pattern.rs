use nom::{combinator::map, multi::many1, IResult};

use crate::entity::Entity;

#[derive(Debug, PartialEq)]
pub struct Pattern<'a>(pub Vec<Entity<'a>>);

impl<'a> Pattern<'a> {
    pub fn format(&self) -> String {
        format!(
            "{}",
            self.0
                .iter()
                .map(|e| e.format())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(many1(Entity::parse), Pattern)(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expression::Expression, key_value::KeyValue, label::Label, line::Line, node::Node,
        literal::map::Map, relationship::Relationship,
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
                    properties: Some(Map(vec![KeyValue {
                        key: "a",
                        value: Expression::decimal_int("10"),
                    }])),
                }),
                Entity::Relationship(Relationship {
                    variable: Some("rel1"),
                    labels: vec![Label("ARelationship")],
                    properties: Some(Map(vec![KeyValue {
                        key: "r",
                        value: Expression::decimal_int("15"),
                    }])),
                    right_line: Line("-"),
                    left_line: Line("<-"),
                }),
                Entity::Node(Node {
                    variable: Some("var2"),
                    labels: vec![Label("BLabel")],
                    properties: Some(Map(vec![KeyValue {
                        key: "b",
                        value: Expression::decimal_int("20"),
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
