use ast::{Node, BuiltInFn};
use lisp::Environment;
use lisp::eval;

fn register_builtin(env: &mut Environment,
                    name: &str,
                    args_count: usize,
                    uses_rest: bool,
                    f: fn(&mut Environment, Vec<Node>) -> Node) {

    env.set(name.to_string(),
            Node::BuiltIn(BuiltInFn {
                name: name.to_string(),
                args_count: args_count,
                uses_rest: uses_rest,
                f: f,
            }));
}

pub fn new_builtins_environment() -> Environment {
    let mut env = Environment::new(None);

    register_builtin(&mut env, "+", 2, false, l_plus);
    register_builtin(&mut env, "-", 2, false, l_minus);
    register_builtin(&mut env, "do", 1, true, l_do);

    env
}

pub fn l_plus(env: &mut Environment, args: Vec<Node>) -> Node {
    if let (Some(&Node::Num(a)), Some(&Node::Num(b))) = (args.get(0), args.get(1)) {
        Node::Num(a + b)
    } else {
        Node::Error("+: non numeric or wrong number of arguments given".to_string())
    }
}

pub fn l_minus(env: &mut Environment, args: Vec<Node>) -> Node {
    if let (Some(&Node::Num(a)), Some(&Node::Num(b))) = (args.get(0), args.get(1)) {
        Node::Num(a - b)
    } else {
        Node::Error("+: non numeric or wrong number of arguments given".to_string())
    }
}

pub fn l_do(env: &mut Environment, args: Vec<Node>) -> Node {
    let mut last_result = Node::Nil;

    for arg in args {
        last_result = eval(env, arg);
    }

    last_result
}
