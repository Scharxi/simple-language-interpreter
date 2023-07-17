use interpreter::{Env, interpret};
use parser::parse;
use tokenizer::tokenize;

pub mod parser;
pub mod tokenizer;
pub mod interpreter; 



fn main() {
    let src = "a_123 = 654 * (2 * 1)
    print(a_123)";

    let tokens = tokenize(src);

    let exprs = parse(tokens);

    for expr in exprs.iter() {
        println!("{:#?}", expr)
    }
    
    let mut env = Env::new();
    interpret(&exprs, &mut env);
}
