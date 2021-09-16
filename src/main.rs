mod atom;
mod cypher;
mod projection_body;
mod r#return;
mod single_part_query;
mod symbolic_name;
mod parameter;
mod clause;
mod entity;
mod expression;
mod key;
mod key_value;
mod label;
mod line;
mod literal;
mod node;
mod pattern;
mod properties;
mod relationship;
mod shared;
mod subclause;
mod token;
mod value;

fn main() {
    let query = "MATCH (n:SomeNode) WHERE n.id IN [1,2,3,4] RETURN n, true as someValue";
}
