use nom::{
    bytes::complete::tag,
    IResult, Parser,
};

pub fn is_alphanumeric(c: char) -> bool {
    nom::character::is_alphanumeric(c as u8)
}

pub fn is_alphabetic(c: char) -> bool {
    nom::character::is_alphabetic(c as u8)
}

pub fn double_qoute(input: &str) -> IResult<&str, &str> {
    tag("\"")(input)
}

pub fn single_qoute(input: &str) -> IResult<&str, &str> {
    tag("'")(input)
}

pub fn optional<'a, E, P, T>(
    mut parser: P,
) -> impl FnMut(&'a str) -> IResult<&'a str, Option<T>, E>
where
    P: Parser<&'a str, T, E>,
{
    move |input: &str| {
        if let Ok((input, result)) = parser.parse(input) {
            return Ok((input, Some(result)));
        }
        Ok((input, None))
    }
}

pub fn optional_signed(input: &str) -> IResult<&str, Option<&str>> {
    optional(tag("-"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_optional_signed() {
        let expected = Ok(("123", Some("-")));
        let actual = optional_signed("-123");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_optional_signed_missing() {
        let expected = Ok(("123", None));
        let actual = optional_signed("123");
        assert_eq!(expected, actual);
    }
}
