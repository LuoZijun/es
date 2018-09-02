use super::IdentifierName;
use super::pattern::{ BindingIdentifier, BindingPattern, };

use ast::class::ClassDeclaration;
use ast::function::FunctionDeclaration;
use ast::expression::{ Expression, AssignmentExpression, };



#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexicalDeclarationKind {
    Let,
    Const,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LexicalBinding {
    Identifier((BindingIdentifier, Option<AssignmentExpression>)),
    Pattern((BindingPattern, AssignmentExpression)),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LexicalDeclaration {
    pub kind: LexicalDeclarationKind,
    pub declarators: Vec<LexicalBinding>,
}

impl LexicalDeclaration {
    pub fn is_let(&self) -> bool {
        match self.kind {
            LexicalDeclarationKind::Let => true,
            _ => false,
        }
    }

    pub fn is_const(&self) -> bool {
        match self.kind {
            LexicalDeclarationKind::Const => true,
            _ => false,
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct VariableStatement {
    pub declarators: Vec<LexicalBinding>,
}


#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Hoistable(FunctionDeclaration),
    Class(ClassDeclaration),
    LetOrConst(LexicalDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub enum HoistableDeclaration {
    FunctionDeclaration,
    GeneratorDeclaration,
    AsyncFunctionDeclaration,
    AsyncGeneratorDeclaration,
}