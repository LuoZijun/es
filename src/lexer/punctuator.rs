use std::str::FromStr;


pub const PUNCT_DOTDOTDOT: &[char]     = &[ '.', '.', '.', ];
pub const PUNCT_FATARROW: &[char]      = &[ '=', '>', ];
pub const PUNCT_INCREMENT: &[char]     = &[ '+', '+', ];
pub const PUNCT_DECREMENT: &[char]     = &[ '-', '-', ];
pub const PUNCT_AND: &[char]           = &[ '&', '&', ];
pub const PUNCT_OR: &[char]            = &[ '|', '|', ];
pub const PUNCT_POW: &[char]           = &[ '*', '*', ];
pub const PUNCT_BITSHL: &[char]        = &[ '<', '<', ];
pub const PUNCT_BITSHR: &[char]        = &[ '>', '>', ];
pub const PUNCT_BITUSHR: &[char]       = &[ '>', '>', '>', ];
pub const PUNCT_ADDASSIGN: &[char]     = &[ '+', '=', ];
pub const PUNCT_SUBASSIGN: &[char]     = &[ '-', '=', ];
pub const PUNCT_MULASSIGN: &[char]     = &[ '*', '=', ];
pub const PUNCT_DIVASSIGN: &[char]     = &[ '/', '=', ];
pub const PUNCT_REMASSIGN: &[char]     = &[ '%', '=', ];
pub const PUNCT_POWASSIGN: &[char]     = &[ '*', '*', '=', ];
pub const PUNCT_BITANDASSIGN: &[char]  = &[ '&', '=', ];
pub const PUNCT_BITORASSIGN: &[char]   = &[ '|', '=', ];
pub const PUNCT_BITXORASSIGN: &[char]  = &[ '^', '=', ];
pub const PUNCT_BITSHLASSIGN: &[char]  = &[ '<', '<', '=', ];
pub const PUNCT_BITSHRASSIGN: &[char]  = &[ '>', '>', '=', ];
pub const PUNCT_BITUSHRASSIGN: &[char] = &[ '>', '>', '>', '=', ];
pub const PUNCT_EQ: &[char]            = &[ '=', '=', ];
pub const PUNCT_STRICTEQ: &[char]      = &[ '=', '=', '=', ];
pub const PUNCT_NEQ: &[char]           = &[ '!', '=', ];
pub const PUNCT_STRICTNEQ: &[char]     = &[ '!', '=', '=', ];
pub const PUNCT_GTEQ: &[char]          = &[ '>', '=', ];
pub const PUNCT_LTEQ: &[char]          = &[ '<', '=', ];


// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-punctuators
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PunctuatorKind {
    Colon,          // :
    Question,       // ?
    Semicolon,      // ;
    Comma,          // ,
    Dot,            // .
    DotDotDot,      // ... , Spread

    LParen,         // (
    RParen,         // )
    LBracket,       // [
    RBracket,       // ]
    LBrace,         // {
    RBrace,         // }
    FatArrow,       // =>

    Increment,      // ++ , Maybe unary operator
    Decrement,      // -- , Maybe unary operator

    // Logical operator
    Not,            //  ! , unary operator
    And,            // &&
    Or,             // ||

    // Binary operators
    Add,            //  + , Maybe unary operator
    Sub,            //  - , Maybe unary operator
    Mul,            //  *
    Div,            //  /
    Rem,            //  %
    Pow,            // **
    BitNot,         // ~ , unary operator
    BitAnd,         // &
    BitOr,          // |
    BitXor,         // ^
    BitShl,         // <<
    BitShr,         // >>
    BitUShr,        // >>>

    // assignment operator
    Assign,         //  =
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

    // compare operator
    Eq,             // ==
    StrictEq,       // ===
    Gt,             // >
    Lt,             // <
    Neq,            // !=
    StrictNeq,      // !==
    GtEq,           // >=
    LtEq,           // <=
}

impl FromStr for PunctuatorKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::PunctuatorKind::*;

        match s {
            "?" => Ok(Question),
            "." => Ok(Dot),
            ";" => Ok(Semicolon),
            ":" => Ok(Colon),
            "," => Ok(Comma),
            "..." => Ok(DotDotDot),
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

