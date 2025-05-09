use std::env;

mod parser;

use parser::exp_tree::expression_tree;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1
    {
        panic!("No expression provided...");
    }
    else 
    {
        let expression = &args[1];
        let value = expression_tree::parse_and_evaluate(expression).expect("Error occured while evaluating expression");
        println!("Result: {}", value);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn eval_1()
    {
        let expression = String::from("1 + 2");
        let result = expression_tree::parse_and_evaluate(&expression).expect("Error occured while evaluating expression");

        assert_eq!(result, 3);
    }

    #[test]
    fn eval_2()
    {
        let expression = String::from("5 * (5 / 5) + (4 - 1)");
        let result = expression_tree::parse_and_evaluate(&expression).expect("Error occured while evaluating expression");

        assert_eq!(result, 8);
    }

    #[test]
    fn eval_3()
    {
        let expression = String::from("5 * (4 - 1)");
        let result = expression_tree::parse_and_evaluate(&expression).expect("Error occured while evaluating expression");

        assert_eq!(result, 15);
    }
}
