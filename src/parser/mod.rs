
mod parser;
mod expression;
mod statement;

pub use self::parser::{ Parser, parse, };





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