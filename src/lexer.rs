use crate::token::{ArithOperator, Bracket, Token};
use core::result;
use std::fmt::{Display, Formatter};
use std::option::Option::Some;

#[derive(Default)]
pub struct Lexer {
    index: usize,
    forward: usize,
    source: Vec<char>,
}

pub struct Pos {
    column: usize,
}

type ErrorReason = &'static str;

const ERROR_UNEXPECTED_CHARACTER: ErrorReason = "unexpected character";
const ERROR_NUMBER_START_WITH_ZERO: ErrorReason = "non-decimal start with zero";
const ERROR_NUMBER_END_WITH_NON_NUMERIC: ErrorReason = "number end with non-numeric";
const ERROR_NUMBER_REPEAT_POINT: ErrorReason = "repeat point in number";

pub struct Error {
    reason: String,
    location: Pos,
    character: char,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "column {}: {} `{}`",
            self.location.column + 1,
            self.reason,
            self.character,
        )
    }
}

type Result<T> = result::Result<T, Error>;

impl Lexer {
    pub fn read_line<T: AsRef<str>>(input: T) -> Result<Vec<Token>> {
        Self::from_line(input).token()
    }

    fn from_line<T: AsRef<str>>(input: T) -> Self {
        Lexer {
            index: 0,
            forward: 0,
            source: input.as_ref().chars().collect(),
        }
    }

    fn get_char(&self, i: usize) -> Option<char> {
        self.source.get(i).map(|c| *c)
    }

    fn next_char(&mut self) -> Option<char> {
        self.index += 1;
        self.forward = self.index;
        self.get_char(self.index - 1)
    }

    fn current_char(&self) -> char {
        self.get_char(self.index - 1)
            .expect("current_char should call at valid index")
    }

    fn forward_char(&mut self) -> Option<char> {
        self.forward += 1;
        self.get_char(self.forward - 1)
    }

    fn error_at<E: Display>(&self, reason: E, column: usize) -> Error {
        Error {
            reason: reason.to_string(),
            location: Pos { column },
            character: self
                .source
                .get(column)
                .expect(format!("column({}) out of source", column).as_str())
                .clone(),
        }
    }

    fn token(&mut self) -> Result<Vec<Token>> {
        let mut result = vec![];
        while let Some(ch) = self.next_char() {
            if !ch.is_ascii_whitespace() {
                result.push(match ch {
                    '(' | ')' => Token::Bracket(Bracket::from_char(ch)),
                    '+' | '-' | '*' | '/' | '^' => {
                        Token::ArithOperator(ArithOperator::from_char(ch))
                    }
                    '0'..='9' => self.test_number()?,
                    _ => return Err(self.error_at(ERROR_UNEXPECTED_CHARACTER, self.index - 1)),
                });
            }
        }

        Ok(result)
    }

    fn test_number(&mut self) -> Result<Token> {
        let mut number = vec![];
        number.push(self.current_char());

        let mut point = false;
        while let Some(ch) = self.forward_char() {
            match ch {
                '+' | '-' | '*' | '/' | '^' | ')' | ' ' => break,
                '0'..'9' => number.push(ch),
                '.' => (!point)
                    .then(|| {
                        number.push(ch);
                        point = true;
                    })
                    .ok_or(self.error_at(ERROR_NUMBER_REPEAT_POINT, self.forward - 1))?,
                _ => return Err(self.error_at(ERROR_UNEXPECTED_CHARACTER, self.forward - 1)),
            }
        }

        if !number.last().unwrap().is_ascii_digit() {
            return Err(self.error_at(ERROR_NUMBER_END_WITH_NON_NUMERIC, self.forward - 1));
        }

        if !point && number.starts_with(&['0']) {
            return Err(self.error_at(ERROR_NUMBER_START_WITH_ZERO, self.index - 1));
        }

        self.index += number.len() - 1;

        number
            .iter()
            .collect::<String>()
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|err| self.error_at(err, self.index))
    }
}
