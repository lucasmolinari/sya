use crate::tokenizer::Operator;

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
            precedence: tokenizer::Precedence::SUM
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
            precedence: tokenizer::Precedence::MULT
        }),
        tokens[4]
    );
    assert_eq!(
        Token::Operator(Operator {
            sign: '/',
            precedence: tokenizer::Precedence::MULT
        }),
        tokens[5]
    );
    assert_eq!(Token::IntegerLiteral(1), tokens[6]);
    assert_eq!(
        Token::Operator(Operator {
            sign: '^',
            precedence: tokenizer::Precedence::EXPO
        }),
        tokens[7]
    );
    assert_eq!(Token::IntegerLiteral(99), tokens[8]);
}
