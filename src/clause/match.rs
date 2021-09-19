use nom::{branch::alt, bytes::complete::tag, character::complete::space0, IResult};

use crate::{pattern::Pattern, subclause::r#where::Where};

#[derive(Debug, PartialEq)]
pub struct Match<'a> {
    optional: bool,
    patterns: Vec<Pattern<'a>>,
    r#where: Option<Where<'a>>,
}

impl<'a> Match<'a> {
    fn optional_str(&self) -> &str {
        if self.optional {
            return "OPTIONAL ";
        }
        ""
    }
    pub fn format(&self) -> String {
        format!(
            "{}MATCH\n{}\n{}\n",
            self.optional_str(),
            self.patterns
                .iter()
                .map(|p| p.format())
                .collect::<Vec<_>>()
                .join(" "),
            self.r#where.as_ref().map(|w| w.format()).unwrap_or("".to_string())
        )
    }
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
