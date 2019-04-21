use crate::ast::numberic::{ Float, Numberic, };

use crate::lexer::span::{ Span, Loc, };
use crate::lexer::keyword::KeywordKind;
use crate::lexer::punctuator::PunctuatorKind;

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
    pub loc: Loc,
    pub span: Span,
    pub value: &'ast [char],
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct WhiteSpaces {
    pub loc: Loc,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LineTerminator {
    pub loc: Loc,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Comment<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub is_multi_line: bool,
    pub value: &'ast [char],
}


/// Keyword or IdentifierName
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Identifier<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub raw: &'ast [char],
    // if has_escaped_char { Some(cooked) } else { None }
    pub cooked: Option<&'ast [char]>,
}

impl<'ast> Identifier<'ast> {
    pub fn to_keyword_or_literal(&self) -> Option<Token<'ast>> {
        if self.cooked.is_some() {
            // NOTE: 含有转义序列的 Ident 不能作为 Keyword, LiteralNull, LiteralBoolean
            return None;
        }
        
        match self.raw {
            LITERAL_NULL  => Some(Token::LiteralNull(LiteralNull { span: self.span, loc: self.loc})),
            LITERAL_TRUE  => Some(Token::LiteralBoolean(LiteralBoolean { span: self.span, loc: self.loc, value: true })),
            LITERAL_FALSE => Some(Token::LiteralBoolean(LiteralBoolean { span: self.span, loc: self.loc, value: false })),
            _ => match KeywordKind::try_from(&self.raw) {
                Ok(kw_kind) => Some(Token::Keyword(Keyword { span: self.span, loc: self.loc, kind: kw_kind })),
                Err(_) => None,
            }
        }
    }
}

impl<'ast> fmt::Debug for Identifier<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Identifier({})", self.raw.iter().collect::<String>())
    }
}




#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralNull {
    pub loc: Loc,
    pub span: Span,
}
impl fmt::Debug for LiteralNull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LiteralNull")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralBoolean {
    pub loc: Loc,
    pub span: Span,
    pub value: bool,
}
impl fmt::Debug for LiteralBoolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LiteralBoolean({})", self.value)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralString<'ast> {
    pub loc: Loc,
    pub span: Span,
    // pub delimiter: StringDelimiter,
    pub raw: &'ast [char],
    // if has_escaped_char { Some(cooked) } else { None }
    pub cooked: Option<&'ast [char]>,
}
impl<'ast> fmt::Debug for LiteralString<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LiteralString({:?})", self.raw.iter().collect::<String>())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralNumeric<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub raw: &'ast [char],
    pub value: Numberic,
}

impl<'ast> fmt::Debug for LiteralNumeric<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LiteralNumeric({:?})", self.value)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralRegularExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub body: &'ast [char],
    pub flags: Option<&'ast [char]>,
}
impl<'ast> fmt::Debug for LiteralRegularExpression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LiteralRegularExpression({:?}, {:?})",
            self.body.iter().collect::<String>(),
            self.flags.map(|flags| flags.iter().collect::<String>()))
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralTemplate<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub raw: &'ast [char],
    pub strings: &'ast [ LiteralString<'ast> ],
    pub bounds: &'ast [ &'ast [ Token<'ast> ] ],
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Punctuator {
    pub loc: Loc,
    pub span: Span,
    pub kind: PunctuatorKind,
}
impl fmt::Debug for Punctuator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Punctuator_{:?}", self.kind)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Keyword {
    pub loc: Loc,
    pub span: Span,
    pub kind: KeywordKind,
}
impl fmt::Debug for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Keyword_{:?}", self.kind)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Token<'ast> {
    // HashBang(HashBang<'ast>),
    // WhiteSpaces,
    // Comment(Comment<'ast>),

    LineTerminator,
    /// include Keyword, LiteralNull, LiteralTrue, LiteralFalse
    Identifier(Identifier<'ast>),
    Keyword(Keyword),
    LiteralNull(LiteralNull),
    LiteralBoolean(LiteralBoolean),
    Punctuator(Punctuator),

    LiteralString(LiteralString<'ast>),
    LiteralNumeric(LiteralNumeric<'ast>),
    LiteralRegularExpression(LiteralRegularExpression<'ast>),
    LiteralTemplate(LiteralTemplate<'ast>),

    TemplateOpenning,
}
impl<'ast> fmt::Debug for Token<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::LineTerminator => write!(f, "LineTerminator"),
            Token::TemplateOpenning => write!(f, "TemplateOpenning"),
            Token::Identifier(inner) => fmt::Debug::fmt(&inner, f),
            Token::Keyword(inner) => fmt::Debug::fmt(&inner, f),
            Token::LiteralNull(inner) => fmt::Debug::fmt(&inner, f),
            Token::LiteralBoolean(inner) => fmt::Debug::fmt(&inner, f),
            Token::Punctuator(inner) => fmt::Debug::fmt(&inner, f),
            Token::LiteralString(inner) => fmt::Debug::fmt(&inner, f),
            Token::LiteralNumeric(inner) => fmt::Debug::fmt(&inner, f),
            Token::LiteralRegularExpression(inner) => fmt::Debug::fmt(&inner, f),
            Token::LiteralTemplate(inner) => fmt::Debug::fmt(&inner, f),
        }
        
    }
}

// args: Punctuated<Expr, Comma>
// pub struct Punctuated<T, P> {
//     inner: Vec<(T, P)>,
//     last: Option<Box<T>>,
// }
