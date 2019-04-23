// document: https://facebook.github.io/jsx/

use crate::lexer::span::{ Loc, Span, LineColumn, };
use crate::lexer::token::{ Identifier, LiteralString, LiteralBoolean, LiteralNumeric, };
use crate::ast::statement::Statement;
use crate::ast::expression::{ Expression, ParenthesizedExpression, };


// React only.
pub const CREATE_JSX_ELEMENT: &str = "React.createElement";
pub const CREATE_JSX_FRAGMENT: &str = "React.Fragment";

// IdentifierStart
// JSXIdentifier IdentifierPart
// JSXIdentifier NO WHITESPACE OR COMMENT -
pub type JSXIdentifier<'ast> = Identifier<'ast>;
pub type JSXAttributes<'ast> = &'ast [ JSXAttribute<'ast> ];

pub type JSXMemberExpression<'ast> = &'ast [ JSXIdentifier<'ast> ];
// SourceCharacter but not one of {, <, > or }
pub type JSXText<'ast> = LiteralString<'ast>;
pub type JSXChildExpression<'ast> = &'ast [ Expression<'ast> ];

pub type JSXChildren<'ast> = &'ast [ JSXChild<'ast> ];


// PrimaryExpression: JSXFragment, JSXElement
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JSXFragment<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub children: Option<JSXChildren<'ast>>,
}

impl<'ast> JSXFragment<'ast> {
    pub fn loc(&self) -> Loc {
        self.loc
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JSXElement<'ast> {
    SelfClosing(JSXSelfClosingElement<'ast>),
    Normal(JSXNormalElement<'ast>),
}

impl<'ast> JSXElement<'ast> {
    pub fn loc(&self) -> Loc {
        match *self {
            JSXElement::SelfClosing(inner) => inner.loc,
            JSXElement::Normal(inner) => inner.loc,
        }
    }

    pub fn span(&self) -> Span {
        match *self {
            JSXElement::SelfClosing(inner) => inner.span,
            JSXElement::Normal(inner) => inner.span,
        }
    }

    pub fn is_self_closing(&self) -> bool {
        match *self {
            JSXElement::SelfClosing(_) => true,
            JSXElement::Normal(_) => false,
        }
    }

    pub fn is_element_name_match(&self) -> bool {
        match *self {
            JSXElement::SelfClosing(_) => true,
            JSXElement::Normal(ref elem) => elem.opening.name == elem.closing.name,
        }
    }
}



#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JSXOpeningElement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub name: JSXElementName<'ast>,
    pub attrs: Option<JSXAttributes<'ast>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JSXClosingElement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub name: JSXElementName<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JSXSelfClosingElement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub name: JSXElementName<'ast>,
    pub attrs: Option<JSXAttributes<'ast>>,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JSXNormalElement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub opening: JSXOpeningElement<'ast>,
    pub children: Option<JSXChildren<'ast>>,
    pub closing: JSXClosingElement<'ast>,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JSXElementName<'ast> {
    Identifier(JSXIdentifier<'ast>),
    NamespacedName(JSXNamespacedName<'ast>),
    MemberExpression(JSXMemberExpression<'ast>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JSXNamespacedName<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub namespace: JSXIdentifier<'ast>,
    pub name: JSXIdentifier<'ast>,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JSXAttribute<'ast> {
    Normal(JSXNormalAttribute<'ast>),
    Spread(Expression<'ast>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JSXNormalAttribute<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub name: JSXNormalAttributeName<'ast>,
    pub init: Option<JSXNormalAttributeInitializer<'ast>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JSXNormalAttributeName<'ast> {
    Identifier(JSXIdentifier<'ast>),
    NamespacedName(JSXNamespacedName<'ast>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JSXNormalAttributeInitializer<'ast> {
    Identifier(JSXIdentifier<'ast>),
    Assignment(Expression<'ast>),
    Element(JSXElement<'ast>),
    Fragment(JSXFragment<'ast>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JSXChild<'ast> {
    Text(JSXText<'ast>),
    Element(JSXElement<'ast>),
    ChildExpression(Option<JSXChildExpression<'ast>>),
}
