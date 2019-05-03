
mod parser;
mod expression;
mod statement;

mod class;
mod function;
mod pattern;


pub use self::parser::{ Parser, parse, };


#[derive(Debug)]
pub enum ParserError {
    UnexpectedCharacter,
    UnexpectedToken,
    UnexpectedExpression,
    UnexpectedStatement,
    UnexpectedEOF,
    Custom(&'static str),
}