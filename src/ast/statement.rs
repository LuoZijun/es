// pub use super::class::ClassDeclaration;
// pub use super::function::FunctionDeclaration;


use ast::expression::{ Expression, };


pub type StatementList = Vec<Statement>;


#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Variable(Box<VariableStatement>),
    LetOrConst(Box<LexicalDeclaration>),
    
    // Function(FunctionDeclaration),
    // Class(ClassDeclaration),

    Expression(Box<Expression>),

    Empty,
    Block(Box<BlockStatement>),
    If(Box<IfStatement>),

    // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-iteration-statements
    DoWhile(Box<DoWhileStatement>),
    While(Box<WhileStatement>),
    For(Box<ForStatement>),
    ForIn(Box<ForInStatement>),
    ForOf(Box<ForOfStatement>),
    ForAwaitOf(Box<ForAwaitOfStatement>),

    Continue(Box<ContinueStatement>),
    Break(Box<BreakStatement>),
    Return(Box<ReturnStatement>),
    With(Box<WithStatement>),
    Switch(Box<SwitchStatement>),
    Labelled(Box<LabelledStatement>),
    Throw(Box<ThrowStatement>),
    Try(Box<TryStatement>),
    
    Debugger,
}

impl Statement {
    pub fn is_declaration(&self) -> bool {
        match *self {
            Statement::Variable(_)
            | Statement::LetOrConst(_)
            // | Statement::Function(_)
            // | Statement::Class(_)
            => true,
            _ => false,
        }
    }
    
    pub fn is_expression(&self) -> bool {
        match *self {
            Statement::Expression(_) => true,
            _ => false,
        }
    }

    pub fn is_statement(&self) -> bool {
        !self.is_declaration() && !self.is_expression()
    }

    pub fn is_iteration_statement(&self) -> bool {
        match *self {
            Statement::DoWhile(_)
            | Statement::While(_)
            | Statement::For(_)
            | Statement::ForIn(_)
            | Statement::ForOf(_)
            | Statement::ForAwaitOf(_) => true,
            _ => false,
        }
    }

    pub fn is_breakable_statement(&self) -> bool {
        // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-BreakableStatement
        match *self {
            Statement::Switch(_) => true,
            _ => self.is_iteration_statement(),
        }
    }

    pub fn is_hoistable(&self) -> bool {
        unimplemented!()
        // match *self {
        //     // Statement::Variable(_) ?
        //     Statement::Function(_) => true,
        //     _ => false,
        // }
    }

    pub fn is_hoistable_declaration(&self) -> bool {
        unimplemented!()
        // match *self {
        //     Statement::Function(_) => true,
        //     _ => false,
        // }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexicalDeclarationKind {
    Let,
    Const,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LexicalBinding {
    pub name: Box<Expression>,
    pub initializer: Option<Box<Expression>>,
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
pub struct BlockStatement {
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EmptyStatement;


#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub and_then: Box<Statement>,
    pub or_else: Box<Statement>,
}


// Iteration Statements
#[derive(Debug, PartialEq, Clone)]
pub struct DoWhileStatement {
    pub condition: Box<Expression>,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    pub condition: Box<Expression>,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ForStatementInit {
    LetOrConst(LexicalDeclaration),
    Variable(VariableStatement),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    // initialization
    pub init: Option<ForStatementInit>,
    pub condition: Option<Box<Expression>>,
    pub finally: Option<Box<Expression>>,
    pub body: Box<Statement>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct ForInStatement {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForOfStatement {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForAwaitOfStatement {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub body: Box<Statement>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct ContinueStatement {
    pub label: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakStatement {
    pub label: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub value: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WithStatement {
    pub condition: Box<Expression>,
    pub then: Box<Statement>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStatementCaseClause {
    pub value: Box<Expression>,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStatement {
    pub value: Box<Expression>,
    pub clauses: Vec<SwitchStatementCaseClause>,
    pub default_clause: SwitchStatementCaseClause,
}


#[derive(Debug, PartialEq, Clone)]
pub struct LabelledStatement {
    pub label: String,
    // FunctionDeclaration is not allowed.
    pub item: Box<Statement>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct ThrowStatement {
    pub value: Box<Expression>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct TryStatement {
    pub body: BlockStatement,
    pub catch_parameter: Option<Box<Expression>>,
    pub catch_body: BlockStatement,
    pub finally: Option<BlockStatement>,
}

