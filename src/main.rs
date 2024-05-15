#[cfg(test)]
mod tests;

mod tokenizer;
use tokenizer::{Operator, Token, Tokenizer};

#[derive(Debug)]
struct Sya {
    pub input: Vec<Token>,
    pub rpn_stack: Vec<Token>,
    pub out: String,
}
impl Sya {
    fn new(input: &str) -> Result<Sya, String> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.parse()?;

        Ok(Sya {
            input: tokens,
            rpn_stack: Vec::new(),
            out: String::new(),
        })
    }
    fn rpn(&mut self) {
        let mut holding_stack: Vec<&Operator> = Vec::new();
        for token in &self.input {
            match token {
                Token::IntegerLiteral(_) => {
                    self.rpn_stack.push(token.to_owned());
                }
                Token::Operator(o) => {
                    while let Some(&last) = holding_stack.last() {
                        if last.precedence <= o.precedence {
                            break;
                        }
                        self.rpn_stack.push(Token::Operator(last.to_owned()));
                        holding_stack.pop();
                    }

                    holding_stack.push(o);
                }
            };
        }

        for o in holding_stack {
            self.rpn_stack.push(Token::Operator(o.to_owned()));
        }
    }
}

fn main() -> Result<(), String> {
    let mut sya = Sya::new("1 + 2 * 4 - 3")?;
    sya.rpn();
    Ok(())
}
