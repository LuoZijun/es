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
    pub span: Span,
    pub loc: Loc,
    pub value: &'ast [char],
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct WhiteSpaces {
    pub span: Span,
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LineTerminator {
    pub span: Span,
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Comment<'ast> {
    pub span: Span,
    pub loc: Loc,
    pub is_multi_line: bool,
    pub value: &'ast [char],
}


/// Keyword or IdentifierName
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Identifier<'ast> {
    pub span: Span,
    pub loc: Loc,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralString<'ast> {
    pub span: Span,
    pub loc: Loc,
    // pub delimiter: StringDelimiter,
    pub raw: &'ast [char],
    // if has_escaped_char { Some(cooked) } else { None }
    pub cooked: Option<&'ast [char]>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralNumeric<'ast> {
    pub span: Span,
    pub loc: Loc,
    pub raw: &'ast [char],
    pub value: Numberic,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralRegularExpression<'ast> {
    pub span: Span,
    pub loc: Loc,
    pub body: &'ast [char],
    pub flags: Option<&'ast [char]>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LiteralTemplate<'ast> {
    pub span: Span,
    pub loc: Loc,
    pub raw: &'ast [char],
    pub strings: &'ast [ LiteralString<'ast> ],
    pub bounds: &'ast [ &'ast [ Token<'ast> ] ],
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Punctuator {
    pub span: Span,
    pub loc: Loc,
    pub kind: PunctuatorKind,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Keyword {
    pub span: Span,
    pub loc: Loc,
    pub kind: KeywordKind,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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


// // args: Punctuated<Expr, Comma>
// pub struct Punctuated<T, P> {
//     inner: Vec<(T, P)>,
//     last: Option<Box<T>>,
// }
