#[derive(Debug, Clone)]
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
            '+' => {
                flush_number(&mut tokens, &mut buff);
                tokens.push(Token::Plus);
            }
            '-' => {
                flush_number(&mut tokens, &mut buff);
                tokens.push(Token::Minus);
            }
            '*' => {
                flush_number(&mut tokens, &mut buff);
                tokens.push(Token::Mul);
            }
            '/' => {
                flush_number(&mut tokens, &mut buff);
                tokens.push(Token::Div);
            }
            '(' => {
                flush_number(&mut tokens, &mut buff);
                tokens.push(Token::LParem);
            }
            ')' => {
                flush_number(&mut tokens, &mut buff);
                tokens.push(Token::RParem);
            }
            ' ' => {
                flush_number(&mut tokens, &mut buff);
            }
            _ => panic!("Unknown character: {}", ch),
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

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Expr {
    Number(f64),
    Binary(Box<Expr>, Op, Box<Expr>),
}

fn precedence(op: Op) -> u8 {
    match op {
        Op::Add | Op::Sub => 1,
        Op::Mul | Op::Div => 2,
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn parse(&mut self) -> Expr {
        self.parse_expr(0)
    }

    fn parse_expr(&mut self, min_prec: u8) -> Expr {
        let mut lhs = self.parse_primary();

        while let Some(op) = self.peek_op() {
            let prec = precedence(op);
            if prec < min_prec {
                break;
            }

            self.pos += 1; // consume operator
            let rhs = self.parse_expr(prec + 1);
            lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
        }

        lhs
    }

    fn parse_primary(&mut self) -> Expr {
        match self.next() {
            Some(Token::Number(n)) => Expr::Number(n),
            Some(Token::LParem) => {
                let expr = self.parse_expr(0);
                self.expect(Token::RParem);
                expr
            }
            _ => panic!("Unexpected token"),
        }
    }

    fn peek_op(&self) -> Option<Op> {
        match self.tokens.get(self.pos) {
            Some(Token::Plus) => Some(Op::Add),
            Some(Token::Minus) => Some(Op::Sub),
            Some(Token::Mul) => Some(Op::Mul),
            Some(Token::Div) => Some(Op::Div),
            _ => None,
        }
    }

    fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    fn expect(&mut self, expected: Token) {
        let tok = self.next().expect("Unexpected end of input");
        if std::mem::discriminant(&tok) != std::mem::discriminant(&expected) {
            panic!("Expected {:?}, got {:?}", expected, tok);
        }
    }
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Binary(lhs, op, rhs) => {
            let l = eval(lhs);
            let r = eval(rhs);
            match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => l / r,
            }
        }
    }
}

fn main() {
    let expr = "10 + 20 * 3";
    let tokens = tokenizer(expr);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("{:?}", ast);
    println!("Result = {}", eval(&ast));
}
