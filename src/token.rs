use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Expr<'input> {
    Application(Box<Expr<'input>>, Box<Expr<'input>>),
    Lambda(&'input str, Box<Expr<'input>>),
    Name(&'input str),
}

impl<'input> Expr<'input> {
    pub fn reduce(expr: Expr<'input>, name: &'input str, value: Expr<'input>) -> Expr<'input> {
        match expr {
            Expr::Lambda(a, b) => if a == name {
                Expr::Lambda(a, b)
            } else {
                Expr::Lambda(a, Box::new(Self::reduce(*b, name, value)))
            }
            Expr::Name(a) => if a == name {
                value.clone()
            } else {
                Expr::Name(a)
            }
            Expr::Application(a, b) => {
                Expr::Application(
                    Box::new(Self::reduce(*a, name, value.clone())), 
                    Box::new(Self::reduce(*b, name, value))
                )
            }
        }
    }
    pub fn evaluate(mut expr: Expr<'input>) -> Expr<'input> {
        loop {
            expr = match expr {
                Expr::Application(a, b) => {
                    match *a {
                        Expr::Lambda(name, value) => Self::reduce(*value, name, *b),
                        a => Expr::Application(Box::new(Self::evaluate(a)), b),
                    }
                }
                v => return v,
            }
        }
    }
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
