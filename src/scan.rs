use std::iter::Peekable;


#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    IntLit(i32),
}

pub fn scanfile(content: &str) {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut chars: Peekable<_> = content.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            if c == '\n' {
                line += 1;
            }
            chars.next();
            continue;
        }

        if c.is_ascii_digit() {
            let mut num_str = String::new();
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    num_str.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            let value = num_str.parse::<i32>().unwrap();
            tokens.push(Token::IntLit(value));
            continue;
        }

        match c {
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
                continue;
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
                continue;
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
                continue;
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
                continue;
            }

            other => {
                panic!("Unrecognized character `{}` on line {}", other, line);
            }
        }
    }

    for t in tokens {
        println!("{:?}", t);
    }
}

