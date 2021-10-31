use crate::token::Token;
use crate::token::{ArithOperator, Bracket};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::statement::*;

pub struct Error {
    exp: &'static [&'static str],
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "expect {:?}", self.exp.join(" or "))
    }
}

type Result<T> = core::result::Result<T, Error>;

pub struct Parser {
    token: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn read_line(token: Vec<Token>) -> Result<Statement> {
        (Parser { token, index: 0 }).test_statement()
    }

    fn expected(&self, exp: &'static [&'static str]) -> Error {
        Error { exp }
    }

    fn next_word(&mut self) -> Option<&Token> {
        let token = self.token.get(self.index);
        if self.index < self.token.len() {
            self.index += 1;
        }

        token
    }

    fn look_ahead(&self) -> Option<&Token> {
        self.token.get(self.index)
    }

    fn forward(&mut self) {
        if self.index < self.token.len() {
            self.index += 1;
        }
    }

    fn test_statement(&mut self) -> Result<Statement> {
        self.test_expression().map(Statement::Expression)
    }

    fn test_expression(&mut self) -> Result<Expression> {
        let negative = match self.look_ahead() {
            Some(&Token::ArithOperator(ArithOperator::Plus)) => {
                self.forward();
                false
            }
            Some(&Token::ArithOperator(ArithOperator::Minus)) => {
                self.forward();
                true
            }
            _ => false,
        };

        let left = self.test_term()?;
        let rights = &self.test_test_expression_prime()?;
        Ok(rights.iter().fold(
            Expression {
                right: Rc::new(left),
                left: None,
                negative,
            },
            |acc, (op, term)| -> Expression {
                Expression {
                    right: term.clone(),
                    left: Some((*op, Box::new(acc))),
                    negative: false,
                }
            },
        ))
    }

    fn test_test_expression_prime(&mut self) -> Result<Vec<(ArithOperator, Rc<Term>)>> {
        let mut res = vec![];
        while let Some(&Token::ArithOperator(op @ (ArithOperator::Plus | ArithOperator::Minus))) =
            self.look_ahead()
        {
            self.forward();
            res.push((op, Rc::new(self.test_term()?)));
        }

        Ok(res)
    }

    fn test_term(&mut self) -> Result<Term> {
        let left = self.test_factor()?;
        let right = &self.test_term_prime()?;
        Ok(right.iter().fold(
            Term {
                left: None,
                right: Rc::new(left),
            },
            |acc, (op, factor)| -> Term {
                Term {
                    left: Some((*op, Box::new(acc))),
                    right: factor.clone(),
                }
            },
        ))
    }

    fn test_term_prime(&mut self) -> Result<Vec<(ArithOperator, Rc<Factor>)>> {
        let mut res = vec![];
        while let Some(&Token::ArithOperator(
            op @ (ArithOperator::Multiply | ArithOperator::Division),
        )) = self.look_ahead()
        {
            self.forward();
            res.push((op, Rc::new(self.test_factor()?)));
        }

        Ok(res)
    }

    fn test_factor(&mut self) -> Result<Factor> {
        let left = match self.next_word() {
            Some(Token::Bracket(Bracket::Left)) => {
                let expr = self.test_expression()?;
                if !matches!(self.next_word(), Some(Token::Bracket(Bracket::Right))) {
                    return Err(self.expected(&[")"]));
                }
                Factor::Expression(Box::new(expr))
            }
            Some(&Token::Number(num)) => Factor::Number(num),
            _ => return Err(self.expected(&["(", "Number"])),
        };

        Ok(match self.test_factor_prime()? {
            Some(right) => Factor::Power(Box::new(left), Box::new(right)),
            None => left,
        })
    }

    fn test_factor_prime(&mut self) -> Result<Option<Factor>> {
        if matches!(
            self.look_ahead(),
            Some(Token::ArithOperator(ArithOperator::Power))
        ) {
            self.forward();
            return Ok(Some(self.test_factor()?));
        }

        Ok(None)
    }
}
