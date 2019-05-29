mod id;
mod white_space;

use crate::id::{ ZWJ, ZWNJ, DOLLAR_SIGN, LOW_LINE, };
use crate::white_space::{
    TAB, VT, FF, SP, NBSP, ZWNBSP,
    CR, LF, LS, PS,
};

pub use crate::id::{ is_id_start, is_id_continue, is_id_continue_only, };
pub use crate::white_space::{ is_es_line_terminator, is_es_whitespace, };

// Single Escape Character
// const BACKSPACE: char = '\u{0008}';  // \b
// const SLASH: char     = '/';         // /
const BACKSLASH: char = '\\';        // \


#[inline]
pub fn is_es_punctuator(ch: char) -> bool {
    // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-punctuators
    match ch {
        '{' | '}' | '(' | ')' | '[' | ']' 
        | '.' | ';' | ':' | ',' | '?'
        | '/' | '!' | '|' | '&' | '^'
        | '<' | '>' | '~' | '%' | '=' 
        | '+' | '-' | '*' | '`' | '#'
        | '\'' | '"' => true,
        _ => false,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Category {
    /// ```
    /// TAB, VT, FF, SP, NBSP, ZWNBSP, 
    /// '\u{0085}', '\u{1680}', '\u{2000}' ..= '\u{200A}',
    /// '\u{202F}', '\u{205F}', '\u{3000}'
    /// ```
    WhiteSpace,
    /// ```
    /// CR: '\r', LF: '\n', LS: '\u{2028}', PS: '\u{2029}'
    /// ```
    LineTerminator,
    /// ```text
    /// '{' | '}' | '(' | ')' | '[' | ']' 
    /// | '.' | ';' | ':' | ',' | '?'
    /// | '/' | '!' | '|' | '&' | '^'
    /// | '<' | '>' | '~' | '%' | '=' 
    /// | '+' | '-' | '*' | '`' | '#'
    /// | '\'' | '"'
    /// ```
    Punctuator,
    /// '\'
    BackSlash, // escape
    // Unicode ID_Continue or Unicode ID_Start
    /// IdentifierStart or IdentifierPart
    IDContinue,
    /// IdentifierPart Only
    IDContinueOnly,
    /// 0 ... 9
    Digit,
    /// Unexpected Character
    Unspecified,
}


#[inline]
pub fn eschar_category(ch: char) -> Category {
    match ch {
        '0' ... '9' => Category::Digit,
        BACKSLASH   => Category::BackSlash,
        CR | LF | LS | PS => Category::LineTerminator,
        TAB | VT | FF | SP | NBSP | ZWNBSP
        | '\u{0085}'
        | '\u{1680}'
        | '\u{2000}' ..= '\u{200A}'
        | '\u{202F}'
        | '\u{205F}'
        | '\u{3000}' => Category::WhiteSpace,
        '{' | '}' | '(' | ')' | '[' | ']' 
        | '.' | ';' | ':' | ',' | '?'
        | '/' | '!' | '|' | '&' | '^'
        | '<' | '>' | '~' | '%' | '=' 
        | '+' | '-' | '*' | '`' | '#'
        | '\'' | '"' => Category::Punctuator,
        // Fast path
        DOLLAR_SIGN | LOW_LINE | 'a' ... 'z' | 'A' ... 'Z' => Category::IDContinue,
        ZWJ | ZWNJ => Category::IDContinueOnly,
        // Slow Path
        _ => match is_id_continue(ch) {
            true => match is_id_continue_only(ch) {
                true => Category::IDContinueOnly,
                false => Category::IDContinue,
            },
            false => Category::Unspecified,
        },
    }
}

