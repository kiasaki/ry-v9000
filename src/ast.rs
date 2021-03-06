use std::fmt;

use tokenizer::Token;
use lisp::Environment;

pub struct BuiltInFn {
    pub name: String,
    pub args_count: usize,
    pub uses_rest: bool,
    pub f: fn(&mut Environment, Vec<Node>) -> Node,
}

impl fmt::Debug for BuiltInFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "builtin#{}", self.name)
    }
}

impl fmt::Display for BuiltInFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "builtin#{}", self.name)
    }
}

impl PartialEq for BuiltInFn {
    fn eq(&self, other: &BuiltInFn) -> bool {
        self.name == other.name
    }
}

impl Clone for BuiltInFn {
    fn clone(&self) -> Self {
        BuiltInFn {
            name: self.name.clone(),
            args_count: self.args_count,
            uses_rest: self.uses_rest,
            f: self.f,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
    Sym(String),
    List(Vec<Node>),

    Error(String),
    BuiltIn(BuiltInFn),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_node(self.clone()))
    }
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
        Node::Error(ref v) => format!("ERROR {}", v),
        Node::BuiltIn(ref v) => format!("{}", v),
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

    fn build(&mut self) -> Result<Node, String> {
        let mut root: Vec<Node> = vec![Node::Sym("do".to_string())];

        while self.position < self.tokens.len() {
            match self.build_node() {
                Ok(n) => root.push(n),
                Err(mess) => return Err(mess),
            }
        }

        Ok::<Node, String>(Node::List(root))
    }

    fn build_node(&mut self) -> Result<Node, String> {
        let result = match self.current().unwrap() {
            Token::Nil => Ok(Node::Nil),
            Token::Bool(v) => Ok(Node::Bool(v)),
            Token::Num(v) => Ok(Node::Num(v)),
            Token::Str(v) => Ok(Node::Str(v)),
            Token::Sym(v) => Ok(Node::Sym(v)),
            Token::ParenOpen => {
                let mut list_items = vec![];
                let starting_position = self.position;

                // Skip opening parens
                self.advance();

                loop {
                    match self.current() {
                        Some(Token::ParenClose) => break,
                        Some(..) => {
                            match self.build_node() {
                                Ok(n) => list_items.push(n),
                                Err(mess) => return Err(mess),
                            }
                        }
                        None => {
                            return Err(format!("unmatched open parens starting at: {}",
                                               starting_position))
                        }
                    };
                }

                Ok(Node::List(list_items))
            }
            Token::ParenClose => {
                Err("found closing parens not matching openning parens".to_string())
            }
        };

        self.advance();
        result
    }

    fn advance(&mut self) {
        self.position += 1
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
