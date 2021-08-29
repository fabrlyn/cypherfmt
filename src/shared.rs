use nom::{IResult, bytes::complete::tag};

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

