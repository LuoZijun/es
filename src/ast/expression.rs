use crate::lexer::span::{ Loc, Span, LineColumn, };
use crate::lexer::token::{
    Identifier, LiteralNull, LiteralBoolean, LiteralString, LiteralNumeric,
    LiteralRegularExpression, 
    Punctuator, Keyword, Comment,
};
use crate::lexer::operator::{ PrefixOperator, InfixOperator, PostfixOperator, AssignmentOperator, };

use crate::ast::numberic::{ Float, Numberic, };
use crate::ast::class::ClassExpression;
use crate::ast::function::{ FunctionExpression, ArrowFunctionExpression, };
use crate::ast::jsx::{ JSXFragment, JSXElement, };

use std::fmt;


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


#[derive(PartialEq, Clone, Copy)]
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
    /// Array Initializer
    // ArrayLiteral(&'ast ArrayLiteral<'ast>),
    /// Object Initializer
    // ObjectLiteral(&'ast ObjectLiteral<'ast>),
    Function(&'ast FunctionExpression<'ast>),
    ArrowFunction(&'ast ArrowFunctionExpression<'ast>),
    Class(&'ast ClassExpression<'ast>),
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
    ObjectBindingPattern(&'ast ObjectBindingPattern<'ast>),
    ArrayBindingPattern(&'ast ArrayBindingPattern<'ast>),
    
    // JSX
    JSXFragment(&'ast JSXFragment<'ast>),
    JSXElement(&'ast JSXElement<'ast>),
}

impl<'ast> fmt::Debug for Expression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::This(inner) => fmt::Debug::fmt(inner, f),
            Expression::Spread(inner) => fmt::Debug::fmt(inner, f),
            Expression::Super(inner) => fmt::Debug::fmt(inner, f),

            // BindingElement(inner) => fmt::Debug::fmt(inner, f),
            // BindingProperty(inner) => fmt::Debug::fmt(inner, f),
            
            Expression::Identifier(inner) => fmt::Debug::fmt(inner, f),
            Expression::Null(inner) => fmt::Debug::fmt(inner, f),
            Expression::Boolean(inner) => fmt::Debug::fmt(inner, f),
            Expression::String(inner) => fmt::Debug::fmt(inner, f),
            Expression::Numeric(inner) => fmt::Debug::fmt(inner, f),
            Expression::RegularExpression(inner) => fmt::Debug::fmt(inner, f),
            Expression::Template(inner) => fmt::Debug::fmt(inner, f),
            
            // ArrayLiteral(inner) => fmt::Debug::fmt(inner, f),
            // ObjectLiteral(inner) => fmt::Debug::fmt(inner, f),
            Expression::Function(inner) => fmt::Debug::fmt(inner, f),
            Expression::ArrowFunction(inner) => fmt::Debug::fmt(inner, f),
            Expression::Class(inner) => fmt::Debug::fmt(inner, f),
            Expression::Parenthesized(inner) => fmt::Debug::fmt(inner, f),

            Expression::Member(inner) => fmt::Debug::fmt(inner, f),

            Expression::TaggedTemplate(inner) => fmt::Debug::fmt(inner, f),

            Expression::NewTarget(inner) => fmt::Debug::fmt(inner, f),
            Expression::Call(inner) => fmt::Debug::fmt(inner, f),
            Expression::New(inner) => fmt::Debug::fmt(inner, f),

            Expression::Prefix(inner) => fmt::Debug::fmt(inner, f),
            Expression::Infix(inner) => fmt::Debug::fmt(inner, f),
            Expression::Postfix(inner) => fmt::Debug::fmt(inner, f),
            Expression::Assignment(inner) => fmt::Debug::fmt(inner, f),

            Expression::Conditional(inner) => fmt::Debug::fmt(inner, f),
            Expression::Yield(inner) => fmt::Debug::fmt(inner, f),
            
            Expression::Comma(inner) => fmt::Debug::fmt(inner, f),

            Expression::ObjectBindingPattern(inner) => fmt::Debug::fmt(inner, f),
            Expression::ArrayBindingPattern(inner) => fmt::Debug::fmt(inner, f),

            Expression::JSXFragment(inner) => fmt::Debug::fmt(inner, f),
            Expression::JSXElement(inner) => fmt::Debug::fmt(inner, f),
        }
    }
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
            Expression::Function(inner) => inner.loc,
            Expression::ArrowFunction(inner) => inner.loc,
            Expression::Class(inner) => inner.loc,
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

            Expression::ObjectBindingPattern(inner) => inner.loc,
            Expression::ArrayBindingPattern(inner) => inner.loc,

            Expression::JSXFragment(inner) => inner.loc(),
            Expression::JSXElement(inner) => inner.loc(),
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
            Expression::Function(inner) => inner.span,
            Expression::ArrowFunction(inner) => inner.span,
            Expression::Class(inner) => inner.span,
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

            Expression::ObjectBindingPattern(inner) => inner.span,
            Expression::ArrayBindingPattern(inner) => inner.span,

            Expression::JSXFragment(inner) => inner.span(),
            Expression::JSXElement(inner) => inner.span(),
        }
    }

    pub fn is_primitive_literal(&self) -> bool {
        unimplemented!()
    }

    pub fn is_numeric_literal(&self) -> bool {
        match *self {
            Expression::Numeric(_) => true,
            _ => false,
        }
    }

    pub fn is_identifier(&self) -> bool {
        match *self {
            Expression::Identifier(_) => true,
            _ => false,
        }
    }
    
    pub fn is_prefix_increment(&self) -> bool {
        match *self {
            Expression::Prefix(inner) => inner.operator == PrefixOperator::Increment,
            _ => false,
        }
    }

    pub fn is_prefix_decrement(&self) -> bool {
        match *self {
            Expression::Prefix(inner) => inner.operator == PrefixOperator::Decrement,
            _ => false,
        }
    }

    pub fn is_postfix_increment(&self) -> bool {
        match *self {
            Expression::Postfix(inner) => inner.operator == PostfixOperator::Increment,
            _ => false,
        }
    }

    pub fn is_postfix_decrement(&self) -> bool {
        match *self {
            Expression::Postfix(inner) => inner.operator == PostfixOperator::Decrement,
            _ => false,
        }
    }

    pub fn is_member_expression(&self) -> bool {
        match *self {
            Expression::Member(_) => true,
            _ => false,
        }
    }

    pub fn is_comma_expression(&self) -> bool {
        match *self {
            Expression::Comma(_) => true,
            _ => false,
        }
    }

    pub fn is_parenthesized_expression(&self) -> bool {
        match *self {
            Expression::Parenthesized(_) => true,
            _ => false,
        }
    }

    pub fn is_call_expression(&self) -> bool {
        match *self {
            Expression::Call(_) => true,
            _ => false,
        }
    }

    pub fn is_function_expression(&self) -> bool {
        match *self {
            Expression::Function(_) => true,
            _ => false,
        }
    }

    pub fn is_arrow_function_expression(&self) -> bool {
        match *self {
            Expression::ArrowFunction(_) => true,
            _ => false,
        }
    }

    pub fn is_primary_expression(&self) -> bool {
        unimplemented!()
    }

    pub fn is_left_hand_side_expression(&self) -> bool {
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

    pub fn precedence(&self) -> i8 {
        // https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#Table
        match *self {
            Expression::This(inner) => -1,
            Expression::Spread(inner) => 1,
            Expression::Super(inner) => -1,

            // BindingElement(inner) => inner.span,
            // BindingProperty(inner) => inner.span,
            
            Expression::Identifier(inner) => -1,
            Expression::Null(inner) => -1,
            Expression::Boolean(inner) => -1,
            Expression::String(inner) => -1,
            Expression::Numeric(inner) => -1,
            Expression::RegularExpression(inner) => -1,
            Expression::Template(inner) => -1,

            // ArrayLiteral(inner) => inner.span,
            // ObjectLiteral(inner) => inner.span,
            Expression::Function(inner) => -1,
            Expression::ArrowFunction(inner) => -1,
            Expression::Class(inner) => -1,
            Expression::Parenthesized(inner) => 20,

            Expression::Member(inner) => 19,

            Expression::TaggedTemplate(inner) => 19,
            
            Expression::NewTarget(inner) => -1,
            Expression::Call(inner) => 19,
            Expression::New(inner) => if inner.arguments.is_some() { 19 } else { 18 },

            Expression::Prefix(inner) => 16,
            Expression::Infix(inner) => inner.operator.precedence(),
            Expression::Postfix(inner) => 17,
            Expression::Assignment(inner) => 3,

            // ... ? ... : ...
            Expression::Conditional(inner) => 4,
            Expression::Yield(inner) => 2,
            
            Expression::Comma(inner) => 0,

            Expression::ObjectBindingPattern(inner) => -1,
            Expression::ArrayBindingPattern(inner) => -1,

            Expression::JSXFragment(inner) => -1,
            Expression::JSXElement(inner) => -1,
        }
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
    pub items: &'ast [ Expression<'ast> ],
}

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
    pub arguments: ParenthesizedExpression<'ast>,
}


// new abc( ... )
// new abc.asd( ... )
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NewExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub callee: Expression<'ast>,
    // NOTE: 支持无参数
    pub arguments: Option<ParenthesizedExpression<'ast>>,
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

#[derive(PartialEq, Clone, Copy)]
pub struct InfixExpression<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub left: Expression<'ast>,
    pub operator: InfixOperator,
    pub right: Expression<'ast>,
}

impl<'ast> fmt::Debug for InfixExpression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InfixExpression")
            .field("left", &self.left)
            .field("operator", &self.operator)
            .field("right", &self.right)
            .finish()
    }
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
    pub star: bool,                   // *
    pub item: Expression<'ast>,       // AssignmentExpression
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


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BindingElement<'ast> {
    pub loc: Loc,
    pub span: Span,
    // Identifier           Option<Initializer>
    // ArrayBindingPattern  Initializer
    // ObjectBindingPattern Initializer
    pub key: Expression<'ast>,
    pub initializer: Option<Expression<'ast>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ArrayBindingPattern<'ast> {
    pub loc: Loc,
    pub span: Span,
    pub elems: &'ast [ Option<BindingElement<'ast>> ],
    pub rest_elem: Option<SpreadExpression<'ast>>,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BindingProperty<'ast> {
    pub loc: Loc,
    pub span: Span,
    // Identifier           Option<Initializer>
    // ArrayBindingPattern  Initializer
    // ObjectBindingPattern Initializer
    pub key: Expression<'ast>,
    pub initializer: Option<Expression<'ast>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ObjectBindingPattern<'ast> {
    pub loc: Loc,
    pub span: Span,
    // BindingProperty, BindingProperty, Option<SpreadExpression>
    pub properties: &'ast [ BindingProperty<'ast> ],
    pub rest_property: Option<SpreadExpression<'ast>>,  // BindingIdentifier
}
