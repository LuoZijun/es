pub mod span;
pub mod htmlentity;
pub mod eschar;
pub mod escape;
pub mod numberic;
pub mod punctuator;
pub mod keyword;
pub mod token;

pub mod operator;
pub mod utf8;


use crate::toolshed::{ Arena, };
use crate::unicode_xid::UnicodeXID;

use crate::error::{ Error, ErrorKind, };
use crate::ast::numberic::{ Float, Numberic, };

use crate::lexer::eschar::{
    ESChar,
    CR, LF, LS, PS,
    TAB, VT, FF, SP, NBSP, ZWNBSP,
};
use crate::lexer::span::{ LineColumn, Span, Loc, };
use crate::lexer::numberic::parse_numberic;
use crate::lexer::escape::{ unescape_string, unescape_template, unescape_identifier, };
use crate::lexer::punctuator::PunctuatorKind;
use crate::lexer::token::{
    Comment, HashBang, LiteralTemplate, LiteralRegularExpression,
    Token, Punctuator, Identifier, 
    LiteralNumeric, LiteralString, LiteralBoolean, LiteralNull, 
};

use crate::parser::Parser;

use self::LexerErrorKind::*;


macro_rules! bump_with_token {
    ( $lexer:expr, $token:expr ) => {
        {
            let _ = $lexer.bump();

            return Ok(Some($token));
        }
        
    };
}

macro_rules! bump_or_with_token {
    ( $lexer:expr, $token:expr ) => {
        match $lexer.bump() {
            Ok(_) => { },
            Err(_) => {
                return Ok($token);
            },
        }
    };
}

macro_rules! bump_or_with_error {
    ( $lexer:expr, $err:expr ) => {
        match $lexer.bump() {
            Ok(_) => { },
            Err(_) => {
                return Err($lexer.error($err));
            },
        }
    };
}


#[derive(Debug)]
pub enum LexerErrorKind {
    UnexpectedCharacter,
    UnexpectedEOF,
    Custom(&'static str),
}


pub struct Lexer<'ast> {
    arena: &'ast Arena,
    source: &'ast [char],
    filename: &'ast str,
    
    pub offset: usize,
    pub line_offset: usize,
    pub line: usize,
    pub column: usize,

    token_start_offset: usize,
    token_start_line_offset: usize,
    token_start_line: usize,
    token_start_column: usize,
}

impl<'ast> Lexer<'ast> {
    pub fn new(arena: &'ast Arena, source: &'ast [char], filename: &'ast str) -> Self {
        assert_eq!(source.len() > 0, true);

        Self {
            arena,
            source,
            filename,

            offset: 0,
            line_offset: 0,
            line: 0,
            column: 0,

            token_start_offset: 0,
            token_start_line_offset: 0,
            token_start_line: 0,
            token_start_column: 0,
        }
    }

    #[inline]
    pub fn source(&self) -> &'ast [char] {
        &self.source
    }

    #[inline]
    pub fn filename(&self) -> &'ast str {
        self.filename
    }
    
    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn mark_token_start(&mut self) {
        self.token_start_offset = self.offset;
        self.token_start_line_offset = self.line_offset;
        self.token_start_line = self.line;
        self.token_start_column = self.column;
    }

    #[inline]
    pub fn loc_start(&self) -> usize {
        self.token_start_offset
    }
    
    #[inline]
    pub fn loc_end(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn loc(&self) -> Loc {
        Loc { start: self.loc_start(), end: self.loc_end() }
    }

    #[inline]
    pub fn span_start(&self) -> LineColumn {
        LineColumn {
            offset: self.token_start_line_offset,
            line: self.token_start_line,
            column: self.token_start_column,
        }
    }

    #[inline]
    pub fn span_end(&self) -> LineColumn {
        LineColumn {
            offset: self.line_offset,
            line: self.line,
            column: self.column,
        }
    }

    #[inline]
    pub fn span(&self) -> Span {
        Span {
            start: self.span_start(),
            end: self.span_end(),
        }
    }

    #[inline]
    pub fn error_line(&self) -> String {
        let mut idx = self.line_offset;
        for c in &self.source[self.line_offset..] {
            if (*c as char).is_es_line_terminator() {
                break;
            }

            idx += 1;
        }

        let code_line = self.source[self.line_offset..idx].iter().collect::<String>();

        let prefix_width = format!("{}", self.line).len() + 1;
        let prefix = " ".repeat(prefix_width);

        let line_number = format!("{:<width$}", self.line + 1, width=prefix_width);

        format!("{}|\n{}| {}\n{}| {}^", prefix, line_number, code_line, prefix, 
            " ".repeat(self.column))
    }

    #[inline]
    pub fn error(&self, lexer_error_kind: LexerErrorKind) -> Error {
        let kind = ErrorKind::SyntaxError;
        let message = match lexer_error_kind {
            LexerErrorKind::UnexpectedCharacter => {
                let ch = if self.eof() { *self.source.last().unwrap() } else { self.character() };
                format!("Unexpected Character `{}` ", ch.escape_default().collect::<String>())
            },
            LexerErrorKind::UnexpectedEOF => {
                format!("Unexpected EOF")
            },
            LexerErrorKind::Custom(msg) => {
                msg.to_string()
            }
        };

        let filename = self.filename;
        let line_number = self.line;
        let column_number = self.column;
        let line = self.error_line();

        let mut err = Error::new(kind, message);
        err.set_stack(filename, line_number, column_number, Some(line));

        err
    }

    #[inline]
    pub fn bump(&mut self) -> Result<(), ()> {
        let max_len = self.source.len() - 1;

        if self.offset < max_len {
            self.offset += 1;
            self.column += 1;
            Ok(())
        } else if self.offset == max_len {
            self.offset += 1;
            Err(())
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn bump_line(&mut self) {
        self.line += 1;
        self.column = 0;
        self.line_offset = self.offset;
    }
    
    #[inline]
    pub fn eof(&self) -> bool {
        if self.offset >= self.source.len() {
            true
        } else {
            false
        }
    }

    #[inline]
    fn character(&self) -> char {
        self.source[self.offset]
    }

    #[inline]
    fn scan_line_terminator(&mut self) {
        match self.character() {
            CR => {
                match self.bump() {
                    Ok(_) => {
                        if self.character() == LF {
                            let _ = self.bump();
                        }
                    },
                    Err(_) => { }
                }
            },
            _ => {
                let _ = self.bump();
            }
        }

        self.bump_line();
    }

    #[inline]
    fn peek(&mut self) -> Result<Option<Token<'ast>>, Error> {
        unimplemented!()
    }
    
    #[inline]
    fn scan_unicode_escape_seq(&mut self) -> Result<(), Error> {
        bump_or_with_error!(self, UnexpectedEOF);
        
        let c = self.character();
        
        match c {
            '{' => {
                bump_or_with_error!(self, UnexpectedEOF);

                // let start = self.offset;
                loop {
                    match self.character() {
                        '}' => {
                            break;
                        },
                        c => {
                            if !c.is_es_hex_digit() {
                                return Err(self.error(UnexpectedCharacter));
                            }

                            bump_or_with_error!(self, UnexpectedEOF);
                        }
                    }
                }
            },
            _ => {
                if !c.is_es_hex_digit() {
                    return Err(self.error(UnexpectedCharacter));
                }

                bump_or_with_error!(self, UnexpectedEOF);
                if !self.character().is_es_hex_digit() {
                    return Err(self.error(UnexpectedCharacter));
                }

                bump_or_with_error!(self, UnexpectedEOF);
                if !self.character().is_es_hex_digit() {
                    return Err(self.error(UnexpectedCharacter));
                }

                bump_or_with_error!(self, UnexpectedEOF);
                if !self.character().is_es_hex_digit() {
                    return Err(self.error(UnexpectedCharacter));
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn scan_hex_escape_seq(&mut self) -> Result<(), Error> {
        bump_or_with_error!(self, UnexpectedEOF);
        if !self.character().is_es_hex_digit() {
            return Err(self.error(UnexpectedCharacter));
        }

        bump_or_with_error!(self, UnexpectedEOF);
        if !self.character().is_es_hex_digit() {
            return Err(self.error(UnexpectedCharacter));
        }

        Ok(())
    }

    #[inline]
    pub fn read_literal_string(&mut self) -> Result<Option<Token<'ast>>, Error> {
        let openning = self.character();
        bump_or_with_error!(self, UnexpectedEOF);
        
        self.mark_token_start();

        let mut has_escape_character = false;

        loop {
            let c = self.character();
            match c {
                '\\' => {
                    if !has_escape_character {
                        has_escape_character = true;
                    }

                    bump_or_with_error!(self, UnexpectedEOF);
                    match self.character() {
                        '0' => {
                            bump_or_with_error!(self, UnexpectedEOF);
                            if self.character().is_es_decimal_digit() {
                                return Err(self.error(UnexpectedCharacter));
                            }
                        },
                        'x' => {
                            self.scan_hex_escape_seq()?;
                            bump_or_with_error!(self, UnexpectedEOF);
                        },
                        'u' => {
                            self.scan_unicode_escape_seq()?;
                            bump_or_with_error!(self, UnexpectedEOF);
                        },
                        CR => {
                            bump_or_with_error!(self, UnexpectedEOF);
                            if self.character() == LF {
                                bump_or_with_error!(self, UnexpectedEOF);
                            }
                            self.bump_line();
                        },
                        LF | LS | PS => {
                            bump_or_with_error!(self, UnexpectedEOF);
                            self.bump_line();
                        },
                        _ => {
                            bump_or_with_error!(self, UnexpectedEOF);
                        }
                    }
                },
                _ => {
                    if c == openning {
                        let _ = self.bump();
                        break;
                    } else {
                        if c.is_es_line_terminator() {
                            return Err(self.error(UnexpectedCharacter));
                        }

                        bump_or_with_error!(self, UnexpectedEOF);
                    }
                }
            }
        }

        let loc = self.loc();
        let span = self.span();
        let raw = &self.source[loc.start..loc.end-1];
        let mut cooked: Option<&'ast [char]> = None;

        if has_escape_character {
            match unescape_string(raw) {
                Ok(s) => {
                    cooked = Some(self.arena.alloc_vec(s));
                },
                Err(e) => {
                    let mut offset = e.offset();
                    self.offset = self.token_start_offset + offset;
                    self.line = self.token_start_line;
                    self.column = self.token_start_column;
                    // FIXME: fix offset ?
                    // repr
                    // while offset > 0 {

                    // }
                    return Err(self.error(UnexpectedCharacter));
                }
            }
        }

        let es_str = LiteralString { loc, span, raw, cooked };

        Ok(Some(Token::LiteralString(es_str)))
    }

    #[inline]
    fn scan_literal_numberic_float(&mut self) -> Result<(), Error> {
        bump_or_with_error!(self, UnexpectedEOF);

        loop {
            match self.character() {
                '0' ... '9' => {
                    if let Err(_) = self.bump() {
                        break;
                    }
                },
                'e' | 'E' => {
                    return self.scan_literal_numberic_scientific();
                },
                _ => {
                    break;
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn scan_literal_numberic_scientific(&mut self) -> Result<(), Error> {
        bump_or_with_error!(self, UnexpectedEOF);
        
        match self.character() {
            '-' | '+' => {
                bump_or_with_error!(self, UnexpectedEOF);
            },
            _ => {

            },
        }

        loop {
            match self.character() {
                '0' ... '9' => {
                    if let Err(_) = self.bump() {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }

        Ok(())
    }

    #[inline]
    pub fn read_literal_numberic(&mut self) -> Result<Option<Token<'ast>>, Error> {
        self.mark_token_start();
        
        match self.character() {
            '0' => {
                match self.bump() {
                    Ok(_)  => { },
                    Err(_) => {
                        let loc = self.loc();
                        let span = self.span();
                        let raw = &self.source[loc.start..loc.end];
                        let value = Numberic::ZERO;
                        let zero = LiteralNumeric { span, loc, raw, value };
                        let token = Token::LiteralNumeric(zero);
                        
                        return Ok(Some(token));
                    },
                }

                let c = self.character();
                match c {
                    '0' ... '9' => {
                        return Err(self.error(UnexpectedCharacter));
                    },
                    'b' | 'B' => {
                        bump_or_with_error!(self, UnexpectedEOF);

                        loop {
                            match self.character() {
                                '0' | '1' => {
                                    match self.bump() {
                                        Ok(_) => { },
                                        Err(_) => {
                                            break;
                                        }
                                    }
                                },
                                _ => {
                                    break;
                                }
                            }
                        }
                    },
                    'o' | 'O' => {
                        bump_or_with_error!(self, UnexpectedEOF);

                        loop {
                            match self.character() {
                                '0' ... '7' => {
                                    match self.bump() {
                                        Ok(_) => { },
                                        Err(_) => {
                                            break;
                                        }
                                    }
                                },
                                _ => {
                                    break;
                                }
                            }
                        }
                    },
                    'x' | 'X' => {
                        bump_or_with_error!(self, UnexpectedEOF);

                        loop {
                            match self.character() {
                                '0' ... '9' | 'a' ... 'f' | 'A' ... 'F' => {
                                    match self.bump() {
                                        Ok(_) => { },
                                        Err(_) => {
                                            break;
                                        }
                                    }
                                },
                                _ => {
                                    break;
                                }
                            }
                        }
                    },
                    'e' | 'E' => {
                        self.scan_literal_numberic_scientific()?;
                    },
                    '.' => {
                        self.scan_literal_numberic_float()?;
                    },
                    _ => {

                    }
                }
            },
            '1' ... '9' => {
                if let Ok(_) = self.bump() {
                    loop {
                        let c = self.character();
                        match c {
                            '0' ... '9' => {
                                if let Err(_) = self.bump() {
                                    break;
                                }
                            },
                            'e' | 'E' => {
                                self.scan_literal_numberic_scientific()?;
                            },
                            '.' => {
                                self.scan_literal_numberic_float()?;
                            },
                            _ => {
                                break;
                            }
                        }
                    }
                }
            },
            _ => unreachable!(),
        }

        let loc = self.loc();
        let span = self.span();
        let raw = &self.source[loc.start..loc.end];

        match parse_numberic(raw) {
            Ok(value) => {
                let num = LiteralNumeric { span, loc, raw, value };
                
                Ok(Some(Token::LiteralNumeric(num)))
            },
            Err(e) => {
                let mut offset = e.offset();
                self.offset = self.token_start_offset + offset;
                self.line = self.token_start_line;
                self.column = self.token_start_column;
                // FIXME: fix offset ?
                // repr
                // while offset > 0 {

                // }
                Err(self.error(UnexpectedCharacter))
            }
        }
    }
    
    #[inline]
    pub fn read_identifier(&mut self) -> Result<Option<Token<'ast>>, Error> {
        // IdentifierName and Keywords
        self.mark_token_start();

        let mut has_escape_character: bool = false;

        loop {
            let c = self.character();
            match c {
                '\\' => {
                    if !has_escape_character {
                        has_escape_character = true;
                    }

                    bump_or_with_error!(self, UnexpectedEOF);

                    match self.character() {
                        'u' => {
                            self.scan_unicode_escape_seq()?;
                            if let Err(_) = self.bump() {
                                break;
                            }
                        },
                        _ => {
                            return Err(self.error(UnexpectedCharacter));
                        }
                    }
                },
                _ => {
                    if !c.is_es_identifier_part() {
                        break;
                    }
                    
                    if let Err(_) = self.bump() {
                        break;
                    }
                }
            }
        }

        let loc = self.loc();
        let span = self.span();
        let raw = &self.source[loc.start..loc.end];
        let mut cooked: Option<&'ast [char]> = None;
        
        if has_escape_character {
            match unescape_identifier(raw) {
                Ok(s) => {
                    cooked = Some(self.arena.alloc_vec(s));
                },
                Err(e) => {
                    let mut offset = e.offset();
                    self.offset = self.token_start_offset + offset;
                    self.line = self.token_start_line;
                    self.column = self.token_start_column;
                    // FIXME: fix offset ?
                    // repr
                    // while offset > 0 {

                    // }
                    return Err(self.error(UnexpectedCharacter));
                }
            }
        }

        match cooked {
            Some(ref s) => {
                if !s[0].is_es_identifier_start() {
                    self.offset = self.token_start_offset;
                    self.line = self.token_start_line;
                    self.column = self.token_start_column;
                    return Err(self.error(UnexpectedCharacter));
                }

                for part in &s[1..] {
                    if !part.is_es_identifier_part() {
                        self.offset = self.token_start_offset;
                        self.line = self.token_start_line;
                        self.column = self.token_start_column;
                        return Err(self.error(UnexpectedCharacter));
                    }
                }
            },
            None => { }
        }

        let ident = Identifier { loc, span, raw, cooked };

        Ok(Some(Token::Identifier(ident)))
    }

    #[inline]
    pub fn read_punctuator(&mut self) -> Result<Option<Token<'ast>>, Error> {
        self.mark_token_start();

        let c = self.character();

        // '{' | '}' | '(' | ')' | '[' | ']' 
        // | '+' | '-' | '*' | '/'
        // | '.' | ';' | ':' | ',' | '?'
        // | '!' | '|' | '&' | '^'
        // | '<' | '>' | '~' | '%' | '=' 
        macro_rules! punct {
            ( $punctuator:ident ) => {
                {
                    let loc = self.loc();
                    let span = self.span();
                    let kind = PunctuatorKind::$punctuator;
                    let token = Token::Punctuator(Punctuator { loc, span, kind });
                    token
                }
            };
        }

        macro_rules! bump_with_punct {
            ( $punctuator:ident ) => {
                bump_with_token!(self, punct!($punctuator))
            };
        }

        macro_rules! bump_or_with_punct {
            ( $punctuator:ident ) => {
                bump_or_with_token!(self, Some(punct!($punctuator)))
            };
        }

        match c {
            '{' => bump_with_punct!(LBrace),
            '}' => bump_with_punct!(RBrace),
            '(' => bump_with_punct!(LParen),
            ')' => bump_with_punct!(RParen),
            '[' => bump_with_punct!(LBracket),
            ']' => bump_with_punct!(RBracket),
            ',' => bump_with_punct!(Comma),
            ':' => bump_with_punct!(Colon),
            ';' => bump_with_punct!(Semicolon),
            '?' => bump_with_punct!(Question),
            '.' => {
                bump_or_with_punct!(Dot);
                
                match self.character() {
                    '.' => {
                        bump_or_with_error!(self, UnexpectedCharacter);

                        if self.character() == '.' {
                            bump_with_punct!(DotDotDot)
                        } else {
                            return Err(self.error(UnexpectedCharacter));
                        }
                    },
                    _ => Ok(Some(punct!(Dot)))
                }
            },
            '/' => {
                bump_or_with_punct!(Div);
                match self.character() {
                    '/' => {
                        // //
                        loop {
                            if let Err(_) = self.bump() {
                                break;
                            }

                            if self.character().is_es_line_terminator() {
                                break;
                            }
                        }

                        return self.consume();
                    },
                    '*' => {
                        // /*
                        bump_or_with_error!(self, UnexpectedEOF);

                        loop {
                            let c = self.character();
                            match c {
                                '*' => {
                                    bump_or_with_error!(self, UnexpectedEOF);
                                    
                                    if self.character() == '/' {
                                        break;
                                    }
                                },
                                _ => {
                                    if c.is_es_line_terminator() {
                                        self.scan_line_terminator();
                                    } else {
                                        bump_or_with_error!(self, UnexpectedEOF);
                                    }
                                },
                            }
                        }
                        
                        let _ = self.bump();

                        return self.consume();
                    },
                    '=' => {
                        // /=
                        bump_with_punct!(DivAssign)
                    },
                    _ => Ok(Some(punct!(Div))),
                }
            },
            '*' => {
                bump_or_with_punct!(Mul);
                
                match self.character() {
                    '*' => {
                        bump_or_with_punct!(Pow);

                        match self.character() {
                            '=' => {
                                bump_with_punct!(PowAssign);
                            },
                            _ => Ok(Some(punct!(Pow)))
                        }
                    },
                    '=' => {
                        bump_with_punct!(MulAssign);
                    },
                    _ => Ok(Some(punct!(Mul)))
                }
            },
            '+' => {
                bump_or_with_punct!(Add);

                match self.character() {
                    '+' => {
                        bump_with_punct!(Increment);
                    },
                    '=' => {
                        bump_with_punct!(AddAssign);
                    },
                    _ => Ok(Some(punct!(Add)))
                }
            },
            '-' => {
                bump_or_with_punct!(Sub);

                match self.character() {
                    '-' => {
                        bump_with_punct!(Decrement);
                    },
                    '=' => {
                        bump_with_punct!(SubAssign);
                    },
                    _ => Ok(Some(punct!(Sub)))
                }
            },
            '%' => {
                bump_or_with_punct!(Rem);

                match self.character() {
                    '=' => {
                        bump_with_punct!(RemAssign);
                    },
                    _ => Ok(Some(punct!(Rem)))
                }
            },
            '|' => {
                bump_or_with_punct!(BitOr);

                match self.character() {
                    '=' => {
                        bump_with_punct!(BitOrAssign);
                    },
                    _ => Ok(Some(punct!(BitOr)))
                }
            },
            '&' => {
                bump_or_with_punct!(BitAnd);

                match self.character() {
                    '=' => {
                        bump_with_punct!(BitAndAssign);
                    },
                    _ => Ok(Some(punct!(BitAnd)))
                }
            },
            '^' => {
                bump_or_with_punct!(BitXor);

                match self.character() {
                    '=' => {
                        bump_with_punct!(BitXorAssign);
                    },
                    _ => Ok(Some(punct!(BitXor)))
                }
            },
            '~' => bump_with_punct!(BitNot),
            '=' => {
                bump_or_with_punct!(Assign);

                match self.character() {
                    '=' => {
                        bump_or_with_punct!(Eq);

                        match self.character() {
                            '=' => {
                                bump_with_punct!(StrictEq);
                            },
                            _ => Ok(Some(punct!(Eq)))
                        }
                    },
                    '>' => {
                        // =>
                        bump_with_punct!(FatArrow);
                    },
                    _ => Ok(Some(punct!(Assign)))
                }
            },
            '!' => {
                bump_or_with_punct!(Not);
                
                match self.character() {
                    '=' => {
                        bump_or_with_punct!(Neq);

                        match self.character() {
                            '=' => {
                                bump_with_punct!(StrictNeq);
                            },
                            _ => Ok(Some(punct!(Neq)))
                        }
                    },
                    _ => Ok(Some(punct!(Not)))
                }
            },
            '<' => {
                // <
                // Lt,             // <
                // LtEq,           // <=
                // BitShl,         // <<
                // BitShlAssign,   // <<=
                bump_or_with_punct!(Lt);
                
                match self.character() {
                    '=' => bump_with_punct!(LtEq),
                    '<' => {
                        bump_or_with_punct!(BitShl);

                        match self.character() {
                            '=' => {
                                bump_with_punct!(BitShlAssign);
                            },
                            _ => Ok(Some(punct!(BitShl)))
                        }
                    }
                    _ => Ok(Some(punct!(Lt)))
                }
            },
            '>' => {
                // >
                // Gt,             // >
                // GtEq,           // >=
                // BitShr,         // >>
                // BitShrAssign,   // >>=
                // BitUShr,        // >>>
                // BitUShrAssign,  // >>>=
                bump_or_with_punct!(Gt);
                
                match self.character() {
                    '=' => bump_with_punct!(GtEq),
                    '>' => {
                        bump_or_with_punct!(BitShr);

                        match self.character() {
                            '=' => bump_with_punct!(BitShrAssign),
                            '>' => {
                                bump_or_with_punct!(BitUShr);
                                match self.character() {
                                    '=' => bump_with_punct!(BitUShrAssign),
                                    _ => Ok(Some(punct!(BitUShr))),
                                }
                            }
                            _ => Ok(Some(punct!(BitShr)))
                        }
                    }
                    _ => Ok(Some(punct!(Gt)))
                }
            },
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn read_literal_regular_expression(&mut self) -> Result<Option<Token<'ast>>, Error> {
        if self.eof() {
            return Err(self.error(UnexpectedEOF));
        }

        loop {
            let c = self.character();
            match c {
                '/' => {
                    break;
                },
                '\\' => {
                    bump_or_with_error!(self, UnexpectedEOF);
                    let c = self.character();
                    if c.is_es_line_terminator() {
                        return Err(self.error(UnexpectedCharacter));
                    }

                    bump_or_with_error!(self, UnexpectedEOF);
                },
                _ => {
                    if c.is_es_line_terminator() {
                        return Err(self.error(UnexpectedCharacter));
                    }

                    bump_or_with_error!(self, UnexpectedEOF);
                }
            }
        }
        
        let body_start = self.token_start_offset + 1;
        let body_end   = self.offset;

        let body = &self.source[body_start..body_end];

        // read flags
        // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_Expressions#Advanced_searching_with_flags_2
        // g, i, m, s, u, y
        let flags = match self.bump() {
            Ok(_) => {
                
                let start = self.offset;

                loop {
                    let c = self.character();
                    
                    // It is a Syntax Error if identifierPart contains a Unicode escape sequence.
                    if !c.is_es_identifier_part() {
                        break;
                    }

                    if !c.is_ascii_alphabetic() {
                        warn!("regular flags must is one of `a-Z` or `A-Z`. ");
                        return Err(self.error(UnexpectedCharacter));
                    }

                    if let Err(_) = self.bump() {
                        break;
                    }
                }

                let end = self.offset;

                if end == start {
                    None
                } else {
                    let flags = &self.source[start..end];
                    Some(flags)
                }
            },
            Err(_) => None,
        };

        let span = self.span();
        let loc = self.loc();
        let item = LiteralRegularExpression { span, loc, body, flags };
        let token = Token::LiteralRegularExpression(item);
        
        Ok(Some(token))
    }

    #[inline]
    pub fn read_literal_template_string(&mut self) -> Result<(LiteralString<'ast>, bool), Error> {
        // NOTE: 可以和 read_literal_string 函数合并，绝大部分代码是相同的。
        if self.eof() {
            return Err(self.error(UnexpectedEOF));
        }

        self.mark_token_start();

        let mut has_escape_character = false;
        
        let start = self.offset;

        #[allow(unused_assignments)]
        let mut end: usize = start;

        let mut is_end: bool = false;

        loop {
            let c = self.character();
            match c {
                '\\' => {
                    if !has_escape_character {
                        has_escape_character = true;
                    }

                    bump_or_with_error!(self, UnexpectedEOF);
                    match self.character() {
                        '0' => {
                            bump_or_with_error!(self, UnexpectedEOF);
                            if self.character().is_es_decimal_digit() {
                                return Err(self.error(UnexpectedCharacter));
                            }
                        },
                        'x' => {
                            self.scan_hex_escape_seq()?;
                            bump_or_with_error!(self, UnexpectedEOF);
                        },
                        'u' => {
                            self.scan_unicode_escape_seq()?;
                            bump_or_with_error!(self, UnexpectedEOF);
                        },
                        CR => {
                            bump_or_with_error!(self, UnexpectedEOF);
                            if self.character() == LF {
                                bump_or_with_error!(self, UnexpectedEOF);
                            }
                            self.bump_line();
                        },
                        LF | LS | PS => {
                            bump_or_with_error!(self, UnexpectedEOF);
                            self.bump_line();
                        },
                        _ => {
                            bump_or_with_error!(self, UnexpectedEOF);
                        }
                    }
                },
                CR => {
                    bump_or_with_error!(self, UnexpectedEOF);
                    if self.character() == LF {
                        bump_or_with_error!(self, UnexpectedEOF);
                    }
                    self.bump_line();
                },
                LF | LS | PS => {
                    bump_or_with_error!(self, UnexpectedEOF);
                    self.bump_line();
                },
                '`' => {
                    end = self.offset;
                    let _ = self.bump();

                    is_end = true;

                    break;
                },
                '$' => {
                    bump_or_with_error!(self, UnexpectedEOF);
                    if self.character() == '{' {
                        bump_or_with_error!(self, UnexpectedEOF);

                        end = self.offset - 2;
                        break;
                    }
                },
                _ => {
                    bump_or_with_error!(self, UnexpectedEOF);
                }
            }
        }

        let loc = self.loc();
        let span = self.span();
        let raw = &self.source[start..end];
        let mut cooked: Option<&'ast [char]> = None;

        if has_escape_character {
            match unescape_template(raw) {
                Ok(s) => {
                    cooked = Some(self.arena.alloc_vec(s));
                },
                Err(e) => {
                    let mut offset = e.offset();
                    self.offset = self.token_start_offset + offset;
                    self.line = self.token_start_line;
                    self.column = self.token_start_column;
                    // FIXME: fix offset ?
                    // repr
                    // while offset > 0 {

                    // }
                    return Err(self.error(UnexpectedCharacter));
                }
            }
        }

        Ok((LiteralString { loc, span, raw, cooked }, is_end))
    }

    #[inline]
    pub fn consume(&mut self) -> Result<Option<Token<'ast>>, Error> {
        loop {
            if self.eof() {
                return Ok(None);
            }

            let c = self.character();
            match c {
                '#' => {
                    // HashBang: #!
                    if self.line != 0 || self.column != 0 {
                        return Err(self.error(UnexpectedCharacter));
                    }

                    bump_or_with_error!(self, UnexpectedEOF);

                    if self.character() != '!' {
                        return Err(self.error(UnexpectedCharacter));
                    }
                    
                    loop {
                        bump_or_with_token!(self, None);

                        let c = self.character();
                        if c.is_es_line_terminator() {
                            break;
                        }
                    }
                },
                TAB | VT | FF | SP | NBSP | ZWNBSP => {
                    bump_or_with_token!(self, None);
                },
                CR | LF | LS | PS => {
                    self.mark_token_start();

                    self.scan_line_terminator();
                    return Ok(Some(Token::LineTerminator));
                },
                '"' | '\'' => {
                    return self.read_literal_string();
                },
                '`' => {
                    // 指示模版开始，在 Parser 里面处理。
                    self.mark_token_start();

                    bump_or_with_error!(self, UnexpectedEOF);

                    return Ok(Some(Token::TemplateOpenning))
                },
                '0' ... '9' => {
                    return self.read_literal_numberic();
                },
                '\\' => {
                    // 携带 Unicode 转义序列的 identifier
                    return self.read_identifier();
                },
                '$' | '_' | 'a' ... 'z' | 'A' ... 'Z' => {
                    return self.read_identifier();
                },
                // punctuators
                '{' | '}' | '(' | ')' | '[' | ']' 
                | '.' | ';' | ':' | ',' | '?'
                | '/' | '!' | '|' | '&' | '^'
                | '<' | '>' | '~' | '%' | '=' 
                | '+' | '-' | '*'
                // | '`' | '#'
                => {
                    return self.read_punctuator();
                },
                _ => {
                    // Unicode
                    // Slow
                    if c.is_whitespace() {
                        // unicode whitespace
                        bump_or_with_token!(self, None);
                        continue;
                    }

                    if c.is_es_identifier_start() {
                        return self.read_identifier();
                    }

                    return Err(self.error(UnexpectedCharacter));
                },
            }
        }
    }

    #[inline]
    pub fn lookahead(&mut self, ch: char) -> bool {
        match self.source.get(self.offset+1) {
            Some(c) => c == &ch,
            None => false,
        }
    }
}


pub fn tokenize(source: &str, filename: &str) {
    // WARN: 因为 正则表达式 和 模版字面量 这两个 Token 需要上下文才能界定边界，
    //       所以请确保源代码当中不含 这些 字面量。
    let arena = Arena::new();
    let code = arena.alloc_vec(source.chars().collect::<Vec<char>>());
    let filename = arena.alloc_str(filename);
    
    let mut lexer = Lexer::new(&arena, &code, &filename);

    loop {
        match lexer.consume() {
            Ok(Some(token)) => {
                println!("{:?}", token);
            },
            Ok(None) => {
                trace!("EOF.");
                break;
            },
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
}