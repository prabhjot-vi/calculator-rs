use core::panic;

#[derive(Debug)]
enum Token {
    Number(u16),
    LParem,
    RParem,
    Mul,
    Div,
    Add,
    Sub,
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
                '+' => tokens.push(Token::Add),
                '-' => tokens.push(Token::Sub),
                e => panic!("Unknown char: {}", e),
            }
        }
    }

    tokens
}

#[derive(Debug)]
enum Expression {
    Atom(u16),
    Operation(Token, Box<Expression>, Box<Expression>),
    None,
}

fn parser(tokens: Vec<Token>) -> Expression {
    let root = Expression::Operation(
        Token::Add,
        Box::new(Expression::Operation(
            Token::Mul,
            Box::new(Expression::Atom(10)),
            Box::new(Expression::Atom(5)),
        )),
        Box::new(Expression::Atom(20)),
    );

    root
}

fn main() {
    let expr = "1 + 2 * 3";
    let tokens = lexer(expr);

    println!("{:?}", tokens);

    let root = parser(tokens);
    println!("{:?}", root);
}
