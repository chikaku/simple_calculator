use crate::parser;
use crate::parser::Parser;
use crate::statement::Executor;
use crate::{lexer, lexer::Lexer};
use std::fmt::{Debug, Display, Formatter};

pub struct VM {
    debug: bool,
    history: Vec<String>,
}

// TODO: 写一个过程宏实现下面两个输出
pub enum Error {
    Lexer(lexer::Error),
    Parser(parser::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Lexer(err) => {
                write!(f, "{}", err)
            }
            Error::Parser(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Lexer(err) => {
                write!(f, "lex error: {}", err)
            }
            Error::Parser(err) => {
                write!(f, "parse error {}", err)
            }
        }
    }
}

type Result = core::result::Result<f64, Error>;

impl VM {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            history: vec![],
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn exec_line<T: AsRef<str> + Copy>(&mut self, line: T) -> Result {
        let token = Lexer::read_line(line).map_err(Error::Lexer)?;
        let stmt = Parser::read_line(token).map_err(Error::Parser)?;

        self.history.push(line.as_ref().to_string());
        self.debug.then(|| println!("{:?}", stmt));

        Ok(stmt.execute())
    }
}
