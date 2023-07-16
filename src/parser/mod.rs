use crate::tokenizer::token::{TokenType, Token};

use self::expr::{Expr, Assignment, Variable, BinaryOp, Number, FunCall};

pub mod expr;

pub fn parse(mut tokens: Vec<Token>) -> Vec<Expr> {
    tokens.reverse();

    let mut res = Vec::new();

    while tokens.len() > 0 {
        let expr = parse_expr(&mut tokens);

        expect(TokenType::Newline, &mut tokens);

        res.push(expr);
    }

    res
}

fn expect(expected: TokenType, tokens: &mut Vec<Token>) {
    match tokens.pop() {
        None => {}
        Some(tok) => {
            if tok.t_type != expected {
                panic!("Expected token {:?} got {:?} instead", expected, tok)
            }
        }
    }
}

fn parse_expr(tokens: &mut Vec<Token>) -> Expr {
    return parse_assignment(tokens);
}

fn parse_assignment(tokens: &mut Vec<Token>) -> Expr {
    return if tokens.len() > 1 && tokens[tokens.len() - 2].t_type == TokenType::Equal {
        let var = parse_var(tokens);
        expect(TokenType::Equal, tokens);
        let value = parse_expr(tokens);

        Expr::Assignment(Assignment {
            target: var,
            value: Box::new(value),
        })
    } else {
        parse_term(tokens)
    };
}

fn parse_var(tokens: &mut Vec<Token>) -> Variable {
    let token = tokens.pop().unwrap();
    return if token.t_type == TokenType::Ident {
        Variable { name: token }
    } else {
        panic!("Expected identifier, found {:?} instead.", token);
    };
}

fn parse_term(tokens: &mut Vec<Token>) -> Expr {
    let mut res = parse_factor(tokens);

    while tokens.len() > 0 {
        let next_token = &tokens[tokens.len() - 1];
        match next_token.t_type {
            TokenType::Plus | TokenType::Minus => {
                let op_token = tokens.pop().unwrap();
                let rhs = parse_factor(tokens);

                res = Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(res),
                    operation: op_token,
                    rhs: Box::new(rhs),
                })
            }
            _ => break,
        }
    }
    res
}

fn parse_factor(tokens: &mut Vec<Token>) -> Expr {
    let mut res = parse_primary(tokens);

    while tokens.len() > 1 {
        let next_token = &tokens[tokens.len() - 1];
        match next_token.t_type {
            TokenType::Star | TokenType::Slash => {
                let operand = tokens.pop().unwrap();

                let rhs = parse_primary(tokens);
                res = Expr::BinaryOp(BinaryOp {
                    lhs: Box::new(res),
                    operation: operand,
                    rhs: Box::new(rhs),
                })
            }
            _ => break,
        }
    }

    res
}

fn parse_primary(tokens: &mut Vec<Token>) -> Expr {
    let token = tokens.pop().unwrap();

    match token.t_type {
        TokenType::NumberLiteral => {
            return Expr::Number(Number {
                value: parse_number(&token.lexeme),
                token,
            })
        }
        TokenType::Ident => {
            if tokens.len() > 0 {
                let next_token = &tokens[tokens.len() - 1];
                if next_token.t_type == TokenType::LeftParen {
                    let func_name = Variable { name: token };

                    tokens.pop().unwrap(); // remove left paren from the tokens

                    let args = parse_expr(tokens);

                    expect(TokenType::RightParen, tokens);

                    return Expr::FunCall(FunCall {
                        name: func_name,
                        arg: Box::new(args),
                    });
                }
            }
            return Expr::Variable(Variable { name: token });
        }
        TokenType::LeftParen => {
            let expr = parse_expr(tokens);
            expect(TokenType::RightParen, tokens);
            return expr;
        }
        t => panic!("Unexpected token type: `{:?}`", t),
    }
}

fn parse_number(s: &str) -> i32 {
    return s.parse::<i32>().unwrap();
}
