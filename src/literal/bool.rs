use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

#[derive(Debug, PartialEq)]
pub struct Bool<'a>(pub &'a str);

impl<'a> Bool<'a> {
    pub fn format(&self) -> String {
        self.0.to_string()
    }

    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(alt((tag("TRUE"), tag("FALSE"))), Bool)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bool_true() {
        let expected = Ok((" data", Bool("TRUE")));
        let actual = Bool::parse("TRUE data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_bool_false() {
        let expected = Ok((" data", Bool("FALSE")));
        let actual = Bool::parse("FALSE data");
        assert_eq!(expected, actual);
    }
}
