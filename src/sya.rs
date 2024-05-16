use crate::tokenizer::Precedence;

use super::tokenizer::{Operator, Token, Tokenizer};

#[derive(Debug)]
pub struct Sya {
    pub input: Vec<Token>,
    pub rpn_stack: Vec<Token>,
    pub out: Option<i64>,
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
        self.input = tokenizer.parse()?;
        self.rpn_stack.clear();
        self.out = None;
        Ok(())
    }

    pub fn calculate(&mut self) -> Result<(), String> {
        self.rpn()?;
        let mut operation_stack = Vec::new();
        for token in &self.rpn_stack {
            match token {
                Token::IntegerLiteral(i) => operation_stack.push(*i),
                Token::Operator(o) => {
                    if operation_stack.len() < 2 {
                        return Err("Operation stack doesn't have 2 elements.".to_string());
                    }
                    let b = operation_stack.pop().unwrap();
                    let a = operation_stack.pop().unwrap();

                    let result = match o.sign {
                        '+' => a.checked_add(b),
                        '-' => a.checked_sub(b),
                        '*' => a.checked_mul(b),
                        '/' => a.checked_div(b),
                        '^' => a.checked_pow(b as u32),
                        _ => None,
                    };

                    match result {
                        Some(i) => operation_stack.push(i),
                        None => return Err("Invalid Operation".to_string()),
                    }
                }
            }
        }
        let last = operation_stack.last().ok_or("Couldn't find a result")?;
        self.out = Some(*last);
        Ok(())
    }

    fn rpn(&mut self) -> Result<(), &str> {
        let mut holding_stack: Vec<&Operator> = Vec::new();
        for token in &self.input {
            match token {
                Token::IntegerLiteral(_) => self.rpn_stack.push(token.clone()),
                Token::Operator(o) => match o.precedence {
                    Precedence::OPEN => holding_stack.push(o),
                    Precedence::CLOSE => {
                        while let Some(&last) = holding_stack.last() {
                            if last.precedence == Precedence::OPEN {
                                break;
                            }
                            self.rpn_stack.push(Token::Operator(last.clone()));
                            holding_stack.pop();
                        }
                        match holding_stack.last() {
                            Some(_) => holding_stack.pop(),
                            None => return Err("Expected Open Parenthesis '('"),
                        };
                    }
                    _ => {
                        while let Some(&last) = holding_stack.last() {
                            if last.precedence <= o.precedence {
                                break;
                            }
                            self.rpn_stack.push(Token::Operator(last.clone()));
                            holding_stack.pop();
                        }
                        holding_stack.push(o);
                    }
                },
            }
        }

        for o in holding_stack {
            self.rpn_stack.push(Token::Operator(o.clone()));
        }
        Ok(())
    }

    pub fn rpn_formatted(&self) -> String {
        self.rpn_stack
            .iter()
            .map(|token| match token {
                Token::IntegerLiteral(i) => i.to_string(),
                Token::Operator(o) => o.sign.to_string(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
