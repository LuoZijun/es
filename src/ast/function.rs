use crate::lexer::span::{ Loc, Span, LineColumn, };
use crate::lexer::token::Identifier;
use crate::ast::statement::Statement;
use crate::ast::expression::{ Expression, ParenthesizedExpression, };


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FunctionDeclaration<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub name: Identifier<'ast>,
    pub func: Function<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FunctionExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub name: Option<Identifier<'ast>>,
    pub func: Function<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Function<'ast> {
    pub is_async: bool,
    pub is_generator: bool,
    pub params: ParenthesizedExpression<'ast>,
    pub body: &'ast [ Statement<'ast> ],
}

pub type FunctionBody<'ast> = &'ast [ Statement<'ast> ];


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ArrowFunctionExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub is_async: bool,
    pub name: Option<Identifier<'ast>>,
    pub params: ArrowParameters<'ast>,
    pub body: ConciseBody<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ArrowParameters<'ast> {
    One(Identifier<'ast>),
    // CoverParenthesizedExpressionAndArrowParameterList::try_into::<ArrowFormalParameters>()
    Many(ParenthesizedExpression<'ast>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConciseBody<'ast> {
    One(Expression<'ast>),
    Many(FunctionBody<'ast>),
}

// pub type FormalParameter = BindingElement;

// #[derive(Debug, PartialEq, Clone)]
// pub struct FormalParameters {
//     pub items: Vec<FormalParameter>,
//     pub rest_parameter: Option<BindingRestElement>,
// }

// pub type ArrowFormalParameters = UniqueFormalParameters;
// pub type UniqueFormalParameters = FormalParameters;
