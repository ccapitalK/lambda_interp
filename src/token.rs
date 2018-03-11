use std::collections::HashMap as Map;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Expr<'input> {
    Application(Box<Expr<'input>>, Box<Expr<'input>>),
    Lambda(&'input str, Box<Expr<'input>>),
    Name(&'input str),
}

impl<'input> Display for Expr<'input> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &Expr::Application(ref a, ref b) => {
                write!(f, "({} {})", a, b)
            }
            &Expr::Lambda(ref a, ref b) => {
                write!(f, "(λ {} . {})", a, b)
            }
            &Expr::Name(n) => {
                write!(f, "{}", n)
            }
        }
    }
}

#[derive(Default)]
pub struct Evaluator<'input> {
    map: Map<&'input str, Expr<'input>>,
}

impl<'input> Evaluator<'input> {
    pub fn reduce(&'input mut self, expr: &Expr<'input>) -> Expr<'input> {
        unimplemented!()
    }
    pub fn evaluate(&'input mut self, expr: &Expr<'input>) -> Expr<'input> {
        unimplemented!()
    }
}

