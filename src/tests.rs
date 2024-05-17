use crate::tokenizer::{Operator, Precedence, Token, Tokenizer};

use super::*;

fn op(sign: char, precedence: Precedence) -> Operator {
    Operator { sign, precedence }
}
#[test]
fn test_tokenizer() {
    let mut tokenizer = Tokenizer::new("- 10 + 2 * 4 (5^2)");
    let tokens = tokenizer.parse().expect("Should Parse");

    assert_eq!(Token::Operator(op('-', Precedence::UNARY)), tokens[0]);
    assert_eq!(Token::IntegerLiteral(10), tokens[1]);
    assert_eq!(Token::Operator(op('+', Precedence::SUM)), tokens[2]);
    assert_eq!(Token::IntegerLiteral(2), tokens[3]);
    assert_eq!(Token::Operator(op('*', Precedence::MUL)), tokens[4]);
    assert_eq!(Token::IntegerLiteral(4), tokens[5]);
    assert_eq!(Token::Operator(op('(', Precedence::OPEN)), tokens[6]);
    assert_eq!(Token::IntegerLiteral(5), tokens[7]);
    assert_eq!(Token::Operator(op('^', Precedence::EXP)), tokens[8]);
    assert_eq!(Token::IntegerLiteral(2), tokens[9]);
    assert_eq!(Token::Operator(op(')', Precedence::CLOSE)), tokens[10]);
}

#[test]
fn test_unary_parse() {
    let mut tokenizer = Tokenizer::new("-(10 + 5) - -(+3 - 2)");
    let tokens = tokenizer.parse().expect("Should Parse");

    assert_eq!(Token::Operator(op('-', Precedence::UNARY)), tokens[0]);
    assert_eq!(Token::Operator(op('(', Precedence::OPEN)), tokens[1]);
    assert_eq!(Token::IntegerLiteral(10), tokens[2]);
    assert_eq!(Token::Operator(op('+', Precedence::SUM)), tokens[3]);
    assert_eq!(Token::IntegerLiteral(5), tokens[4]);
    assert_eq!(Token::Operator(op(')', Precedence::CLOSE)), tokens[5]);
    assert_eq!(Token::Operator(op('-', Precedence::MIN)), tokens[6]);
    assert_eq!(Token::Operator(op('-', Precedence::UNARY)), tokens[7]);
    assert_eq!(Token::Operator(op('(', Precedence::OPEN)), tokens[8]);
    assert_eq!(Token::Operator(op('+', Precedence::UNARY)), tokens[9]);
    assert_eq!(Token::IntegerLiteral(3), tokens[10]);
    assert_eq!(Token::Operator(op('-', Precedence::MIN)), tokens[11]);
    assert_eq!(Token::IntegerLiteral(2), tokens[12]);
    assert_eq!(Token::Operator(op(')', Precedence::CLOSE)), tokens[13]);
}

#[test]
fn test_unary_results() {
    let mut sya = Sya::new("--5").expect("Should Construct");
    assert_eq!(Ok(()), sya.calculate());
    assert_eq!(Some(5), sya.out);

    sya.new_input("-(-5)").expect("Should Parse");
    assert_eq!(Ok(()), sya.calculate());
    assert_eq!(Some(5), sya.out);

    sya.new_input("--+++--+-+9").expect("Should Parse");
    assert_eq!(Ok(()), sya.calculate());
    assert_eq!(Some(-9), sya.out);

    sya.new_input("-(-(5+(8-3))*(+4^2))").expect("Should Parse");
    assert_eq!(Ok(()), sya.calculate());
    assert_eq!(Some(160), sya.out);

    sya.new_input("-(10*(2+-3))/-(4-(2*+3))")
        .expect("Should Parse");
    assert_eq!(Ok(()), sya.calculate());
    assert_eq!(Some(5), sya.out);
}

#[test]
fn test_rpn() {
    let mut sya = Sya::new("1 + 2 * 4 - 3").expect("Should Construct");
    sya.calculate().expect("Should Calculate");
    let rpn = &sya.rpn_stack;

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

    assert_eq!(sya.rpn_formatted(), "1 2 4 * + 3 -")
}

#[test]
fn test_calculate() {
    let mut sya = Sya::new("1 + 2 * 4 - 3").expect("Should Construct");

    assert_eq!(Ok(()), sya.calculate());
    assert_eq!(Some(6), sya.out);

    sya.new_input("1 - 2").expect("Should Parse");
    assert_eq!(Ok(()), sya.calculate());
    assert_eq!(Some(-1), sya.out);

    sya.new_input("").expect("Should Parse");
    assert_eq!(Err("Couldn't find a result".to_string()), sya.calculate());
    assert_eq!(None, sya.out);
}

#[test]
fn test_paren() {
    let mut sya = Sya::new("5 * (1 + 2)").expect("Should Construct");

    assert_eq!(Ok(()), sya.calculate());
    assert_eq!(Some(15), sya.out);

    sya.new_input("(1 + (1 + 2))").expect("Should Parse");
    assert_eq!(Ok(()), sya.calculate());
    assert_eq!(Some(4), sya.out);

    sya.new_input("(((((1)))))").expect("Should Parse");
    assert_eq!(Ok(()), sya.calculate());
    assert_eq!(Some(1), sya.out);

    sya.new_input("5 ) + 1").expect("Should Parse");
    assert_eq!(
        Err("Expected Open Parenthesis '('".to_string()),
        sya.calculate()
    );
}
