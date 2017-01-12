extern crate rustbox;
extern crate ry;

use rustbox::{Color, RustBox};
use rustbox::Key;
use std::error::Error;

use ry::tokenizer;
use ry::ast;

fn main() {
    let tokens = tokenizer::tokenize("(+ 2 (- 5 1))".to_string());
    let ast = ast::build(tokens);
    println!("{}", ast::format_node(ast));
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
