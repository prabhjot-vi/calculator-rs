#[derive(Debug)]
enum Token {
    Number(f64),
    LParem,
    RParem,
    Plus,
    Minus,
    Mul,
    Div,
}

fn tokenizer(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buff = String::new();

    for ch in expr.chars() {
        match ch {
            '0'..='9' | '.' => buff.push(ch),
            '+' => { flush_number(&mut tokens, &mut buff); tokens.push(Token::Plus); }
            '-' => { flush_number(&mut tokens, &mut buff); tokens.push(Token::Minus); }
            '*' => { flush_number(&mut tokens, &mut buff); tokens.push(Token::Mul); }
            '/' => { flush_number(&mut tokens, &mut buff); tokens.push(Token::Div); }
            '(' => { flush_number(&mut tokens, &mut buff); tokens.push(Token::LParem); }
            ')' => { flush_number(&mut tokens, &mut buff); tokens.push(Token::RParem); }
            ' ' => { flush_number(&mut tokens, &mut buff); }
            _   => panic!("Unknown character: {}", ch),
        }
    }

    flush_number(&mut tokens, &mut buff);
    tokens
}

fn flush_number(tokens: &mut Vec<Token>, buff: &mut String) {
    if !buff.is_empty() {
        tokens.push(Token::Number(buff.parse().unwrap()));
        buff.clear();
    }
}

fn main() {
    let expr = "10 + 20";
    println!("{:?}", tokenizer(expr));
}
