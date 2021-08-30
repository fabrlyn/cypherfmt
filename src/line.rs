use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

#[derive(Debug, PartialEq)]
pub struct Line<'a>(pub &'a str);

impl<'a> Line<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(alt((tag("<-"), tag("->"), tag("-"))), Line)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_undirected() {
        let expected = Ok((" data", Line("-")));
        let actual = Line::parse("- data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_line_left() {
        let expected = Ok((" data", Line("<-")));
        let actual = Line::parse("<- data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_line_right() {
        let expected = Ok((" data", Line("->")));
        let actual = Line::parse("-> data");
        assert_eq!(expected, actual);
    }
}
