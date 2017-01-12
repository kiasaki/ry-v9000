use std::str::FromStr;

pub enum Token {
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

fn is_valid_symbol_char(c: char) -> bool {
    (c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
    c == '_' || c == '-' || c == '+' || c == '*' || c == '/'
}

pub fn format_token(t: &Token) -> String {
    match *t {
        Token::ParenOpen => "paren_open".to_string(),
        Token::ParenClose => "paren_close".to_string(),
        Token::Nil => "nil".to_string(),
        Token::Bool(ref v) => format!("bool[{}]", v),
        Token::Num(ref v) => format!("number[{}]", v),
        Token::Str(ref v) => format!("string[\"{}\"]", v),
        Token::Sym(ref v) => format!("symbol[{}]", v),
    }
}

struct Tokenizer {
    position: usize,
    input: String,
    tokens: Vec<Token>,
}

impl Tokenizer {
    fn new(input: String) -> Tokenizer {
        Tokenizer {
            position: 0,
            input: input,
            tokens: vec![],
        }
    }

    fn push(&mut self, t: Token) {
        self.tokens.push(t)
    }

    fn current(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn last(&self) -> Option<char> {
        self.input.chars().nth(self.position - 1)
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    fn backup(&mut self) -> Option<char> {
        self.position -= 1;
        self.current()
    }

    fn advance(&mut self) -> Option<char> {
        self.position += 1;
        self.current()
    }
}

pub fn tokenize(input: String) -> Vec<Token> {
    if input == "" {
        return vec![];
    }

    let mut tokenizer = Tokenizer::new(input);
    let mut ch = tokenizer.current();

    'top: while ch.is_some() {
        let mut c = ch.unwrap();

        if c == '(' || c == '[' {
            tokenizer.push(Token::ParenOpen);
            ch = tokenizer.advance();
            continue;
        }
        if c == ')' || c == ']' {
            tokenizer.push(Token::ParenClose);
            ch = tokenizer.advance();
            continue;
        }
        if is_valid_number_char(c) {
            let mut value = String::new();

            while is_valid_number_char(c) {
                value.push(c);
                match tokenizer.peek() {
                    // Handle EOF
                    None => {
                        if value == "-" {
                            break;
                        } else {
                            tokenizer.push(Token::Num(f64::from_str(&value).unwrap()));
                            break 'top;
                        }
                    }
                    _ => {}
                }
                ch = tokenizer.advance();
                c = ch.unwrap();
            }

            if value == "-" {
                ch = tokenizer.backup();
                c = ch.unwrap();
                // no contiue do that we let the symbol if handle this char
            } else {
                tokenizer.push(Token::Num(f64::from_str(&value).unwrap()));
                ch = tokenizer.advance();
                continue;
            }
        }
        if c == '"' {
            let mut value = String::new();

            // Move to 1st char after string openning ('"')
            ch = tokenizer.advance();
            if ch.is_none() {
                panic!("unterminated string literal at position {}",
                       tokenizer.position)
            }
            let mut c = ch.unwrap();

            while c != '"' || (c != '"' && tokenizer.last().unwrap() == '\\') {
                value.push(c);

                match tokenizer.peek() {
                    // Handle EOF
                    None => panic!("unterminated string literal starting with: '{}'", value),
                    _ => {}
                }

                ch = tokenizer.advance();
                c = ch.unwrap();
            }

            tokenizer.push(Token::Str(value));
            // Skip closing '"'
            tokenizer.advance();
            ch = tokenizer.advance();
            continue;
        }
        if is_valid_symbol_char(c) {
            let mut value = String::new();
            let mut c = c;

            while is_valid_symbol_char(c) {
                value.push(c);
                match tokenizer.peek() {
                    // Handle EOF
                    None => {
                        tokenizer.push(Token::Sym(value));
                        break 'top;
                    }
                    _ => {}
                }
                ch = tokenizer.advance();
                c = ch.unwrap();
            }

            tokenizer.push(Token::Sym(value));
            ch = tokenizer.advance();
            continue;
        }

        // Didn't match on anything, throw away and go on
        ch = tokenizer.advance();
    }

    tokenizer.tokens
}
