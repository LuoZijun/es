use crate::toolshed::{ Arena, };

use crate::version::ECMAScriptVersion;
use crate::error::{ ErrorKind, Error, };

use crate::lexer::Lexer;
use crate::lexer::token::{ Token, LiteralString, LiteralRegularExpression, LiteralTemplate, };
use crate::lexer::punctuator::PunctuatorKind;
use crate::lexer::keyword::KeywordKind;
use crate::lexer::LexerErrorKind;

use crate::parser::parser::Parser;
use crate::parser::parser::ParserErrorKind::{ self, * };

use crate::ast::numberic::{ Numberic, Float, };
use crate::ast::statement::{ 
    Statement, StatementList,
    VariableStatement, LexicalDeclarationKind, LexicalBinding,
};
use crate::ast::expression::{
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
