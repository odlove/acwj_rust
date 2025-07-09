
use crate::token::Token;
use std::str::Chars;
use std::iter::Peekable;

pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
}

impl <'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner {
            chars: input.chars().peekable(),
            line: 1,
        }
    }

    fn skip(&mut self) -> Option<char> {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                }
                self.chars.next();
                continue;
            }
            break;
        }
        self.chars.next()
    }

    fn scanint(&mut self, mut n: u32) -> Token {
        while let Some(&c) = self.chars.peek() {
            if let Some(d) = c.to_digit(10) {
                self.chars.next();
                n = n * 10 + d;
            } else {
                break;
            }
        }
        Token::IntLit(n as i32)
    }

    pub fn scan(&mut self) -> Token {
        if let Some(c) = self.skip() {
            if c.is_ascii_digit() {
                return self.scanint(c.to_digit(10).unwrap());
            }

            match c {
                '+' => return Token::Plus,
                '-' => return Token::Minus,
                '*' => return Token::Star,
                '/' => return Token::Slash,
                _ => {
                    println!("Unrecognised character {c} on line {0}", self.line);
                    unreachable!()
                },
            }
        }
        Token::EOF
    }
}

