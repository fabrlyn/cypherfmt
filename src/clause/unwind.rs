use crate::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct Unwind<'a> {
    expressions: Vec<Expression<'a>>,
    variable: &'a str,
}
