use std::fmt;
use std::cmp;
use std::str::FromStr;
use std::convert::TryFrom;


const UNDEFINED: &[char] = &['u', 'n', 'd', 'e', 'f', 'i', 'n', 'e', 'd'];
const NULL: &[char]      = &['n', 'u', 'l', 'l'];
const TRUE: &[char]      = &['t', 'r', 'u', 'e'];
const FALSE: &[char]     = &['f', 'a', 'l', 's', 'e'];
const NAN: &[char]       = &['N', 'a', 'N'];
const INFINITY: &[char]  = &['I', 'n', 'f', 'i', 'n', 'i', 't', 'y'];


#[derive(Debug, Clone)]
pub struct Loc<T> {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
    pub item: T,
}

impl<T> Loc<T> {
    pub fn new(start: usize,
               end: usize,
               line: usize,
               column: usize,
               item: T) -> Self {
        Self { start, end, line, column, item, }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    // https://www.ecma-international.org/ecma-262/#sec-keywords
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
    // https://www.ecma-international.org/ecma-262/#sec-future-reserved-words
    Enum,
    // Use of the following tokens within strict mode code is also reserved
    Implements,
    Package,
    Protected,
    Interface,
    Private,
    Public,
}

// https://www.ecma-international.org/ecma-262/#sec-punctuators
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Punctuator {
    // Sharp,       // #
    // Slash        // /
    // BackSlash    // \
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

#[derive(Clone)]
pub enum TemplateItem {
    String(String),
    Exp(Vec<Token>),
}

#[derive(Clone)]
pub struct TemplateLiteral {
    pub items: Vec<TemplateItem>,
}

impl PartialEq for TemplateLiteral {
    fn eq(&self, other: &TemplateLiteral) -> bool {
        false
    }
}
impl Eq for TemplateLiteral { }

impl fmt::Debug for TemplateItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TemplateItem::String(ref s) => write!(f, "{:?}", s),
            TemplateItem::Exp(ref tokens) => {
                write!(f, "Exp( ");
                write!(f, "{}", tokens.iter()
                                    .map(|token| format!("{:?}", token.item))
                                    .collect::<Vec<std::string::String>>()
                                    .join(", ")
                                    );
                write!(f, " )")
            }
        }
    }
}

impl fmt::Debug for TemplateLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Template( ");
        let s = &self.items.iter().map(|item| format!("{:?}", item))
            .collect::<Vec<std::string::String>>()
            .join(", ");

        write!(f, "{}", s);
        write!(f, " )");

        Ok(())
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct RegularExpressionLiteral {
    pub body: String,
    pub flag: Option<String>,  // g/i/m/u/y/s
}

#[derive(Clone)]
pub enum Literal {
    Undefined,
    Null,
    Boolean(bool),
    String(String),
    Numeric(f64),
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Literal::Undefined => write!(f, "undefined"),
            Literal::Null => write!(f, "null"),
            Literal::Boolean(v) => write!(f, "{}", v),
            Literal::String(ref v) => write!(f, "{:?}", v),
            Literal::Numeric(v) => write!(f, "{}", v),
        }
    }
}

impl std::hash::Hash for Literal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match *self {
            Literal::Numeric(n) => {
                let num = if n.is_nan() && n.is_sign_negative() {
                    std::f64::NAN
                } else if n == 0f64 && n.is_sign_negative() {
                    0f64
                } else {
                    n
                };

                num.to_bits().hash(state);
            },
            _ => self.hash(state),
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match *self {
            Literal::Numeric(a) => {
                match *other {
                    Literal::Numeric(b) => {
                        if a.is_nan() {
                            if b.is_nan() {
                                true
                            } else {
                                false
                            }
                        } else {
                            a == b
                        }
                    },
                    _ => false,
                }
            },
            _ => self.eq(other),
        }
    }
}

impl Eq for Literal {}



#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    HashBang,           // #!
    Identifier(String), // not include keywords
    Keyword(Keyword),
    Literal(Literal),
    TemplateLiteral(TemplateLiteral),

    Comment,
    Punctuator(Punctuator),

    // WhiteSpaces,
    LineTerminator,
    
    EndOfProgram,
    UnexpectedToken,
    UnexpectedEof,
}


pub type Token = Loc<TokenKind>;



impl TryFrom<&[char]> for Literal {
    type Error = ();
    
    fn try_from(value: &[char]) -> Result<Self, Self::Error> {
        match value {
            UNDEFINED => Ok(Literal::Undefined),
            NULL      => Ok(Literal::Null),
            TRUE      => Ok(Literal::Boolean(true)),
            FALSE     => Ok(Literal::Boolean(false)),
            NAN       => Ok(Literal::Numeric(std::f64::NAN)),
            INFINITY  => Ok(Literal::Numeric(std::f64::INFINITY)),
            _         => Err(()),
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


impl FromStr for Literal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "undefined" => Ok(Literal::Undefined),
            "null"      => Ok(Literal::Null),
            "true"      => Ok(Literal::Boolean(true)),
            "false"     => Ok(Literal::Boolean(false)),
            "NaN"       => Ok(Literal::Numeric(std::f64::NAN)),
            "Infinity"  => Ok(Literal::Numeric(std::f64::INFINITY)),
            _           => Err(()),
        }
    }
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

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Keyword::*;

        match *self {
            Await => write!(f, "await"),
            Break => write!(f, "break"),
            Case => write!(f, "case"),
            Catch => write!(f, "catch"),
            Class => write!(f, "class"),
            Const => write!(f, "const"),
            Continue => write!(f, "continue"),
            Debugger => write!(f, "debugger"),
            Default => write!(f, "default"),
            Delete => write!(f, "delete"),
            Do => write!(f, "do"),
            Else => write!(f, "else"),
            Export => write!(f, "export"),
            Extends => write!(f, "extends"),
            Finally => write!(f, "finally"),
            For => write!(f, "for"),
            Function => write!(f, "function"),
            If => write!(f, "if"),
            Import => write!(f, "import"),
            In => write!(f, "in"),
            Instanceof => write!(f, "instanceof"),
            New => write!(f, "new"),
            Return => write!(f, "return"),
            Super => write!(f, "super"),
            Switch => write!(f, "switch"),
            This => write!(f, "this"),
            Throw => write!(f, "throw"),
            Try => write!(f, "try"),
            Typeof => write!(f, "typeof"),
            Var => write!(f, "var"),
            Void => write!(f, "void"),
            While => write!(f, "while"),
            With => write!(f, "with"),
            Yield => write!(f, "yield"),
            Let => write!(f, "let"),
            Static => write!(f, "static"),
            Enum => write!(f, "enum"),
            Implements => write!(f, "implements"),
            Package => write!(f, "package"),
            Protected => write!(f, "protected"),
            Interface => write!(f, "interface"),
            Private => write!(f, "private"),
            Public => write!(f, "public"),
        }
    }
}


impl fmt::Display for Punctuator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Punctuator::*;
        
        match *self {
            BackTick => write!(f, "`"),
            Question => write!(f, "?"),
            DotMark => write!(f, "."),
            Semicolon => write!(f, ";"),
            Colon => write!(f, ":"),
            Comma => write!(f, ","),
            Spread => write!(f, "..."),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            LBracket => write!(f, "["),
            RBracket => write!(f, "]"),
            LBrace => write!(f, "{{"),
            RBrace => write!(f, "}}"),
            FatArrow => write!(f, "=>"),
            Increment => write!(f, "++"),
            Decrement => write!(f, "--"),
            Assign => write!(f, "="),
            Not => write!(f, "!"),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Rem => write!(f, "%"),
            Pow => write!(f, "**"),
            BitNot => write!(f, "~"),
            BitAnd => write!(f, "&"),
            BitOr => write!(f, "|"),
            BitXor => write!(f, "^"),
            BitShl => write!(f, "<<"),
            BitShr => write!(f, ">>"),
            BitUShr => write!(f, ">>>"),
            AddAssign => write!(f, "+="),
            SubAssign => write!(f, "-="),
            MulAssign => write!(f, "*="),
            DivAssign => write!(f, "/="),
            RemAssign => write!(f, "%="),
            PowAssign => write!(f, "**="),
            BitAndAssign => write!(f, "&="),
            BitOrAssign => write!(f, "|="),
            BitXorAssign => write!(f, "^="),
            BitShlAssign => write!(f, "<<="),
            BitShrAssign => write!(f, ">>="),
            BitUShrAssign => write!(f, ">>>="),
            Eq => write!(f, "=="),
            StrictEq => write!(f, "==="),
            Gt => write!(f, ">"),
            Lt => write!(f, "<"),
            Neq => write!(f, "!="),
            StrictNeq => write!(f, "!=="),
            GtEq => write!(f, ">="),
            LtEq => write!(f, "<="),
        }
    }
}

