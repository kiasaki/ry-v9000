#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate rustbox;

use std::str::FromStr;
use std::error::Error;
use std::default::Default;

use regex::Regex;

use rustbox::{Color, RustBox};
use rustbox::Key;

enum Token {
    ParenOpen,
    ParenClose,
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
    Sym(String),
}

fn is_valid_number_char(c: char) -> bool {
    (c >= '0' && c <= '9') || c == '-' || c == '.'
}

fn format_token(t: &Token) -> String {
    match *t {
        Token::ParenOpen => "paren_open".to_string(),
        Token::ParenClose => "paren_close".to_string(),
        Token::Nil => "nil".to_string(),
        Token::Bool(ref value) => format!("bool[{}]", value),
        Token::Num(ref v) => format!("number[{}]", v),
        Token::Str(ref v) => format!("string[\"{}\"]", v),
        Token::Sym(ref v) => format!("symbol[{}]", v),
    }
}

fn tokenize(input: String) -> Vec<Token> {
    lazy_static! {
        static ref NUMBER_RE: Regex = Regex::new(r"^\d+(\.\d+)?$").unwrap();
    }

    let mut tokens = vec![];
    let mut i = 0;

    while i < input.len() {
        let c = input.chars().nth(i).unwrap();

        if c == '(' || c == '[' {
            tokens.push(Token::ParenOpen)
        } else if c == ')' || c == ']' {
            tokens.push(Token::ParenClose)
        } else if is_valid_number_char(c) {
            let mut value = String::new();
            loop {
                value.push(c);
                i += 1;
                let c = input.chars().nth(i).unwrap();

                if i > input.len() || !is_valid_number_char(c) {
                    // Backup the one char too far we went and add token
                    i -= 1;

                    // ensure the value is not just "-" which is a valid number char
                    // but, alone, not a valid number
                    if value == "-" {
                        i -= 1;
                        break;
                    }

                    // TODO handle parsing error
                    tokens.push(Token::Num(f64::from_str(&value).unwrap()));
                    break;
                }
            }
        } else if c == '"' {
            let mut value = String::new();
            loop {
                value.push(c);
                i += 1;
                let c = input.chars().nth(i).unwrap();

                if i > input.len() || (c == '"' && input.chars().nth(i - 1).unwrap() != '\\') {
                    tokens.push(Token::Str(value));
                    break;
                }
            }
        };

        i += 1
    }

    tokens
}

fn main() {
    let tokens = tokenize("(+ 2 (- 5 1))".to_string());
    for t in &tokens {
        print!("{}", format_token(t));
        print!(" ");
    }
    println!("");
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
