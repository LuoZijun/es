
mod parser;
mod expression;
mod statement;

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


// https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#Table
// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// pub enum Precedence {
//     Parenthesized,  // ()
    
//     MemberAccessor,
//     NewWithArguments,
//     Call,

//     NewWithoutArguments,

//     PostIncrement,
//     PostDecrement,

//     // TODO
// }