use nom::{combinator::map, IResult};

use crate::{atom::Atom, pattern::Pattern};

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
