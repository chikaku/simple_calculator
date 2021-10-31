use simple_calculator_deliver::TokenWord;
use std::fmt::{Debug, Formatter};

#[derive(TokenWord, PartialEq)]
pub enum Bracket {
    #[char('(')]
    Left,
    #[char(')')]
    Right,
}

#[derive(TokenWord, PartialEq, Copy, Clone)]
pub enum ArithOperator {
    #[char('+')]
    Plus,
    #[char('-')]
    Minus,
    #[char('*')]
    Multiply,
    #[char('/')]
    Division,
    #[char('^')]
    Power,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Bracket(Bracket),
    ArithOperator(ArithOperator),
    Number(Number),
}

pub type Number = f64;
