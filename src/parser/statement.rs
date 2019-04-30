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
    EmptyStatement, DebuggerStatement,
    BlockStatement,
    VariableStatement, LexicalDeclarationKind, LexicalBinding,
};
use crate::ast::expression::{
    Expression, LiteralTemplateExpression,
};


impl<'ast> Parser<'ast> {
    pub fn parse_statement(&mut self, token: Token<'ast>) -> Result<Statement<'ast>, Error> {
        match token {
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::Semicolon => {
                        // Empty ?
                        // self.parse_statement(token)
                        let loc =  punct.loc;
                        let span = punct.span;

                        let item = EmptyStatement { loc, span };
                        
                        return Ok(Statement::Empty(self.alloc(item)))
                    },
                    _ => {
                        unimplemented!()
                    }
                }
            },
            Token::Keyword(kw) => {
                match kw.kind {
                    KeywordKind::Async => {
                        // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-expression-statement
                        // NOTE: 由于该关键字有歧义，需要单独处理
                        // AsyncFunctionDeclaration       STMT
                        // AsyncGeneratorDeclaration      STMT
                        // AsyncArrowFunctionExpression   EXPR
                        // AsyncArrowGeneratorExpression  EXPR
                        let token2 = self.token3()?;
                        match token2 {
                            Token::LineTerminator => {
                                return Err(self.unexpected_token(token2));
                            },
                            Token::Keyword(kw) => {
                                match kw.kind {
                                    KeywordKind::Function => {
                                        let mut func_decl = self.parse_function_declaration(token2)?;
                                        func_decl.is_async = true;

                                        return Ok(Statement::Function(self.alloc(func_decl)));
                                    },
                                    _ => {
                                        return Err(self.unexpected_token(token2));
                                    }
                                }
                            },
                            _ => {
                                return Err(self.unexpected_token(token2));
                            }
                        }
                    },
                    KeywordKind::Class => {
                        let class_stmt = self.parse_class_declaration(token)?;
                        return Ok(Statement::Class(self.alloc(class_stmt)));
                    },
                    KeywordKind::Function => {
                        // FunctionDeclaration
                        // GeneratorDeclaration
                        let func_decl = self.parse_function_declaration(token)?;
                        return Ok(Statement::Function(self.alloc(func_decl)));
                    },
                    KeywordKind::Debugger => {
                        let loc =  kw.loc;
                        let span = kw.span;

                        let item = DebuggerStatement { loc, span };
                        
                        return Ok(Statement::Debugger(self.alloc(item)))
                    },
                    _ => {
                        unimplemented!()
                    }
                }
            },
            _ => {
                unimplemented!()
            }
        }
    }

    fn parse_lexical_binding(&mut self) -> Result<Vec<LexicalBinding>, Error> {
        unimplemented!()
    }

    pub fn parse_variable_statement(&mut self, token: Token<'ast>) -> Result<Statement<'ast>, Error> {
        // var/let/const
        unimplemented!()
    }

    pub fn parse_async_statement(&mut self, token: Token<'ast>) -> Result<Statement<'ast>, Error> {
        // AsyncFunctionDeclaration       STMT
        // AsyncGeneratorDeclaration      STMT
        // AsyncArrowFunctionExpression   EXPR
        // AsyncArrowGeneratorExpression  EXPR
        unimplemented!()
    }

    pub fn parse_block_statement(&mut self, token: Token<'ast>) -> Result<BlockStatement<'ast>, Error> {
        // { }
        let (mut loc, mut span) = match token {
            Token::Punctuator(punct) => {
                assert_eq!(punct.kind, PunctuatorKind::LBrace);
                (punct.loc, punct.span)
            },
            _ => unreachable!(),
        };

        let mut body: Vec<Statement<'ast>> = vec![];
        loop {
            let token = self.token4()?;
            match token {
                Token::Punctuator(punct) => {
                    match punct.kind {
                        PunctuatorKind::RBrace => {
                            // }
                            loc.end = punct.loc.end;
                            span.end = punct.span.end;
                            break;
                        },
                        _ => {

                        }
                    }
                },
                _ => {

                }
            }

            let stmt = self.process(token)?;
            body.push(stmt);
        }
        
        let item = BlockStatement { loc, span, body: self.arena.alloc_vec(body), };
        Ok(item)
    }
}
