#[cfg(test)]
use crate::test;
use crate::unicode_xid::UnicodeXID;


pub mod token;


use self::token::{ Loc, Token, Keyword, Literal, Punctuator, TokenKind, TemplateItem, TemplateLiteral, };


use std::str::FromStr;
use std::convert::TryFrom;


// Format-Control Code Point
// https://www.ecma-international.org/ecma-262/#sec-unicode-format-control-characters
const ZWNJ  : char = '\u{200C}';  // identifier part
const ZWJ   : char = '\u{200D}';  // identifier part
const ZWNBSP: char = '\u{FEFF}';  // whitespace

// White Space Code Points
// https://www.ecma-international.org/ecma-262/#sec-white-space
const TAB: char  = '\u{0009}';
const VT: char   = '\u{000B}';
const FF: char   = '\u{000C}';
const SP: char   = '\u{0020}';
const NBSP: char = '\u{00A0}';

// Line Terminator Code Points
// https://www.ecma-international.org/ecma-262/#sec-line-terminators
const CR: char = '\u{000D}';   // \r
const LF: char = '\u{000A}';   // \n
const LS: char = '\u{2028}';
const PS: char = '\u{2029}';


#[inline]
pub fn is_line_terminator(c: char) -> bool {
    // https://www.ecma-international.org/ecma-262/#sec-line-terminators
    match c {
        CR | LF | LS | PS => true,
        _ => false,
    }
}

#[inline]
pub fn is_whitespace(c: char) -> bool {
    // https://www.ecma-international.org/ecma-262/#sec-white-space
    match c {
        TAB | VT | FF | SP | NBSP | ZWNBSP => true,
        _ => {
            if is_line_terminator(c) {
                false
            } else {
                c.is_whitespace()
            }
        },
    }
}

#[inline]
pub fn is_punctuator(c: char) -> bool {
    // https://www.ecma-international.org/ecma-262/#sec-punctuators
    match c {
        '{' | '}' | '(' | ')' | '[' | ']' 
        | '.' | ';' | ':' | ',' | '?'
        | '/' | '!' | '|' | '&' | '^'
        | '<' | '>' | '~' | '%' | '=' 
        | '+' | '-' | '*' => true,
        _ => false,
    }
}

#[inline]
pub fn is_identifier_start(c: char) -> bool {
    // https://www.ecma-international.org/ecma-262/#prod-IdentifierStart
    match c {
        '_' | '$' => true,
        _ => UnicodeXID::is_xid_start(c),
    }
}

#[inline]
pub fn is_identifier_part(c: char) -> bool {
    // https://www.ecma-international.org/ecma-262/#prod-IdentifierPart
    match c {
        '$' | ZWNJ | ZWJ => true,
        _ => UnicodeXID::is_xid_continue(c),
    }
}

#[inline]
pub fn is_binary_digit(c: char) -> bool {
    match c {
        '0' | '1' => true,
        _ => false,
    }
}

#[inline]
pub fn is_octal_digit(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => true,
        _ => false,
    }
}

#[inline]
pub fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}


#[inline]
pub fn is_escape_character(c: char) -> bool {
    // https://www.ecma-international.org/ecma-262/#prod-SingleEscapeCharacter
    // ', ", \, b, f, n, r, t, v
    match c {
        '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => true,
        _ => false,
    }
}

#[inline]
pub fn escape_character_to_char(c: char) -> char {
    match c {
        '\'' | '"' | '\\' => c,
        'b' => '\u{0008}',
        'f' => '\u{000c}',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        'v' => '\u{000b}',
        _ => unreachable!(),
    }
}

#[inline]
pub fn is_regular_expression_first_char(c: char) -> bool {
    // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-RegularExpressionFirstChar
    if is_line_terminator(c) {
        return false;
    }
    
    match c {
        '*' | '\\' | '/' | '[' => false,
        _ => true,
    }
}

macro_rules! bump_with_token {
    ( $lexer:expr, $token:expr ) => {
        let _ = $lexer.bump();
        return $lexer.token($token);
    };
}

macro_rules! bump_or_with_token {
    ( $lexer:expr, $token:expr ) => {
        match $lexer.bump() {
            Ok(_) => { },
            Err(_) => return $lexer.token($token),
        }
    };
}




pub struct Lexer<'a> {
    code: &'a [char],
    
    index: usize,
    token_start: usize,
    max_index: usize,
    
    line: usize,
    line_start: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a [char]) -> Self {
        let code_len = code.len();

        Lexer {
            code: code,
            index: 0,
            token_start: 0,
            max_index: if code_len == 0 { 0 } else { code_len - 1 },
            line: 1,
            line_start: 0,
            column: 0,
        }
    }

    #[inline]
    fn character(&self) -> char {
        self.code[self.index]
    }

    #[inline]
    fn bump(&mut self) -> Result<(), ()> {
        if self.index < self.max_index {
            self.index += 1;
            self.column += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn token(&mut self, kind: TokenKind) -> Token {
        let start = self.token_start;
        let end = self.index;
        let line = self.line;

        // let column = self.token_start - self.line_start;
        let column = self.column;

        Loc::new(start, end, line, column, kind, )
    }

    #[inline]
    fn read_hashbang(&mut self) -> Token {
        // #!
        self.token_start = self.index;

        if self.index != 0 {
            return self.token(TokenKind::UnexpectedToken);
        }

        bump_or_with_token!(self, TokenKind::UnexpectedToken);

        match self.character() {
            '!' => {
                loop {
                    bump_or_with_token!(self, TokenKind::HashBang);
                    
                    if is_line_terminator(self.character()) {
                        break;
                    }
                }
                return self.token(TokenKind::HashBang);
            },
            _ => {
                return self.token(TokenKind::UnexpectedToken);
            }
        }
    }

    #[inline]
    fn read_punctuator(&mut self) -> Token {
        // '{' | '}' | '(' | ')' | '[' | ']' | '.' | ';' | ':' | ',' | '?'
        // '/' | '!' | '|' | '&' | '^' | '<' | '>' | '~' | '%' | '=' | '+'
        // '-' | '*' | '?'
        use self::Punctuator::*;
        
        let c = self.character();
        match c {
            '?' => {
                bump_with_token!(self, TokenKind::Punctuator(Question));
            },
            ';' => {
                bump_with_token!(self, TokenKind::Punctuator(Semicolon));
            },
            '.' => {
                // . ...
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                match self.character() {
                    '.' => {
                        // ...
                        bump_or_with_token!(self, TokenKind::UnexpectedEof);

                        if self.character() != '.' {
                            return self.token(TokenKind::UnexpectedToken);
                        }

                        bump_or_with_token!(self, TokenKind::Punctuator(Spread));

                        return self.token(TokenKind::Punctuator(Spread));
                    },
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        // .1
                        match self.read_float() {
                            Ok(_) => {
                                let number = &self.code[self.token_start..self.index].iter().collect::<String>();

                                match f64::from_str(&number) {
                                    Ok(n) => return self.token(TokenKind::Literal(Literal::Numeric(n.into()))),
                                    Err(e) => {
                                        error!("parse number error: {:?}", e);
                                        return self.token(TokenKind::UnexpectedToken);
                                    }
                                }
                            },
                            Err(token) => return token,
                        }
                    },
                    _ => {
                        // .
                        return self.token(TokenKind::Punctuator(DotMark));
                    }
                }
            },
            ':' => {
                bump_with_token!(self, TokenKind::Punctuator(Colon));
            },
            ',' => {
                bump_with_token!(self, TokenKind::Punctuator(Comma));
            },
            '(' => {
                bump_with_token!(self, TokenKind::Punctuator(LParen));
            },
            ')' => {
                bump_with_token!(self, TokenKind::Punctuator(RParen));
            },
            '[' => {
                bump_with_token!(self, TokenKind::Punctuator(LBracket));
            },
            ']' => {
                bump_with_token!(self, TokenKind::Punctuator(RBracket));
            },
            '{' => {
                bump_with_token!(self, TokenKind::Punctuator(LBrace));
            },
            '}' => {
                bump_with_token!(self, TokenKind::Punctuator(RBrace));
            },
            '=' => {
                // =, ==, ===, =>, 
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                let c = self.character();
                match c {
                    '=' => {
                        bump_or_with_token!(self, TokenKind::UnexpectedEof);

                        let c = self.character();
                        if c == '=' {
                            // ===
                            bump_with_token!(self, TokenKind::Punctuator(StrictEq));
                        } else {
                            // ==
                            return self.token(TokenKind::Punctuator(Eq));
                        }
                    },
                    '>' => {
                        // =>
                        bump_with_token!(self, TokenKind::Punctuator(FatArrow));
                    },
                    _ => {
                        // =
                        return self.token(TokenKind::Punctuator(Assign));
                    }
                }
            },
            '+' => {
                // +, ++, +=
                bump_or_with_token!(self, TokenKind::UnexpectedEof);
                
                let c = self.character();
                match c {
                    '+' => {
                        // ++
                        bump_with_token!(self, TokenKind::Punctuator(Increment));
                    },
                    '=' => {
                        // +=
                        bump_with_token!(self, TokenKind::Punctuator(AddAssign));
                    },
                    _ => {
                        // +
                        return self.token(TokenKind::Punctuator(Add));
                    },
                }
            },
            '-' => {
                // -. --. -=
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                let c = self.character();
                match c {
                    '-' => {
                        // --
                        bump_with_token!(self, TokenKind::Punctuator(Decrement));
                    },
                    '=' => {
                        // -=
                        bump_with_token!(self, TokenKind::Punctuator(SubAssign));
                    },
                    _ => {
                        // -
                        return self.token(TokenKind::Punctuator(Sub));
                    },
                }
            },
            '*' => {
                // *, **, *=, **=
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                let c = self.character();
                match c {
                    '*' => {
                        // **
                        bump_or_with_token!(self, TokenKind::UnexpectedEof);

                        let c = self.character();
                        if c == '=' {
                            // **=
                            bump_with_token!(self, TokenKind::Punctuator(PowAssign));
                        } else {
                            return self.token(TokenKind::Punctuator(Pow));
                        }
                    },
                    '=' => {
                        // *=
                        bump_with_token!(self, TokenKind::Punctuator(MulAssign));
                    },
                    _ => {
                        // *
                        return self.token(TokenKind::Punctuator(Mul));
                    },
                }
            },
            '/' => {
                // /, /=, //, /*
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                let c = self.character();
                match c {
                    '/' => {
                        // Single Line Comment
                        // //
                        loop {
                            bump_or_with_token!(self, TokenKind::Comment);

                            if is_line_terminator(self.character()) {
                                break;
                            }
                        }

                        return self.token(TokenKind::Comment);
                    },
                    '*' => {
                        // /*
                        // Multi Line Comment
                        bump_or_with_token!(self, TokenKind::UnexpectedEof);

                        loop {
                            let c2 = self.character();
                            match c2 {
                                '*' => {
                                    bump_or_with_token!(self, TokenKind::UnexpectedEof);

                                    if self.character() == '/' {
                                        // */
                                        bump_with_token!(self, TokenKind::Comment);
                                    }
                                },
                                _ => {
                                    if is_line_terminator(c2) {
                                        let _token = self.read_line_terminator();
                                        if _token.item != TokenKind::LineTerminator {
                                            bump_or_with_token!(self, TokenKind::UnexpectedEof);
                                        }
                                    } else {
                                        bump_or_with_token!(self, TokenKind::UnexpectedEof);
                                    }
                                }
                            }
                        }
                    },
                    '=' => {
                        // /=
                        bump_with_token!(self, TokenKind::Punctuator(DivAssign));
                    },
                    _ => {
                        // /
                        return self.token(TokenKind::Punctuator(Div));
                    },
                }
            },
            '%' => {
                // %, %=
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                let c = self.character();
                match c {
                    '=' => {
                        // *=
                        bump_with_token!(self, TokenKind::Punctuator(RemAssign));
                    },
                    _ => {
                        // %
                        return self.token(TokenKind::Punctuator(Rem));
                    },
                }
            },
            '!' => {
                // !, !=, !==
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                let c = self.character();
                match c {
                    '=' => {
                        // !=, !==
                        bump_or_with_token!(self, TokenKind::UnexpectedEof);

                        let c = self.character();
                        if c == '=' {
                            // !==
                            bump_with_token!(self, TokenKind::Punctuator(StrictNeq));
                        } else {
                            // !=
                            return self.token(TokenKind::Punctuator(Neq));
                        }
                    },
                    _ => {
                        // !
                        return self.token(TokenKind::Punctuator(Not));
                    }
                }
            },
            '&' => {
                // &, &&, &=
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                let c = self.character();
                match c {
                    '&' => {
                        // &&
                        bump_with_token!(self, TokenKind::Punctuator(And));
                    },
                    '=' => {
                        // &=
                        bump_with_token!(self, TokenKind::Punctuator(BitAndAssign));
                    },
                    _ => {
                        // &
                        return self.token(TokenKind::Punctuator(BitAnd));
                    }
                }
            },
            '~' => {
                // ~
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                return self.token(TokenKind::Punctuator(BitNot));
            },
            '|' => {
                // |, |=, ||
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                let c = self.character();
                match c {
                    '=' => {
                        // |=
                        bump_with_token!(self, TokenKind::Punctuator(BitOrAssign));
                    },
                    '|' => {
                        // ||
                        bump_with_token!(self, TokenKind::Punctuator(Or));
                    },
                    _ => {
                        // |
                        return self.token(TokenKind::Punctuator(BitOr));
                    }
                }
            },
            '^' => {
                // ^, ^=
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                let c = self.character();
                match c {
                    '=' => {
                        // ^=
                        bump_with_token!(self, TokenKind::Punctuator(BitXorAssign));
                    },
                    _ => {
                        // ^
                        return self.token(TokenKind::Punctuator(BitXor));
                    }
                }
            },
            '<' => {
                // <, <<, <=, <<=
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                let c = self.character();
                match c {
                    '<' => {
                        // <<
                        bump_or_with_token!(self, TokenKind::UnexpectedEof);

                        let c = self.character();
                        if c == '=' {
                            // <<=
                            return self.token(TokenKind::Punctuator(BitShlAssign));
                        } else {
                            return self.token(TokenKind::Punctuator(BitShl));
                        }
                    },
                    '=' => {
                        // <=
                        bump_with_token!(self, TokenKind::Punctuator(LtEq));
                    },
                    _ => {
                        // <
                        return self.token(TokenKind::Punctuator(Lt)) ;
                    }
                }
            },
            '>' => {
                // >, >>, >=, >>=, >>>, >>>=
                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                let c = self.character();
                match c {
                    '>' => {
                        bump_or_with_token!(self, TokenKind::UnexpectedEof);

                        let c = self.character();
                        match c {
                            '>' => {
                                bump_or_with_token!(self, TokenKind::UnexpectedEof);

                                let c = self.character();
                                if c == '=' {
                                    // >>>=
                                    bump_with_token!(self, TokenKind::Punctuator(BitUShrAssign));
                                } else {
                                    // >>>
                                    bump_with_token!(self, TokenKind::Punctuator(BitUShr));
                                }
                            },
                            '=' => {
                                // >>=
                                bump_with_token!(self, TokenKind::Punctuator(BitShrAssign));
                            },
                            _ => {
                                // >>
                                bump_with_token!(self, TokenKind::Punctuator(BitShr));
                            }
                        }
                    },
                    '=' => {
                        // >=
                        bump_with_token!(self, TokenKind::Punctuator(GtEq));
                    },
                    _ => {
                        // >
                        return self.token(TokenKind::Punctuator(Gt));
                    },
                }
            },
            _ => {
                unreachable!();
            }
        }
    }

    #[inline]
    fn read_line_terminator(&mut self) -> Token {
        // \r
        // \n
        // \r\n
        // \n\r
        // <LS>
        // <PS>
        self.token_start = self.index;

        self.line += 1;
        self.column = 0;

        // self.line_start = self.index;

        let c = self.character();
        match c {
            CR => {
                // \r
                bump_or_with_token!(self, TokenKind::LineTerminator);

                if self.character() == LF {
                    // \r\n
                    bump_or_with_token!(self, TokenKind::LineTerminator);
                }
            },
            LF => {
                // \n
                bump_or_with_token!(self, TokenKind::LineTerminator);

                if self.character() == CR {
                    // \n\r
                    bump_or_with_token!(self, TokenKind::LineTerminator);
                }
            },
            LS | PS => {
                bump_or_with_token!(self, TokenKind::LineTerminator);
            },
            _ => unreachable!(),
        }

        return self.token(TokenKind::LineTerminator);
    }

    #[inline]
    fn read_binary_int(&mut self) -> Result<(u128, usize), Token> {
        // BinaryInteger
        let mut number = String::new();
        loop {
            let c = self.character();
            if !is_binary_digit(c) {
                break;
            }

            number.push(c);

            if let Err(_) = self.bump() {
                break;
            }
        }

        if number.len() == 0 {
            return Err(self.token(TokenKind::UnexpectedToken));
        }

        u128::from_str_radix(&number, 2).map_err(|_| {
            warn!("BinaryInteger is too big!");
            self.token(TokenKind::UnexpectedToken)
        }).map(|n| (n, number.len()))
    }

    #[inline]
    fn read_octal_int(&mut self) -> Result<(u128, usize), Token> {
        // OctalInteger
        let mut number = String::new();
        loop {
            let c = self.character();
            if !is_octal_digit(c) {
                break;
            }

            number.push(c);

            if let Err(_) = self.bump() {
                break;
            }
        }

        if number.len() == 0 {
            return Err(self.token(TokenKind::UnexpectedToken));
        }

        u128::from_str_radix(&number, 8).map_err(|_| {
            warn!("OctalInteger is too big!");
            self.token(TokenKind::UnexpectedToken)
        }).map(|n| (n, number.len()))
    }

    #[inline]
    fn read_hex_int(&mut self, bytes: Option<usize>) -> Result<(u128, usize), Token> {
        // HexInteger
        let mut hex_number = String::new();
        
        loop {
            match bytes {
                Some(len) => {
                    if hex_number.len() == len {
                        break;
                    }
                },
                None => { }
            }
            let c = self.character();
            if !is_hex_digit(c) {
                break;
            }

            hex_number.push(c);

            if let Err(_) = self.bump() {
                break;
            }
        }

        if hex_number.len() == 0 {
            return Err(self.token(TokenKind::UnexpectedToken));
        }

        u128::from_str_radix(&hex_number, 16).map_err(|_| {
            warn!("HexNumber is too big!");
            self.token(TokenKind::UnexpectedToken)
        }).map(|n| (n, hex_number.len()))
    }

    #[inline]
    fn read_float(&mut self) -> Result<(), Token> {
        match self.bump() {
            Ok(_) => { },
            Err(_) => return Ok(()),
        }

        loop {
            let c = self.character();
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    match self.bump() {
                        Ok(_) => { },
                        Err(_) => return Ok(()),
                    }
                },
                'e' | 'E' => {
                    return self.read_scientific();
                },
                _ => {
                    break;
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn read_scientific(&mut self) -> Result<(), Token> {
        match self.bump() {
            Ok(_) => { },
            Err(_) => return Err(self.token(TokenKind::UnexpectedToken)),
        }

        let c = self.character();
        match c {
            '-' | '+' => {
                match self.bump() {
                    Ok(_) => { },
                    Err(_) => return Err(self.token(TokenKind::UnexpectedToken)),
                }
            },
            _ => { }
        }

        let c = self.character();
        if !c.is_ascii_digit() {
            return Err(self.token(TokenKind::UnexpectedToken));
        }

        loop {
            match self.bump() {
                Ok(_) => { },
                Err(_) => return Err(self.token(TokenKind::UnexpectedToken)),
            }

            let c = self.character();
            if !c.is_ascii_digit() {
                return Ok(())
            }
        }
    }

    #[inline]
    fn read_numeric_literal(&mut self) -> Token {
        // Numeric Literal
        // https://www.ecma-international.org/ecma-262/#sec-literals-numeric-literals
        self.token_start = self.index;

        let c = self.character();
        match c {
            '0' => {
                // 0 | 0b, 0o, 0x, 
                match self.bump() {
                    Ok(_) => {},
                    Err(_) => {
                        return self.token(TokenKind::Literal(Literal::Numeric(0u32.into())));
                    }
                }

                let c = self.character();
                match c {
                    'b' | 'B' => {
                        bump_or_with_token!(self, TokenKind::UnexpectedEof);
                        match self.read_binary_int() {
                            Ok((n, _)) => {
                                let n= n as f64;
                                return self.token(TokenKind::Literal(Literal::Numeric(n.into())));
                            },
                            Err(token) => return token,
                        }
                    },
                    'o' | 'O' => {
                        bump_or_with_token!(self, TokenKind::UnexpectedEof);
                        match self.read_octal_int() {
                            Ok((n, _)) => {
                                let n = n as f64;
                                return self.token(TokenKind::Literal(Literal::Numeric(n.into())))
                            },
                            Err(token) => return token,
                        }
                    },
                    'x' | 'X' => {
                        bump_or_with_token!(self, TokenKind::UnexpectedEof);
                        match self.read_hex_int(None) {
                            Ok((n, _)) => {
                                let n = n as f64;
                                return self.token(TokenKind::Literal(Literal::Numeric(n.into())))
                            },
                            Err(token) => return token,
                        }
                    },
                    '.'       => {
                        match self.read_float() {
                            Ok(_) => {
                                let number = &self.code[self.token_start..self.index].iter().collect::<String>();

                                match f64::from_str(&number) {
                                    Ok(n) => return self.token(TokenKind::Literal(Literal::Numeric(n.into()))),
                                    Err(e) => {
                                        error!("parse number error: {:?}", e);
                                        return self.token(TokenKind::UnexpectedToken);
                                    }
                                }
                            },
                            Err(token) => return token,
                        }
                    },
                    'e' | 'E' => {
                        match self.read_scientific() {
                            Ok(_) => {
                                let number = &self.code[self.token_start..self.index].iter().collect::<String>();

                                match f64::from_str(&number) {
                                    Ok(n) => return self.token(TokenKind::Literal(Literal::Numeric(n.into()))),
                                    Err(e) => {
                                        error!("parse number error: {:?}", e);
                                        return self.token(TokenKind::UnexpectedToken);
                                    }
                                }
                            },
                            Err(token) => return token,
                        }
                    },
                    _ => {
                        if c.is_ascii_digit() {
                            // WARN: Not recommand this style. (e.g: 0123)
                            warn!("please don't add zero on numeric's head.");
                            return self.token(TokenKind::UnexpectedToken);
                        } else {
                            // zero
                            return self.token(TokenKind::Literal(Literal::Numeric(0u32.into())));
                        }
                    }
                }
            },
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                loop {
                    match self.bump() {
                        Ok(_) => {},
                        Err(_) => {
                            let n = f64::from_str(&c.to_string()).unwrap();
                            return self.token(TokenKind::Literal(Literal::Numeric(n.into())));
                        }
                    }

                    let c = self.character();
                    match c {
                        '.'       => {
                            match self.read_float() {
                                Ok(_) => {
                                    let number = &self.code[self.token_start..self.index].iter().collect::<String>();

                                    match f64::from_str(&number) {
                                        Ok(n) => return self.token(TokenKind::Literal(Literal::Numeric(n.into()))),
                                        Err(e) => {
                                            error!("parse number error: {:?}", e);
                                            return self.token(TokenKind::UnexpectedToken);
                                        }
                                    }
                                },
                                Err(token) => return token,
                            }
                        },
                        'e' | 'E' => {
                            match self.read_scientific() {
                                Ok(_) => {
                                    let number = &self.code[self.token_start..self.index].iter().collect::<String>();

                                    match f64::from_str(&number) {
                                        Ok(n) => return self.token(TokenKind::Literal(Literal::Numeric(n.into()))),
                                        Err(e) => {
                                            error!("parse number error: {:?}", e);
                                            return self.token(TokenKind::UnexpectedToken);
                                        }
                                    }
                                },
                                Err(token) => return token,
                            }
                        },
                        _ => {
                            if !c.is_ascii_digit() {
                                let number = &self.code[self.token_start..self.index].iter().collect::<String>();

                                match f64::from_str(&number) {
                                    Ok(n) => return self.token(TokenKind::Literal(Literal::Numeric(n.into()))),
                                    Err(e) => {
                                        error!("parse number error: {:?}", e);
                                        return self.token(TokenKind::UnexpectedToken);
                                    }
                                }
                            }
                        }
                    }
                }
            },
            _ => {
                unreachable!();
            }
        }
    }

    #[inline]
    fn read_unciode_char(&mut self) -> Result<char, Token> {
        let t = self.character();

        self.bump().map_err(|_| self.token(TokenKind::UnexpectedEof) )?;
        
        let c = self.character();
        match c {
            '{' => {
                self.bump().map_err(|_| self.token(TokenKind::UnexpectedEof) )?;

                match self.read_hex_int(None) {
                    Ok((n, size)) => {
                        if size == 0 {
                            return Err(self.token(TokenKind::UnexpectedToken));
                        }
                        
                        if self.character() != '}' {
                            return Err(self.token(TokenKind::UnexpectedToken));
                        }
                        
                        let _ = self.bump();

                        char::try_from(n as u32).map_err(|_| {
                            warn!("not an escape character.");
                            self.token(TokenKind::UnexpectedToken)
                        })
                    },
                    Err(token) => Err(token),
                }
            },
            _ => {
                match self.read_hex_int(Some(4)) {
                    Ok((n, size)) => {
                        if t == 'u' {
                            if size != 4 {
                                return Err(self.token(TokenKind::UnexpectedToken));
                            }
                        } else if t == 'x' {
                            if size == 0 || size > 2 {
                                return Err(self.token(TokenKind::UnexpectedToken));
                            }
                        }

                        char::try_from(n as u32).map_err(|_| {
                            warn!("not an escape character.");
                            self.token(TokenKind::UnexpectedToken)
                        })
                    },
                    Err(token) => Err(token),
                }
            }
        }
    }

    #[inline]
    fn read_string_literal(&mut self) -> Token {
        // String Literal
        // https://www.ecma-international.org/ecma-262/#sec-literals-string-literals
        self.token_start = self.index;

        let c = self.character();
        assert_eq!(c == '"' || c == '\'', true);

        bump_or_with_token!(self, TokenKind::UnexpectedEof);
        
        let mut string_literal: String = String::new();
        loop {
            let c2 = self.character();

            match c2 {
                '\\' => {
                    bump_or_with_token!(self, TokenKind::UnexpectedEof);

                    let c3 = self.character();
                    match c3 {
                        '0' => {
                            // \0
                            bump_or_with_token!(self, TokenKind::UnexpectedEof);
                            if self.character().is_ascii_digit() {
                                return self.token(TokenKind::UnexpectedToken);
                            }
                            string_literal.push('\0');
                        },
                        'u' | 'x' => {
                            // \u00001, \u{01}, \x1, \x1111
                            match self.read_unciode_char() {
                                Ok(c) => {
                                    string_literal.push(c);
                                },
                                Err(token) => return token,
                            }
                        },
                        _ => {
                            if is_escape_character(c3) {
                                string_literal.push(escape_character_to_char(c3));
                                bump_or_with_token!(self, TokenKind::UnexpectedEof);
                            } 
                            // else if is_line_terminator(c3) {
                            //     // FIXME: 需要更新行数统计？
                            //     string_literal.push(c3);
                            //     bump_or_with_token!(self, TokenKind::UnexpectedEof);
                            // } 
                            else {
                                return self.token(TokenKind::UnexpectedToken);
                            }
                        }
                    }
                },
                _ => {
                    if is_line_terminator(c2) {
                        return self.token(TokenKind::UnexpectedToken);
                    }

                    if c2 == c {
                        // end
                        let token = TokenKind::Literal(Literal::String(string_literal.into()));
                        bump_with_token!(self, token);
                    }

                    string_literal.push(c2);
                    bump_or_with_token!(self, TokenKind::UnexpectedEof);
                }
            }
        }
    }

    #[inline]
    fn read_identifier(&mut self, mut name: String) -> Token {
        // identifier keyword
        // Literals: undefined, null, true, false, NaN, Infinity
        self.token_start = self.index;

        match self.bump() {
            Ok(_) => { },
            Err(_) => {
                return self.token(TokenKind::Identifier(name.into()));
            }
        }

        loop {
            let mut c = self.character();
            if c == '\\' {
                bump_or_with_token!(self, TokenKind::UnexpectedToken);

                if self.character() != 'u' {
                    return self.token(TokenKind::UnexpectedToken);
                }
                
                match self.read_unciode_char() {
                    Ok(_c)  => {
                        c = _c;
                        self.index -= 1;
                    },
                    Err(token) => {
                        return token;
                    }
                }
            }

            if !is_identifier_part(c) {
                break;
            }
            
            name.push(c);

            if let Err(_) = self.bump() {
                break;
            }
        }

        if let Ok(literal) = Literal::from_str(&name) {
            // undefined, null, true, false, NaN, Infinity
            return self.token(TokenKind::Literal(literal));
        }

        if let Ok(kw) = Keyword::from_str(&name) {
            // Keyword
            return self.token(TokenKind::Keyword(kw));
        } else {
            // name
            return self.token(TokenKind::Identifier(name.into()));
        }
    }

    #[inline]
    fn read_regular_literal(&mut self) -> Token {
        // NOTE: 考虑到复杂性，这会在语法层处理。
        // Regular Expression Literals
        // https://www.ecma-international.org/ecma-262/#sec-literals-regular-expression-literals
        self.token_start = self.index;

        unimplemented!()
    }

    #[inline]
    fn read_template_literal(&mut self) -> Token {
        // Template Literal
        // https://www.ecma-international.org/ecma-262/#sec-template-literal-lexical-components
        self.token_start = self.index;

        let c = self.character();

        bump_or_with_token!(self, TokenKind::UnexpectedEof);
        
        let mut template_literal = TemplateLiteral { items: Vec::new() };

        let mut string_literal: String = String::new();
        loop {
            let c2 = self.character();

            match c2 {
                '\\' => {
                    bump_or_with_token!(self, TokenKind::UnexpectedEof);

                    let c3 = self.character();
                    match c3 {
                        '0' => {
                            // \0
                            bump_or_with_token!(self, TokenKind::UnexpectedEof);
                            if self.character().is_ascii_digit() {
                                return self.token(TokenKind::UnexpectedToken);
                            }
                            string_literal.push('\0');
                        },
                        'u' | 'x' => {
                            // \u00001, \u{01}, \x1, \x1111
                            match self.read_unciode_char() {
                                Ok(c) => {
                                    string_literal.push(c);
                                },
                                Err(token) => return token,
                            }
                        },
                        _ => {
                            if is_escape_character(c3) {
                                string_literal.push(escape_character_to_char(c3));
                                bump_or_with_token!(self, TokenKind::UnexpectedEof);
                            } else {
                                return self.token(TokenKind::UnexpectedToken);
                            }
                        }
                    }
                },
                '$' => {
                    bump_or_with_token!(self, TokenKind::UnexpectedEof);
                    if self.character() != '{' {
                        string_literal.push('$');
                        continue;
                    }

                    if string_literal.len() > 0 {
                        template_literal.items.push(TemplateItem::String(string_literal.clone().into()));
                        string_literal.clear();
                    }

                    bump_or_with_token!(self, TokenKind::UnexpectedEof);

                    let mut exp: Vec<Token> = vec![];
                    loop {
                        let token = self.consume();

                        match token.item {
                            TokenKind::UnexpectedToken | TokenKind::UnexpectedEof => return token,
                            TokenKind::EndOfProgram => return self.token(TokenKind::UnexpectedToken),
                            TokenKind::Punctuator(Punctuator::RBrace) => {
                                break;
                            },
                            _ => {
                                exp.push(token);
                            }
                        }
                    }
                    
                    template_literal.items.push(TemplateItem::Exp(exp));
                },
                _ => {
                    if is_line_terminator(c2) {
                        self.line += 1;
                        self.column = 0;
                    }

                    if c2 == '`' {
                        // end
                        if string_literal.len() > 0 {
                            template_literal.items.push(TemplateItem::String(string_literal.into()));
                        }

                        let token = TokenKind::TemplateLiteral(template_literal);
                        bump_with_token!(self, token);
                    }

                    string_literal.push(c2);
                    bump_or_with_token!(self, TokenKind::UnexpectedEof);
                }
            }
        }
    }

    #[inline]
    pub fn consume(&mut self) -> Token {
        loop {
            if self.index >= self.max_index {
                self.token_start = self.index;
                return self.token(TokenKind::EndOfProgram);
            }

            let c = self.character();
            
            if is_whitespace(c) {
                bump_or_with_token!(self, TokenKind::EndOfProgram);
                continue;
            }

            match c {
                CR | LF | LS | PS => {
                    return self.read_line_terminator()
                },
                '#' => {
                    return self.read_hashbang()
                },
                '"' | '\'' => {
                    return self.read_string_literal()
                },
                '`' => {
                    return self.read_template_literal()
                },
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    return self.read_numeric_literal()
                },
                '\\' => {
                    self.token_start = self.index;
                    bump_or_with_token!(self, TokenKind::UnexpectedToken);

                    let c = self.character();
                    if c == 'u' {
                        // Unicode Escape Sequence
                        // https://www.ecma-international.org/ecma-262/#prod-UnicodeEscapeSequence
                        match self.read_unciode_char() {
                            Ok(c) => {
                                if is_identifier_start(c) {
                                    let mut name = String::new();
                                    name.push(c);
                                    self.index -= 1;
                                    return self.read_identifier(name);
                                }
                            },
                            Err(token) => return token,
                        }
                    } else {
                        return self.token(TokenKind::UnexpectedToken);
                    }
                },
                _ => {
                    self.token_start = self.index;

                    if is_punctuator(c) {
                        return self.read_punctuator();
                    } else if is_identifier_start(c) {
                        let mut name = String::new();
                        name.push(c);
                        return self.read_identifier(name);
                    } else {
                        return self.token(TokenKind::UnexpectedToken);
                    }
                }
            }
        }
    }
}

#[bench]
fn bench_tokenization(b: &mut test::Bencher) {
    let source = include_str!("../../data/react-16.8.3.development.js");
    let mut code = source.chars().collect::<Vec<char>>();
    code.push('\0'); // EOF

    b.bytes = source.len() as _;

    b.iter(|| {
        let mut lexer = Lexer::new(&code);
        loop {
            let token = lexer.consume();
            let kind = token.item;

            if kind == TokenKind::UnexpectedToken {
                break;
            }
            if kind == TokenKind::UnexpectedEof {
                break;
            }
            if kind == TokenKind::EndOfProgram {
                break;
            }
        }
    })
}


pub fn tokenize(source: &str) {
    let mut code = source.chars().collect::<Vec<char>>();
    code.push('\0'); // EOF

    let mut lexer = Lexer::new(&code);
    
    loop {
        let token = lexer.consume();
        let start = token.start;
        let end = token.end;
        let line = token.line;
        let column = token.column;

        match token.item {
            TokenKind::UnexpectedEof => {
                error!("UnexpectedEof");
            },
            TokenKind::UnexpectedToken => {
                let end = std::cmp::min(code.len(), start + 20);
                error!("Token: {:?} Text: #{}#", &token, 
                            &code[start..end].iter().collect::<String>());
            },
            TokenKind::LineTerminator => {
                debug!("Token: {:?}", token);
            },
            TokenKind::EndOfProgram => {
                debug!("EOF.");
            },
            _ => {
                debug!("Token: {:?} Text: #{}#", &token, 
                            &code[start..end].iter().collect::<String>());
            },
        }

        if token.item == TokenKind::UnexpectedToken {
            break;
        }
        if token.item == TokenKind::UnexpectedEof {
            break;
        }
        if token.item == TokenKind::EndOfProgram {
            break;
        }
    }
}