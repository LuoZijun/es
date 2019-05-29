mod id;
mod white_space;

pub use crate::id::{
    is_id_start, is_id_continue, is_id_continue_only,
    ZWJ, ZWNJ, DOLLAR_SIGN, LOW_LINE,
};
pub use crate::white_space::{
    is_es_line_terminator, is_es_whitespace,
    TAB, VT, FF, SP, NBSP, ZWNBSP,
    CR, LF, LS, PS,
};

// Single Escape Character
pub const BACKSPACE: char = '\u{0008}';  // \b
pub const SLASH: char     = '/';         // /
pub const BACKSLASH: char = '\\';        // \


#[inline]
pub fn is_es_punctuator(ch: char) -> bool {
    // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-punctuators
    match ch {
        '{' | '}' | '(' | ')' | '[' | ']' 
        | '.' | ';' | ':' | ',' | '?'
        | '/' | '!' | '|' | '&' | '^'
        | '<' | '>' | '~' | '%' | '=' 
        | '+' | '-' | '*' | '`' | '#' => true,
        _ => false,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Category {
    WhiteSpace,
    LineTerminator,
    // ECMAScript Punctuator
    Punctuator,
    // \
    BackSlash, // escape
    // IdentifierPart: Include IdentifierStart
    // Unicode ID_Continue or Unicode ID_Start
    IDContinue,
    // IdentifierPart: ID_Continue
    IDContinueOnly,
    // 0 ... 9
    Digit,
    Unspecified,
}


#[inline]
pub fn eschar_category(ch: char) -> Category {
    match ch {
        '0' ... '9' => Category::Digit,
        BACKSLASH   => Category::BackSlash,
        CR | LF | LS | PS => Category::WhiteSpace,
        TAB | VT | FF | SP | NBSP | ZWNBSP
        | '\u{0085}'
        | '\u{1680}'
        | '\u{2000}' ..= '\u{200A}'
        | '\u{202F}'
        | '\u{205F}'
        | '\u{3000}' => Category::LineTerminator,
        '{' | '}' | '(' | ')' | '[' | ']' 
        | '.' | ';' | ':' | ',' | '?'
        | '/' | '!' | '|' | '&' | '^'
        | '<' | '>' | '~' | '%' | '=' 
        | '+' | '-' | '*' | '`' | '#' => Category::Punctuator,
        // Fast
        '$' | '_' | 'a' ... 'z' | 'A' ... 'Z' => Category::IDContinue,
        ZWJ | ZWNJ => Category::IDContinueOnly,
        _ => match is_id_continue(ch) {
            true => match is_id_continue_only(ch) {
                true => Category::IDContinueOnly,
                false => Category::IDContinue,
            },
            false => Category::Unspecified,
        },
    }
}

