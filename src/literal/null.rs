use nom::{bytes::complete::tag_no_case, combinator::map, IResult};

#[derive(Debug, PartialEq)]
pub struct Null;

impl Null {
    pub fn format(&self) -> String {
        "NULL".to_string()
    }

    pub fn format_mut(&self, out: &mut String) {
        out.push_str("NULL");
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(tag_no_case("NULL"), |_| Null)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_null() {
        let expected = Ok((" data", Null));
        let actual = Null::parse("null data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_null() {
        let expected = "NULL";
        let actual = Null.format();
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_mut_null() {
        let expected = "NULL";
        let mut actual = String::new();
        Null.format_mut(&mut actual);
        assert_eq!(expected, actual);
    }
}
