use nom::{branch::alt, bytes::complete::tag, character::complete::space0, IResult};

use crate::{pattern::Pattern, subclause::r#where::Where};

#[derive(Debug, PartialEq)]
pub struct Match<'a> {
    optional: bool,
    patterns: Vec<Pattern<'a>>,
    r#where: Option<Where<'a>>,
}

impl<'a> Match<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, keyword) = alt((tag("MATCH"), tag("OPTIONAL MATCH")))(input)?;
        let optional = keyword == "OPTIONAL MATCH";
        let (input, _) = space0(input)?;
        let (input, pattern) = Pattern::parse(input)?;

        Ok((
            input,
            Match {
                optional,
                patterns: vec![pattern],
                r#where: None,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{entity::Entity, label::Label, node::Node};

    use super::*;

    #[test]
    fn parse_match() {
        let expected = Ok((
            " data",
            Match {
                optional: false,
                patterns: vec![Pattern(vec![Entity::Node(Node {
                    variable: Some("a"),
                    properties: None,
                    labels: vec![Label("ALabel")],
                })])],
                r#where: None,
            },
        ));

        let actual = Match::parse("MATCH (a:ALabel) data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_match_optional() {
        let expected = Ok((
            " data",
            Match {
                optional: true,
                patterns: vec![Pattern(vec![Entity::Node(Node {
                    variable: Some("a"),
                    properties: None,
                    labels: vec![Label("ALabel")],
                })])],
                r#where: None,
            },
        ));

        let actual = Match::parse("OPTIONAL MATCH (a:ALabel) data");
        assert_eq!(expected, actual);
    }
}
