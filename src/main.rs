extern crate rustbox;
extern crate ry;

use std::rc::Rc;
use std::error::Error;

use rustbox::{Color, RustBox};
use rustbox::Key;

use ry::tokenizer;
use ry::ast;
use ry::lisp;
use ry::builtins;

fn main() {
    let tokens = tokenizer::tokenize("1\n2 \"asd\" () (+ 2 (- 5 1))".to_string());

    match ast::build(tokens) {
        Ok(ast) => {
            println!("ast: {}", ast::format_node(ast.clone()));

            let mut env =
                lisp::Environment::new(Some(Rc::new(builtins::new_builtins_environment())));
            let result = lisp::eval(&mut env, ast.clone());
            println!("result: {}", ast::format_node(result))
        }
        Err(mess) => println!("error: {}", mess),
    }
}

fn rustbox_example() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1,
                  1,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  "Hello, world!");
    rustbox.print(1,
                  3,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  "Press 'q' to quit.");
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e.description()),
            _ => {}
        }
    }
}
