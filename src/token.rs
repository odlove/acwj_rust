#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    IntLit(i32),
    EOF,
}

