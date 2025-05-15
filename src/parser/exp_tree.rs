pub mod expression_tree {
    fn precedence(operator: &char) -> i32 {
        match operator {
            op if *op == '+' || *op == '-' => 1,
            op if *op == '*' || *op == '/' || *op == '%' => 2,
            _ => 0,
        }
    }

    fn operation(a: &f32, b: &f32, operator: &char) -> f32 {
        match operator {
            op if *op == '+' => {
                eprintln!("{}", a + b);
                a + b
            }
            op if *op == '-' => {
                eprintln!("{}", a - b);
                a - b
            }
            op if *op == '*' => {
                eprintln!("{}", a * b);
                a * b
            }
            op if *op == '/' && *b != 0.0 => {
                eprintln!("{}", a / b);
                a / b
            }
            _ => {
                eprintln!("Invalid operator {}...defaulting to 0", operator);
                0.0
            }
        }
    }

    pub fn parse_and_evaluate(expression: &String) -> Result<f32, String> {
        let mut operand_stack: Vec<f32> = vec![];
        let mut operator_stack: Vec<char> = vec![];
        println!("provided argument {}", *expression);

        let mut braces: u32 = 0;

        let mut idx = 0;

        let expression_vec: Vec<char> = expression.chars().collect();
        println!("{:?}", expression_vec);

        let mut current_num = String::new();

        while idx < expression_vec.len() {
            if expression_vec[idx].is_whitespace() {
                if !current_num.is_empty() {
                    operand_stack.push(current_num.clone().parse::<f32>().expect("not a number"));
                    current_num.clear();
                }
                idx += 1;
                continue;
            }
            if expression_vec[idx].is_digit(10) || expression_vec[idx] == '.' {
                current_num.push(expression_vec[idx]);
            } else {
                if !current_num.is_empty() {
                    operand_stack.push(current_num.clone().parse::<f32>().expect("not a number"));
                    current_num.clear();
                }

                match expression_vec[idx] {
                    ex if ex == '(' => {

                        braces += 1;
                        operator_stack.push(ex);
                    }
                    ex if ex == ')' => {
                        if !current_num.is_empty() {
                            operand_stack
                                .push(current_num.clone().parse::<f32>().expect("not a number"));
                            current_num.clear();
                        }
                        if braces > 0 {
                            while operator_stack[operator_stack.len() - 1] != '(' {
                                let b = match operand_stack.pop() {
                                    Some(x) => x,
                                    None => 0.0,
                                };
                                let a = match operand_stack.pop() {
                                    Some(x) => x,
                                    None => 0.0,
                                };

                                let operator = match operator_stack.pop() {
                                    Some(x) => x,
                                    None => {
                                        return Err(String::from(
                                            "Not a operator defaulting to +.",
                                        ));
                                    }
                                };

                                operand_stack.push(operation(&a, &b, &operator));
                            }
                            operator_stack.pop();

                            braces -= 1;
                        }
                    }
                    _ => {
                        while !operator_stack.is_empty()
                            && precedence(&operator_stack[operator_stack.len() - 1])
                                >= precedence(&expression_vec[idx])
                        {
                            let b = match operand_stack.pop() {
                                Some(x) => x,
                                None => 0.0,
                            };
                            let a = match operand_stack.pop() {
                                Some(x) => x,
                                None => 0.0,
                            };

                            let operator = match operator_stack.pop() {
                                Some(x) => x,
                                None => {
                                    return Err(String::from("Not a operator defaulting to +."));
                                }
                            };

                            operand_stack.push(operation(&a, &b, &operator));
                        }
                        operator_stack.push(expression_vec[idx]);
                    }
                }
            }

            idx += 1;
        }
        if !current_num.is_empty() {
            operand_stack.push(current_num.clone().parse::<f32>().expect("not a number"));
            current_num.clear();
        }
        if braces == 0 {
            while !operator_stack.is_empty() {
                let b = match operand_stack.pop() {
                    Some(x) => x,
                    None => 0.0,
                };
                let a = match operand_stack.pop() {
                    Some(x) => x,
                    None => 0.0,
                };

                let operator = match operator_stack.pop() {
                    Some(x) => x,
                    None => {
                        return Err(String::from("Not a operator defaulting to +."));
                    }
                };

                operand_stack.push(operation(&a, &b, &operator));
            }

            match operator_stack.len() {
                stack_len if stack_len == 0 => Ok(operand_stack[0]),
                _ => Err(String::from("Expression not balanced")),
            }
        } else {
            return Err(String::from("Expression not balanced"));
        }

        // println!("Result: {}", operand_stack[0]);
    }
}
