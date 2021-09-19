use crate::cypher::Cypher;

mod atom;
mod clause;
mod cypher;
mod entity;
mod expression;
mod key;
mod key_value;
mod label;
mod line;
mod literal;
mod node;
mod parameter;
mod pattern;
mod projection_body;
mod properties;
mod relationship;
mod r#return;
mod shared;
mod single_part_query;
mod subclause;
mod symbolic_name;
mod token;
mod value;

fn main() {
    let query = "MATCH (n:SomeNode) RETURN n";
    let formatted = Cypher::parse(query).unwrap().1;
    println!("{}", formatted.format());
}
