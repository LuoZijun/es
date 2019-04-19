use lexer::span::{ Loc, Span, LineColumn, };
use lexer::token::{
    Identifier, LiteralNull, LiteralBoolean, LiteralString, LiteralNumeric,
    LiteralRegularExpression, 
    Punctuator, Keyword, Comment,
};
use lexer::operator::{ PrefixOperator, InfixOperator, PostfixOperator, AssignmentOperator, };

use ast::numberic::{ Float, Numberic, };
// use ast::class::ClassExpression;
// use ast::function::{ FunctionExpression, ArrowFunctionExpression, UniqueFormalParameters, };
// use ast::jsx::{ JSXFragment, JSXElement, };


pub type ExpressionList<'ast> = Vec<Expression<'ast>>;


// https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#Table
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::LeftToRight
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Expression<'ast> {
    // Comment(&'ast Comment<'ast>),

    This(&'ast Keyword),

    // NOTE: 特殊表达式，`Spread` 和 `Super` 无法独立成为一个表达式，必须要依附于其它表达式类型。
    Spread(&'ast SpreadExpression<'ast>),
    Super(&'ast Keyword),
    // BindingElement(&'ast BindingElement<'ast>),
    // BindingProperty(&'ast BindingProperty<'ast>),
    
    Identifier(&'ast Identifier<'ast>),
    Null(&'ast LiteralNull),
    Boolean(&'ast LiteralBoolean),
    String(&'ast LiteralString<'ast>),
    Numeric(&'ast LiteralNumeric<'ast>),
    RegularExpression(&'ast LiteralRegularExpression<'ast>),
    Template(&'ast LiteralTemplateExpression<'ast>),
    
    // TODO: 
    // ArrayLiteral(&'ast ArrayLiteral<'ast>),
    // ObjectLiteral(&'ast ObjectLiteral<'ast>),

    Parenthesized(&'ast ParenthesizedExpression<'ast>),

    Member(&'ast MemberExpression<'ast>),

    TaggedTemplate(&'ast TaggedTemplateExpression<'ast>),
    /// This is the `new.target` expression that was introduced in ES2015. 
    /// This tells you if the function was called with the new operator.
    NewTarget(&'ast NewTargetExpression),
    Call(&'ast CallExpression<'ast>),
    New(&'ast NewExpression<'ast>),

    Prefix(&'ast PrefixExpression<'ast>),
    Infix(&'ast InfixExpression<'ast>),
    Postfix(&'ast PostfixExpression<'ast>),
    Assignment(&'ast AssignmentExpression<'ast>),

    Conditional(&'ast ConditionalExpression<'ast>),
    Yield(&'ast YieldExpression<'ast>),
    
    Comma(&'ast CommaExpression<'ast>),

    // TODO:
    // ObjectBindingPattern(&'ast ObjectBindingPattern<'ast>),
    // ArrayBindingPattern(&'ast ArrayBindingPattern<'ast>),
    
    // JSX
    // JSXFragment(JSXFragment),
    // JSXElement(JSXElement),
}

impl<'ast> Expression<'ast> {
    pub fn loc(&self) -> Loc {
        match *self {
            Expression::This(inner) => inner.loc,
            Expression::Spread(inner) => inner.loc,
            Expression::Super(inner) => inner.loc,

            // BindingElement(inner) => inner.loc,
            // BindingProperty(inner) => inner.loc,
            
            Expression::Identifier(inner) => inner.loc,
            Expression::Null(inner) => inner.loc,
            Expression::Boolean(inner) => inner.loc,
            Expression::String(inner) => inner.loc,
            Expression::Numeric(inner) => inner.loc,
            Expression::RegularExpression(inner) => inner.loc,
            Expression::Template(inner) => inner.loc,
            
            // ArrayLiteral(inner) => inner.loc,
            // ObjectLiteral(inner) => inner.loc,

            Expression::Parenthesized(inner) => inner.loc,

            Expression::Member(inner) => inner.loc,

            Expression::TaggedTemplate(inner) => inner.loc,

            Expression::NewTarget(inner) => inner.loc,
            Expression::Call(inner) => inner.loc,
            Expression::New(inner) => inner.loc,

            Expression::Prefix(inner) => inner.loc,
            Expression::Infix(inner) => inner.loc,
            Expression::Postfix(inner) => inner.loc,
            Expression::Assignment(inner) => inner.loc,

            Expression::Conditional(inner) => inner.loc,
            Expression::Yield(inner) => inner.loc,
            
            Expression::Comma(inner) => inner.loc,
        }
    }

    pub fn span(&self) -> Span {
        match *self {
            Expression::This(inner) => inner.span,
            Expression::Spread(inner) => inner.span,
            Expression::Super(inner) => inner.span,

            // BindingElement(inner) => inner.span,
            // BindingProperty(inner) => inner.span,
            
            Expression::Identifier(inner) => inner.span,
            Expression::Null(inner) => inner.span,
            Expression::Boolean(inner) => inner.span,
            Expression::String(inner) => inner.span,
            Expression::Numeric(inner) => inner.span,
            Expression::RegularExpression(inner) => inner.span,
            Expression::Template(inner) => inner.span,
            
            // ArrayLiteral(inner) => inner.span,
            // ObjectLiteral(inner) => inner.span,

            Expression::Parenthesized(inner) => inner.span,

            Expression::Member(inner) => inner.span,

            Expression::TaggedTemplate(inner) => inner.span,
            
            Expression::NewTarget(inner) => inner.span,
            Expression::Call(inner) => inner.span,
            Expression::New(inner) => inner.span,

            Expression::Prefix(inner) => inner.span,
            Expression::Infix(inner) => inner.span,
            Expression::Postfix(inner) => inner.span,
            Expression::Assignment(inner) => inner.span,

            Expression::Conditional(inner) => inner.span,
            Expression::Yield(inner) => inner.span,
            
            Expression::Comma(inner) => inner.span,
        }
    }

    pub fn is_primitive_literal(&self) -> bool {
        unimplemented!()
    }

    pub fn is_numeric_literal(&self) -> bool {
        unimplemented!()
    }

    pub fn is_primary_expression(&self) -> bool {
        unimplemented!()
    }

    pub fn is_left_hand_side_expression(&self) -> bool {
        unimplemented!()
    }

    pub fn is_member_expression(&self) -> bool {
        unimplemented!()
    }

    pub fn is_call_expression(&self) -> bool {
        unimplemented!()
    }

    pub fn is_assignment_expression(&self) -> bool {
        // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-AssignmentExpression
        // use self::Expression::*;

        // match *self {
        //     Conditional(_)
        //     | Yield(_)
        //     // AssignmentOperator
        //     => true,

        //     _ => false,
        // }
        unimplemented!()
    }

    pub fn precedence(&self) -> u8 {
        // https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#Table
        unimplemented!()
    }
}

// ... target
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SpreadExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub item: Expression<'ast>,
}

// `template string ${ a + b} end.`
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LiteralTemplateExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub strings: &'ast [ LiteralString<'ast> ],
    pub bounds: &'ast [ Expression<'ast> ],
}

// [ ]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ArrayLiteral {
    pub loc: Loc,
    pub span: Span,
    // TODO:
}

// { }
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ObjectLiteral {
    pub loc: Loc,
    pub span: Span,
    // TODO:
}

// ( Expression, + )
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ParenthesizedExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub elems: &'ast [ Expression<'ast> ],
}

// FunctionExpression
// ClassExpression
// GeneratorExpression
// AsyncFunctionExpression
// AsyncGeneratorExpression



// Left-Hand-Side Expressions
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MemberExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub left: Expression<'ast>,
    pub right: Expression<'ast>,
    pub computed: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TaggedTemplateExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub tag: Expression<'ast>,
    pub template: LiteralTemplateExpression<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NewTargetExpression {
    pub loc: Loc,
    pub span: Span,
}

// https://www.ecma-international.org/ecma-262/9.0/index.html#prod-CallExpression
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CallExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub callee: Expression<'ast>,
    // TODO:
    pub arguments: &'ast [ Expression<'ast> ],
}


// new abc( ... )
// new abc.asd( ... )
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NewExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub callee: Expression<'ast>,
    // NOTE: 支持无参数
    pub arguments: Option<&'ast [ Expression<'ast> ]>,
}

// https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#Table
// Unary
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PrefixExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub operator: PrefixOperator,
    pub operand: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InfixExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub left: Expression<'ast>,
    pub operator: InfixOperator,
    pub right: Expression<'ast>,
}

// --
// ++
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PostfixExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub operator: PostfixOperator,
    pub operand: Expression<'ast>,
}


// https://www.ecma-international.org/ecma-262/9.0/index.html#prod-ConditionalExpression
// LogicalORExpression ? AssignmentExpression : AssignmentExpression 
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ConditionalExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub condition: Expression<'ast>,
    pub and_then: Expression<'ast>,
    pub or_else: Expression<'ast>,
}

// https://www.ecma-international.org/ecma-262/9.0/index.html#prod-YieldExpression
// yield AssignmentExpression
// yield *AssignmentExpression
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct YieldExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    // no LineTerminator here
    pub star: bool,                    // *
    pub value: Expression<'ast>,       // AssignmentExpression
                                       // * AssignmentExpression
}

// https://www.ecma-international.org/ecma-262/9.0/index.html#prod-AssignmentExpression
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AssignmentExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub left: Expression<'ast>,
    pub operator: AssignmentOperator,
    pub right: Expression<'ast>,
}

// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-comma-operator
// Expression, +
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CommaExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    // TODO:
    pub items: &'ast [ Expression<'ast> ],
}


// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct BindingElement {
//     // Identifier           Option<Initializer>
//     // ArrayBindingPattern  Initializer
//     // ObjectBindingPattern Initializer
//     pub key: Box<Expression>,
//     pub initializer: Option<Box<Expression>>,
// }

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct ArrayBindingPattern {
//     pub elems: Vec<Option<BindingElement>>,
//     pub rest_elem: Option<Box<Expression>>,
// }


// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct BindingProperty {
//     // Identifier           Option<Initializer>
//     // ArrayBindingPattern  Initializer
//     // ObjectBindingPattern Initializer
//     pub key: Box<Expression>,
//     pub initializer: Option<Box<Expression>>,
// }

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct ObjectBindingPattern {
//     pub properties: Vec<BindingProperty>,
//     pub rest_property: Option<Box<Expression>>,  // BindingIdentifier
// }
