use expression::Expression;
use node::Node;

use crate::cypher::Cypher;

mod atom;
mod clause;
mod cypher;
mod entity;
mod expression;
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

fn basic_query() {
    let query = "MATCH (n:SomeNode)-[r:Rel]-(m:OtherNode) RETURN [1,2,3]";
    let formatted = Cypher::parse(query).unwrap().1;
    println!("{:?}", formatted);
    println!("{}", formatted.format());
}

fn list_literal() {
    let formatted = Expression::parse("[  1   ,2, 3 , 4]").unwrap().1.format();
    println!("{}", formatted);
}

fn properties_literal() {
    let formatted = Expression::parse("{   a: 10, b:20,c:40}")
        .unwrap()
        .1
        .format();
    println!("{}", formatted);
}

fn node_literal() {
    let formatted = Node::parse("( a : ANode:BNode :CNode: DNode{ a:10, b: 'abc', c: [1,2,3]})")
        .unwrap()
        .1
        .format();
    println!("{}", formatted);
}

fn main() {
    //basic_query();
    //list_literal();
    //properties_literal();
    node_literal();
}
