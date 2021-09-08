use nom::{bytes::complete::tag, combinator::map, IResult};

#[derive(Debug, PartialEq)]
pub struct Null<'a>(pub &'a str);

impl<'a> Null<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(tag("NULL"), Null)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_null() {
        let expected = Ok((" data", Null("NULL")));
        let actual = Null::parse("NULL data");
        assert_eq!(expected, actual);
    }
}
