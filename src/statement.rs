use crate::token::{ArithOperator, Number};
use std::fmt::{Debug, Formatter};
use std::option::Option::Some;
use std::rc::Rc;

pub enum Statement {
    Expression(Expression),
}

pub struct Expression {
    pub left: Option<(ArithOperator, Box<Expression>)>,
    pub right: Rc<Term>,
    pub negative: bool,
}

pub struct Term {
    pub left: Option<(ArithOperator, Box<Term>)>,
    pub right: Rc<Factor>,
}

pub enum Factor {
    Number(Number),
    Expression(Box<Expression>),
    Power(Box<Factor>, Box<Factor>),
}

type Value = f64;

pub trait Executor {
    fn execute(&self) -> Value;
}

impl Executor for Statement {
    fn execute(&self) -> Value {
        match self {
            Statement::Expression(expr) => expr.execute(),
        }
    }
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Expression(expr) => write!(f, "{:?}", expr),
        }
    }
}

impl Executor for Expression {
    fn execute(&self) -> Value {
        let right = computation(
            self.right.execute(),
            ArithOperator::Multiply,
            if self.negative {
                -1.0 as Value
            } else {
                1.0 as Value
            },
        );

        match &self.left {
            Some((op, expr)) => computation(expr.execute(), *op, right),
            _ => right,
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.left {
            Some((op, expr)) => write!(f, "{:?} {:?} {:?}", expr, op, self.right),
            _ => {
                if self.negative {
                    write!(f, "(-{:?})", self.right)
                } else {
                    write!(f, "{:?}", self.right)
                }
            }
        }
    }
}

impl Executor for Term {
    fn execute(&self) -> Value {
        match &self.left {
            Some((op, term)) => computation(term.execute(), *op, self.right.execute()),
            _ => self.right.execute(),
        }
    }
}

impl Debug for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.left {
            Some((op, term)) => {
                write!(f, "({:?} {:?} {:?})", term, op, self.right)
            }
            None => write!(f, "{:?}", self.right),
        }
    }
}

impl Executor for Factor {
    fn execute(&self) -> Value {
        match &self {
            Factor::Number(num) => *num,
            Factor::Expression(expr) => expr.execute(),
            Factor::Power(left, right) => {
                computation(left.execute(), ArithOperator::Power, right.execute())
            }
        }
    }
}

impl Debug for Factor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Factor::Number(num) => write!(f, "{:?}", num),
            Factor::Expression(expr) => write!(f, "{:?}", expr),
            Factor::Power(left, right) => write!(f, "({:?} ^ {:?})", left, right),
        }
    }
}

fn computation(left: Value, op: ArithOperator, right: Value) -> Value {
    match op {
        ArithOperator::Plus => left + right,
        ArithOperator::Minus => left - right,
        ArithOperator::Multiply => left * right,
        ArithOperator::Division => left / right,
        ArithOperator::Power => left.powf(right),
    }
}
