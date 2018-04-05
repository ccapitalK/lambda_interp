# lambda_interp
Simple evaluator for untyped lambda calculus

## How to use

lambda_interp can be run in REPL mode, or on an input file. If a file is passed in, it will assume 
the whole file contains exactly one expression, and will evaluate it. The interpreter will attempt
to beta reduce the expression as much as possible, and then exit.

Expressions are expected to be of one of the following forms:
```
\ x . y   (x is an identifier, y is an expression)
x y       (x and y are both expresssions)
x         (x is an identifier)
(x)       (x is an expression)
```

Free variables inside expressions are not allowed.

**Note: Unbounded recursion will cause the interpreter to hang.**
