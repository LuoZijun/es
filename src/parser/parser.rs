use crate::toolshed::{ Arena, };

use version::ECMAScriptVersion;
use error::{ ErrorKind, Error, };

use lexer::Lexer;
use lexer::token::Token;
use lexer::punctuator::PunctuatorKind;
use lexer::keyword::KeywordKind;
use lexer::LexerErrorKind;

use ast::numberic::{ Numberic, Float, };
use ast::statement::{ 
    Statement, StatementList,
    VariableStatement, LexicalDeclarationKind, LexicalBinding,
};
use ast::expression::{
    Expression, LiteralTemplateExpression,
};

use self::ParserErrorKind::*;


#[derive(Debug)]
pub enum ParserErrorKind {
    UnexpectedToken,
    UnexpectedEOF,
    Custom(&'static str),
}

pub struct Parser<'ast> {
    pub(crate) arena: &'ast Arena,
    pub(crate) lexer: Lexer<'ast>,
    
    pub token: Vec<Token<'ast>>,

    pub body: Vec<Statement<'ast>>,
    pub tokens: Vec<Token<'ast>>,
}

impl<'ast> Parser<'ast> {
    pub fn new(arena: &'ast Arena, source: &'ast [char], filename: &'ast str) -> Self {
        let lexer = Lexer::new(arena, source, filename);

        Self { arena, lexer, body: vec![], token: Vec::with_capacity(1), tokens: vec![] }
    }
    
    pub fn error(&mut self) -> Error {
        // TODO
        unimplemented!()
    }

    pub fn error_line(&mut self) -> String {
        // TODO
        unimplemented!()
    }

    pub fn unexpected_token(&mut self, token: Token<'ast>) -> Error {
        // TODO
        debug!("{:?}", token);
        self.lexer.error(LexerErrorKind::Custom("Unexpected Token"))
    }

    pub fn token(&mut self) -> Result<Option<Token<'ast>>, Error> {
        if self.token.len() > 0 {
            Ok(self.token.pop())
        } else {
            self.lexer.consume()
        }
    }

    pub fn token2(&mut self) -> Result<Token<'ast>, Error> {
        // NOTE: 不允许 EOF 的出现
        match self.token() {
            Ok(Some(token)) => Ok(token),
            Ok(None) => Err(self.lexer.error(LexerErrorKind::UnexpectedEOF)),
            Err(e) => Err(e),
        }
    }

    pub fn alloc<T: Copy>(&mut self, item: T) -> &'ast T {
        self.arena.alloc(item)
    }

    pub fn process(&mut self) -> Result<Statement<'ast>, Error> {
        unimplemented!()
    }
    
    pub fn parse(&mut self) -> Result<(), Error> {
        loop {
            match self.token()? {
                None        => break,
                Some(token) => {
                    match token {
                        Token::LineTerminator => continue,

                        Token::Keyword(_)
                        | Token::LiteralNull(_)
                        | Token::LiteralBoolean(_) => unreachable!(),

                        Token::LiteralRegularExpression(_)
                        | Token::LiteralTemplate(_) => unreachable!(),

                        Token::Identifier(ident) => {
                            match ident.to_keyword_or_literal() {
                                Some(token) => {
                                    match token {
                                        Token::Keyword(kw) => {
                                            match kw.kind {
                                                KeywordKind::Var
                                                | KeywordKind::Let
                                                | KeywordKind::Const
                                                | KeywordKind::Function
                                                | KeywordKind::Class
                                                | KeywordKind::Debugger => {
                                                    // self.parse_statement(token)?;
                                                    unimplemented!()
                                                },
                                                KeywordKind::This
                                                | KeywordKind::Super
                                                | KeywordKind::Delete
                                                | KeywordKind::Void
                                                | KeywordKind::TypeOf => {
                                                    let expr = self.parse_expression(token)?;
                                                    let stmt = Statement::Expression(self.alloc(expr));
                                                    self.body.push(stmt);
                                                },
                                                _ => {
                                                    unimplemented!()
                                                }
                                            }
                                        },
                                        Token::LiteralNull(_)
                                        | Token::LiteralBoolean(_) => {
                                            let expr = self.parse_expression(token)?;
                                            let stmt = Statement::Expression(self.alloc(expr));
                                            self.body.push(stmt);
                                        },
                                        _ => unreachable!(),
                                    }
                                },
                                None => {
                                    // ident
                                    let token = Token::Identifier(ident);
                                    let expr = self.parse_expression(token)?;
                                    let stmt = Statement::Expression(self.alloc(expr));
                                    self.body.push(stmt);
                                }
                            }
                        },
                        Token::Punctuator(punct) => {
                            match punct.kind {
                                PunctuatorKind::Semicolon => {
                                    // self.parse_statement(token)?;
                                    unimplemented!()
                                },
                                PunctuatorKind::Div
                                | PunctuatorKind::Add
                                | PunctuatorKind::Sub
                                | PunctuatorKind::Increment
                                | PunctuatorKind::Decrement
                                | PunctuatorKind::Not
                                | PunctuatorKind::BitNot => {
                                    // unary operator
                                    let expr = self.parse_expression(token)?;
                                    let stmt = Statement::Expression(self.alloc(expr));
                                    self.body.push(stmt);
                                },
                                _ => {
                                    return Err(self.unexpected_token(token));
                                }
                            }
                        },
                        Token::LiteralString(_)
                        | Token::LiteralNumeric(_)
                        | Token::TemplateOpenning => {
                            let expr = self.parse_expression(token)?;
                            let stmt = Statement::Expression(self.alloc(expr));
                            self.body.push(stmt);
                        },
                    }
                },
            }
        }

        Ok(())
    }
}

pub fn parse(source: &str, filename: &str) {
    let arena = Arena::new();

    let code = arena.alloc_vec(source.chars().collect::<Vec<char>>());
    let filename = arena.alloc_str(filename);

    let mut parser = Parser::new(&arena, &code, &filename);
    match parser.parse() {
        Ok(_) => {
            println!("TokenList:");
            for token in parser.tokens {
                println!("    {:?}", token);
            }
            println!("\n");

            println!("StatementList:");
            for stmt in parser.body {
                println!("    {:?}", stmt);
            }
            println!("\n");

            trace!("EOF.");
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
