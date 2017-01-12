use std::rc::Rc;
use itertools::Itertools;

#[derive(Clone)]
pub enum Node {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
    Sym(String),
    List(Rc<Node>, Rc<Node>),
}

pub fn node_is_nil(n: &Node) -> bool {
    match *n {
        Node::Nil => true,
        _ => false,
    }
}

pub fn node_is_list(n: &Node) -> bool {
    match *n {
        Node::List(..) => true,
        _ => false,
    }
}

pub fn list_to_vec(n: &Node) -> Vec<Node> {
    if let Node::List(ref first, ref second) = *n {
        let mut list: Vec<Node> = Vec::new();
        let ref unboxed_first: Node = **first;
        list.push(unboxed_first.clone());

        let mut rest = second;
        while !node_is_nil(&rest) {
        }

        list
    } else {
        panic!("list_to_vec: given non-list node")
    }
}

pub fn format_node(n: &Node) -> String {
    match *n {
        Node::Nil => "nil".to_string(),
        Node::Bool(ref v) => format!("{}", v),
        Node::Num(ref v) => format!("{}", v),
        Node::Str(ref v) => format!("\"{}\"", v),
        Node::Sym(ref v) => format!("{}", v),
        Node::List(..) => {
            let formated_nodes: Vec<String> = list_to_vec(n)
                .iter()
                .map(format_node)
                .collect();
            format!("({})", formated_nodes.join(" "))
        }
    }
}
