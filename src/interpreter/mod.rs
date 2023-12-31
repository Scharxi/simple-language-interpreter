use std::collections::HashMap;

use crate::{parser::expr::{Expr, Number, BinaryOp, Assignment, Variable, FunCall}, tokenizer::token::TokenType};

pub type Env = HashMap<String, i32>;

pub fn interpret(exprs: &Vec<Expr>, env: &mut Env) {
    for expr in exprs {
        evaluate(expr, env);
    }
}

/// Expressions yield values, statements do not
fn evaluate(expr: &Expr, env: &mut Env) -> i32 {
    match expr {
        Expr::Number(Number { value, .. }) => {
            return *value;
        }
        Expr::BinaryOp(BinaryOp {
            lhs,
            operation,
            rhs,
        }) => {
            let lhs_value = evaluate(lhs, env);
            let rhs_value = evaluate(rhs, env);

            let res = match operation.t_type {
                TokenType::Plus => lhs_value + rhs_value,
                TokenType::Minus => lhs_value - rhs_value,
                TokenType::Star => lhs_value * rhs_value,
                TokenType::Slash => lhs_value / rhs_value,
                t => panic!("Invalid binary operation: {:?}", t),
            };

            res
        }
        Expr::Assignment(Assignment { target, value }) => {
            let value = evaluate(value, env);
            env.insert(target.name.lexeme.clone(), value);

            return value;
        }
        Expr::Variable(Variable { name }) => {
            if let Some(value) = env.get(&name.lexeme) {
                return *value;
            } else {
                panic!("Variable {} is not defined", name.lexeme)
            }
        }
        Expr::FunCall(FunCall { name, arg }) => {
            if name.name.lexeme == "print" {
                let value = evaluate(&arg, env);
                println!("{}", value);

                return value; 
            } else {
                panic!("Undefined function {}", name.name.lexeme)
            }

        }
    }
}