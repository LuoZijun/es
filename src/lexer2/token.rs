use super::keyword::Keyword;
use super::punctuator::Punctuator;
use super::span::{ LineColumn, Span, };

use ast::numberic::{ Float,  };

use std::fmt;
use std::cmp;
use std::hash;
use std::str::FromStr;
use std::convert::From;
use std::convert::TryFrom;
use std::marker::PhantomData;


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum WhiteSpace {
    TAB,
    VT,
    FF,
    SP,
    NBSP,
    ZWNBSP,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LineTerminator {
    CarriageReturn,     // CR   : \r
    LineFeed,           // LF   : \n
    EndOfLine,          // CR+LF: \r\n
    LineSeparator,      // LS   : U+2028
    ParagraphSeparator, // PS   : U+2029
    // NextLine,           // NEL  : U+0085
}




// MemSize: 16 Bytes
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    EndOfProgram,
    UnexpectedToken,
    UnexpectedEof,

    HashBang,
    
    SingleLineComment,
    MultiLineComment,
    
    WhiteSpaces,
    LineTerminator,

    Punctuator(Punctuator),
    Keyword(Keyword),
    Identifier(Vec<char>),
    
    LiteralNull,
    LiteralBoolean(bool),
    
    LiteralString(Vec<char>),
    // LiteralHexNumeric(u64),
    // LiteralBinaryNumeric(u64),
    // LiteralOctalNumeric(u64),
    LiteralDecimalNumeric(u64),
    LiteralFloatNumeric(Float),

    // LiteralRegularExpression(RegularExpressionLiteral),
}

impl Token {
    pub fn is_error(&self) -> bool {
        use self::Token::*;

        match *self {
            UnexpectedToken | UnexpectedEof => true,
            _ => false,
        }
    }

    pub fn is_hashbang(&self) -> bool {
        match *self {
            Token::HashBang => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        use self::Token::*;

        match *self {
            LiteralNull
            | LiteralBoolean(_) 
            | LiteralString(_)
            | LiteralDecimalNumeric(_)
            | LiteralFloatNumeric(_) => true,
            _ => false,
        }
    }
}

pub type SpannedToken = Span<Token>;

