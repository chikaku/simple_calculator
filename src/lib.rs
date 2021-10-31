#![feature(exclusive_range_pattern)]
#![feature(bool_to_option)]

mod lexer;
mod parser;
mod statement;
mod token;
mod vm;

pub use vm::{Error, VM};
