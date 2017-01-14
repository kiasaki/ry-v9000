extern crate rustbox;
extern crate ry;
extern crate termbox_sys as termbox;

use std::rc::Rc;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

use rustbox::{Color, RustBox};
use rustbox::Key;

use ry::tokenizer;
use ry::ast;
use ry::ast::Node;
use ry::lisp;
use ry::lisp::Environment;
use ry::builtins;

fn fatal(message: String) -> ! {
    unsafe {
        termbox::tb_shutdown();
    }
    println!("error: {}", message);
    std::process::exit(1)
}

fn parse_source_file(filename: String) -> Result<Node, String> {
    let mut file = match File::open(filename.clone()) {
        Ok(f) => f,
        Err(_) => return Err(format!("can't open file \"{}\" for reading", filename)),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => return Err(format!("can't read from file \"{}\"", filename)),
    };
    let tokens = tokenizer::tokenize(contents);

    ast::build(tokens)
}

fn setup_runtime(env: &mut Environment) {
    let ast = match parse_source_file("init.ry".to_string()) {
        Ok(v) => v,
        Err(mess) => fatal(mess),
    };

    lisp::eval(env, ast);
}

fn run(env: &mut Environment, code: String) -> Node {
    let tokens = tokenizer::tokenize(code);

    let ast = match ast::build(tokens) {
        Ok(a) => a,
        Err(mess) => fatal(mess),
    };
    lisp::eval(env, ast)
}

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Ok(v) => v,
        Err(e) => fatal(format!("rustbox: {}", e)),
    };

    let builtins_env = Some(Rc::new(builtins::new_builtins_environment()));
    let mut env = lisp::Environment::new(builtins_env);
    setup_runtime(&mut env);

    rustbox.present();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::ResizeEvent(w, h)) => {
                env.set("*ry-width*".to_string(), Node::Num(w as f64));
                env.set("*ry-height*".to_string(), Node::Num(h as f64));
            }
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Ctrl('q') => break,
                    _ => {
                        run(&mut env, format!("(handle-key \"{}\")", key_to_string(key)));
                    }
                }
            }
            Err(e) => panic!("{}", e.description()),
            _ => {}
        }
    }
}

fn key_to_string(key: rustbox::Key) -> String {
    match key {
        Key::Char(c) => c.to_string(),
        Key::Enter => "enter".to_string(),
        _ => "unknown".to_string(),
    }
}

// rustbox.print(1,
// 3,
// rustbox::RB_BOLD,
// Color::White,
// Color::Black,
// "Press 'q' to quit.");
//
