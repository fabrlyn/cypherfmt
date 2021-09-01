use nom::{bytes::complete::tag, character::complete::space0, IResult};

use crate::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct Where<'a>(Vec<Expression<'a>>);

impl<'a> Where<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, _) = tag("WHERE")(input)?;
        let (input, _) = space0(input)?;
        let (input, expression) = Expression::parse(input)?;
        Ok((input, Where(vec![expression])))
    }
}

#[cfg(test)]
mod tests {
    use crate::{entity::Entity, label::Label, node::Node, pattern::Pattern};

    use super::*;

    #[test]
    fn parse_where() {
        let actual = Ok((
            " data",
            Where(vec![Expression::Pattern(Pattern(vec![Entity::Node(
                Node {
                    variable: Some("a"),
                    labels: vec![Label("ALabel")],
                    properties: None,
                },
            )]))]),
        ));

        let expected = Where::parse("WHERE (a:ALabel) data");
        assert_eq!(expected, actual);
    }
}
