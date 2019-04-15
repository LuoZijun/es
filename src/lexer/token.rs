use ast::numberic::{ Float, Numberic, };

use super::span::{ Span, Loc, };
use super::keyword::Keyword;
use super::punctuator::PunctuatorKind;

use std::fmt;
use std::cmp;
use std::hash;
use std::str::FromStr;


pub const LITERAL_NULL: &[char]   = &['n', 'u', 'l', 'l'];
pub const LITERAL_TRUE: &[char]   = &['t', 'r', 'u', 'e'];
pub const LITERAL_FALSE: &[char]  = &['f', 'a', 'l', 's', 'e'];


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StringDelimiter {
    SingleQuote,
    MultiQuote,
    Template,
}


/// #!/xx/xxx
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct HashBang<'ast> {
    pub span: Span,
    pub loc: Loc,
    pub value: &'ast [char],
}


/// Keyword or IdentifierName
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Identifier<'ast> {
    pub span: Span,
    pub loc: Loc,
    pub raw: &'ast [char],
    // if has_escaped_char { Some(cooked) } else { None }
    pub cooked: Option<Vec<char>>,
}

impl<'ast> Identifier<'ast> {
    pub fn to_keyword(&self) -> Option<Keyword> {
        if self.cooked.is_some() {
            return None;
        }

        Keyword::try_from(&self.raw).ok()
    }

    pub fn to_null(&self) -> Option<Token<'ast>> {
        if self.cooked.is_some() {
            return None;
        }
        
        if self.raw == LITERAL_NULL {
            Some(Token::LiteralNull(LiteralNull { span: self.span, loc: self.loc}))
        } else {
            None
        }
        
    }

    pub fn to_bool(&self) -> Option<Token<'ast>> {
        if self.cooked.is_some() {
            return None;
        }
        
        let val = match self.raw {
            LITERAL_TRUE => true,
            LITERAL_FALSE => false,
            _ => return None,
        };

        Some(Token::LiteralBoolean(LiteralBoolean { span: self.span, loc: self.loc, value: val }))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Comment<'ast> {
    pub span: Span,
    pub loc: Loc,
    pub is_multi_line: bool,
    pub value: &'ast [char],
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralNull {
    pub span: Span,
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralBoolean {
    pub span: Span,
    pub loc: Loc,
    pub value: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LiteralString<'ast> {
    pub span: Span,
    pub loc: Loc,
    // pub delimiter: StringDelimiter,
    pub raw: &'ast [char],
    // if has_escaped_char { Some(cooked) } else { None }
    pub cooked: Option<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LiteralNumeric<'ast> {
    pub span: Span,
    pub loc: Loc,
    pub raw: &'ast [char],
    pub value: Numberic,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LiteralRegularExpression<'ast> {
    pub span: Span,
    pub loc: Loc,
    pub body: &'ast [char],
    pub flags: &'ast [char],
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LiteralTemplate<'ast> {
    pub span: Span,
    pub loc: Loc,
    pub raw: &'ast [char],
    pub strings: Vec<LiteralString<'ast>>,
    pub bounds: Vec<Token<'ast>>,
}


// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// pub struct WhiteSpaces {
//     pub span: Span,
//     pub loc: Loc,
// }

// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// pub struct LineTerminator {
//     pub span: Span,
//     pub loc: Loc,
// }

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Punctuator {
    pub span: Span,
    pub loc: Loc,
    pub kind: PunctuatorKind,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token<'ast> {
    // HashBang(HashBang<'ast>),
    // WhiteSpaces,
    // Comment(Comment<'ast>),
    LineTerminator,
    /// including keyword
    Identifier(Identifier<'ast>),

    LiteralNull(LiteralNull),
    LiteralBoolean(LiteralBoolean),
    LiteralString(LiteralString<'ast>),
    LiteralNumeric(LiteralNumeric<'ast>),
    LiteralRegularExpression(LiteralRegularExpression<'ast>),
    LiteralTemplate(LiteralTemplate<'ast>),

    TemplateOpenning,
    // InstanceOf,    // instanceof
    // In,            // in
    Punctuator(Punctuator),
}


// // args: Punctuated<Expr, Comma>
// pub struct Punctuated<T, P> {
//     inner: Vec<(T, P)>,
//     last: Option<Box<T>>,
// }

