use std::rc::Rc;
use std::collections::HashMap;

use ast::Node;

pub struct Environment {
    values: HashMap<String, Node>,
    parent: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new(parent: Option<Rc<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            parent: parent,
        }
    }

    pub fn set(&mut self, key: String, value: Node) {
        self.values.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<Node> {
        if self.values.contains_key(&key) {
            Some(self.values.get(&key).unwrap().clone())
        } else {
            match self.parent {
                Some(ref e) => e.get(key),
                None => None,
            }
        }
    }

    pub fn get_as_node(&self, key: String) -> Node {
        if self.values.contains_key(&key) {
            self.values.get(&key).unwrap().clone()
        } else {
            match self.parent {
                Some(ref e) => e.get_as_node(key),
                None => Node::Nil,
            }
        }
    }
}

pub fn eval(env: &mut Environment, ast: Node) -> Node {
    match ast {
        Node::Nil |
        Node::Bool(..) |
        Node::Num(..) |
        Node::Str(..) |
        Node::Error(..) |
        Node::BuiltIn(..) => ast,
        Node::Sym(v) => env.get_as_node(v),
        Node::List(v) => {
            if v.len() == 0 {
                Node::List(vec![]) // empty lists do nothing
            } else {
                let callee = eval(env, v.first().unwrap().clone());
                if let Node::Nil = callee {
                    Node::Error(format!("list with non-callable first item: {}",
                                        v.first().unwrap()))
                } else {
                    let args = v.iter().skip(1).map(|n| eval(env, n.clone())).collect();
                    eval_call(env, callee, args)
                }
            }
        }
    }
}

pub fn eval_call(env: &mut Environment, callee: Node, args: Vec<Node>) -> Node {
    match callee {
        Node::BuiltIn(builtin) => (builtin.f)(env, args),
        _ => Node::Error(format!("list with non-callable first item: {}", callee)),
    }
}
