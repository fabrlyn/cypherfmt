use nom::{combinator::map, IResult};

use crate::{literal::Literal, parameter::Parameter, symbolic_name};

#[derive(Debug, PartialEq)]
pub enum Atom<'a> {
    Literal(Literal<'a>),
    Parameter(Parameter<'a>),
    Variable(&'a str),
}

impl<'a> Atom<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        if let Ok((input, literal)) = Literal::parse(input) {
            return Ok((input, Atom::Literal(literal)));
        }

        if let Ok((input, parameter)) = Parameter::parse(input) {
            return Ok((input, Atom::Parameter(parameter)));
        }

        map(symbolic_name::parse, Atom::Variable)(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::literal::bool::Bool;

    use super::*;

    #[test]
    fn parse_atom_literal() {
        let expected = Ok((" data", Atom::Literal(Literal::Bool(Bool("TRUE")))));
        let actual = Atom::parse("TRUE data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_atom_parameter() {
        let expected = Ok((" data", Atom::Parameter(Parameter("$aParameter"))));
        let actual = Atom::parse("$aParameter data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_atom_variable() {
        let expected = Ok((" data", Atom::Variable("someNode")));
        let actual = Atom::parse("someNode data");
        assert_eq!(expected, actual);
    }
}
