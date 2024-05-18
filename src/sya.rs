use crate::number::Number;

use super::tokenizer::{Token, Tokenizer};

#[derive(Debug)]
pub struct Sya {
    pub input: Vec<Token>,
    pub rpn_stack: Vec<Token>,
    pub out: Option<Number>,
}
impl Sya {
    pub fn new(input: &str) -> Result<Sya, String> {
        let mut s = Sya {
            input: Vec::new(),
            rpn_stack: Vec::new(),
            out: None,
        };
        s.new_input(input)?;
        Ok(s)
    }

    pub fn new_input(&mut self, input: &str) -> Result<(), String> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.parse()?;
        self.input = tokens.clone();
        self.rpn_stack.clear();
        self.out = None;
        Ok(())
    }

    pub fn calculate(&mut self) -> Result<(), String> {
        self.rpn()?;
        let mut operation_stack = Vec::new();
        for token in &self.rpn_stack {
            match token {
                Token::Number(i) => operation_stack.push(i.clone()),
                Token::UNARY(s) => {
                    let n = match operation_stack.pop() {
                        Some(n) => n,
                        None => return Err("Empty operation stack".to_string()),
                    };
                    match s {
                        '-' => operation_stack.push(n.negate()),
                        '+' => operation_stack.push(n),
                        _ => return Err("Wrong unary sign".to_string()),
                    };
                }

                Token::Operator(o) => {
                    if operation_stack.len() < 2 {
                        return Err(format!(
                            "Operation stack doesn't have enough arguments. OP: {:?} STACK:{:?}",
                            o, operation_stack
                        ));
                    }

                    let b = operation_stack.pop().unwrap();
                    let a = operation_stack.pop().unwrap();

                    let result = match o.sign {
                        '+' => a.checked_add(b),
                        '-' => a.checked_sub(b),
                        '*' => a.checked_mul(b),
                        '/' => a.checked_div(b),
                        '^' => a.checked_pow(b.as_u32()),
                        _ => None,
                    };

                    match result {
                        Some(i) => operation_stack.push(i),
                        None => return Err("Invalid Operation".to_string()),
                    }
                }
                _ => return Err("Invalid Token fould in RPN".to_string()),
            }
        }

        if operation_stack.len() != 1 {
            return Err("Invalid Operation".to_string());
        }
        let last = operation_stack.last().unwrap();
        self.out = Some(last.clone());
        Ok(())
    }

    fn rpn(&mut self) -> Result<(), &str> {
        let mut holding_stack: Vec<&Token> = Vec::new();
        for token in &self.input {
            match token {
                Token::Number(_) => self.rpn_stack.push(token.clone()),
                Token::OPEN | Token::UNARY(_) => holding_stack.push(token),
                Token::CLOSE => {
                    while let Some(&last) = holding_stack.last() {
                        if last == &Token::OPEN {
                            break;
                        }
                        self.rpn_stack.push(last.clone());
                        holding_stack.pop();
                    }
                    match holding_stack.last() {
                        Some(_) => holding_stack.pop(),
                        None => return Err("Expected Open Parenthesis '('"),
                    };
                }
                Token::Operator(o) => {
                    while let Some(&last) = holding_stack.last() {
                        if last.precedence() < Some(&o.precedence) {
                            break;
                        }
                        self.rpn_stack.push(last.clone());
                        holding_stack.pop();
                    }
                    holding_stack.push(token);
                }
            }
        }

        while let Some(o) = holding_stack.pop() {
            self.rpn_stack.push(o.clone());
        }
        Ok(())
    }

    pub fn rpn_formatted(&self) -> String {
        self.rpn_stack
            .iter()
            .map(|token| match token {
                Token::Number(n) => n.to_string(),
                Token::Operator(o) => o.sign.to_string(),
                Token::UNARY(s) => format!("u{}", s),
                Token::OPEN => "(".to_string(),
                Token::CLOSE => ")".to_string(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
