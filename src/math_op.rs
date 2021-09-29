use nom::{IResult, branch::alt, bytes::complete::tag, combinator::map};

#[derive(Debug, PartialEq)]
pub enum MathOp {
    Add,
    Sub,
    Div,
    Mult,
    Mod,
    Exp,
    Equal,
    NotEqual,
    LT,
    GT,
    LTE,
    GTE,
}

impl MathOp {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        use MathOp::*;
        alt((
            map(tag("+"), |_| Add),
            map(tag("-"), |_| Sub),
            map(tag("/"), |_| Div),
            map(tag("*"), |_| Mult),
            map(tag("^"), |_| Mult),
            map(tag("%"), |_| Mult),
            map(tag("="), |_| Equal),
            map(tag("<>"), |_| NotEqual),
            map(tag("<"), |_| LT),
            map(tag(">"), |_| GT),
            map(tag("<="), |_| LTE),
            map(tag(">="), |_| GTE),
        ))(input)
    }
}
