pub mod expression_tree
{

    
    fn precedence(operator: &char) -> i32
    {
        match operator {
            op if *op == '+' || *op == '-' => { 1 }
            op if *op == '*' || *op == '/' || *op == '%' => { 2 }
            _ => 0
        }
    }

    fn operation(a: &u32, b: &u32, operator: &char) -> u32
    {
        match operator
        {
            op if *op == '+' => {
                a + b
            }
            op if *op == '-' => {
                a - b
            }
            op if *op == '*' => {
                a * b
            }
            op if *op == '/' && *b != 0 => {
                a / b
            }
            _ => {
                a % b
            }
        }
    }


    pub fn parse_and_evaluate(expression: &String) -> Result<u32, String>
    {
        let mut operand_stack: Vec<u32> = vec![];
        let mut operator_stack: Vec<char> = vec![];
        println!("provided argument {}", *expression);

        for ex in expression.chars()
        {
            match ex {
                ex if ex == ' ' => continue,
                ex if ex.is_digit(10) => {
                    operand_stack.push(match ex.to_digit(10)
                    {
                        Some(e) => e,
                        None => {
                            return Err(String::from("cannot convert to num considering it as 0"));}
                    });
                },
                ex if ex == '(' => {
                    operator_stack.push(ex);
                }
                ex if ex == ')' => {
                    while operator_stack[operator_stack.len() - 1] != '('
                    {
                        let b = match operand_stack.pop() { Some(x) => x, None => 0 };
                        let a = match operand_stack.pop() { Some(x) => x, None => 0 };

                        let operator = match operator_stack.pop() { Some(x) => x, None => {return Err(String::from("Not a operator defaulting to +."));}};

                        operand_stack.push(operation(&a, &b, &operator));
                    }
                    operator_stack.pop();
                }
                _ => 
                {
                    while !operator_stack.is_empty() && precedence(&operator_stack[operator_stack.len() - 1]) >= precedence(&ex)
                    {
                        let b = match operand_stack.pop() { Some(x) => x, None => 0 };
                        let a = match operand_stack.pop() { Some(x) => x, None => 0 };

                        let operator = match operator_stack.pop() { Some(x) => x, None => {return Err(String::from("Not a operator defaulting to +."));}};

                        operand_stack.push(operation(&a, &b, &operator));
                        
                    }
                    operator_stack.push(ex);
                } 
            }
        }
        while !operator_stack.is_empty()
        {
            let b = match operand_stack.pop() { Some(x) => x, None => 0 };
            let a = match operand_stack.pop() { Some(x) => x, None => 0 };

            let operator = match operator_stack.pop() { Some(x) => x, None => {return Err(String::from("Not a operator defaulting to +."));}};

            operand_stack.push(operation(&a, &b, &operator));
        }

        // println!("Result: {}", operand_stack[0]);
        Ok(operand_stack[0])
    }
}