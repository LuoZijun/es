use crate::lexer::token::Identifier;
use crate::ast::statement::Statement;
use crate::ast::expression::{ Expression, ParenthesizedExpression, };
use crate::ast::function::{ Function, FunctionBody, };


// PropertyName
//      IdentifierName
//      StringLiteral
//      NumericLiteral
//      Expression

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ClassDeclaration<'ast> {
    pub name: Identifier<'ast>,
    pub class: Class<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ClassExpression<'ast> {
    pub name: Option<Identifier<'ast>>,
    pub class: Class<'ast>,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Method<'ast> {
    pub is_async: bool,
    pub is_generator: bool,
    pub name: Expression<'ast>,
    pub params: ParenthesizedExpression<'ast>,
    pub body: FunctionBody<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Getter<'ast> {
    pub name: Expression<'ast>,
    pub body: FunctionBody<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Setter<'ast> {
    pub name: Expression<'ast>,
    pub params: ParenthesizedExpression<'ast>,
    pub body: FunctionBody<'ast>,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MethodDefinition<'ast> {
    Method(Method<'ast>),
    Getter(Getter<'ast>),
    Setter(Setter<'ast>),
}

impl<'ast> MethodDefinition<'ast> {
    pub fn name(&self) -> &Expression<'ast> {
        match *self {
            MethodDefinition::Getter(ref getter) => &getter.name,
            MethodDefinition::Setter(ref setter) => &setter.name,
            MethodDefinition::Method(ref method) => &method.name,
        }
    }

    pub fn is_async(&self) -> bool {
        match *self {
            MethodDefinition::Getter(_)
            | MethodDefinition::Setter(_) => false,
            MethodDefinition::Method(ref method) => method.is_async,
        }
    }

    pub fn is_generator(&self) -> bool {
        match *self {
            MethodDefinition::Getter(_)
            | MethodDefinition::Setter(_) => false,
            MethodDefinition::Method(ref method) => method.is_generator,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ClassMethodDefinition<'ast> {
    pub is_static: bool,
    pub method: MethodDefinition<'ast>,
}

impl<'ast> ClassMethodDefinition<'ast> {
    pub fn name(&self) -> &Expression<'ast> {
        self.method.name()
    }
    
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    
    pub fn is_async(&self) -> bool {
        self.method.is_async()
    }

    pub fn is_generator(&self) -> bool {
        self.method.is_generator()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Class<'ast> {
    pub heritage: Expression<'ast>, // extend
    pub body: &'ast [ ClassMethodDefinition<'ast> ],
}
