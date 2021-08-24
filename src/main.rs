use std::error::Error;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::{alpha0, alpha1},
    combinator::map_res,
    multi::many0,
    sequence::delimited,
    IResult,
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Variable(String);

#[derive(Debug)]
struct Label(String);

#[derive(Debug)]
struct Node {
    variable: Option<Variable>,
    labels: Vec<Label>,
}

#[derive(Debug, PartialEq)]
enum Line {
    Undirected,
    AnonymousRelationship,
    Right,
    RightAnonymousRelationship,
    LeftAnonymousRelationship,
    Left,
}

#[derive(Debug)]
struct Relationship {
    variable: Option<Variable>,
    labels: Vec<Label>,
}

#[derive(Debug)]
enum Pattern {
    Node(Node),
    Line(Line),
    Relationship(Relationship),
}

#[derive(Debug)]
struct PathPattern {
    pattern: Vec<Pattern>,
}

enum Expresion {
    PathPattern(),
}

#[derive(Debug)]
enum Number {}

#[derive(Debug, PartialEq)]
enum Clause {
    Match,
    Return,
}

struct Query {}

fn from_keyword(input: &str) -> Result<Clause, String> {
    match input.to_lowercase() {
        i if i == "match" => Ok(Clause::Match),
        i if i == "return" => Ok(Clause::Return),
        _ => Err("Not a keyword".to_string()),
    }
}

fn keyword(input: &str) -> IResult<&str, &str> {
    alt((tag("match"), tag("return")))(input)
}

fn fmt(query: &str) {
    let result: IResult<&str, &str> = tag("match")(query);

    let (rest, m) = result.as_ref().unwrap();
    let result: IResult<_, _> = take_until("return")(rest.clone());
}

fn path_pattern(input: &str) -> IResult<&str, PathPattern> {
    let (input, node) = node(input)?;
    let (input, line) = line(input)?;
    let (input, relationship) = relationship(input)?;
    let pattern = vec![
        Pattern::Node(node),
        Pattern::Line(line),
        Pattern::Relationship(relationship),
    ];
    Ok((input, PathPattern { pattern }))
}

fn label(input: &str) -> IResult<&str, Label> {
    let (input, _) = tag(":")(input)?;
    let (input, label) = alpha1(input)?;
    Ok((input, Label(label.to_string())))
}

fn relationship(input: &str) -> IResult<&str, Relationship> {
    let (_, input) = delimited(tag("["), is_not("]"), tag("]"))(input)?;

    let (input, variable) = alpha0(input)?;
    let (input, labels) = many0(label)(input)?;

    let variable = if variable.len() == 0 {
        None
    } else {
        Some(Variable(variable.to_string()))
    };

    Ok((input, Relationship { variable, labels }))
}

fn from_line(input: &str) -> Result<Line, Box<dyn Error>> {
    match input {
        "-" => Ok(Line::Undirected),
        _ => Err("Not a line".into()),
    }
}

fn line(input: &str) -> IResult<&str, Line> {
    map_res(tag("-"), from_line)(input)
}

fn node(input: &str) -> IResult<&str, Node> {
    let (_, input) = delimited(tag("("), is_not(")"), tag(")"))(input)?;

    let (input, variable) = alpha0(input)?;
    let (input, labels) = many0(label)(input)?;

    let variable = if variable.len() == 0 {
        None
    } else {
        Some(Variable(variable.to_string()))
    };

    Ok((input, Node { variable, labels }))
}

#[cfg(test)]
mod tests {
    use crate::Clause;

    #[test]
    fn keyword() {
        assert_eq!(
            super::keyword("match (n) return"),
            Ok((" (n) return", "match"))
        );
    }

    #[test]
    fn fmt_query() {
        super::fmt("match (n:Node)-[r:Relationship]-(m:AnotherNode) return *");
    }

    #[test]
    fn from_keyword() {
        let keywords = vec!["match", "return", "notakeyword"];
        let actual = keywords
            .into_iter()
            .map(super::from_keyword)
            .collect::<Vec<_>>();

        let expected = vec![
            Ok(Clause::Match),
            Ok(Clause::Return),
            Err("Not a keyword".to_string()),
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn nom_node() {
        let node_str = "(n:Node)";
        let node = super::node(node_str);
        println!("node: {:?}", node);

        let node_str = "(n:Node:Another)";
        let node = super::node(node_str);
        println!("node: {:?}", node);

        let node_str = "(:Node:Another)";
        let node = super::node(node_str);
        println!("node: {:?}", node);

        let node_str = "(n)";
        let node = super::node(node_str);
        println!("node: {:?}", node);
    }

    #[test]
    fn nom_relationship() {
        let rel_str = "[r]";
        let rel = super::relationship(rel_str);
        println!("relationship: {:?}", rel);

        let rel_str = "[r:Relationship]";
        let rel = super::relationship(rel_str);
        println!("relationship: {:?}", rel);

        let rel_str = "[:Relationship]";
        let rel = super::relationship(rel_str);
        println!("relationship: {:?}", rel);

        let rel_str = "[:Relationship:SomeOther]";
        let rel = super::relationship(rel_str);
        println!("relationship: {:?}", rel);

        let rel_str = "[r:Relationship:SomeOther]";
        let rel = super::relationship(rel_str);
        println!("relationship: {:?}", rel);
    }

    #[test]
    fn nom_line() {
        let line_str = "-";
        let (_, line) = super::line(line_str).unwrap();
        assert_eq!(super::Line::Undirected, line);
    }

    #[test]
    fn nom_path_pattern() {
        let pattern_str = "(n:Node)-[r:Relationship]";
        let path_pattern = super::path_pattern(pattern_str);
        println!("path_pattern: {:?}", path_pattern);
    }
}
