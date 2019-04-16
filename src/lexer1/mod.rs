use crate::unicode_xid::UnicodeXID;

use ast::numberic::{ Float,  };

use std::fmt;
use std::cmp;
use std::str::FromStr;
use std::convert::TryFrom;


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
pub fn is_identifier_start(c: char) -> bool {
    // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-IdentifierStart
    match c {
        '_' | '$' => true,
        _ => UnicodeXID::is_xid_start(c),
    }
}

#[inline]
pub fn is_identifier_part(c: char) -> bool {
    // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-IdentifierPart
    const ZWNJ  : char = '\u{200C}';  // identifier part
    const ZWJ   : char = '\u{200D}';  // identifier part
    match c {
        '$' | ZWNJ | ZWJ => true,
        _ => UnicodeXID::is_xid_continue(c),
    }
}


#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LineColumn {
    pub line_offset: usize,
    pub line: usize,
    pub column: usize,
}

impl fmt::Debug for LineColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl fmt::Display for LineColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}



#[derive(Debug, Eq, Clone)]
pub struct Span<T: fmt::Debug + PartialEq + Clone> {
    pub start: LineColumn,
    pub end: LineColumn,
    pub item: T,
}

impl<T: fmt::Debug + PartialEq + Clone> PartialEq for Span<T> {
    fn eq(&self, other: &Span<T>) -> bool {
        self.item == other.item
    }
}

impl Default for LineColumn {
    fn default() -> Self {
        LineColumn { line_offset: 0, line: 0, column: 0 }
    }
}





#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    HashBang,
    
    SingleLineComment(Vec<char>),
    MultiLineComment(Vec<char>),
    
    WhiteSpaces,
    LineTerminator(LineTerminator),

    Keyword(Keyword),
    Identifier(Vec<char>),
    
    Punctuator(Punctuator),

    LiteralUndefined,
    LiteralNull,
    LiteralString(Vec<char>),
    LiteralBoolean(bool),
    LiteralHexNumeric(u64),
    LiteralBinaryNumeric(u64),
    LiteralOctalNumeric(u64),
    LiteralDecimalNumeric(u64),
    LiteralFloatNumeric(Float),
}

pub type SpannedToken = Span<Token>;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WhiteSpace {
    TAB,
    VT,
    FF,
    SP,
    NBSP,
    ZWNBSP,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LineTerminator {
    CarriageReturn,     // CR   : \r
    LineFeed,           // LF   : \n
    EndOfLine,          // CR+LF: \r\n
    LineSeparator,      // LS   : U+2028
    ParagraphSeparator, // PS   : U+2029
    // NextLine,           // NEL  : U+0085
}

// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-keywords
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Export,
    Extends,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Return,
    Super,
    Switch,
    This,
    Throw,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Yield,

    Let,
    Static,

    // FutureReservedWord
    // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-future-reserved-words
    Enum,
    // Use of the following tokens within strict mode code is also reserved
    Implements,
    Package,
    Protected,
    Interface,
    Private,
    Public,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Keyword::*;

        match s {
            "await" => Ok(Await),
            "break" => Ok(Break),
            "case" => Ok(Case),
            "catch" => Ok(Catch),
            "class" => Ok(Class),
            "const" => Ok(Const),
            "continue" => Ok(Continue),
            "debugger" => Ok(Debugger),
            "default" => Ok(Default),
            "delete" => Ok(Delete),
            "do" => Ok(Do),
            "else" => Ok(Else),
            "export" => Ok(Export),
            "extends" => Ok(Extends),
            "finally" => Ok(Finally),
            "for" => Ok(For),
            "function" => Ok(Function),
            "if" => Ok(If),
            "import" => Ok(Import),
            "in" => Ok(In),
            "instanceof" => Ok(Instanceof),
            "new" => Ok(New),
            "return" => Ok(Return),
            "super" => Ok(Super),
            "switch" => Ok(Switch),
            "this" => Ok(This),
            "throw" => Ok(Throw),
            "try" => Ok(Try),
            "typeof" => Ok(Typeof),
            "var" => Ok(Var),
            "void" => Ok(Void),
            "while" => Ok(While),
            "with" => Ok(With),
            "yield" => Ok(Yield),
            "let" => Ok(Let),
            "static" => Ok(Static),
            "enum" => Ok(Enum),
            "implements" => Ok(Implements),
            "package" => Ok(Package),
            "protected" => Ok(Protected),
            "interface" => Ok(Interface),
            "private" => Ok(Private),
            "public" => Ok(Public),
            _ => Err(())
        }
    }
}

impl TryFrom<&[char]> for Keyword {
    type Error = ();
    
    fn try_from(value: &[char]) -> Result<Self, Self::Error> {
        use self::Keyword::*;

        match value {
            &['a', 'w', 'a', 'i', 't'] => Ok(Await),
            &['b', 'r', 'e', 'a', 'k'] => Ok(Break),
            &['c', 'a', 's', 'e'] => Ok(Case),
            &['c', 'a', 't', 'c', 'h'] => Ok(Catch),
            &['c', 'l', 'a', 's', 's'] => Ok(Class),
            &['c', 'o', 'n', 's', 't'] => Ok(Const),
            &['c', 'o', 'n', 't', 'i', 'n', 'u', 'e'] => Ok(Continue),
            &['d', 'e', 'b', 'u', 'g', 'g', 'e', 'r'] => Ok(Debugger),
            &['d', 'e', 'f', 'a', 'u', 'l', 't'] => Ok(Default),
            &['d', 'e', 'l', 'e', 't', 'e'] => Ok(Delete),
            &['d', 'o'] => Ok(Do),
            &['e', 'l', 's', 'e'] => Ok(Else),
            &['e', 'x', 'p', 'o', 'r', 't'] => Ok(Export),
            &['e', 'x', 't', 'e', 'n', 'd', 's'] => Ok(Extends),
            &['f', 'i', 'n', 'a', 'l', 'l', 'y'] => Ok(Finally),
            &['f', 'o', 'r'] => Ok(For),
            &['f', 'u', 'n', 'c', 't', 'i', 'o', 'n'] => Ok(Function),
            &['i', 'f'] => Ok(If),
            &['i', 'm', 'p', 'o', 'r', 't'] => Ok(Import),
            &['i', 'n'] => Ok(In),
            &['i', 'n', 's', 't', 'a', 'n', 'c', 'e', 'o', 'f'] => Ok(Instanceof),
            &['n', 'e', 'w'] => Ok(New),
            &['r', 'e', 't', 'u', 'r', 'n'] => Ok(Return),
            &['s', 'u', 'p', 'e', 'r'] => Ok(Super),
            &['s', 'w', 'i', 't', 'c', 'h'] => Ok(Switch),
            &['t', 'h', 'i', 's'] => Ok(This),
            &['t', 'h', 'r', 'o', 'w'] => Ok(Throw),
            &['t', 'r', 'y'] => Ok(Try),
            &['t', 'y', 'p', 'e', 'o', 'f'] => Ok(Typeof),
            &['v', 'a', 'r'] => Ok(Var),
            &['v', 'o', 'i', 'd'] => Ok(Void),
            &['w', 'h', 'i', 'l', 'e'] => Ok(While),
            &['w', 'i', 't', 'h'] => Ok(With),
            &['y', 'i', 'e', 'l', 'd'] => Ok(Yield),

            &['l', 'e', 't'] => Ok(Let),
            &['s', 't', 'a', 't', 'i', 'c'] => Ok(Static),

            &['e', 'n', 'u', 'm'] => Ok(Enum),
            &['i', 'm', 'p', 'l', 'e', 'm', 'e', 'n', 't', 's'] => Ok(Implements),
            &['p', 'a', 'c', 'k', 'a', 'g', 'e'] => Ok(Package),
            &['p', 'r', 'o', 't', 'e', 'c', 't', 'e', 'd'] => Ok(Protected),
            &['i', 'n', 't', 'e', 'r', 'f', 'a', 'c', 'e'] => Ok(Interface),
            &['p', 'r', 'i', 'v', 'a', 't', 'e'] => Ok(Private),
            &['p', 'u', 'b', 'l', 'i', 'c'] => Ok(Public),
            _ => Err(()),
        }
    }
}


// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-punctuators
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Punctuator {
    BackTick,    // `
    Question,    // ?
    DotMark,     // .
    Semicolon,   // ;
    Colon,       // :
    Comma,       // ,
    Spread,      // ...

    LParen,      // (
    RParen,      // )
    LBracket,    // [
    RBracket,    // ]
    LBrace,      // {
    RBrace,      // }

    FatArrow,    // =>

    Increment,   // ++
    Decrement,   // --

    Assign,      // =

    Not,         // !
    And,         // &&
    Or,          // ||

    Add,         // +
    Sub,         // -
    Mul,         // *
    Div,         // /
    Rem,         // %
    Pow,         // **

    BitNot,         // ~
    BitAnd,         // &
    BitOr,          // |
    BitXor,         // ^
    
    BitShl,         // <<
    BitShr,         // >>
    BitUShr,        // >>>

    AddAssign,      // +=
    SubAssign,      // -=
    MulAssign,      // *=
    DivAssign,      // /=
    RemAssign,      // %=
    PowAssign,      // **=

    BitAndAssign,   // &=
    BitOrAssign,    // |=
    BitXorAssign,   // ^=
    BitShlAssign,   // <<=
    BitShrAssign,   // >>=
    BitUShrAssign,  // >>>=

    Eq,          // ==
    StrictEq,    // ===
    Gt,          // >
    Lt,          // <
    Neq,         // !=
    StrictNeq,   // !==
    GtEq,        // >=
    LtEq,        // <=
}

impl FromStr for Punctuator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Punctuator::*;

        match s {
            "`" => Ok(BackTick),
            "?" => Ok(Question),
            "." => Ok(DotMark),
            ";" => Ok(Semicolon),
            ":" => Ok(Colon),
            "," => Ok(Comma),
            "..." => Ok(Spread),
            "(" => Ok(LParen),
            ")" => Ok(RParen),
            "[" => Ok(LBracket),
            "]" => Ok(RBracket),
            "{" => Ok(LBrace),
            "}" => Ok(RBrace),
            "=>" => Ok(FatArrow),
            "++" => Ok(Increment),
            "--" => Ok(Decrement),
            "=" => Ok(Assign),
            "!" => Ok(Not),
            "&&" => Ok(And),
            "||" => Ok(Or),
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            "%" => Ok(Rem),
            "**" => Ok(Pow),
            "~" => Ok(BitNot),
            "&" => Ok(BitAnd),
            "|" => Ok(BitOr),
            "^" => Ok(BitXor),
            "<<" => Ok(BitShl),
            ">>" => Ok(BitShr),
            ">>>" => Ok(BitUShr),
            "+=" => Ok(AddAssign),
            "-=" => Ok(SubAssign),
            "*=" => Ok(MulAssign),
            "/=" => Ok(DivAssign),
            "%=" => Ok(RemAssign),
            "**=" => Ok(PowAssign),
            "&=" => Ok(BitAndAssign),
            "|=" => Ok(BitOrAssign),
            "^=" => Ok(BitXorAssign),
            "<<=" => Ok(BitShlAssign),
            ">>=" => Ok(BitShrAssign),
            ">>>=" => Ok(BitUShrAssign),
            "==" => Ok(Eq),
            "===" => Ok(StrictEq),
            ">" => Ok(Gt),
            "<" => Ok(Lt),
            "!=" => Ok(Neq),
            "!==" => Ok(StrictNeq),
            ">=" => Ok(GtEq),
            "<=" => Ok(LtEq),
            _ => Err(())
        }
    }
}



// // NotEscapeSequence
// pub enum Character {
//     EOF,
//     Char(char),           // 
//     Zero,                 // \0
//     Hex(u8),              // \x HexDigit HexDigit
//     UnicodeHex(u32),      // \u HexDigit HexDigit HexDigit HexDigit
//     UnicodePoint(u32),    // \u{ CodePoint }
//     SingleEscapeCharacter(char), // ' " \ b f n r t v
// }


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LexError {
    EndOfProgram,
    UnexpectedToken,
    UnexpectedEof,
    UnclosedToken,
}

const ZWNBSP: char = '\u{FEFF}';  // whitespace

// White Space Code Points
// https://www.ecma-international.org/ecma-262/#sec-white-space
const TAB: char  = '\u{0009}';
const VT: char   = '\u{000B}';
const FF: char   = '\u{000C}';
const SP: char   = '\u{0020}';
const NBSP: char = '\u{00A0}';

const WHITE_SPACES: &[char] = &[ TAB, VT, FF, SP, NBSP, ZWNBSP, ];


const CR_LF: &str = "\u{000D}\u{000A}";   // \r\n
const CR: char = '\u{000D}';              // \r
const LF: char = '\u{000A}';              // \n
const LS: char = '\u{2028}';
const PS: char = '\u{2029}';

const LINE_TERMINATORS: &[char] = &[ CR, LF, LS, PS ];
const DECIMAL_DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const KEYWORDS: &[&str] = &[
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "export",
    "extends",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "new",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
    "let",
    "static",
    "enum",
    "implements",
    "package",
    "protected",
    "interface",
    "private",
    "public",
];

const PUNCTUATORS: &[&str] = &[
    "`",
    "?",
    ".",
    ";",
    ":",
    ",",
    "...",
    "(",
    ")",
    "[",
    "]",
    "{",
    "}",
    "=>",
    "++",
    "--",
    "=",
    "!",
    "&&",
    "||",
    "+",
    "-",
    "*",
    "/",
    "%",
    "**",
    "~",
    "&",
    "|",
    "^",
    "<<",
    ">>",
    ">>>",
    "+=",
    "-=",
    "*=",
    "/=",
    "%=",
    "**=",
    "&=",
    "|=",
    "^=",
    "<<=",
    ">>=",
    ">>>=",
    "==",
    "===",
    ">",
    "<",
    "!=",
    "!==",
    ">=",
    "<=",
];

pub struct Lexer<'a> {
    source: &'a str,
    line_start: usize,
    line: usize,
    column: usize,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source,
            line_start: 0,
            line: 0,
            column: 0,
            offset: 0,
        }
    }

    #[inline]
    pub fn advance(&mut self, amt: usize) {
        if amt == 0 {
            return ();
        }

        self.source = &self.source[amt..];
        self.offset += amt;
        self.column += amt;
    }

    #[inline]
    pub fn bump(&mut self, amt: usize) {
        self.source = &self.source[amt..];
        self.offset += amt;

        self.line_start = self.offset;
        self.line += 1;
        self.column = 0;
    }

    #[inline]
    pub fn eof(&self) -> bool {
        self.source.len() == 0
    }

    #[inline]
    pub fn line_column(&self) -> LineColumn {
        let line_offset = self.line_start;
        let line = self.line;
        let column = self.column;

        LineColumn { line_offset, line, column, }
    }

    #[inline]
    pub fn skip_whitespaces(&mut self) {
        loop {
            if self.source.len() == 0 {
                break;
            }

            let c = self.source.chars().next().unwrap();
            if is_whitespace(c) {
                self.advance(c.len_utf8());
            } else {
                break;
            }
            // for whitespace in WHITE_SPACES {
            //     if self.source.starts_with(*whitespace) {
            //         self.advance(whitespace.len_utf8());
            //         continue;
            //     }
            // }
            // break;
        }
    }

    #[inline]
    pub fn read_line_terminator(&mut self) -> LineTerminator {
        if self.source.starts_with(CR_LF) {
            let amt = CR_LF.len();
            self.bump(amt);

            return LineTerminator::EndOfLine;
        }

        for line_terminator in LINE_TERMINATORS {
            if self.source.starts_with(*line_terminator) {
                let amt = line_terminator.len_utf8();
                self.bump(amt);

                let value = match *line_terminator {
                    CR    => LineTerminator::CarriageReturn,
                    LF    => LineTerminator::LineFeed,
                    LS    => LineTerminator::LineSeparator,
                    PS    => LineTerminator::ParagraphSeparator,
                    _     => unreachable!(),
                };

                return value;
            }
        }

        unreachable!()
    }

    #[inline]
    pub fn read_hashbang(&mut self) -> SpannedToken {
        assert_eq!(self.source.starts_with("#!"), true);
        assert_eq!(self.line, 0);

        let start: LineColumn = self.line_column();

        let mut amt: usize = 2;
        // let mut value: String = String::new();

        loop {
            let source =  &self.source[amt..];
            
            if source.len() == 0 {
                break;
            }

            if source.starts_with(LINE_TERMINATORS) {
                break;
            }
            
            amt += source.chars().next().unwrap().len_utf8();
        }

        self.advance(amt);

        let end: LineColumn = self.line_column();
        let span = Span { start, end, item: Token::HashBang };

        return span;
    }
    
    #[inline]
    pub fn read_unicode_char(&mut self) -> Result<char, LexError> {
        let mut s = String::with_capacity(4);

        let source = self.source;

        if source.starts_with("\\u{") {
            for c in source[3..].chars() {
                if c == '}' {
                    break;
                }

                if s.len() == 4 {
                    return Err(LexError::UnexpectedToken);
                }

                if c.is_ascii_hexdigit() {
                    s.push(c);
                } else {
                    return Err(LexError::UnexpectedToken);
                }
            }

            if s.len() == 0 {
                return Err(LexError::UnexpectedToken);
            }
            
            match u32::from_str_radix(&s, 16) {
                Ok(n) => {
                    let ch = char::try_from(n).unwrap();
                    let amt = 3 + s.len() + 1;
                    self.advance(amt);

                    return Ok(ch);
                },
                Err(e) => {
                    return Err(LexError::UnexpectedToken);
                }
            }
        } else if source.starts_with("\\u") {
            if source.len() < 6 {
                return Err(LexError::UnexpectedEof);
            }
            
            for c in source[2..].chars() {
                if s.len() == 4 {
                    break;
                }
                if c.is_ascii_hexdigit() {
                    s.push(c);
                } else {
                    return Err(LexError::UnexpectedToken);
                }
            }

            if s.len() != 4 {
                return Err(LexError::UnexpectedToken);
            }

            match u32::from_str_radix(&s, 16) {
                Ok(n) => {
                    let ch = char::try_from(n).unwrap();
                    let amt = 6;
                    self.advance(amt);

                    return Ok(ch);
                },
                Err(e) => {
                    return Err(LexError::UnexpectedToken);
                }
            }
        } else {
            unreachable!()
        }
    }

    #[inline]
    pub fn read_string_literal(&mut self,
                               openning_delimiter_len_utf8: usize,
                               closing_delimiter: &str,
                               allow_line_terminator: bool,
                               unescape: bool,
                               ) -> Result<SpannedToken, LexError> {
        // string literal
        // template literal
        // assert_eq!(openning_delimiter == "\"" || openning_delimiter == "'" || openning_delimiter == "`", true);
        let start: LineColumn = self.line_column();
        let mut string: Vec<char> = Vec::new();

        let mut amt: usize = openning_delimiter_len_utf8;
        self.advance(amt);

        loop {
            let source =  self.source;
            
            if source.len() == 0 {
                return Err(LexError::UnexpectedEof);
            }

            if source.starts_with(CR_LF) || source.starts_with(LINE_TERMINATORS) {
                if allow_line_terminator {
                    match self.read_line_terminator() {
                        LineTerminator::EndOfLine => {
                            string.push(CR);
                            string.push(LF);
                        },
                        LineTerminator::CarriageReturn => {
                            string.push(CR);
                        },
                        LineTerminator::LineFeed => {
                            string.push(LF);
                        },
                        LineTerminator::LineSeparator => {
                            string.push(LS);
                        },
                        LineTerminator::ParagraphSeparator => {
                            string.push(PS);
                        },
                    }
                    continue;
                } else {
                    return Err(LexError::UnexpectedToken);
                }
            }
            
            if unescape {
                if source.starts_with("\\x") {
                    // \x d d
                    if source.len() < 4 {
                        return Err(LexError::UnexpectedEof);
                    }
                    match u8::from_str_radix(&source[2..4], 16) {
                        Ok(n) => {
                            string.push(n as char);
                            let amt = 4;
                            self.advance(amt);
                            continue;
                        },
                        Err(e) => {
                            return Err(LexError::UnexpectedToken);
                        }
                    }
                } else if source.starts_with("\\0") {
                    // \0
                    if source.len() < 3 {
                        return Err(LexError::UnexpectedEof);
                    }

                    let lookahead = source.chars().nth(2).unwrap();
                    if lookahead.is_ascii_digit() {
                        return Err(LexError::UnexpectedToken);
                    }

                    string.push('\0');
                    let amt = 2;
                    self.advance(amt);
                    continue;
                } else if source.starts_with("\\'") || source.starts_with("\\\"") || source.starts_with("\\\\") {
                    // '\'' | '"' | '\\'
                    string.push(source.chars().nth(1).unwrap());
                    let amt = 2;
                    self.advance(amt);
                    continue;
                } else if source.starts_with("\\b") {
                    // \b
                    string.push('\u{0008}');
                    let amt = 2;
                    self.advance(amt);
                    continue;
                } else if source.starts_with("\\f") {
                    // \f
                    string.push('\u{000c}');
                    let amt = 2;
                    self.advance(amt);
                    continue;
                } else if source.starts_with("\\n") {
                    // \n
                    string.push(LF);
                    let amt = 2;
                    self.advance(amt);
                    continue;
                } else if source.starts_with("\\r") {
                    // \r
                    string.push(CR);
                    let amt = 2;
                    self.advance(amt);
                    continue;
                } else if source.starts_with("\\t") {
                    // \t
                    string.push('\t');
                    let amt = 2;
                    self.advance(amt);
                    continue;
                } else if source.starts_with("\\v") {
                    // \v
                    string.push('\u{000b}');
                    let amt = 2;
                    self.advance(amt);
                    continue;
                } else if source.starts_with("\\u{") || source.starts_with("\\u") {
                    let c = self.read_unicode_char()?;
                    string.push(c);
                    continue;
                } else if source.starts_with("\\") {
                    if source.len() < 2 {
                        return Err(LexError::UnexpectedEof);
                    }

                    let c = source[1..].chars().nth(0).unwrap();
                    if c.is_ascii_digit() {
                        return Err(LexError::UnexpectedToken);
                    }

                    if source[1..].starts_with(LINE_TERMINATORS) {
                        return Err(LexError::UnexpectedToken);
                    }

                    string.push(c);
                    let amt = 2;
                    self.advance(amt);
                    continue;
                }
            }
            
            if source.starts_with(closing_delimiter) {
                let amt = closing_delimiter.len();
                self.advance(amt);
                break;
            }

            let c = source.chars().nth(0).unwrap();
            string.push(c);
            let amt = c.len_utf8();
            self.advance(amt);
        }


        let end: LineColumn = self.line_column();
        let span = Span { start, end, item: Token::LiteralString(string.into()) };

        return Ok(span);
    }
    
    #[inline]
    pub fn read_identifier(&mut self, first_char: char, mut has_escape_character: bool) -> Result<(Vec<char>, bool), LexError> {
        assert_eq!(is_identifier_start(first_char), true);

        let mut identifier: Vec<char> = Vec::new();
        identifier.push(first_char);

        loop {
            if self.source.len() == 0 {
                break;
            }

            let ch: char;
            if self.source.starts_with("\\u") {
                if !has_escape_character {
                    has_escape_character = true;
                }
                ch = self.read_unicode_char()?;
            } else {
                ch = self.source.chars().nth(0).unwrap();
            }
            
            if !is_identifier_part(ch) {
                break;
                // if is_whitespace(ch) || is_line_terminator(ch) {
                //     break;
                // } else {
                //     return Err(LexError::UnexpectedToken);
                // }
            }

            self.advance(ch.len_utf8());
            identifier.push(ch);
        }

        return Ok((identifier, has_escape_character))
    }

    #[inline]
    fn read_float(&mut self, numberic_string: &mut String) -> Result<(), LexError> {
        numberic_string.push('.');
        self.advance(1);

        loop {
            if self.source.len() == 0 {
                break;
            }

            let c = self.source.chars().next().unwrap();
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    numberic_string.push(c);
                    self.advance(1);
                },
                'e' | 'E' => {
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
    fn read_scientific(&mut self, numberic_string: &mut String) -> Result<(), LexError> {
        assert_eq!(self.source.starts_with("e") || self.source.starts_with("E"), true);

        numberic_string.push('e');
        self.advance(1);
        
        if self.source.len() == 0 {
            return Err(LexError::UnexpectedEof);
        }
        let c = self.source.chars().next().unwrap();
        if c == '-' || c == '+' {
            numberic_string.push(c);
            self.advance(1);
        }

        if self.source.len() == 0 {
            return Err(LexError::UnexpectedEof);
        }
        let c = self.source.chars().next().unwrap();
        if !c.is_ascii_digit() {
            return Err(LexError::UnexpectedToken);
        }
        numberic_string.push(c);
        self.advance(1);

        loop {
            if self.source.len() == 0 {
                break;
            }

            let c = self.source.chars().next().unwrap();
            if !c.is_ascii_digit() {
                break;
            }

            numberic_string.push(c);
            self.advance(1);
        }

        Ok(())
    }

    #[inline]
    pub fn read_numeric_literal(&mut self) -> Result<SpannedToken, LexError> {
        // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
        let mut numberic_string = String::with_capacity(16);
        
        // println!("{:?}", self.source);

        let start: LineColumn = self.line_column();

        if self.source.starts_with("0b") || self.source.starts_with("0B") {
            self.advance(2);
            
            loop {
                if self.source.len() == 0 {
                    break;
                }

                let c = self.source.chars().next().unwrap();
                
                if c != '0' && c != '1' {
                    break;
                }

                numberic_string.push(c);
                self.advance(1);
            }

            if numberic_string.len() == 0 {
                return Err(LexError::UnexpectedToken);
            }

            match u64::from_str_radix(&numberic_string, 2) {
                Ok(n) => {
                    let end: LineColumn = self.line_column();
                    let span = Span { start, end, item: Token::LiteralBinaryNumeric(n) };
                    return Ok(span);
                },
                Err(e) => {
                    return Err(LexError::UnexpectedToken);
                }
            }
        } else if self.source.starts_with("0o") || self.source.starts_with("0O") {
            self.advance(2);
            
            loop {
                if self.source.len() == 0 {
                    break;
                }

                let c = self.source.chars().next().unwrap();
                
                match c {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => { },
                    _ => break,
                }

                numberic_string.push(c);
                self.advance(1);
            }

            if numberic_string.len() == 0 {
                return Err(LexError::UnexpectedToken);
            }

            match u64::from_str_radix(&numberic_string, 8) {
                Ok(n) => {
                    let end: LineColumn = self.line_column();
                    let span = Span { start, end, item: Token::LiteralOctalNumeric(n) };
                    return Ok(span);
                },
                Err(e) => {
                    return Err(LexError::UnexpectedToken);
                }
            }
        } else if self.source.starts_with("0x") || self.source.starts_with("0X") {
            self.advance(2);
            
            loop {
                if self.source.len() == 0 {
                    break;
                }

                let c = self.source.chars().next().unwrap();
                
                if !c.is_ascii_hexdigit() {
                    break;
                }
                
                numberic_string.push(c);
                self.advance(1);
            }

            if numberic_string.len() == 0 {
                return Err(LexError::UnexpectedToken);
            }

            match u64::from_str_radix(&numberic_string, 16) {
                Ok(n) => {
                    let end: LineColumn = self.line_column();
                    let span = Span { start, end, item: Token::LiteralHexNumeric(n) };
                    return Ok(span);
                },
                Err(e) => {
                    return Err(LexError::UnexpectedToken);
                }
            }
        } else if self.source.starts_with("0") {
            numberic_string.push('0');
            self.advance(1);

            let mut second_digit: Option<char> = None;
            loop {
                if self.source.len() == 0 {
                    break;
                }

                let c = self.source.chars().next().unwrap();
                match c {
                    '.' => {
                        self.read_float(&mut numberic_string)?;
                        break;
                    },
                    'e' | 'E' => {
                        self.read_scientific(&mut numberic_string)?;
                        break;
                    },
                    _ => {
                        if c.is_ascii_digit() {
                            warn!("please don't add zero on numeric's head.");
                            return Err(LexError::UnexpectedToken);
                        } else {
                            break;
                        }

                        numberic_string.push(c);
                        self.advance(1);
                    }
                }
            }

            if numberic_string.len() == 0 {
                return Err(LexError::UnexpectedToken);
            }

            if numberic_string.contains('.') || numberic_string.contains('e') || numberic_string.contains('E') {
                match &numberic_string.parse::<f64>() {
                    Ok(n) => {
                        let end: LineColumn = self.line_column();
                        let span = Span { start, end, item: Token::LiteralFloatNumeric(Float(*n)) };
                        return Ok(span);
                    },
                    Err(e) => {
                        return Err(LexError::UnexpectedToken);
                    }
                }
            } else {
                match &numberic_string.parse::<u64>() {
                    Ok(n) => {
                        let end: LineColumn = self.line_column();
                        let span = Span { start, end, item: Token::LiteralDecimalNumeric(*n) };
                        return Ok(span);
                    },
                    Err(e) => {
                        return Err(LexError::UnexpectedToken);
                    }
                }
            }
        } else {
            // 1, 2, 3, 4, 5, 6, 7, 8, 9
            let c = self.source.chars().next().unwrap();
            numberic_string.push(c);
            self.advance(1);

            let mut second_digit: Option<char> = None;
            loop {
                if self.source.len() == 0 {
                    break;
                }

                let c = self.source.chars().next().unwrap();
                match c {
                    '.' => {
                        self.read_float(&mut numberic_string)?;
                        break;
                    },
                    'e' | 'E' => {
                        self.read_scientific(&mut numberic_string)?;
                        break;
                    },
                    _ => {
                        if !c.is_ascii_digit() {
                            break;
                        }

                        numberic_string.push(c);
                        self.advance(1);
                    }
                }
            }

            if numberic_string.len() == 0 {
                return Err(LexError::UnexpectedToken);
            }
            
            if numberic_string.contains('.') || numberic_string.contains('e') || numberic_string.contains('E') {
                match &numberic_string.parse::<f64>() {
                    Ok(n) => {
                        let end: LineColumn = self.line_column();
                        let span = Span { start, end, item: Token::LiteralFloatNumeric(Float(*n)) };
                        return Ok(span);
                    },
                    Err(e) => {
                        return Err(LexError::UnexpectedToken);
                    }
                }
            } else {
                match &numberic_string.parse::<u64>() {
                    Ok(n) => {
                        let end: LineColumn = self.line_column();
                        let span = Span { start, end, item: Token::LiteralDecimalNumeric(*n) };
                        return Ok(span);
                    },
                    Err(e) => {
                        return Err(LexError::UnexpectedToken);
                    }
                }
            }
        }
    }
    

    #[inline]
    pub fn read_comment(&mut self, is_multi_line: bool) -> Result<SpannedToken, LexError> {
        // single_line
        // // ...
        // /* ... */
        let start: LineColumn = self.line_column();
        
        let mut amt: usize = 2;
        let mut value: Vec<char> = Vec::new();

        if is_multi_line {
            self.advance(2);

            loop {
                let source =  self.source;
                
                if source.len() == 0 {
                    return Err(LexError::UnexpectedEof);
                }

                if source.starts_with("*/") {
                    self.advance(2);
                    break;
                }
                
                let c = source.chars().next().unwrap();
                
                if is_line_terminator(c) {
                    match self.read_line_terminator() {
                        LineTerminator::EndOfLine => {
                            value.push(CR);
                            value.push(LF);
                        },
                        LineTerminator::CarriageReturn => {
                            value.push(CR);
                        },
                        LineTerminator::LineFeed => {
                            value.push(LF);
                        },
                        LineTerminator::LineSeparator => {
                            value.push(LS);
                        },
                        LineTerminator::ParagraphSeparator => {
                            value.push(PS);
                        },
                    }
                    continue;
                }

                value.push(c);
                self.advance(c.len_utf8());
            }

            let end: LineColumn = self.line_column();
            let span = Span { start, end, item: Token::MultiLineComment(value.into()) };

            return Ok(span);
        } else {
            loop {
                let source =  &self.source[amt..];
                
                if source.len() == 0 {
                    break;
                }

                if source.starts_with(LINE_TERMINATORS) {
                    break;
                }
                
                let c = source.chars().next().unwrap();
                value.push(c);
                amt += c.len_utf8();
            }

            self.advance(amt);

            let end: LineColumn = self.line_column();
            let span = Span { start, end, item: Token::SingleLineComment(value.into()) };

            return Ok(span);
        }
    }
    
    #[inline]
    pub fn consume(&mut self) -> Result<SpannedToken, LexError> {
        loop {
            if self.eof() {
                return Err(LexError::EndOfProgram);
            }

            // if self.source.starts_with(WHITE_SPACES) {
            //     self.skip_whitespaces();
            // }
            self.skip_whitespaces();

            if self.source.starts_with(CR_LF) || self.source.starts_with(LINE_TERMINATORS) {
                let start: LineColumn = self.line_column();
                let line_terminator = self.read_line_terminator();
                let end: LineColumn = self.line_column();
                let token = Span { start, end, item: Token::LineTerminator(line_terminator) };
                return Ok(token);
            }

            if self.source.starts_with("#!") {
                // #!
                if self.line != 0 {
                    return Err(LexError::UnexpectedToken);
                }

                return Ok(self.read_hashbang());
            }

            // StringLiteral
            if self.source.starts_with("\""){
                let openning_delimiter_len_utf8 = 1;
                let closing_delimiter = "\"";
                let allow_line_terminator = false;
                let unescape = true;
                return self.read_string_literal(openning_delimiter_len_utf8, closing_delimiter, allow_line_terminator, unescape);
            }
            if self.source.starts_with("'"){
                let openning_delimiter_len_utf8 = 1;
                let closing_delimiter = "'";
                let allow_line_terminator = false;
                let unescape = true;
                return self.read_string_literal(openning_delimiter_len_utf8, closing_delimiter, allow_line_terminator, unescape);
            }
            if self.source.starts_with("`"){
                let openning_delimiter_len_utf8 = 1;
                let closing_delimiter = "`";
                let allow_line_terminator = true;
                let unescape = true;
                return self.read_string_literal(openning_delimiter_len_utf8, closing_delimiter, allow_line_terminator, unescape);
            }


            // comments
            if self.source.starts_with("//"){
                return self.read_comment(false);
            }
            if self.source.starts_with("/*"){
                return self.read_comment(true);
            }

            // LiteralNumeric
            if self.source.starts_with(DECIMAL_DIGITS) {
                return self.read_numeric_literal();
            }

            // punctuator
            for punctuator in PUNCTUATORS {
                if self.source.starts_with(punctuator) {
                    let start: LineColumn = self.line_column();
                    self.advance(punctuator.len());
                    let end: LineColumn = self.line_column();
                    let punc = Punctuator::from_str(punctuator).unwrap();
                    let token = Span { start, end, item: Token::Punctuator(punc) };
                    return Ok(token);
                }
            }

            const UNDEFINED: &[char] = &['u', 'n', 'd', 'e', 'f', 'i', 'n', 'e', 'd'];
            const NULL: &[char]      = &['n', 'u', 'l', 'l'];
            const TRUE: &[char]      = &['t', 'r', 'u', 'e'];
            const FALSE: &[char]     = &['f', 'a', 'l', 's', 'e'];
            const NAN: &[char]       = &['N', 'a', 'N'];
            const INFINITY: &[char]  = &['I', 'n', 'f', 'i', 'n', 'i', 't', 'y'];

            // unicode ident
            if self.source.starts_with("\\u") {
                let start: LineColumn = self.line_column();
                let first_char = self.read_unicode_char()?;
                
                if is_identifier_start(first_char) {
                    let (identifier, has_escape_character) = self.read_identifier(first_char, true)?;
                    match identifier.as_slice() {
                        UNDEFINED | NULL
                        | TRUE | FALSE
                        | NAN | INFINITY => {
                            return Err(LexError::UnexpectedToken);
                        },
                        _ => {
                            match Keyword::try_from(identifier.as_slice()) {
                                Ok(_) => return Err(LexError::UnexpectedToken),
                                _ => { },
                            }
                        }
                    }
                    let end: LineColumn = self.line_column();
                    let token = Span { start, end, item: Token::Identifier(identifier.into()) };
                    return Ok(token);
                } else {
                    return Err(LexError::UnexpectedToken);
                }
            }

            let first_char = self.source.chars().nth(0).unwrap();
            if is_identifier_start(first_char) {
                let start: LineColumn = self.line_column();
                self.advance(first_char.len_utf8());
                let (identifier, has_escape_character) = self.read_identifier(first_char, false)?;
                
                let token = if !has_escape_character {
                    match identifier.as_slice() {
                        NAN => Token::LiteralFloatNumeric(Float(std::f64::INFINITY)),
                        INFINITY => Token::LiteralFloatNumeric(Float(std::f64::INFINITY)),
                        UNDEFINED => Token::LiteralUndefined,
                        NULL => Token::LiteralNull,
                        TRUE => Token::LiteralBoolean(true),
                        FALSE => Token::LiteralBoolean(false),
                        _ => {
                            match Keyword::try_from(identifier.as_slice()) {
                                Ok(keyword) => Token::Keyword(keyword),
                                _ => Token::Identifier(identifier.into()),
                            }
                        }
                    }
                } else {
                    Token::Identifier(identifier.into())
                };

                let end: LineColumn = self.line_column();
                let token = Span { start, end, item: token };
                return Ok(token);
            } else {
                return Err(LexError::UnexpectedToken);
            }
        }
    }
}