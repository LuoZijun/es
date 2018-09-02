use ast::expression::{ AssignmentExpression, };
use ast::pattern::{
    BindingIdentifier, BindingPattern,
    BindingElement, BindingRestElement,
};
use ast::statement::Statement;


#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub func: Function,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionExpression {
    pub name: Option<String>,
    pub func: Function,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrowFunctionExpression {
    pub is_async: bool,
    pub name: Option<String>,
    pub params: ArrowParameters,
    pub body: ConciseBody,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArrowParameters {
    One(BindingIdentifier),
    // CoverParenthesizedExpressionAndArrowParameterList::try_into::<ArrowFormalParameters>()
    Many(ArrowFormalParameters),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConciseBody {
    One(AssignmentExpression),
    Many(FunctionBody),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub is_async: bool,
    pub is_generator: bool,
    pub params: FormalParameters,
    pub body: FunctionBody,
}

pub type FunctionBody = Vec<Statement>;
pub type FormalParameter = BindingElement;

#[derive(Debug, PartialEq, Clone)]
pub struct FormalParameters {
    pub items: Vec<FormalParameter>,
    pub rest_parameter: Option<BindingRestElement>,
}

pub type ArrowFormalParameters = UniqueFormalParameters;
pub type UniqueFormalParameters = FormalParameters;
