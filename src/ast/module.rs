use super::IdentifierName;
use super::pattern::{ BindingIdentifier, BindingPattern, };
use super::function::{ FunctionDeclaration, };
use super::class::{ ClassDeclaration, };
use super::expression::{ AssignmentExpression, };
pub use super::declaration::{
    LexicalDeclaration, VariableStatement,
};


pub type ModuleSpecifier = String;


// https://www.ecma-international.org/ecma-262/9.0/index.html#prod-ImportDeclaration
#[derive(Debug, PartialEq, Clone)]
pub struct ImportDeclaration {
    pub clause: Option<ImportClause>,  // import './xxx.js'
                                       // import xxx from './xxx.js'
    pub module_specifier: ModuleSpecifier,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportSpecifier {
    Direct(BindingIdentifier),                   // xxx
    Named((IdentifierName, BindingIdentifier)),  // xxx as xxx2
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportClause {
    ImportedDefaultBinding(BindingIdentifier),  // import XXX
    NameSpaceImport(BindingIdentifier),         // import * as XXX
    NamedImports(Vec<ImportSpecifier>),         // { XXX, XXX as XXX2 }
    DefaultBindingWithNameSpaceImport((BindingIdentifier, BindingIdentifier)), // import XXX, * as XXX
    DefaultBindingWithNamedImports((BindingIdentifier, Vec<ImportSpecifier>)), // import XXX, { XXX, XXX as XXX2 }
}



// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-exports
#[derive(Debug, PartialEq, Clone)]
pub struct ExportDeclaration {
    pub clause: ExportClause,
}

pub type ExportSpecifier = ImportSpecifier;


#[derive(Debug, PartialEq, Clone)]
pub enum ExportClause {
    NameSpaceExportWithFrom(ModuleSpecifier),                          // export * from '...'
    NamedSpaceExportWithFrom((Vec<ExportSpecifier>, ModuleSpecifier)), // export { ... } from '...'
    NamedSpaceExport(Vec<ExportSpecifier>),                            // export { ... }
    Statement(ExportStatement),
    DefaultStatement(ExportDefaultStatement),
}

impl ExportClause {
    pub fn is_default(&self) -> bool {
        match *self {
            ExportClause::DefaultStatement(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExportStatement {
    LetOrConst(LexicalDeclaration),
    Variable(VariableStatement),
    Function(FunctionDeclaration),
    Class(ClassDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExportDefaultStatement {
    Function(FunctionDeclaration),
    Class(ClassDeclaration),
    // Not including: function expr, function* expr, class expr,
    //                async function expr, async function* expr
    Assignment(AssignmentExpression),
}
