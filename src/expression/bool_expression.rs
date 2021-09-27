use nom::{
    character::complete::{space0, space1},
    combinator::map,
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::{atom::Atom, label::Label, property_lookup::PropertyLookup};

use super::bool_keyword::BoolKeyword;

#[derive(Debug, PartialEq)]
pub struct BoolExpression<'a> {
    pub keyword: BoolKeyword,
    pub atom: Atom<'a>,
    pub property_lookups: Vec<PropertyLookup<'a>>,
    pub labels: Vec<Label<'a>>,
}

impl<'a> BoolExpression<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, keyword) =
            map(tuple((BoolKeyword::parse, space1)), |(result, _)| result)(input)?;

        let (input, atom) = map(tuple((Atom::parse, space1)), |(result, _)| result)(input)?;

        let (input, property_lookups) = many0(PropertyLookup::parse)(input)?;

        let (input, labels) = many0(map(
            tuple((space0, Label::parse, space0)),
            |(_, result, _)| result,
        ))(input)?;

        Ok((
            input,
            BoolExpression {
                keyword,
                atom,
                property_lookups,
                labels,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::literal::{string, Literal};

    use super::*;

    #[test]
    fn parse_bool_expression_starts_with() {
        let expected = Ok((
            "data",
            BoolExpression {
                keyword: BoolKeyword::StartsWith,
                atom: Atom::Literal(Literal::String(string::String("'Pet'"))),
                property_lookups: vec![],
                labels: vec![],
            },
        ));
        let actual = BoolExpression::parse("STARTS WITH 'Pet' data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_bool_expression_ends_with() {
        let expected = Ok((
            "data",
            BoolExpression {
                keyword: BoolKeyword::EndsWith,
                atom: Atom::Literal(Literal::String(string::String("'Pet'"))),
                property_lookups: vec![],
                labels: vec![],
            },
        ));
        let actual = BoolExpression::parse("Ends With 'Pet' data");
        assert_eq!(expected, actual);
    }
}
