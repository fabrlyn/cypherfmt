use nom::{branch::alt, bytes::complete::tag_no_case, combinator::map, IResult};

#[derive(Debug, PartialEq)]
pub enum Combinator {
    Or,
    And,
    Xor,
}

impl Combinator {
    fn parse_or(input: &str) -> IResult<&str, Self> {
        map(tag_no_case("OR"), |_| Combinator::Or)(input)
    }

    fn parse_and(input: &str) -> IResult<&str, Self> {
        map(tag_no_case("AND"), |_| Combinator::And)(input)
    }

    fn parse_xor(input: &str) -> IResult<&str, Self> {
        map(tag_no_case("XOR"), |_| Combinator::Xor)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_or, Self::parse_and, Self::parse_xor))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_combinator_or() {
        let expected = Ok((" data", Combinator::Or));
        let actual = Combinator::parse("or data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_combinator_and() {
        let expected = Ok((" data", Combinator::And));
        let actual = Combinator::parse("and data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_combinator_xor() {
        let expected = Ok((" data", Combinator::Xor));
        let actual = Combinator::parse("xor data");
        assert_eq!(expected, actual);
    }
}
