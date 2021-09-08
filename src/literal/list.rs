use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list0, sequence::delimited, IResult,
};

use crate::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct List<'a>(pub Vec<Expression<'a>>);

impl<'a> List<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("["),
                separated_list0(tag(", "), Expression::parse),
                tag("]"),
            ),
            List,
        )(input)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        atom::Atom,
        literal::{
            integer::{Decimal, Integer},
            number::Number,
            Literal,
        },
    };

    use super::*;

    #[test]
    fn parse_list_of_decimals() {
        let expected = Ok((
            " data",
            List(vec![
                Expression(Atom::Literal(Literal::Number(Number::Integer(
                    Integer::Decimal(Decimal("10")),
                )))),
                Expression(Atom::Literal(Literal::Number(Number::Integer(
                    Integer::Decimal(Decimal("11")),
                )))),
                Expression(Atom::Literal(Literal::Number(Number::Integer(
                    Integer::Decimal(Decimal("12")),
                )))),
            ]),
        ));

        let actual = List::parse("[10, 11, 12] data");
        assert_eq!(expected, actual);
    }
}
