use parsemath::parser::{ParseError, Parser};
use parsemath::*;
use std::io;

mod parsemath;

fn main() {
    println!("Welcome!");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match evaluate(input) {
                Ok(val) => println!("The computed number is {}\n", val),
                Err(_) => println!("Error evaluating"),
            },
            Err(e) => println!("Error! {}", e),
        }
    }
}

fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("Generated AST is {:?}", ast);
    Ok(ast::eval(ast)?)
}
