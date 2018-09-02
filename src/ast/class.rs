use super::function::{
    FormalParameters, UniqueFormalParameters, 
    Function, FunctionBody, FormalParameter,
};
use super::expression::LeftHandSideExpression;
use super::statement::Statement;
use super::pattern::PropertyName;


#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration {
    pub name: String,
    pub class: Class,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassExpression {
    pub name: Option<String>,
    pub class: Class,
}



#[derive(Debug, PartialEq, Clone)]
pub struct Method {
    pub is_async: bool,
    pub is_generator: bool,
    pub name: PropertyName,
    pub params: UniqueFormalParameters,
    pub body: FunctionBody,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Getter {
    pub name: PropertyName,
    pub body: FunctionBody,
}

pub type PropertySetParameterList = UniqueFormalParameters;

#[derive(Debug, PartialEq, Clone)]
pub struct Setter {
    pub name: PropertyName,
    pub params: PropertySetParameterList,
    pub body: FunctionBody,
}


#[derive(Debug, PartialEq, Clone)]
pub enum MethodDefinition {
    Method(Method),
    Getter(Getter),
    Setter(Setter),
}

impl MethodDefinition {
    pub fn name(&self) -> &PropertyName {
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

#[derive(Debug, PartialEq, Clone)]
pub struct ClassMethodDefinition {
    pub is_static: bool,
    pub method: MethodDefinition,
}

impl ClassMethodDefinition {
    pub fn name(&self) -> &PropertyName {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Class {
    pub heritage: LeftHandSideExpression, // extend
    pub body: Vec<ClassMethodDefinition>,
}
