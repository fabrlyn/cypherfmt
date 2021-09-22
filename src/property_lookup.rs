use nom::{bytes::complete::tag, character::complete::space0, combinator::map, IResult};

use crate::symbolic_name;

#[derive(Debug, PartialEq)]
pub struct PropertyLookup<'a>(pub &'a str);

impl<'a> PropertyLookup<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, _) = tag(".")(input)?;
        let (input, _) = space0(input)?;
        map(symbolic_name::parse, PropertyLookup)(input)
    }

    pub fn format(&self) -> String {
        format!(".{}", self.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_property_lookup() {
        let expected = Ok((" data", PropertyLookup(".   someProperty")));
        let actual = PropertyLookup::parse(".   someProperty data");
        assert_eq!(expected, actual);
    }

    #[test]
    fn format_property_lookup() {
        let expected = ".someProperty";
        let actual = PropertyLookup::parse(".   someProperty data")
            .unwrap()
            .1
            .format();
        assert_eq!(expected, actual);
    }
}
