use nom::{combinator::map, IResult};

use crate::{literal::Literal, parameter::Parameter, symbolic_name};

#[derive(Debug, PartialEq)]
pub enum Atom<'a> {
    Literal(Literal<'a>),
    Parameter(Parameter<'a>),
    Variable(&'a str),
}

impl<'a> Atom<'a> {
    pub fn format(&self) -> String {
        match self {
            Atom::Literal(l) => l.format(),
            Atom::Parameter(p) => p.format(),
            Atom::Variable(v) => v.to_string(),
        }
    }
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
    use crate::{
        expression::{
            calculable_expression::CalculableExpression,
            combinable_expression::CombinableExpression, Expression,
        },
        literal::{bool::Bool, list::List},
    };

    use super::*;

    #[test]
    fn parse_atom_literal() {
        let expected = Ok((" data", Atom::Literal(Literal::Bool(Bool(true)))));
        let actual = Atom::parse("TRUE data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_list_of_bool_literal() {
        let list = Atom::Literal(Literal::List(List(vec![Expression {
            expressions: vec![CombinableExpression {
                calculables: vec![CalculableExpression {
                    atom: Atom::Literal(Literal::Bool(Bool(true))),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        }])));
        let expected = Ok((" data", list));
        let actual = Atom::parse("[true] data");
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
