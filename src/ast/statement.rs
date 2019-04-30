
use crate::lexer::span::{ Loc, Span, LineColumn, };
use crate::lexer::token::{
    Identifier, LiteralNull, LiteralBoolean, LiteralString, LiteralNumeric,
    LiteralRegularExpression, 
    Punctuator, Keyword,
};
use crate::ast::expression::{ Expression, };
// use ast::class::ClassDeclaration;
// use ast::function::FunctionDeclaration;


pub type StatementList<'ast> = Vec<Statement<'ast>>;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Statement<'ast> {
    Empty(&'ast EmptyStatement),
    Debugger(&'ast DebuggerStatement),
    
    Expression(&'ast Expression<'ast>),

    Variable(&'ast VariableStatement<'ast>),
    // Function(FunctionDeclaration),
    // Class(ClassDeclaration),

    Block(&'ast BlockStatement<'ast>),
    If(&'ast IfStatement<'ast>),

    // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-iteration-statements
    DoWhile(&'ast DoWhileStatement<'ast>),
    While(&'ast WhileStatement<'ast>),
    For(&'ast ForStatement<'ast>),
    ForIn(&'ast ForInStatement<'ast>),
    ForOf(&'ast ForOfStatement<'ast>),
    ForAwaitOf(&'ast ForAwaitOfStatement<'ast>),

    Continue(&'ast ContinueStatement<'ast>),
    Break(&'ast BreakStatement<'ast>),
    Return(&'ast ReturnStatement<'ast>),
    With(&'ast WithStatement<'ast>),
    Switch(&'ast SwitchStatement<'ast>),
    Labelled(&'ast LabelledStatement<'ast>),
    Throw(&'ast ThrowStatement<'ast>),
    Try(&'ast TryStatement<'ast>),
}

impl<'ast> Statement<'ast> {
    pub fn loc(&self) -> Loc {
        match *self {
            Statement::Empty(inner) => inner.loc,
            Statement::Debugger(inner) => inner.loc,
            
            Statement::Expression(inner) => inner.loc(),

            Statement::Variable(inner) => inner.loc,
            // Function(FunctionDeclaration),
            // Class(ClassDeclaration),

            Statement::Block(inner) => inner.loc,
            Statement::If(inner) => inner.loc,

            Statement::DoWhile(inner) => inner.loc,
            Statement::While(inner) => inner.loc,
            Statement::For(inner) => inner.loc,
            Statement::ForIn(inner) => inner.loc,
            Statement::ForOf(inner) => inner.loc,
            Statement::ForAwaitOf(inner) => inner.loc,

            Statement::Continue(inner) => inner.loc,
            Statement::Break(inner) => inner.loc,
            Statement::Return(inner) => inner.loc,
            Statement::With(inner) => inner.loc,
            Statement::Switch(inner) => inner.loc,
            Statement::Labelled(inner) => inner.loc,
            Statement::Throw(inner) => inner.loc,
            Statement::Try(inner) => inner.loc,
        }
    }

    pub fn span(&self) -> Span {
        match *self {
            Statement::Empty(inner) => inner.span,
            Statement::Debugger(inner) => inner.span,
            
            Statement::Expression(inner) => inner.span(),

            Statement::Variable(inner) => inner.span,
            // Function(FunctionDeclaration),
            // Class(ClassDeclaration),

            Statement::Block(inner) => inner.span,
            Statement::If(inner) => inner.span,

            Statement::DoWhile(inner) => inner.span,
            Statement::While(inner) => inner.span,
            Statement::For(inner) => inner.span,
            Statement::ForIn(inner) => inner.span,
            Statement::ForOf(inner) => inner.span,
            Statement::ForAwaitOf(inner) => inner.span,

            Statement::Continue(inner) => inner.span,
            Statement::Break(inner) => inner.span,
            Statement::Return(inner) => inner.span,
            Statement::With(inner) => inner.span,
            Statement::Switch(inner) => inner.span,
            Statement::Labelled(inner) => inner.span,
            Statement::Throw(inner) => inner.span,
            Statement::Try(inner) => inner.span,
        }
    }

    pub fn is_declaration(&self) -> bool {
        match *self {
            Statement::Variable(_)
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
pub struct EmptyStatement {
    pub loc: Loc,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DebuggerStatement {
    pub loc: Loc,
    pub span: Span,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexicalDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LexicalBinding<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub name: Expression<'ast>,
    pub initializer: Option<Expression<'ast>>,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VariableStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub kind: LexicalDeclarationKind,
    pub declarators: &'ast [ LexicalBinding<'ast> ],
}

impl<'ast> VariableStatement<'ast> {
    pub fn is_var(&self) -> bool {
        match self.kind {
            LexicalDeclarationKind::Var => true,
            _ => false,
        }
    }

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


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BlockStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub body: &'ast [ Statement<'ast> ],
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct IfStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub condition: Expression<'ast>,
    pub and_then: Statement<'ast>,
    pub or_else: Statement<'ast>,
}

// Iteration Statements
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DoWhileStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub condition: Expression<'ast>,
    pub body: Statement<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WhileStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub condition: Expression<'ast>,
    pub body: Statement<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ForStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    // initialization
    pub init: Option<Statement<'ast>>,       // var/let/const/expr
    pub condition: Option<Expression<'ast>>,
    pub finally: Option<Expression<'ast>>,
    pub body: Statement<'ast>,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ForInStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub left: Expression<'ast>,
    pub right: Expression<'ast>,
    pub body: Statement<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ForOfStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub left: Expression<'ast>,
    pub right: Expression<'ast>,
    pub body: Statement<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ForAwaitOfStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub left: Expression<'ast>,
    pub right: Expression<'ast>,
    pub body: Statement<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ContinueStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub label: Option<Identifier<'ast>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BreakStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub label: Option<Identifier<'ast>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ReturnStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub value: Option<Expression<'ast>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WithStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub condition: Expression<'ast>,
    pub then: Statement<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SwitchStatementCaseClause<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub value: Expression<'ast>,
    pub body: Statement<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SwitchStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub value: Expression<'ast>,
    // default_clause: Option<SwitchStatementCaseClause<'ast>>
    pub clauses: &'ast [ SwitchStatementCaseClause<'ast> ],
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LabelledStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub label: Identifier<'ast>,
    // FunctionDeclaration is not allowed.
    pub item: Statement<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ThrowStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub value: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TryStatement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub body: BlockStatement<'ast>,
    pub catch_parameter: Option<Expression<'ast>>,
    pub catch_body: BlockStatement<'ast>,
    pub finally: Option<BlockStatement<'ast>>,
}
