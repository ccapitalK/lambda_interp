use std::fmt::{Display, Formatter, Result};
use std::collections::HashSet;

/// Type for arbitrary lambda expression
#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// Helper function for has_unbound
    fn has_unbound_r(&self, names: &mut HashSet<&'input str>) -> ::std::result::Result<(), String> {
        match self {
            &Expr::Application(ref a, ref b) => {
                a.has_unbound_r(names)?;
                b.has_unbound_r(names)
            }
            &Expr::Lambda(name, ref val) => {
                let shadow = names.contains(name);
                if !shadow { 
                    names.insert(name); 
                }
                val.has_unbound_r(names)?;
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
        self.has_unbound_r(&mut x)
    }

    /// returns precedence of operator
    fn precedence(&self) -> u8 {
        match self {
            &Expr::Application(_, _) => 1,
            &Expr::Lambda(_, _) => 0,
            &Expr::Name(_) => 2,
        }
    }

    /// returns true if expression is just a name
    fn is_name(&self) -> bool {
        if let &Expr::Name(_) = self {
            true
        } else {
            false
        }
    }

    /// returns true if expression is an application
    #[allow(unused)]
    fn is_application(&self) -> bool {
        if let &Expr::Application(_, _) = self {
            true
        } else {
            false
        }
    }

    /// returns true if expression is a lambda abstraction
    #[allow(unused)]
    fn is_lambda(&self) -> bool {
        if let &Expr::Lambda(_, _) = self {
            true
        } else {
            false
        }
    }
}

impl<'input> Display for Expr<'input> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // TODO: Make this only parenthesize as needed
        match self {
            &Expr::Application(ref a, ref b) => {
                if a.is_lambda() {
                    write!(f, "({}) ", a)?;
                } else {
                    write!(f, "{} ", a)?;
                }
                if !b.is_name() {
                    write!(f, "({})", b)
                } else {
                    write!(f, "{}", b)
                }
            }
            &Expr::Lambda(ref a, ref b) => {
                write!(f, "λ {} . {}", a, b)
            }
            &Expr::Name(n) => {
                write!(f, "{}", n)
            }
        }
    }
}

#[test]
fn test_precedence_associativity() {
    use self::Expr::*;
    {
        let input = r"\ x . x z (\ y . x y)";
        let expr = ::parse::LambdaExprParser::new().parse(input).unwrap();
        // should be (\ x . ((x z) (\ y . (x y))))
        assert_eq!(expr, 
            Lambda(
                "x",
                Box::new(Application(
                    Box::new(Application(
                        Box::new(Name("x")),
                        Box::new(Name("z"))
                    )),
                    Box::new(Lambda(
                        "y",
                        Box::new(Application(
                            Box::new(Name("x")),
                            Box::new(Name("y"))
                        ))
                    ))
                ))
            )
        );
    }
}
