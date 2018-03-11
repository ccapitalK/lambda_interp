mod parse;
mod token;

use std::env::args;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let eval = |input: String| {
        let expr = match parse::parse_LambdaExpr(&input) {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to parse line: {:?}", e);
                return;
            }
        };
        match expr.has_unbound() {
            Err(e) => {
                println!("Input {:?} not valid: {}", expr, e);
                return;
            }
            _ => {}
        };
        println!("Before evaluation:\n{}", expr);
        let expr = token::Expr::evaluate(expr);
        println!("After evaluation:\n{}", expr);
    };
    match args().skip(1).next() {
        Some(v) => {
            let mut input = String::new();
            File::open(v).unwrap().read_to_string(&mut input).unwrap();
            eval(input);
        }
        None => {
            loop {
                print!("> ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)
                    .expect("Failed to read line from stdin");
                if input.len() < 2 { return; }
                eval(input);
            }
        }
    }
}
