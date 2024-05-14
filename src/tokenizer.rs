#[derive(Debug)]
pub enum Precedence {
    OPEN,
    EXPO,
    MULT,
    SUM,
    CLOSE,
}

#[derive(Debug)]
pub struct Operator {
    sign: char,
    precedence: Precedence,
}

#[derive(Debug)]
pub enum Token {
    IntegerLiteral(u64),
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
                ')' => tokens.push(self.op_token(self.ch, Precedence::CLOSE)),
                '+' | '-' => tokens.push(self.op_token(self.ch, Precedence::SUM)),
                '*' | '/' => tokens.push(self.op_token(self.ch, Precedence::MULT)),
                '^' => tokens.push(self.op_token(self.ch, Precedence::EXPO)),
                _ => {
                    if !self.ch.is_digit(10) {
                        return Err(format!("Invalid input received: {}", self.ch));
                    }
                    tokens.push(Token::IntegerLiteral(self.read_int()))
                }
            }
            self.read();
        }
        Ok(tokens)
    }

    fn op_token(&self, sign: char, precedence: Precedence) -> Token {
        Token::Operator(Operator { sign, precedence })
    }

    fn read_int(&mut self) -> u64 {
        let pos = self.position;
        while self.ch.is_digit(10) {
            self.read();
        }

        self.input[pos..self.position].parse::<u64>().unwrap()
    }

    fn skip_space(&mut self) {
        while self.ch.is_whitespace() {
            self.read();
        }
    }
}
