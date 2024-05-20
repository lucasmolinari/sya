use std::{error::Error, fmt::Display};

use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub enum SyaError {
    InvalidToken(Token),
    InvalidChar(char),
    InvalidInput,
    ExpectedChar(char),
    WrongUnary(char),
    InvalidOperation(char),
    NumberOverflow(String),
    Custom(String),
    DivisionByZero,
    ExpectedStackSize(u32),
}
impl Error for SyaError {}
impl Display for SyaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyaError::InvalidToken(t) => write!(f, "Invalid Token '{:?}'", t),
            SyaError::InvalidChar(c) => write!(f, "Invalid Character '{}'", c),
            SyaError::WrongUnary(c) => write!(f, "Invalid Unary '{}'", c),
            SyaError::ExpectedChar(c) => write!(f, "Expected Character '{}'", c),
            SyaError::InvalidOperation(c) => write!(f, "Invalid Operation '{}'", c),
            SyaError::NumberOverflow(i) => write!(f, "Number Overflow '{}'", i),
            SyaError::InvalidInput => write!(f, "Invalid Input"),
            SyaError::ExpectedStackSize(u) => {
                write!(f, "Expected Stack to have at least {} items", u)
            }
            SyaError::DivisionByZero => write!(f, "Tried to divide by zero"),
            SyaError::Custom(s) => write!(f, "{}", s),
        }
    }
}
