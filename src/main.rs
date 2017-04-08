extern crate regex;

use std::io;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Token{
   Add,
   Sub,
   Mul,
   Div,
   Equal,
   ID(char),
   Literal(i32)
}

fn tokenize_stream<T>(x: &mut T) -> io::Result<Vec<Token>> where T: io::Read {
    let mut input = String::new();
    let mut to_return: Vec<Token> = Vec::new();

    x.read_to_string(&mut input)?;

    let mut number = 0;
    for ch in input.chars() {
         match ch {
            '+' => {
                if number != 0 {
                    to_return.push(Token::Literal(number));
                    number = 0;
                }
                to_return.push(Token::Add);
            },
            '-' => {
                if number != 0 {
                    to_return.push(Token::Literal(number));
                    number = 0;
                }
                to_return.push(Token::Sub)
            },
            '*' => {
                if number != 0 {
                    to_return.push(Token::Literal(number));
                    number = 0;
                }
                to_return.push(Token::Mul)
            }
            '/' | '\\' => {
                if number != 0 {
                    to_return.push(Token::Literal(number));
                    number = 0;
                }
                to_return.push(Token::Div)
            }
            '=' => {
                if number != 0 {
                    to_return.push(Token::Literal(number));
                    number = 0;
                }
                to_return.push(Token::Equal);
            }
             _ if ch.is_whitespace() => {}
             _ if ch.is_digit(10) => {
                number = number * 10 + (ch.to_digit(10).unwrap() as i32);
             }
             _ => {
                if number != 0 {
                    to_return.push(Token::Literal(number));
                    number = 0;
                }
                to_return.push(Token::ID(ch));
             }
         }
    }
    if number != 0 {
        to_return.push(Token::Literal(number));
    }

    Ok(to_return)
}

struct Parser{
    tokens: Vec<Token>,
    lookahead: Token,
    table: HashMap<char, i32>,
}

impl Parser{
    fn new(mut arg: Vec<Token>) -> Parser {
        let e = arg.remove(0);
        Parser{ tokens: arg, lookahead: e, table: HashMap::new()}
    }

    fn literal(&mut self) -> i32 {
        if let Token::Literal(s) = self.lookahead {
            if !self.tokens.is_empty() {
                println!("Looking ahead");
                self.lookahead = self.tokens.remove(0);
            }
            s
        } else if let Token::ID(c) = self.lookahead {
            if !self.tokens.is_empty() {
                self.lookahead = self.tokens.remove(0);
            }
            match self.lookahead {
                Token::Equal => {
                    assert!(!self.tokens.is_empty());
                    self.lookahead = self.tokens.remove(0);
                    let v = self.expr();
                    if self.table.contains_key(&c) {
                        let w = self.table.get_mut(&c).unwrap();
                        *w = v;
                        v
                    } else {
                        self.table.insert(c, v);
                        v
                    }
                }
                _ => {
                    *self.table.get(&c).expect("ERROR: use of unassigned variable")
                }
            }
        } else {
            panic!("ERROR: Unexpected token: {:?}", self.lookahead)
        }
    }

    fn factor(&mut self) -> i32 {
        let s = self.literal();
        match self.lookahead {
            Token::Mul => {
                self.lookahead = self.tokens.remove(0);
                s * self.factor()
            },
            Token::Div => {
                self.lookahead = self.tokens.remove(0);
                s / self.factor()
            },
            _ => s
        }
    }

    fn expr(&mut self) -> i32{
        let s = self.factor();
        match self.lookahead {
            Token::Add => {
                self.lookahead = self.tokens.remove(0);
                s + self.expr()
            },
            Token::Sub => {
                self.lookahead = self.tokens.remove(0);
                s - self.expr()
            },
            _ => s
        }
    }

    fn parse(&mut self) -> i32{
        self.expr()
    }
}

fn main() {
    println!("Write an equation!");

    let tokens = tokenize_stream(&mut io::stdin()).expect("Could not tokenize");
    let mut pars = Parser::new(tokens);
    let result = pars.parse();

    println!("result: {}", result);
}
