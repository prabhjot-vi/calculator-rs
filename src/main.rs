use core::panic;
use std::ops::Index;

#[derive(Debug)]
enum Token {
    Number(u16),
    LParem,
    RParem,
    Mul,
    Div,
    Plus,
    Mins,
}

fn lexer(expr: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for word in expr.split_ascii_whitespace().collect::<Vec<&str>>() {
        let ch = word.chars().nth(0).unwrap();

        if ch.is_ascii_digit() {
            tokens.push(Token::Number(word.parse().unwrap()));
        } else {
            match ch {
                '(' => tokens.push(Token::LParem),
                ')' => tokens.push(Token::RParem),
                '*' => tokens.push(Token::Mul),
                '/' => tokens.push(Token::Div),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Mins),
                e => panic!("Unknown char: {}", e),
            }
        }
    }

    tokens
}

fn main() {
    let expr = "1 + 2 * 3";
    let tkns = lexer(expr);

    println!("{:?}", tkns);
}
