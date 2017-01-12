use std::rc::Rc;
use itertools::Itertools;

use tokenizer::Token;

#[derive(Clone)]
pub enum Node {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
    Sym(String),
    List(Rc<Node>, Rc<Node>),
}

pub fn node_is_nil(n: Node) -> bool {
    match n {
        Node::Nil => true,
        _ => false,
    }
}

pub fn node_is_list(n: Node) -> bool {
    match n {
        Node::List(..) => true,
        _ => false,
    }
}

pub fn list_to_vec(n: Node) -> Vec<Node> {
    if let Node::List(ref first, ref second) = n {
        let mut list: Vec<Node> = Vec::new();
        list.push((**first).clone());

        let mut rest = second;
        while !node_is_nil((**rest).clone()) {
            if let Node::List(ref first, ref second) = n {
                list.push((**first).clone());
                rest = second;
            } else {
                panic!("list_to_vec: given list had a cdr that is not nil or a list")
            }
        }

        list
    } else {
        panic!("list_to_vec: given non-list node")
    }
}

pub fn format_node(n: Node) -> String {
    match n {
        Node::Nil => "nil".to_string(),
        Node::Bool(ref v) => format!("{}", v),
        Node::Num(ref v) => format!("{}", v),
        Node::Str(ref v) => format!("\"{}\"", v),
        Node::Sym(ref v) => format!("{}", v),
        Node::List(..) => {
            let formated_nodes: Vec<String> = list_to_vec(n)
                .iter()
                .map(|v| format_node(v.clone()))
                .collect();
            format!("({})", formated_nodes.join(" "))
        }
    }
}

fn append(n1: Node, n2: Node) -> Node {
    if let Node::Nil = n1 {
        Node::List(Rc::new(n2), Rc::new(Node::Nil))
    } else {
        Node::Nil
    }
}

pub fn build(tokens: Vec<Token>) -> Node {
    let mut root = Node::Nil;

    for t in tokens {
        match t {
            Token::Nil => "nil".to_string(),
            Token::Bool(ref v) => format!("{}", v),
            Token::Num(ref v) => format!("{}", v),
            Token::Str(ref v) => format!("\"{}\"", v),
            Token::Sym(ref v) => format!("{}", v),
        }
    }
}
