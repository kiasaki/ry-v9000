use std::rc::Rc;
use std::slice::Iter;

use tokenizer::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
    Sym(String),
    List(Vec<Node>),
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

pub fn format_node(n: Node) -> String {
    match n {
        Node::Nil => "nil".to_string(),
        Node::Bool(ref v) => format!("{}", v),
        Node::Num(ref v) => format!("{}", v),
        Node::Str(ref v) => format!("\"{}\"", v),
        Node::Sym(ref v) => format!("{}", v),
        Node::List(ref v) => {
            let formated_nodes: Vec<String> = v.iter()
                .map(|n| format_node(n.clone()))
                .collect();
            format!("({})", formated_nodes.join(" "))
        }
    }
}

struct Builder {
    position: usize,
    tokens: Vec<Token>,
}

impl Builder {
    fn new(tokens: Vec<Token>) -> Builder {
        Builder {
            position: 0,
            tokens: tokens,
        }
    }

    fn build(&self) -> Result<Node, String> {
        let mut root: Vec<Node> = vec![Node::Sym("do".to_string())];

        while self.position < self.tokens.len() {
            match self.build_node() {
                Ok(n) => root.push(n),
                Err(mess) => return Err(mess),
            }
        }

        Ok::<Node, String>(Node::List(root))
    }

    fn build_node(&self) -> Result<Node, String> {
        match self.current().unwrap() {
            Token::Nil => Ok(Node::Nil),
            Token::Bool(v) => Ok(Node::Bool(v)),
            Token::Num(v) => Ok(Node::Num(v)),
            Token::Str(v) => Ok(Node::Str(v)),
            Token::Sym(v) => Ok(Node::Sym(v)),
            Token::ParenOpen => {
                let mut list_items = vec![];
                while self.current().unwrap() != Token::ParenClose {
                    match self.build_node() {
                        Ok(n) => list_items.push(n),
                        Err(mess) => return Err(mess),
                    }
                }
                Ok(Node::List(list_items))
            }
            Token::ParenClose => {
                Err("found closing parens not matching openning parens".to_string())
            }
        }
    }

    fn current(&self) -> Option<Token> {
        if self.position < self.tokens.len() {
            Some(self.tokens[self.position].clone())
        } else {
            None
        }
    }
}

pub fn build(tokens: Vec<Token>) -> Result<Node, String> {
    Builder::new(tokens).build()
}
