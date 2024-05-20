use crate::errors::SyaError;
use crate::number::Number;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Precedence {
    MIN,
    SUM,
    MUL,
    EXP,
    MAX,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operator {
    pub sign: char,
    pub precedence: Precedence,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(Number),
    Operator(Operator),
    UNARY(char),
    OPEN,
    CLOSE,
}
impl Token {
    pub fn precedence(&self) -> Option<&Precedence> {
        match self {
            Token::Operator(o) => Some(&o.precedence),
            Token::UNARY(_) | Token::CLOSE => Some(&Precedence::MAX),
            Token::OPEN => Some(&Precedence::MIN),
            _ => None,
        }
    }
}

pub struct Tokenizer {
    tokens: Vec<Token>,
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}
impl Tokenizer {
    pub fn new(input: &str) -> Tokenizer {
        let mut t = Tokenizer {
            tokens: Vec::new(),
            input: input.to_owned(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        t.read();
        t
    }

    fn read(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn parse(&mut self) -> Result<&Vec<Token>, SyaError> {
        while self.ch != '\0' {
            self.skip_space();
            match self.ch {
                '(' => self.tokens.push(Token::OPEN),
                '^' => self.tokens.push(self.op_token(self.ch, Precedence::EXP)),
                '/' => self.tokens.push(self.op_token(self.ch, Precedence::MUL)),
                '*' => self.tokens.push(self.op_token(self.ch, Precedence::MUL)),
                '+' => {
                    let token = self.handle_unary(Precedence::SUM);
                    self.tokens.push(token);
                }
                '-' => {
                    let token = self.handle_unary(Precedence::SUM);
                    self.tokens.push(token)
                }
                ')' => self.tokens.push(Token::CLOSE),
                _ => {
                    if !self.ch.is_digit(10) {
                        return Err(SyaError::InvalidChar(self.ch));
                    }
                    let number = self.read_number()?;
                    self.tokens.push(Token::Number(number));
                    continue;
                }
            }
            self.read();
        }
        Ok(&self.tokens)
    }

    fn handle_unary(&mut self, precedence: Precedence) -> Token {
        match self.tokens.last() {
            Some(Token::CLOSE) | Some(Token::Number(_)) => self.op_token(self.ch, precedence),
            _ => Token::UNARY(self.ch),
        }
    }

    fn op_token(&self, sign: char, precedence: Precedence) -> Token {
        Token::Operator(Operator { sign, precedence })
    }

    fn read_number(&mut self) -> Result<Number, SyaError> {
        let pos = self.position;
        while self.ch.is_digit(10) || self.ch == '.' {
            self.read();
        }

        let n = &self.input[pos..self.position];
        if n.contains('.') {
            let parsed = match n.parse::<f64>() {
                Ok(f) => f,
                Err(_) => return Err(SyaError::NumberOverflow(n.to_string())),
            };
            Ok(Number::Float(parsed))
        } else {
            let parsed = match n.parse::<i64>() {
                Ok(f) => f,
                Err(_) => return Err(SyaError::NumberOverflow(n.to_string())),
            };
            Ok(Number::Integer(parsed))
        }
    }

    fn skip_space(&mut self) {
        while self.ch.is_whitespace() {
            self.read();
        }
    }
}
