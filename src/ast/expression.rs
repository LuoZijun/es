use ast::IdentifierName;
// use super::class::ClassExpression;
// use super::function::{ FunctionExpression, ArrowFunctionExpression, UniqueFormalParameters, };

use ast::IdentifierReference;
use ast::span::Span;
use ast::float::{ Float,  };
// use ast::jsx::{ JSXFragment, JSXElement, };

use std::any::Any;
use std::convert::TryFrom;
use std::convert::TryInto;


pub type SpannedExpression = Span<Expression>;


#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    This,
    Identifier(Box<IdentifierReference>),
    NullLiteral,
    BooleanLiteral(bool),
    StringLiteral(Box<StringLiteral>),
    NumericLiteral(Float),
    RegularExpressionLiteral(Box<RegularExpressionLiteral>),
    TemplateLiteral(Box<TemplateLiteral>),
    ArrayLiteral(Box<ArrayLiteral>),
    ObjectLiteral(Box<ObjectLiteral>),
    Parenthesized(Box<ParenthesizedExpression>),

    Member(Box<MemberExpression>),
    SuperMember(Box<SuperMemberExpression>),
    TaggedTemplate(Box<TaggedTemplateExpression>),
    /// This is the `new.target` expression that was introduced in ES2015. 
    /// This tells you if the function was called with the new operator.
    NewTarget,
    Call(Box<CallExpression>),
    SuperCall(Box<SuperCallExpression>),
    New(Box<NewExpression>),


    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),
    Postfix(Box<PostfixExpression>),

    Conditional(Box<ConditionalExpression>),
    Yield(Box<YieldExpression>),
    
    Assignment(Box<AssignmentExpression>),
    Comma(Box<CommaExpression>),

    // 
    ObjectBindingPattern(Box<ObjectBindingPattern>),
    ArrayBindingPattern(Box<ArrayBindingPattern>),
    
    // JSX
    // JSXFragment(JSXFragment),
    // JSXElement(JSXElement),
}

impl Expression {
    pub fn is_primitive_literal(&self) -> bool {
        use self::Expression::*;

        match *self {
            NullLiteral
            | BooleanLiteral(_)
            | StringLiteral(_)
            | NumericLiteral(_)
            // | RegularExpressionLiteral(_) 
            // | TemplateLiteral(_)
            // | ArrayLiteral(_)
            // | ObjectLiteral(_)
            => true,
            _ => false,
        }
    }

    pub fn is_numeric_literal(&self) -> bool {
        use self::Expression::*;

        match *self {
            NumericLiteral(_) => true,
            _ => false,
        }
    }

    pub fn is_primary_expression(&self) -> bool {
        use self::Expression::*;

        match *self {
            | This
            | Identifier(_)
            | NullLiteral
            | BooleanLiteral(_)
            | StringLiteral(_)
            | NumericLiteral(_)
            | RegularExpressionLiteral(_)
            | TemplateLiteral(_)
            | ArrayLiteral(_)
            | ObjectLiteral(_)
            | Parenthesized(_) => true,
            _ => false,
        }
    }

    pub fn is_left_hand_side_expression(&self) -> bool {
        use self::Expression::*;

        match *self {
            Member(_) | SuperMember(_)
            | Call(_) | SuperCall(_)
            | NewTarget | New(_)
            | TaggedTemplate(_) => true,
            _ => false,
        }
    }

    pub fn is_member_expression(&self) -> bool {
        use self::Expression::*;

        match *self {
            Member(_) | SuperMember(_)
            | Call(_) | SuperCall(_)
            | NewTarget | New(_)
            | TaggedTemplate(_) => true,
            _ => false,
        }
    }

    pub fn is_call_expression(&self) -> bool {
        use self::Expression::*;

        match *self {
            Call(_) | SuperCall(_) => true,
            _ => false,
        }
    }

    pub fn is_assignment_expression(&self) -> bool {
        // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-AssignmentExpression
        use self::Expression::*;

        match *self {
            Conditional(_)
            | Yield(_)
            // AssignmentOperator
            => true,

            _ => false,
        }
    }

    pub fn precedence(&self) -> u8 {
        // https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#Table
        use self::Expression::*;

        match *self {
            Parenthesized(_) => 20,
            Member(_) 
            | SuperMember(_)
            | TaggedTemplate(_)
            | NewTarget
            | Call(_)
            | SuperCall(_) => 19,
            New(ref expr) => {
                if expr.arguments.is_some() {
                    19
                } else {
                    18
                }
            },
            Postfix(_) => 17,
            Prefix(_) => 16,
            Infix(ref expr) => expr.operator.precedence(),
            Conditional(_) => 4,
            Assignment(_) => 3,
            Yield(_) => 2,
            // Spread => 1,
            Comma(_) => 0,

            This | Identifier(_) | NullLiteral | BooleanLiteral(_) 
            | StringLiteral(_) | NumericLiteral(_) | RegularExpressionLiteral(_)
            | TemplateLiteral(_)
            | ArrayLiteral(_) | ObjectLiteral(_) => 21,
            ObjectBindingPattern(_) | ArrayBindingPattern(_) => 21,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub raw: Vec<char>,
    pub cooked: Vec<char>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegularExpressionLiteral {
    pub body: Vec<char>,          // raw string
    pub flags: Option<Vec<char>>, // g/i/m/u/y/s
}

#[derive(Debug, PartialEq, Clone)]
pub struct TemplateLiteral {
    pub strings: Vec<StringLiteral>,
    pub bounds: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayLiteral {
    // TODO:
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectLiteral {
    // TODO:
}

// ( Expression, + )
#[derive(Debug, PartialEq, Clone)]
pub struct ParenthesizedExpression {
    pub elems: Vec<Expression>,
}

// FunctionExpression
// ClassExpression
// GeneratorExpression
// AsyncFunctionExpression
// AsyncGeneratorExpression



// Left-Hand-Side Expressions
#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub computed: bool,
}

// super . IdentifierName
// super [ Expression ]
#[derive(Debug, PartialEq, Clone)]
pub struct SuperMemberExpression {
    pub right: Box<Expression>,
    pub computed: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TaggedTemplateExpression {
    pub tag: Box<Expression>,
    pub template: TemplateLiteral,
}



// https://www.ecma-international.org/ecma-262/9.0/index.html#prod-CallExpression
#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    // TODO:
    pub arguments: Vec<char>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SuperCallExpression {
    // TODO:
    pub arguments: Vec<char>,
}


// new abc( ... )
// new abc.asd( ... )
#[derive(Debug, PartialEq, Clone)]
pub struct NewExpression {
    pub callee: Box<Expression>,
    // TODO:
    pub arguments: Option<Vec<char>>,
}





#[derive(Debug, PartialEq, Clone)]
pub enum PrefixOperator {
    Await,     // await
    Delete,    // delete
    Void,      // void
    TypeOf,    // typeof
    Add,       // +
    Sub,       // -
    BitNot,    // ~
    Not,       // !

    Increment, // ++
    Decrement, // --
}

#[derive(Debug, PartialEq, Clone)]
pub enum InfixOperator {
    Pow,
    Mul,
    Div,
    Rem,
    Add,
    Sub,

    BitShl,
    BitShr,
    BitUShr,

    Gt,
    Lt,
    GtEq,
    LtEq,

    Eq,
    Neq,
    StrictEq,
    StrictNeq,

    BitAnd,
    BitXor,
    BitOr,

    And,
    Or,

    InstanceOf,
    In,
}

impl InfixOperator {
    pub fn precedence(&self) -> u8 {
        use self::InfixOperator::*;

        match *self {
            Pow => 15,
            Mul | Div | Rem => 14,
            Add | Sub => 13,


            BitShl | BitShr | BitUShr => 12,

            Gt | Lt | GtEq | LtEq | In | InstanceOf => 11,

            Eq | Neq | StrictEq | StrictNeq => 10,

            BitAnd => 9,
            BitXor => 8,
            BitOr => 7,

            And => 6,
            Or => 5,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOperator {
    Increment, // ++
    Decrement, // --
}


// https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#Table
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Prefix(PrefixOperator),
    Infix(InfixOperator),
    Postfix(PostfixOperator),
}


// Unary
#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpression {
    pub operator: PrefixOperator,
    pub operand: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: InfixOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PostfixExpression {
    pub operator: PostfixOperator,
    pub operand: Box<Expression>,
}


// https://www.ecma-international.org/ecma-262/9.0/index.html#prod-ConditionalExpression
// LogicalORExpression ? AssignmentExpression : AssignmentExpression 
#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalExpression {
    pub condition: Box<Expression>,
    pub and_then: Box<Expression>,
    pub or_else: Box<Expression>,
}



// https://www.ecma-international.org/ecma-262/9.0/index.html#prod-YieldExpression
// yield AssignmentExpression
// yield *AssignmentExpression
#[derive(Debug, PartialEq, Clone)]
pub struct YieldExpression {
    // no LineTerminator here
    pub star: bool,                  // *
    pub value: Box<Expression>, // AssignmentExpression
                                     // * AssignmentExpression
}


#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentOperator {
    Assign,         // =
    AddAssign,      // +=
    SubAssign,      // -=
    MulAssign,      // *=
    DivAssign,      // /=
    RemAssign,      // %=
    PowAssign,      // **=

    BitAndAssign,   // &=
    BitOrAssign,    // |=
    BitXorAssign,   // ^=
    BitShlAssign,   // <<=
    BitShrAssign,   // >>=
    BitUShrAssign,  // >>>=
}


// https://www.ecma-international.org/ecma-262/9.0/index.html#prod-AssignmentExpression
#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpression {
    pub left: Box<Expression>,
    pub operator: AssignmentOperator,
    pub right: Box<Expression>,
}

// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-comma-operator
// Expression, +
#[derive(Debug, PartialEq, Clone)]
pub struct CommaExpression {
    // TODO:
    pub elems: Vec<Expression>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct BindingElement {
    // Identifier           Option<Initializer>
    // ArrayBindingPattern  Initializer
    // ObjectBindingPattern Initializer
    pub key: Box<Expression>,
    pub initializer: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayBindingPattern {
    pub elems: Vec<Option<BindingElement>>,
    pub rest_elem: Option<Box<Expression>>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct BindingProperty {
    // Identifier           Option<Initializer>
    // ArrayBindingPattern  Initializer
    // ObjectBindingPattern Initializer
    pub key: Box<Expression>,
    pub initializer: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBindingPattern {
    pub properties: Vec<BindingProperty>,
    pub rest_property: Option<Box<Expression>>,  // BindingIdentifier
}
