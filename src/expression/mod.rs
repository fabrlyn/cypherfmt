use nom::{combinator::map, IResult};

use crate::{
    atom::Atom,
    literal::{
        bool::Bool,
        integer::{Decimal, Integer},
        list::List,
        number::Number,
        Literal,
    },
};

#[derive(Debug, PartialEq)]
pub struct Expression<'a>(pub Atom<'a>);

impl<'a> Expression<'a> {
    pub fn format(&self) -> String {
        self.0.format()
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(Atom::parse, Expression)(input)
    }
}

#[cfg(test)]
impl<'a> Expression<'a> {
    pub fn decimal_int(i: &'a str) -> Self {
        Expression(Atom::Literal(Literal::Number(Number::Integer(
            Integer::Decimal(Decimal(i)),
        ))))
    }

    pub fn bool(b: bool) -> Self {
        Expression(Atom::Literal(Literal::Bool(Bool(b))))
    }

    pub fn list_of_decimal_ints(ints: &[&'a str]) -> Self {
        let ints = ints.iter().map(|s| Self::decimal_int(s)).collect();
        Expression(Atom::Literal(Literal::List(List(ints))))
    }
}
