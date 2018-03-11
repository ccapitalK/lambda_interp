use std::fmt::{Display, Formatter, Result};
use std::collections::HashSet;

/// Type for arbitrary lambda expression
#[derive(Debug, Clone)]
pub enum Expr<'input> {
    Application(Box<Expr<'input>>, Box<Expr<'input>>),
    Lambda(&'input str, Box<Expr<'input>>),
    Name(&'input str),
}

impl<'input> Expr<'input> {
    /// Performs one B-reduction according to the reduction rules
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
    /// Continually B-reduces whilst the outermost expression is an application
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
    fn is_valid_r(&self, names: &mut HashSet<&'input str>) -> ::std::result::Result<(), String> {
        match self {
            &Expr::Application(ref a, ref b) => {
                a.is_valid_r(names)?;
                b.is_valid_r(names)
            }
            &Expr::Lambda(name, ref val) => {
                let shadow = names.contains(name);
                if !shadow { 
                    names.insert(name); 
                }
                val.is_valid_r(names)?;
                if !shadow { 
                    names.remove(name);
                }
                Ok(())
            }
            &Expr::Name(name) => if names.contains(name) {
                Ok(())
            } else {
                Err(format!("Unbound variable name {}", name))
            }
        }
    }
    /// Checks if an expression has any unbound variables
    pub fn has_unbound(&self) -> ::std::result::Result<(), String> {
        let mut x = Default::default();
        self.is_valid_r(&mut x)
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
