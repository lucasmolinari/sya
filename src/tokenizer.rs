#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Precedence {
    OPEN,
    MIN,
    SUM,
    MUL,
    DIV,
    EXP,
    UNARY,
    CLOSE,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operator {
    pub sign: char,
    pub precedence: Precedence,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    IntegerLiteral(i64),
    Operator(Operator),
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

    pub fn parse(&mut self) -> Result<&Vec<Token>, String> {
        while self.ch != '\0' {
            self.skip_space();
            match self.ch {
                '(' => self.tokens.push(self.op_token(self.ch, Precedence::OPEN)),
                '^' => self.tokens.push(self.op_token(self.ch, Precedence::EXP)),
                '/' => self.tokens.push(self.op_token(self.ch, Precedence::DIV)),
                '*' => self.tokens.push(self.op_token(self.ch, Precedence::MUL)),
                '+' => {
                    let token = self.handle_unary(Precedence::SUM);
                    self.tokens.push(token);
                }
                '-' => {
                    let token = self.handle_unary(Precedence::MIN);
                    self.tokens.push(token)
                }
                ')' => self.tokens.push(self.op_token(self.ch, Precedence::CLOSE)),
                _ => {
                    if !self.ch.is_digit(10) {
                        return Err(format!("Invalid input received: {}", self.ch));
                    }
                    let int = self.read_int();
                    self.tokens.push(Token::IntegerLiteral(int));
                    continue;
                }
            }
            self.read();
        }
        Ok(&self.tokens)
    }

    fn handle_unary(&mut self, precedence: Precedence) -> Token {
        match self.tokens.last() {
            Some(Token::Operator(o)) if o.precedence == Precedence::CLOSE => {
                self.op_token(self.ch, precedence)
            }
            Some(Token::Operator(_)) | None => self.op_token(self.ch, Precedence::UNARY),
            Some(Token::IntegerLiteral(_)) => self.op_token(self.ch, precedence),
        }
    }

    fn op_token(&self, sign: char, precedence: Precedence) -> Token {
        Token::Operator(Operator { sign, precedence })
    }

    fn read_int(&mut self) -> i64 {
        let pos = self.position;
        while self.ch.is_digit(10) {
            self.read();
        }

        self.input[pos..self.position].parse::<i64>().unwrap()
    }

    fn skip_space(&mut self) {
        while self.ch.is_whitespace() {
            self.read();
        }
    }
}
