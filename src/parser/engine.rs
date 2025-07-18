pub mod expression_engine
{
    #[derive(Debug)]
    pub enum Symbols {
        VARIABLE(String),

        INTEGER(f32),

        LPAREN,
        RPAREN,

        ASSIGNMENT,

        EOL,
        EOF
    }

    #[derive(Debug)]
    pub struct Lexer
    {
        pub symbols: Vec<Symbols>
    }

    impl Lexer  {
        pub fn new() -> Self
        {
            Self { symbols: Vec::new() }
        }

        pub fn lexical_analysis(mut self, expression: &String)
        {
            let mut index: usize = 0;

            let expression_vec: Vec<char> = expression.chars().collect();

            while index < expression.len()
            {
                let mut var: String = String::new();
                if expression_vec[index].is_alphabetic() && expression_vec[index] != ' '
                {
                    while expression_vec[index].is_alphabetic() {
                        var.push(expression_vec[index]);

                        index += 1;
                    }

                    self.symbols.push(Symbols::VARIABLE(var.to_owned()));

                    continue;
                }
                else if expression_vec[index] == '='
                {
                    self.symbols.push(Symbols::ASSIGNMENT);
                }
                else if expression_vec[index].is_digit(10) && expression_vec[index] != ' ' {
                    let mut var: String = String::new();
                    let mut got_dot: bool = false;
                    
                    while expression_vec[index].is_digit(10) || expression_vec[index] == '.'
                    {
                        
                        if expression_vec[index] == '.' && !got_dot
                        {
                            got_dot = true;

                            var.push(expression_vec[index]);

                            index += 1;

                            continue
                        }
                        else
                        {
                            var.push(expression_vec[index]);

                            index += 1;
                        }

                        if index >= expression_vec.len()
                        {
                            break;
                        }

                        let var: f32 = var.parse().expect(format!("Error converting {} to float 32", var).as_str());

                        self.symbols.push(Symbols::INTEGER(var));
                    }
                }
                else if expression_vec[index] == '\n'
                {
                    self.symbols.push(Symbols::EOL);
                }

                index += 1;                
            }
            
            self.symbols.push(Symbols::EOF)
        }
    }
}