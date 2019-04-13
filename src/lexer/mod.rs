mod htmlentity;
mod eschar;
pub mod escape;

mod token;
mod token2;

mod punctuator;
mod keyword;

// mod operator;


use crate::unicode_xid::UnicodeXID;


pub use self::eschar::{
    ESChar, Category, 
    CR, LF, LS, PS,
    TAB, VT, FF, SP, NBSP, ZWNBSP,
};
pub use self::token::*;
pub use self::punctuator::*;
pub use self::keyword::*;
pub use self::htmlentity::HTMLEntity;

use error::Error;
use ast::span::{ LineColumn, Span, };
use ast::expression::RegularExpressionLiteral;

use std::str::FromStr;
use std::convert::TryFrom;


pub const LITERAL_NULL: &[char]   = &['n', 'u', 'l', 'l'];
pub const LITERAL_TRUE: &[char]   = &['t', 'r', 'u', 'e'];
pub const LITERAL_FALSE: &[char]  = &['f', 'a', 'l', 's', 'e'];


#[derive(Debug)]
pub enum LexerResult {
    EOF,
    Ok(SpannedToken),
    Err(Error),
}


pub type CharHandler = fn(lexer: &mut Lexer) -> ();


#[inline]
fn ignore(lexer: &mut Lexer) {
    // \0
    lexer.bump();
    lexer.token = Token::EndOfProgram;
}

#[inline]
fn invalid(lexer: &mut Lexer) {
    // invalid character, not an ecmascript source character
    lexer.bump();
    lexer.token = Token::UnexpectedToken;
}

#[inline]
fn whitespace(lexer: &mut Lexer) {
    lexer.bump();

    loop {
        let c = lexer.character();
        if c.is_es_whitespace() {
            lexer.bump();
        } else {
            break;
        }
    }

    lexer.token = Token::WhiteSpaces;
}

#[inline]
fn line_terminator(lexer: &mut Lexer) {
    let c = lexer.character();
    
    lexer.bump();
    lexer.bump_line();

    if c == CR {
        match lexer.character() {
            LF => {
                // \r\n
                lexer.bump();
                lexer.bump_line();
            },
            _ => { }
        }
    }

    lexer.token = Token::LineTerminator;
}

// #!
#[inline]
fn hashbang(lexer: &mut Lexer) {
    if lexer.line != 0 || lexer.column != 0 {
        return lexer.token = Token::UnexpectedToken;
    }

    lexer.bump();

    match lexer.character() {
        '!' => {
            loop {
                lexer.bump();

                let c = lexer.character();
                if c.is_es_line_terminator() {
                    break;
                }
                if c == '\0' {
                    break;
                }
            }
        },
        _ => return lexer.token = Token::UnexpectedToken,
    }
    
    lexer.token = Token::HashBang;
}

// IdentifierName and Keywords
#[inline]
fn identifier(lexer: &mut Lexer) {
    // Unicode ident part
    let mut ident: Vec<char> = Vec::with_capacity(16);
    let mut has_escape_character: bool = false;

    let first_char = lexer.character();
    if first_char == '\\' {
        lexer.bump();
        match lexer.read_unicode_escape() {
            Some(first_char) => {
                if !first_char.is_es_identifier_start() {
                    lexer.token = Token::UnexpectedToken;
                    return ();
                }

                has_escape_character = true;
                ident.push(first_char);
            },
            None => {
                lexer.token = Token::UnexpectedToken;
                return ();
            }
        }
    } else {
        ident.push(first_char);
        lexer.bump();
    }

    loop {
        let c = 
            match lexer.character() {
                '\\' => {
                    lexer.bump();

                    match lexer.read_unicode_escape() {
                        Some(c) => {
                            if !has_escape_character {
                                has_escape_character = true;
                            }

                            c
                        },
                        None => {
                            lexer.token = Token::UnexpectedToken;
                            return ();
                        }
                    }
                },
                '\0' => {
                    break;
                },
                c => c,
            };

        if !c.is_es_identifier_part() {
            if c.is_es_whitespace() || c.is_es_line_terminator() || c.is_es_punctuator_start() {
                break;
            } else {
                lexer.token = Token::UnexpectedToken;
                return ();
            }
        }

        ident.push(c);
        lexer.bump();
    }
    
    if !has_escape_character {
        let ident_ref = ident.as_slice();
        
        if ident_ref == LITERAL_NULL {
            return lexer.token = Token::LiteralNull;
        }
        if ident_ref == LITERAL_TRUE {
            return lexer.token = Token::LiteralBoolean(true);
        }
        if ident_ref == LITERAL_FALSE {
            return lexer.token = Token::LiteralBoolean(false);
        }

        match Keyword::try_from(ident_ref) {
            Ok(kw) => return lexer.token = Token::Keyword(kw),
            Err(_) => { },
        }
    }

    lexer.token = Token::Identifier(ident);
}

#[inline]
fn literal_string(lexer: &mut Lexer) {
    let openning = lexer.character();
    
    lexer.bump();

    let closing_delimiter = openning;
    let is_template = false;
    let allow_line_terminator = false;
    let unescape = true;

    if let Some(chars) = lexer.read_string_literal(closing_delimiter,
                                                   is_template,
                                                   allow_line_terminator,
                                                   unescape) {
        lexer.token = Token::LiteralString(chars);
    }
}

#[inline]
fn digit(lexer: &mut Lexer) {
    match lexer.read_numeric_literal() {
        Some(token) => {
            lexer.token = token;
        },
        None => {
            return ();
        }
    }
}

// `
#[inline]
fn punct_template(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::BackTick);
}

// {
    #[inline]
fn punct_lbrace(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::LBrace);
}

// }
#[inline]
fn punct_rbrace(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::RBrace);
}

// [
#[inline]
fn punct_lbracket(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::LBracket);
}

// ]
#[inline]
fn punct_rbracket(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::RBracket);
}

// (
#[inline]
fn punct_lparen(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::LParen);
}

// )
#[inline]
fn punct_rparen(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::RParen);
}

// .
// ...
#[inline]
fn punct_dotmark(lexer: &mut Lexer) {
    const DOT_MARK: Token = Token::Punctuator(Punctuator::Dot);
    const SPREAD: Token = Token::Punctuator(Punctuator::DotDotDot);

    lexer.bump();
    match lexer.character() {
        '.' => {
            lexer.bump();
            match lexer.character() {
                '.' => {
                    lexer.bump();
                    lexer.token = SPREAD
                },
                _ => lexer.token = Token::UnexpectedToken,
            }
        },
        _ => return lexer.token = DOT_MARK,
    }
}

// ,
#[inline]
fn punct_comma(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::Comma);
}

// :
#[inline]
fn punct_colon(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::Colon);
}

// ;
#[inline]
fn punct_semicolon(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::Semicolon);
}

// ?
#[inline]
fn punct_question(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::Question);
}

// |
// ||
// |=
#[inline]
fn punct_bitor(lexer: &mut Lexer) {
    lexer.bump();

    match lexer.character() {
        '|' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::Or);
        },
        '=' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::BitOrAssign);
        },
        _ => lexer.token = Token::Punctuator(Punctuator::BitOr),
    }
}

// ~
#[inline]
fn punct_bitnot(lexer: &mut Lexer) {
    lexer.bump();
    lexer.token = Token::Punctuator(Punctuator::BitNot);
}

// ^
// ^=
#[inline]
fn punct_bitxor(lexer: &mut Lexer) {
    // BitXor, BitXorAssign
    lexer.bump();
    
    match lexer.character() {
        '=' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::BitXorAssign);
        },
        _ => lexer.token = Token::Punctuator(Punctuator::BitXor),
    }
}

// &
// &&
// &=
#[inline]
fn punct_bitand(lexer: &mut Lexer) {
    // BitAnd, And, BitAndAssign
    lexer.bump();

    match lexer.character() {
        '&' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::And);
        },
        '=' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::BitAndAssign);
        },
        _ => lexer.token = Token::Punctuator(Punctuator::BitAnd),
    }
}

// =
// =>
// ==
// ===
#[inline]
fn punct_assign(lexer: &mut Lexer) {
    // Assign, FatArrow, Eq, StrictEq
    lexer.bump();

    match lexer.character() {
        '>' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::FatArrow);
        },
        '=' => {
            lexer.bump();

            match lexer.character() {
                '=' => {
                    lexer.bump();
                    lexer.token = Token::Punctuator(Punctuator::StrictEq)
                },
                _ => lexer.token = Token::Punctuator(Punctuator::Eq),
            }
        },
        _ => lexer.token = Token::Punctuator(Punctuator::Assign),
    }
}

// <
// Lt,             // <
// LtEq,           // <=
// BitShl,         // <<
// BitShlAssign,   // <<=
#[inline]
fn punct_lt(lexer: &mut Lexer) {
    lexer.bump();

    match lexer.character() {
        '=' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::LtEq);
        },
        '<' => {
            lexer.bump();
            match lexer.character() {
                '=' => {
                    lexer.bump();
                    lexer.token = Token::Punctuator(Punctuator::BitShlAssign);
                },
                _ => lexer.token = Token::Punctuator(Punctuator::BitShl),
            }
        },
        _ => lexer.token = Token::Punctuator(Punctuator::Lt),
    }

}

// >
// Gt,             // >
// GtEq,           // >=
// BitShr,         // >>
// BitShrAssign,   // >>=
// BitUShr,        // >>>
// BitUShrAssign,  // >>>=
#[inline]
fn punct_gt(lexer: &mut Lexer) {
    lexer.bump();

    match lexer.character() {
        '=' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::GtEq);
        },
        '>' => {
            lexer.bump();

            match lexer.character() {
                '=' => {
                    lexer.bump();
                    lexer.token = Token::Punctuator(Punctuator::BitShrAssign);
                },
                '>' => {
                    lexer.bump();
                    
                    match lexer.character() {
                        '=' => {
                            lexer.bump();
                            lexer.token = Token::Punctuator(Punctuator::BitUShrAssign);
                        },
                        _ => lexer.token = Token::Punctuator(Punctuator::BitUShr),
                    }
                },
                _ => lexer.token = Token::Punctuator(Punctuator::BitShr),
            }
        },
        _ => lexer.token = Token::Punctuator(Punctuator::Gt),
    }
}

// +
// +=
// ++
#[inline]
fn punct_add(lexer: &mut Lexer) {
    lexer.bump();

    match lexer.character() {
        '=' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::AddAssign);
        },
        '+' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::Increment);
        },
        _ => lexer.token = Token::Punctuator(Punctuator::Add),
    }
}

// -
// -=
// --
#[inline]
fn punct_sub(lexer: &mut Lexer) {
    // Sub, SubAssign, Decrement, 
    lexer.bump();

    match lexer.character() {
        '=' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::SubAssign);
        },
        '-' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::Decrement);
        },
        _ => lexer.token = Token::Punctuator(Punctuator::Sub),
    }
}

// *
// *=
// **
// **=
#[inline]
fn punct_mul(lexer: &mut Lexer) {
    // Mul, Pow, MulAssign, PowAssign, 
    lexer.bump();

    match lexer.character() {
        '*' => {
            lexer.bump();

            match lexer.character() {
                '=' => {
                    lexer.bump();
                    lexer.token = Token::Punctuator(Punctuator::PowAssign);
                },
                _ => lexer.token = Token::Punctuator(Punctuator::Pow),
            }
        },
        '=' => {
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::MulAssign);
        },
        _ => lexer.token = Token::Punctuator(Punctuator::Mul),
    }
}

// /
// /=
// //
// /*
#[inline]
fn punct_div(lexer: &mut Lexer) {
    // Div, DivAssign, SingleLineComment, MultiLineComment
    lexer.bump();

    match lexer.character() {
        '=' => {
            // /=
            lexer.bump();
            lexer.token = Token::Punctuator(Punctuator::DivAssign);
        },
        '/' => {
            // //
            lexer.bump();
            loop {
                let c = lexer.character();
                if c.is_es_line_terminator() {
                    break;
                }

                if c == '\0' {
                    break;
                }

                lexer.bump();
            }
            
            lexer.token = Token::SingleLineComment;
        },
        '*' => {
            // /*
            lexer.bump();
            loop {
                let c = lexer.character();
                match c {
                    '*' => {
                        lexer.bump();

                        let c2 = lexer.character();
                        if c2 == '/' {
                            lexer.bump();
                            lexer.token = Token::MultiLineComment;
                            break;
                        }
                    },
                    '\0' => {
                        return lexer.token = Token::UnexpectedEof;
                    },
                    _ => {
                        if c.is_es_line_terminator() {
                            let _ = line_terminator(lexer);
                        } else {
                            lexer.bump();
                        }
                    }
                }

            }
        },
        _ => lexer.token = Token::Punctuator(Punctuator::Div),
    }
}

// %
// %=
#[inline]
fn punct_rem(lexer: &mut Lexer) {
    // Rem, RemAssign
    lexer.bump();

    if lexer.character() == '=' {
        lexer.bump();
        lexer.token = Token::Punctuator(Punctuator::RemAssign);
    } else {
        lexer.token = Token::Punctuator(Punctuator::Rem);
    }
}

// !
// !=
// !==
#[inline]
fn punct_not(lexer: &mut Lexer) {
    // Not, Neq, StrictNeq
    const NOT: Token = Token::Punctuator(Punctuator::Not);
    const NEQ: Token = Token::Punctuator(Punctuator::Neq);
    const STRICT_NEQ: Token = Token::Punctuator(Punctuator::StrictNeq);

    lexer.bump();

    match lexer.character() {
        '=' => {
            lexer.bump();

            if lexer.character() == '=' {
                lexer.bump();
                lexer.token = STRICT_NEQ;
            } else {
                lexer.token = NEQ;
            }
        },
        _ => lexer.token = NOT,
    }
}


pub static BASIC_LATIN: [CharHandler; 161] = [
    // 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    ignore, invalid, invalid, invalid, invalid, invalid, invalid, invalid,
    
    //       \t    \n                \r
    //       TAB   LF    VT    FF    CR
    // 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    invalid, whitespace, line_terminator, whitespace, whitespace, line_terminator, invalid, invalid,

    // 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    invalid, invalid, invalid, invalid, invalid, invalid, invalid, invalid,

    // 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    invalid, invalid, invalid, invalid, invalid, invalid, invalid, invalid,

    // SP
    // b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'',
    whitespace, punct_not, literal_string, hashbang, identifier, punct_rem, punct_bitand, literal_string,
    
    // b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
    punct_lparen, punct_rparen, punct_mul, punct_add, punct_comma, punct_sub, punct_dotmark, punct_div,

    // b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
    digit, digit, digit, digit, digit, digit, digit, digit, 

    // b'8', b'9', b':', b';', b'<', b'=', b'>', b'?',
    digit, digit, punct_colon, punct_semicolon, punct_lt, punct_assign, punct_gt, punct_question,

    // b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    invalid, identifier, identifier, identifier, identifier, identifier, identifier, identifier,

    // b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
    identifier, identifier, identifier, identifier, identifier, identifier, identifier, identifier,

    // b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
    identifier, identifier, identifier, identifier, identifier, identifier, identifier, identifier,

    // b'X', b'Y', b'Z', b'[', b'\\', b']', b'^', b'_',
    identifier, identifier, identifier, punct_lbracket, identifier, punct_rbracket, punct_bitxor, identifier,

    // b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
    punct_template,  identifier, identifier, identifier, identifier, identifier, identifier, identifier,

    // b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
    identifier, identifier, identifier, identifier, identifier, identifier, identifier, identifier,

    // b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
    identifier, identifier, identifier, identifier, identifier, identifier, identifier, identifier,

    //                                           DEL
    // b'x', b'y', b'z', b'{', b'|', b'}', b'~', 0x7f,
    identifier, identifier, identifier, punct_lbrace, punct_bitor, punct_rbrace, punct_bitnot, invalid,

    //                               NEL
    // 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
    invalid, invalid, invalid, invalid, invalid, invalid, invalid, invalid,
    // 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
    invalid, invalid, invalid, invalid, invalid, invalid, invalid, invalid,
    // 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
    invalid, invalid, invalid, invalid, invalid, invalid, invalid, invalid,
    // 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
    invalid, invalid, invalid, invalid, invalid, invalid, invalid, invalid,

    // NBSP
    // 0xa0, 
    whitespace,
];



pub struct Lexer<'a> {
    pub source: &'a [char],
    offset: usize,
    max_offset: usize,

    pub token: Token,
    line_offset: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [char]) -> Self {
        let source_len = source.len();
        Self {
            source,
            offset: 0,
            max_offset: source_len - 1,

            token: Token::UnexpectedToken,
            line_offset: 0,
            line: 0,
            column: 0,
        }
    }

    #[inline]
    pub fn bump(&mut self) {
        self.offset += 1;
        self.column += 1;
    }

    #[inline]
    pub fn bump_line(&mut self) {
        self.line_offset = self.offset;
        self.line += 1;
        self.column = 0;
    }
    
    #[inline]
    fn character(&self) -> char {
        self.source[self.offset]
    }

    #[inline]
    pub fn line_column(&self) -> LineColumn {
        let line = self.line;
        let column = self.column;

        LineColumn { offset: self.line_offset, line, column, }
    }

    #[inline]
    pub fn read_unicode_escape(&mut self) -> Option<char> {
        // http://xahlee.info/js/js_unicode_escape_sequence.html
        let mut s = String::with_capacity(6);

        match self.character() {
            'u' => {
                self.bump();
                
                match self.character() {
                    '{' => {
                        // \u{ XXXXXX }
                        // 4 - 6 hex digits
                        loop {
                            if s.len() > 6 {
                                self.token = Token::UnexpectedToken;
                                return None;
                            }

                            self.bump();
                            match self.character() {
                                '}' => {
                                    self.bump();
                                    break;
                                },
                                '\0' => {
                                    self.token = Token::UnexpectedEof;
                                    return None;
                                },
                                c => {
                                    if !c.is_es_hex_digit() {
                                        self.token = Token::UnexpectedToken;
                                        return None;
                                    }

                                    s.push(c);
                                }
                            }
                        }

                        if s.len() == 0 || s.len() > 6 {
                            self.token = Token::UnexpectedToken;
                            return None;
                        }
                    },
                    c => {
                        // \uXXXX
                        // 4 hex digits
                        loop {
                            if s.len() == 4 {
                                break;
                            }

                            let c = self.character();
                            if !c.is_es_hex_digit() {
                                break;
                            }

                            s.push(c);
                            self.bump();
                        }
                        
                        if s.len() != 4 {
                            self.token = Token::UnexpectedToken;
                            return None;
                        }
                    }
                }
            },
            '\0' => {
                self.token = Token::UnexpectedEof;
                return None;
            },
            _    => {
                self.token = Token::UnexpectedToken;
                return None;
            },
        }

        match u32::from_str_radix(&s, 16) {
            Ok(n) => match char::try_from(n) {
                Ok(c) => Some(c),
                Err(_) => {
                    self.token = Token::UnexpectedToken;
                    None
                }
            },
            Err(_) => {
                self.token = Token::UnexpectedToken;
                None
            }
        }
    }

    #[inline]
    pub fn read_string_literal(&mut self,
                               closing_delimiter: char,
                               is_template: bool,
                               allow_line_terminator: bool,
                               unescape: bool,
                               ) -> Option<Vec<char>> {
        
        let mut s: Vec<char> = Vec::new();

        loop {
            let c = self.character();
            match c {
                '\\' => {
                    // http://xahlee.info/js/js_string_escape.html
                    self.bump();

                    if !unescape {
                        s.push(c);
                        continue;
                    }

                    let c2 = self.character();
                    match c2 {
                        'x' => {
                            // \xXX
                            // 2 hex digits
                            let mut buf: String = String::with_capacity(2);
                            
                            loop {
                                if buf.len() == 2 {
                                    break;
                                }

                                self.bump();
                                let c = self.character();
                                if !c.is_es_hex_digit() {
                                    break;
                                }

                                buf.push(c);
                            }
                            
                            if buf.len() != 2 {
                                self.token = Token::UnexpectedEof;
                                return None;
                            }

                            match u32::from_str_radix(&buf, 16) {
                                Ok(n) => match char::try_from(n) {
                                    Ok(c) => {
                                        s.push(c);
                                    },
                                    Err(_) => {
                                        self.token = Token::UnexpectedToken;
                                        return None;
                                    }
                                },
                                Err(_) => {
                                    self.token = Token::UnexpectedToken;
                                    return None;
                                }
                            }
                        },
                        'u' => {
                            match self.read_unicode_escape() {
                                Some(c) => {
                                    s.push(c);
                                },
                                None => {
                                    self.token = Token::UnexpectedToken;
                                    return None;
                                }
                            }
                        },
                        CR => {
                            s.push(CR);
                            self.bump();
                            if self.character() == LF {
                                s.push(LF);
                                self.bump();
                            }
                            self.bump_line();
                        },
                        LF | LS | PS => {
                            s.push(c2);
                            self.bump();
                            self.bump_line();
                        },
                        'b' => {
                            // \b BACKSPACE
                            s.push('\u{0008}');
                            self.bump();
                        },
                        't' => {
                            // \t CHARACTER TABULATION
                            s.push('\t');
                            self.bump();
                        },
                        'n' => {
                            // \n LINE FEED
                            s.push(LF);
                            self.bump();
                        },
                        'v' => {
                            // \v LINE TABULATION
                            s.push('\u{000b}');
                            self.bump();
                        },
                        'f' => {
                            // \f FORM FEED
                            s.push('\u{000c}');
                            self.bump();
                        },
                        'r' => {
                            // \r CARRIAGE RETURN
                            s.push(CR);
                            self.bump();
                        },
                        '0' => {
                            s.push('\0');
                            self.bump();
                            
                            if self.character().is_es_decimal_digit() {
                                self.token = Token::UnexpectedToken;
                                return None;
                            }
                        },
                        _ => {
                            // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-additional-syntax-string-literals
                            if c2.is_es_decimal_digit() {
                                error!("Legacy octal escape is not permitted in strict mode");
                                self.token = Token::UnexpectedToken;
                                return None;
                            }

                            s.push(c2);
                            self.bump();
                        },
                    }
                },
                '\0' => {
                    self.token = Token::UnexpectedEof;
                    return None;
                },
                CR => {
                    if !allow_line_terminator {
                        self.token = Token::UnexpectedToken;
                        return None;
                    }

                    s.push(CR);
                    self.bump();
                    if self.character() == LF {
                        s.push(LF);
                        self.bump();
                    }

                    self.bump_line();
                },
                LF | LS | PS => {
                    if !allow_line_terminator {
                        self.token = Token::UnexpectedToken;
                        return None;
                    }

                    s.push(c);
                    self.bump();
                    self.bump_line();
                },
                _ => {
                    if is_template {
                        if c == '$' {
                            self.bump();
                            if self.character() != '{' {
                                s.push(c);
                                continue;
                            } else {
                                self.bump();
                                return Some(s);
                            }
                        }
                    }

                    if c == closing_delimiter {
                        self.bump();
                        return Some(s);
                    }
                    
                    s.push(c);
                    self.bump();
                }
            }
        }
    }
    
    #[inline]
    fn read_float(&mut self, numberic_string: &mut String) -> Result<(), ()> {
        loop {
            let c = self.character();

            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    numberic_string.push(c);
                    self.bump();
                },
                'e' | 'E' => {
                    self.bump();
                    return self.read_scientific(numberic_string);
                },
                _ => {
                    break;
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn read_scientific(&mut self, numberic_string: &mut String) -> Result<(), ()> {
        let c = self.character();
        match c {
            '+' | '-' => {
                numberic_string.push(c);
                self.bump();
            },
            _ => {

            }
        }

        let c = self.character();
        if !c.is_ascii_digit() {
            self.token = Token::UnexpectedToken;
            return Err(());
        }

        numberic_string.push(c);

        loop {
            self.bump();

            let c = self.character();
            if !c.is_ascii_digit() {
                break;
            }

            numberic_string.push(c);
        }

        Ok(())
    }

    #[inline]
    fn read_numeric_literal(&mut self) -> Option<Token> {
        let mut s = String::with_capacity(32);

        match self.character() {
            '0' => {
                self.bump();

                match self.character() {
                    'b' | 'B' => {
                        loop {
                            self.bump();

                            let c = self.character();
                            if !c.is_es_binary_digit() {
                                break;
                            }

                            s.push(c);
                        }

                        match u64::from_str_radix(&s, 2) {
                            Ok(n) => {
                                return Some(Token::LiteralDecimalNumeric(n));
                            },
                            Err(_) => {
                                self.token = Token::UnexpectedToken;
                                return None;
                            }
                        }
                    },
                    'o' | 'O' => {
                        loop {
                            self.bump();

                            let c = self.character();
                            if !c.is_es_octal_digit() {
                                break;
                            }

                            s.push(c);
                        }
                        
                        match u64::from_str_radix(&s, 8) {
                            Ok(n) => {
                                return Some(Token::LiteralDecimalNumeric(n));
                            },
                            Err(_) => {
                                self.token = Token::UnexpectedToken;
                                return None;
                            }
                        }
                    },
                    'x' | 'X' => {
                        loop {
                            self.bump();

                            let c = self.character();
                            if !c.is_es_hex_digit() {
                                break;
                            }

                            s.push(c);
                        }
                        
                        match u64::from_str_radix(&s, 16) {
                            Ok(n) => {
                                return Some(Token::LiteralDecimalNumeric(n));
                            },
                            Err(_) => {
                                self.token = Token::UnexpectedToken;
                                return None;
                            }
                        }
                    },
                    '.' => {
                        self.bump();
                        
                        s.push('0');
                        s.push('.');

                        match self.read_float(&mut s) {
                            Ok(_) => {
                                match s.parse::<f64>() {
                                    Ok(n) => {
                                        return Some(Token::LiteralFloatNumeric(n.into()));
                                    },
                                    Err(_) => {
                                        self.token = Token::UnexpectedToken;
                                        return None;
                                    }
                                }
                            },
                            Err(_) => {
                                return None;
                            }
                        }
                    },
                    'e' | 'E' => {
                        self.bump();
                        
                        s.push('0');
                        s.push('e');

                        match self.read_scientific(&mut s) {
                            Ok(_) => {
                                match s.parse::<f64>() {
                                    Ok(n) => {
                                        return Some(Token::LiteralFloatNumeric(n.into()));
                                    },
                                    Err(_) => {
                                        self.token = Token::UnexpectedToken;
                                        return None;
                                    }
                                }
                            },
                            Err(_) => {
                                return None;
                            }
                        }
                    },
                    c => {
                        if c.is_es_decimal_digit() {
                            warn!("please don't add zero on numeric's head.");
                            self.token = Token::UnexpectedToken;
                            return None;
                        }

                        self.bump();

                        return Some(Token::LiteralDecimalNumeric(0));
                    }
                }
            },
            c => {
                // 1 .. 9
                s.push(c);

                loop {
                    self.bump();
                    let c = self.character();
                    match c {
                        '.' => {
                            self.bump();
                            
                            s.push('.');

                            match self.read_float(&mut s) {
                                Ok(_) => {
                                    match s.parse::<f64>() {
                                        Ok(n) => {
                                            return Some(Token::LiteralFloatNumeric(n.into()));
                                        },
                                        Err(_) => {
                                            self.token = Token::UnexpectedToken;
                                            return None;
                                        }
                                    }
                                },
                                Err(_) => {
                                    return None;
                                }
                            }
                        },
                        'e' | 'E' => {
                            self.bump();
                            
                            s.push('e');

                            match self.read_scientific(&mut s) {
                                Ok(_) => {
                                    match s.parse::<f64>() {
                                        Ok(n) => {
                                            return Some(Token::LiteralFloatNumeric(n.into()));
                                        },
                                        Err(_) => {
                                            self.token = Token::UnexpectedToken;
                                            return None;
                                        }
                                    }
                                },
                                Err(_) => {
                                    return None;
                                }
                            }
                        },
                        _ => {
                            if !c.is_es_decimal_digit() {
                                match s.parse::<u64>() {
                                    Ok(n) => {
                                        return Some(Token::LiteralDecimalNumeric(n));
                                    },
                                    Err(_) => {
                                        self.token = Token::UnexpectedToken;
                                        return None;
                                    }
                                }
                            }

                            s.push(c);
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub fn read_regular_expression_literal(&mut self) -> Option<RegularExpressionLiteral> {
        let closing_delimiter = '/';
        let is_template = false;
        let allow_line_terminator = false;
        let unescape = false;

        let start = self.line_offset + self.column;
        let body = self.read_string_literal(closing_delimiter,
                                            is_template,
                                            allow_line_terminator,
                                            unescape)?;
        let end = self.line_offset + self.column;

        let mut flags: Option<Vec<char>> = None;
        
        // read flags
        // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_Expressions#Advanced_searching_with_flags_2
        // g, i, m, s, u, y
        loop {
            let c = self.character();

            // It is a Syntax Error if IdentifierPart contains a Unicode escape sequence.
            if !c.is_es_identifier_part() {
                break;
            }

            if !c.is_ascii_alphabetic() {
                warn!("regular flags must is one of `a-Z` or `A-Z`. ");
                break;
            }

            if flags.is_none() {
                flags = Some(Vec::with_capacity(3));
            }

            match &mut flags {
                Some(ref mut data) => {
                    data.push(c);
                },
                None => unreachable!(),
            }
            
            self.bump();
        }

        Some(RegularExpressionLiteral { body, flags, })
    }

    #[inline]
    fn read_template_literal(&mut self) -> Option<Token> {
        // 注: 这个应该在 Parser 层处理。
        unimplemented!()
    }

    #[inline]
    fn handler(&mut self) -> CharHandler {
        let c = self.character();
        const MAX_ASCII_CHAR: char = '\u{00a1}'; // 161

        if c < MAX_ASCII_CHAR {
            // ASCII
            BASIC_LATIN[c as usize]
        } else {
            // Slow
            match c {
                LS | PS => line_terminator, // Unicode Line Terminator
                ZWNBSP  => whitespace,      // Unicode whitespace
                _ => {
                    if UnicodeXID::is_xid_start(c) {
                        // ident part: ZWNJ | ZWJ
                        identifier
                    } else {
                        invalid
                    }
                },
            }
        }
    }

    #[inline]
    pub fn next_token(&mut self) -> LexerResult {
        self.handler()(self);
        unimplemented!()
    }

    #[inline]
    pub fn consume(&mut self) {
        // let start = self.line_column();

        // if self.is_eof() {
        //     self.token = Token::EndOfProgram;
        //     // let end = self.line_column();
        //     return ();
        //     // return Span { start, end, item: token };
        // }
        
        self.handler()(self);

        // loop {
        //     // let end = self.line_column();
        //     // let spanned_token: SpannedToken = Span { start, end, item: self.token.clone() };
        //     // return spanned_token
        // }
    }
}



pub fn tokenize(source: &str) {
    let mut code = source.chars().collect::<Vec<char>>();
    code.push('\0'); // EOF

    let mut lexer = Lexer::new(&code);
    
    loop {

        let start = lexer.line_column().offset + lexer.line_column().column;
        lexer.consume();
        let end = lexer.line_column().offset + lexer.line_column().column;

        match &lexer.token {
            Token::UnexpectedEof => {
                error!("UnexpectedEof");
                break;
            },
            Token::UnexpectedToken => {
                let end = std::cmp::min(code.len(), start + 20);
                error!("Token: {:?} Text: #{}#", &lexer.token, 
                            &code[start..end].iter().collect::<String>());
                break;
            },
            Token::LineTerminator => {
                // debug!("Token: {:?}", &lexer.token);
            },
            Token::EndOfProgram => {
                debug!("EOF.");
                break;
            },
            Token::SingleLineComment | Token::MultiLineComment | Token::WhiteSpaces => {

            },
            Token::Punctuator(Punctuator::Div) => {
                println!("{:?}", lexer.read_regular_expression_literal());
            },
            _ => {
                debug!("Token: {:?} Text: #{}#", &lexer.token, 
                            &code[start..end].iter().collect::<String>());
            },
        }
    }
}

#[bench]
fn bench_tokenization(b: &mut test::Bencher) {
    // let source = include_str!("../../data/react-16.8.3.development.js");
    let source = include_str!("../../data/colors.js");
    let mut code = source.chars().collect::<Vec<char>>();
    code.push('\0'); // EOF

    b.bytes = source.len() as _;

    b.iter(|| {
        let mut lexer = Lexer::new(&code);

        loop {
            lexer.consume();
            let token = &lexer.token;

            match token {
                Token::UnexpectedEof | Token::UnexpectedToken => {
                    panic!("{:?}", token);
                },
                Token::EndOfProgram => {
                    break;
                },
                _ => {

                }
            }
        }
    })
}