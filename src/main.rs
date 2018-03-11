mod parse;
mod token;

fn main() {
    let input = r"(\x.\y.x)(\x.x)";
    let expr = parse::parse_LambdaExpr(input).unwrap();
    println!("Before evaluation:\n{}", expr);
    let mut evaluator: token::Evaluator = Default::default();
    let expr = evaluator.evaluate(&expr);
    println!("After evaluation:\n{:?}", expr);
}
