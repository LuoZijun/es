use crate::lexer::span::{ Loc, Span, LineColumn, };
use crate::lexer::token::Identifier;
use crate::ast::statement::Statement;
use crate::ast::expression::{ Expression, ParenthesizedExpression, };

pub type FunctionBody<'ast> = &'ast [ Statement<'ast> ];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FunctionDeclaration<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub is_async: bool,
    pub is_generator: bool,
    pub name: Identifier<'ast>,
    pub func: Function<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FunctionExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub is_async: bool,
    pub is_generator: bool,
    pub name: Option<Identifier<'ast>>,
    pub func: Function<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Function<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub params: ParenthesizedExpression<'ast>,
    pub body: FunctionBody<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ArrowFunctionExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub is_async: bool,
    // Identifier or ParenthesizedExpression
    pub params: Expression<'ast>,
    pub body: ConciseBody<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConciseBody<'ast> {
    Expr(Expression<'ast>),
    Stmt(FunctionBody<'ast>),
}

// pub type FormalParameter = BindingElement;

// #[derive(Debug, PartialEq, Clone)]
// pub struct FormalParameters {
//     pub items: Vec<FormalParameter>,
//     pub rest_parameter: Option<BindingRestElement>,
// }

// pub type ArrowFormalParameters = UniqueFormalParameters;
// pub type UniqueFormalParameters = FormalParameters;
