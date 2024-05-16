#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Precedence {
    CLOSE,
    MIN,
    SUM,
    MUL,
    DIV,
    EXP,
    OPEN,
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
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}
impl Tokenizer {
    pub fn new(input: &str) -> Tokenizer {
        let mut t = Tokenizer {
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

    pub fn parse(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while self.ch != '\0' {
            self.skip_space();
            match self.ch {
                '(' => tokens.push(self.op_token(self.ch, Precedence::OPEN)),
                '^' => tokens.push(self.op_token(self.ch, Precedence::EXP)),
                '/' => tokens.push(self.op_token(self.ch, Precedence::DIV)),
                '*' => tokens.push(self.op_token(self.ch, Precedence::MUL)),
                '+' => tokens.push(self.op_token(self.ch, Precedence::SUM)),
                '-' => tokens.push(self.op_token(self.ch, Precedence::MIN)),
                ')' => tokens.push(self.op_token(self.ch, Precedence::CLOSE)),
                _ => {
                    if !self.ch.is_digit(10) {
                        return Err(format!("Invalid input received: {}", self.ch));
                    }
                    tokens.push(Token::IntegerLiteral(self.read_int()));
                    continue;
                }
            }
            self.read();
        }
        Ok(tokens)
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
