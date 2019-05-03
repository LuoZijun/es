use crate::toolshed::{ Arena, };

use crate::version::ECMAScriptVersion;
use crate::error::{ ErrorKind, Error, };

use crate::lexer::Lexer;
use crate::lexer::span::{ Loc, Span, LineColumn, };
use crate::lexer::token::{ Token, Punctuator, Identifier, LiteralString, LiteralNumeric, };
use crate::lexer::operator::{ PrefixOperator, InfixOperator, PostfixOperator, AssignmentOperator, };
use crate::lexer::punctuator::PunctuatorKind;
use crate::lexer::keyword::KeywordKind;

use crate::lexer::LexerErrorKind;

use crate::parser::parser::Parser;
use crate::parser::parser::ParserErrorKind::{ self, * };

use crate::ast::numberic::{ Numberic, Float, };
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
use crate::ast::pattern::{
    PropertyName, PropertyDefinition,
    BindingPattern, ObjectBindingPattern, ArrayBindingPattern, BindingRestElement, BindingElement, BindingProperty,
    AssignmentPattern, ObjectAssignmentPattern, ArrayAssignmentPattern, AssignmentElement, AssignmentProperty,
    ObjectLiteral, ArrayLiteral, 
};



impl<'ast> Parser<'ast> {
    pub fn parse_object(&mut self, token: Token<'ast>) -> Result<Expression<'ast>, Error> {
        // ObjectLiteral
        // ObjectBindingPattern
        // ObjectAssignmentPattern
        unimplemented!()
    }

    pub fn parse_array(&mut self, token: Token<'ast>) -> Result<Expression<'ast>, Error> {
        // ArrayLiteral
        // ArrayBindingPattern
        // ArrayAssignmentPattern
        unimplemented!()
    }
}