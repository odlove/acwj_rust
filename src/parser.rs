use crate::scan::Token;
use ASTNode::*;

#[derive(Debug)]
pub enum ASTNode {
    Int(i32),
    Add(Box<ASTNode>, Box<ASTNode>),
    Sub(Box<ASTNode>, Box<ASTNode>),
    Mul(Box<ASTNode>, Box<ASTNode>),
    Div(Box<ASTNode>, Box<ASTNode>),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn parse(&mut self) -> ASTNode {
        // first token, only IntLit is valid
        let mut node = match self.next() {
            Some(Token::IntLit(n)) => ASTNode::Int(n),
            other => panic!("期望 IntLit, 得到 {:?}", other),
        };

        while let Some(op) = self.next() {
            let rhs = match self.next() {
                Some(Token::IntLit(n)) => ASTNode::Int(n),
                other => panic!("expected IntLit, receive {:?}", other),
            };
            node = match op {
                Token::Plus => ASTNode::Add(Box::new(node), Box::new(rhs)),
                Token::Minus => ASTNode::Sub(Box::new(node), Box::new(rhs)),
                Token::Star => ASTNode::Mul(Box::new(node), Box::new(rhs)),
                Token::Slash => ASTNode::Div(Box::new(node), Box::new(rhs)),
                _ => panic!("Invalid operator {:?}", op),
            };
            println!("{:?}", node);
        }

        node
    }
}

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut p = Parser::new(tokens);
    let ast = p.parse();
    if p.pos != p.tokens.len() {
        panic!("多余的 tokens: pos = {} len = {}", p.pos, p.tokens.len());
    }
    ast
}
