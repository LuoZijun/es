use crate::toolshed::{ Arena, };

use crate::version::ECMAScriptVersion;
use crate::error::{ ErrorKind, Error, };

use crate::lexer::Lexer;
use crate::lexer::token::Token;
use crate::lexer::punctuator::PunctuatorKind;
use crate::lexer::keyword::KeywordKind;
use crate::lexer::LexerErrorKind;

use crate::ast::numberic::{ Numberic, Float, };
use crate::ast::statement::{ 
    Statement, StatementList,
    VariableStatement, LexicalDeclarationKind, LexicalBinding,
};
use crate::ast::expression::{
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

    pub errors: Vec<Error>,
}

impl<'ast> Parser<'ast> {
    pub fn new(arena: &'ast Arena, source: &'ast [char], filename: &'ast str) -> Self {
        let lexer = Lexer::new(arena, source, filename);
        
        let body = vec![];
        let token = Vec::with_capacity(1);
        let tokens = vec![];
        let errors = vec![];
        
        Self { arena, lexer, body, token, tokens, errors, }
    }
    
    #[inline]
    pub fn error(&mut self) -> Error {
        // TODO
        unimplemented!()
    }
    
    #[inline]
    pub fn error_line(&mut self) -> String {
        // TODO
        unimplemented!()
    }
    
    #[inline]
    pub fn unexpected_token(&mut self, token: Token<'ast>) -> Error {
        // TODO
        debug!("{:?}", token);
        self.lexer.error(LexerErrorKind::Custom("Unexpected Token"))
    }
    
    #[inline]
    pub fn unexpected_eof(&mut self) -> Error {
        self.lexer.error(LexerErrorKind::UnexpectedEOF)
    }

    #[inline]
    pub fn token(&mut self) -> Result<Option<Token<'ast>>, Error> {
        if self.token.len() > 0 {
            Ok(self.token.pop())
        } else {
            self.lexer.consume()
        }
    }
    
    #[inline]
    pub fn token2(&mut self) -> Result<Token<'ast>, Error> {
        // NOTE: 不允许 EOF 的出现
        match self.token() {
            Ok(Some(token)) => Ok(token),
            Ok(None) => Err(self.unexpected_eof()),
            Err(e) => Err(e),
        }
    }
    
    #[inline]
    pub fn alloc<T: Copy>(&mut self, item: T) -> &'ast T {
        self.arena.alloc(item)
    }
    
    #[inline]
    fn process(&mut self, token: Token<'ast>) -> Result<Statement<'ast>, Error> {
        let expr_precedence = -1i8;

        match token {
            Token::LineTerminator => unreachable!(),
            Token::LiteralTemplate(_) => unreachable!(),
            Token::LiteralRegularExpression(_) => unreachable!(),
            
            Token::LiteralString(_)
            | Token::LiteralNumeric(_)
            | Token::LiteralNull(_)
            | Token::LiteralBoolean(_)
            | Token::TemplateOpenning => {
                let expr = self.parse_expression(token, expr_precedence)?;
                Ok(Statement::Expression(self.alloc(expr)))
            },
            Token::Identifier(ident) => {
                // NOTE: 可以是 ident 也可以是 LabelledStatement
                let expr = self.parse_expression(token, expr_precedence)?;
                Ok(Statement::Expression(self.alloc(expr)))
            },
            Token::Keyword(kw) => {
                match kw.kind {
                    KeywordKind::Async => {
                        // NOTE: 由于该关键字有歧义，需要单独处理
                        // AsyncFunctionDeclaration       STMT
                        // AsyncGeneratorDeclaration      STMT
                        // AsyncArrowFunctionExpression   EXPR
                        unimplemented!()
                    },
                    KeywordKind::Var
                    | KeywordKind::Let
                    | KeywordKind::Const
                    | KeywordKind::Import
                    | KeywordKind::Export
                    | KeywordKind::Function
                    | KeywordKind::Class
                    | KeywordKind::Debugger
                    | KeywordKind::If
                    | KeywordKind::Do
                    | KeywordKind::While
                    | KeywordKind::For
                    | KeywordKind::Continue
                    | KeywordKind::Break
                    | KeywordKind::Return
                    | KeywordKind::With
                    | KeywordKind::Switch
                    | KeywordKind::Throw
                    | KeywordKind::Try => {
                        self.parse_statement(token)
                    },
                    KeywordKind::This
                    | KeywordKind::Super
                    | KeywordKind::Delete
                    | KeywordKind::Void
                    | KeywordKind::TypeOf
                    | KeywordKind::New
                    | KeywordKind::Yield => {
                        let expr = self.parse_expression(token, expr_precedence)?;
                        Ok(Statement::Expression(self.alloc(expr)))
                    },
                    _ => {
                        if kw.kind.is_future_reserved() {
                            // NOTE: 该关键字还未赋予清晰的语义。
                            return Err(self.unexpected_token(token));
                        }

                        return Err(self.unexpected_token(token));
                    }
                }
            },
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::Semicolon => {
                        // Empty ?
                        self.parse_statement(token)
                    },
                    PunctuatorKind::Div
                    | PunctuatorKind::Add
                    | PunctuatorKind::Sub
                    | PunctuatorKind::Increment
                    | PunctuatorKind::Decrement
                    | PunctuatorKind::Not
                    | PunctuatorKind::BitNot => {
                        // literal regular expression
                        // unary operator
                        let expr = self.parse_expression(token, expr_precedence)?;
                        Ok(Statement::Expression(self.alloc(expr)))
                    },
                    PunctuatorKind::LParen => {
                        // (
                        let expr = self.parse_expression(token, 20i8)?;
                        Ok(Statement::Expression(self.alloc(expr)))
                    },
                    PunctuatorKind::LBracket => {
                        // [
                        unimplemented!()
                    },
                    PunctuatorKind::LBrace => {
                        // {
                        unimplemented!()
                    },
                    _ => {
                        return Err(self.unexpected_token(token));
                    }
                }
            },
        }
    }
    
    pub fn parse(&mut self) -> Result<(), Error> {
        loop {
            let token = match self.token()? {
                None => break,
                Some(token) => token,
            };

            match token {
                Token::LineTerminator => continue,
                Token::Identifier(ident) => {
                    let token2 = match ident.to_keyword_or_literal() {
                        Some(new_token) => new_token,
                        None => token,
                    };

                    let stmt = self.process(token2)?;
                    self.body.push(stmt);
                },
                _ => {
                    let stmt = self.process(token)?;
                    self.body.push(stmt);
                },
            }
        }

        Ok(())
    }
}

pub fn parse(source: &str, filename: &str) {
    let arena = Arena::new();
    println!("Code:\n```ecmascript\n{}\n```\n", source);

    let code = arena.alloc_vec(source.chars().collect::<Vec<char>>());
    let filename = arena.alloc_str(filename);

    let mut parser = Parser::new(&arena, &code, &filename);
    match parser.parse() {
        Ok(_) => {
            // println!("TokenList:");
            // for token in parser.tokens {
            //     println!("    {:?}", token);
            // }
            // println!("\n");

            println!("StatementList:");
            for stmt in parser.body {
                println!("    {:#?}", stmt);
            }
            println!("\n");

            trace!("EOF.");
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
