use nom::{branch::alt, bytes::complete::tag_no_case, combinator::map, IResult};

#[derive(Debug, PartialEq)]
pub struct Bool(pub bool);

impl Bool {
    pub fn format(&self) -> String {
        match self.0 {
            true => "TRUE",
            false => "FALSE",
        }
        .to_string()
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            alt((
                map(tag_no_case("TRUE"), |_| true),
                map(tag_no_case("FALSE"), |_| false),
            )),
            Bool,
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bool_true() {
        let expected = Ok((" data", Bool(true)));
        let actual = Bool::parse("true data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_bool_false() {
        let expected = Ok((" data", Bool(false)));
        let actual = Bool::parse("false data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_bool_false() {
        let expected = "FALSE";
        let actual = Bool(false).format();
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_bool_true() {
        let expected = "TRUE";
        let actual = Bool(true).format();
        assert_eq!(expected, actual);
    }
}
