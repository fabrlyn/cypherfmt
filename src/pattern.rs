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
        key::Key, key_value::KeyValue, label::Label, line::Line, node::Node,
        properties::Properties, relationship::Relationship, value::Value,
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
