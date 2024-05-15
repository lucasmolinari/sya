use crate::tokenizer::{Operator, Precedence};

use super::*;

#[test]
fn test_tokenizer() {
    let mut tokenizer = Tokenizer::new("+-()*/1^99");
    let tokens = tokenizer.parse().expect("Should parse");

    assert_eq!(
        Token::Operator(Operator {
            sign: '+',
            precedence: tokenizer::Precedence::SUM
        }),
        tokens[0]
    );
    assert_eq!(
        Token::Operator(Operator {
            sign: '-',
            precedence: tokenizer::Precedence::MIN
        }),
        tokens[1]
    );
    assert_eq!(
        Token::Operator(Operator {
            sign: '(',
            precedence: tokenizer::Precedence::OPEN
        }),
        tokens[2]
    );
    assert_eq!(
        Token::Operator(Operator {
            sign: ')',
            precedence: tokenizer::Precedence::CLOSE
        }),
        tokens[3]
    );
    assert_eq!(
        Token::Operator(Operator {
            sign: '*',
            precedence: tokenizer::Precedence::MUL
        }),
        tokens[4]
    );
    assert_eq!(
        Token::Operator(Operator {
            sign: '/',
            precedence: tokenizer::Precedence::DIV
        }),
        tokens[5]
    );
    assert_eq!(Token::IntegerLiteral(1), tokens[6]);
    assert_eq!(
        Token::Operator(Operator {
            sign: '^',
            precedence: tokenizer::Precedence::EXP
        }),
        tokens[7]
    );
    assert_eq!(Token::IntegerLiteral(99), tokens[8]);
}

#[test]
fn test_rpn() {
    let mut sya = Sya::new("1 + 2 * 4 - 3").expect("Should Construct");
    sya.rpn();
    let rpn = sya.rpn_stack;

    assert_eq!(Token::IntegerLiteral(1), rpn[0]);
    assert_eq!(Token::IntegerLiteral(2), rpn[1]);
    assert_eq!(Token::IntegerLiteral(4), rpn[2]);
    assert_eq!(
        Token::Operator(Operator {
            sign: '*',
            precedence: Precedence::MUL
        }),
        rpn[3]
    );
    assert_eq!(
        Token::Operator(Operator {
            sign: '+',
            precedence: Precedence::SUM
        }),
        rpn[4]
    );
    assert_eq!(Token::IntegerLiteral(3), rpn[5]);
    assert_eq!(
        Token::Operator(Operator {
            sign: '-',
            precedence: Precedence::MIN
        }),
        rpn[6]
    );
}
