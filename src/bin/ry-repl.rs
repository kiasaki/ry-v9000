extern crate ry;

use std::rc::Rc;
use std::io::{self, BufRead, Write};

use ry::builtins;
use ry::tokenizer;
use ry::ast::{self, Node};
use ry::lisp::{self, Environment};

fn run(env: &mut Environment, code: String) -> Node {
    let tokens = tokenizer::tokenize(code);

    let ast = match ast::build(tokens) {
        Ok(a) => a,
        Err(mess) => Node::Error(format!("ast: {}", mess)),
    };
    lisp::eval(env, ast)
}

#[allow(unused_must_use)]
fn main() {
    let builtins_env = Some(Rc::new(builtins::new_builtins_environment()));
    let mut env = lisp::Environment::new(builtins_env);
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        stdout.write(b"> ");
        stdout.flush();
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let res = run(&mut env, line.trim().to_string());
        println!("{}", res);
    }
}
