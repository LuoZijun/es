use crate::toolshed::{ Arena, };

use crate::version::ECMAScriptVersion;
use crate::error::{ ErrorKind, Error, };

use crate::lexer::Lexer;
use crate::lexer::span::{ Loc, Span, LineColumn, };
use crate::lexer::token::{ Token, Punctuator, Identifier, LiteralString, LiteralNumeric, };
use crate::ast::statement::{ 
    Statement,
    VariableStatement, LexicalDeclarationKind, LexicalBinding,
};
use crate::ast::expression::{
    Expression, LiteralTemplateExpression,
    PrefixExpression, InfixExpression, PostfixExpression, AssignmentExpression,
    MemberExpression, NewTargetExpression, NewExpression,
    ConditionalExpression, YieldExpression, CommaExpression,
    TaggedTemplateExpression, SpreadExpression, ParenthesizedExpression,
    CallExpression, 
};
use crate::ast::function::{ FunctionExpression, Function, ArrowFunctionExpression, ConciseBody, };
use crate::ast::class::{
    ClassDeclaration, ClassExpression, Class, ClassMethodDefinition, 
    MethodDefinition, Method, Getter, Setter,
};


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PropertyName<'ast> {
    Identifier(Identifier<'ast>),
    Numberic(LiteralNumeric<'ast>),
    String(LiteralString<'ast>),
    Computed(Expression<'ast>),
}

impl<'ast> PropertyName<'ast> {
    pub fn loc(&self) -> Loc {
        match *self {
            PropertyName::Identifier(inner) => inner.loc,
            PropertyName::Numberic(inner) => inner.loc,
            PropertyName::String(inner) => inner.loc,
            PropertyName::Computed(inner) => inner.loc(),
        }
    }

    pub fn span(&self) -> Span {
        match *self {
            PropertyName::Identifier(inner) => inner.span,
            PropertyName::Numberic(inner) => inner.span,
            PropertyName::String(inner) => inner.span,
            PropertyName::Computed(inner) => inner.span(),
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PropertyDefinition<'ast> {
    // Destructuring or ObjectLiteral
    // Ident,
    Identifier(Identifier<'ast>),
    // Destructuring
    // Key = Value
    CoverInitializedName {
        loc: Loc,
        span: Span,
        name: PropertyName<'ast>,
        init: Expression<'ast>,
    },
    // ObjectLiteral: Key: Value
    // Destructuring: Key: Alias(Ident) Option(= Value)
    // PropertyName(),
    Property {
        loc: Loc,
        span: Span,
        name: PropertyName<'ast>,
        // :
        puct: Punctuator,
        // Ident / Expr
        alias: Expression<'ast>,
        init: Expression<'ast>
    },
    // ObjectLiteral
    // Class Method
    MethodDefinition(MethodDefinition<'ast>),
    // Destructuring or ObjectLiteral
    Spread(SpreadExpression<'ast>),
}


// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-destructuring-binding-patterns
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BindingPattern<'ast> {
    Object(ObjectBindingPattern<'ast>),
    Array(ArrayBindingPattern<'ast>),
}

impl<'ast> BindingPattern<'ast> {
    pub fn loc(&self) -> Loc {
        match *self {
            BindingPattern::Object(inner) => inner.loc,
            BindingPattern::Array(inner) => inner.loc,
        }
    }

    pub fn span(&self) -> Span {
        match *self {
            BindingPattern::Object(inner) => inner.span,
            BindingPattern::Array(inner) => inner.span,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ObjectBindingPattern<'ast> {
    pub loc: Loc,
    pub span: Span,
    // NOTE: Only one REST property elem and must be last
    pub properties: &'ast [ BindingProperty<'ast> ],
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ArrayBindingPattern<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub elems: &'ast [ Option<BindingElement<'ast>> ],
    // NOTE: Only one REST property elem and must be last
    pub rest_elem: Option<&'ast BindingRestElement<'ast>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BindingRestElement<'ast> {
    Identifier(Identifier<'ast>),
    BindingPattern(BindingPattern<'ast>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BindingElement<'ast> {
    SingleNameBinding {
        loc: Loc,
        span: Span,
        name: Identifier<'ast>,
        init: Option<Expression<'ast>>,
    },
    BindingPattern {
        loc: Loc,
        span: Span,
        pattern: BindingPattern<'ast>,
        init: Option<Expression<'ast>>,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BindingProperty<'ast> {
    SingleNameBinding {
        loc: Loc,
        span: Span,
        name: Identifier<'ast>,
        init: Option<Expression<'ast>>,
    },
    Property {
        loc: Loc,
        span: Span,
        name: PropertyName<'ast>,
        puct: Punctuator,             // :
        value: BindingElement<'ast>,
    },
    Spread {
        loc: Loc,
        span: Span,
        puct: Punctuator,            // ...
        name: Identifier<'ast>,
    },
}


// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-destructuring-assignment
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AssignmentPattern<'ast> {
    Object(ObjectAssignmentPattern<'ast>),
    Array(ArrayAssignmentPattern<'ast>),
}

impl<'ast> AssignmentPattern<'ast> {
    pub fn loc(&self) -> Loc {
        match *self {
            AssignmentPattern::Object(inner) => inner.loc,
            AssignmentPattern::Array(inner) => inner.loc,
        }
    }

    pub fn span(&self) -> Span {
        match *self {
            AssignmentPattern::Object(inner) => inner.span,
            AssignmentPattern::Array(inner) => inner.span,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ObjectAssignmentPattern<'ast> {
    pub loc: Loc,
    pub span: Span,
    // NOTE: Only one REST property elem and must be last
    pub properties: &'ast [ AssignmentProperty<'ast> ],
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ArrayAssignmentPattern<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub elems: &'ast [ Option<AssignmentElement<'ast>> ],
    // NOTE: Only one REST property elem and must be last
    pub rest_elem: Option<Expression<'ast>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AssignmentElement<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub elem: Expression<'ast>,         // LeftHandSideExpr
    pub init: Option<Expression<'ast>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AssignmentProperty<'ast> {
    Identifier {
        loc: Loc,
        span: Span,
        name: Identifier<'ast>,
        init: Option<Expression<'ast>>,
    },
    Property {
        loc: Loc,
        span: Span,
        name: PropertyName<'ast>,
        puct: Punctuator,                // :
        // NOTE: elem 看起来似乎必须为 Identifier , 这个在 AST 生成后，再做正确性检查。
        value: AssignmentElement<'ast>,
    },
    Spread {
        loc: Loc,
        span: Span,
        puct: Punctuator,               // ...
        target: Expression<'ast>,       // LeftHandSideExpr
    }
}




// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-object-initializer
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ObjectLiteral<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub properties: &'ast [ ObjectProperty<'ast> ],
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ObjectProperty<'ast> {
    Identifier(Identifier<'ast>),
    Property {
        loc: Loc,
        span: Span,
        name: PropertyName<'ast>,
        puct: Punctuator,                      // :
        value: Expression<'ast>
    },
    MethodDefinition(MethodDefinition<'ast>),
    Spread {
        puct: Punctuator,                     // ...
        target: Expression<'ast>,
    },
}

// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-array-initializer
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ArrayLiteral<'ast> {
    pub loc: Loc,
    pub span: Span,
    // AssignmentExpression 
    // SpreadElement
    pub elems: &'ast [ Option<Expression<'ast>> ],
}


