use nom::{
    bytes::complete::tag_no_case, character::complete::space1, combinator::map, sequence::tuple,
    IResult,
};

use crate::projection_body::ProjectionBody;

#[derive(Debug, PartialEq)]
pub struct Return<'a>(ProjectionBody<'a>);

impl<'a> Return<'a> {
    pub fn format(&self) -> String {
        format!("RETURN {}", self.0.format())
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            tuple((tag_no_case("RETURN"), space1, ProjectionBody::parse)),
            |(_, _, result)| Return(result),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expression::Expression,
        projection_body::{ProjectionBody, ProjectionItem},
        r#return::Return,
    };

    #[test]
    fn parse_return_list_of_integer_decimals() {
        let list = Expression::list_of_decimal_ints(&["10", "11", "12"]);

        let expected = Ok((
            "data",
            Return(ProjectionBody {
                distinct: false,
                wild_card: false,
                projection_items: vec![ProjectionItem {
                    expression: list,
                    variable: None,
                }],
                sort_expressions: vec![],
                skip_expression: None,
                limit_expression: None,
            }),
        ));
        let actual = Return::parse("RETURN [10, 11, 12] data");
        assert_eq!(expected, actual);
    }
}
