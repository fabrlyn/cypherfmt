use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Count;

impl Count {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                tag_no_case("COUNT"),
                space0,
                tag("("),
                space0,
                tag("*"),
                space0,
                tag(")"),
            )),
            |_| Count,
        )(input)
    }

    pub fn format(&self) -> String {
        format!("COUNT (*)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_count() {
        let expected = Ok((" data", Count));
        let actual = Count::parse("count ( *  ) data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_count() {
        let expected = "COUNT (*)";
        let actual = Count::parse("count ( *   )").unwrap().1.format();
        assert_eq!(expected, actual);
    }
}
