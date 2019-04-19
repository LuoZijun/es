use crate::toolshed::{ Arena, };

use version::ECMAScriptVersion;
use error::{ ErrorKind, Error, };

use lexer::Lexer;
use lexer::token::{ Token, LiteralString, LiteralRegularExpression, LiteralTemplate, };
use lexer::punctuator::PunctuatorKind;
use lexer::keyword::KeywordKind;
use lexer::LexerErrorKind;

use parser::parser::Parser;
use parser::parser::ParserErrorKind::{ self, * };

use ast::numberic::{ Numberic, Float, };
use ast::statement::{ 
    Statement, StatementList,
    VariableStatement, LexicalDeclarationKind, LexicalBinding,
};
use ast::expression::{
    Expression, LiteralTemplateExpression,
};


impl<'ast> Parser<'ast> {
    pub fn parse_statement(&mut self, token: Token<'ast>) -> Result<Statement<'ast>, Error> {
        unimplemented!()
    }

    fn parse_lexical_binding(&mut self) -> Result<Vec<LexicalBinding>, Error> {
        unimplemented!()
    }

    pub fn parse_variable_statement(&mut self, token: Token<'ast>) -> Result<Statement<'ast>, Error> {
        // var/let/const
        unimplemented!()
    }
}
