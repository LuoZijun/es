use error::{ ErrorKind, Error, };
use parser::parser::Parser;

use lexer::{ ESChar, Lexer, Token, SpannedToken, Punctuator, Keyword, };
use ast::span::{ LineColumn, Span, };
use ast::float::Float;
use ast::statement::{ 
    SpannedStatement, Statement, StatementList,
    VariableStatement, LexicalDeclaration, LexicalDeclarationKind, LexicalBinding,

};
use ast::expression::{
    SpannedExpression, Expression,
    ObjectBindingPattern, ArrayBindingPattern,
};



impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self, token: SpannedToken) -> Result<SpannedExpression, Error> {
        unimplemented!()
    }

    pub fn parse_literal(&mut self) -> Result<SpannedExpression, Error> {
        // null/true/false
        // string
        // number
        // template
        // regex exp
        match self.lexer.token {
            Token::LiteralNull => {
                unimplemented!()
            },
            Token::LiteralBoolean(ref val) => {
                unimplemented!()
            },
            Token::LiteralString(ref val) => {
                unimplemented!()
            },
            Token::LiteralDecimalNumeric(ref val) => {
                unimplemented!()
            },
            Token::LiteralFloatNumeric(ref val) => {
                unimplemented!()
            },
            Token::Punctuator(Punctuator::BackTick) => {
                unimplemented!()
            },
            _ => unreachable!(),
        }
    }

    fn parse_object_binding_pattern(&mut self) -> Result<ObjectBindingPattern, Error> {
        unimplemented!()
    }
}