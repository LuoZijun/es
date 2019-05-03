use crate::toolshed::{ Arena, };

use crate::version::ECMAScriptVersion;
use crate::error::{ ErrorKind, Error, };

use crate::lexer::Lexer;
use crate::lexer::span::{ Loc, Span, LineColumn, };
use crate::lexer::token::{ Token, Identifier, LiteralString, LiteralRegularExpression, LiteralTemplate, };
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
use crate::ast::function::{
    FunctionDeclaration, FunctionExpression, Function, 
    ArrowFunctionExpression, ConciseBody, FunctionBody, 
};


impl<'ast> Parser<'ast> {
    pub fn parse_function(&mut self, token: Token<'ast>) -> Result<Function<'ast>, Error> {
        let (mut loc, mut span) = match token {
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::LParen => {
                        // (
                        (punct.loc, punct.span)
                    },
                    _ => {
                        return Err(self.unexpected_token(token));
                    }
                }
            },
            _ => {
                return Err(self.unexpected_token(token));
            }
        };
        
        let params = match self.parse_expression(token, 20i8)? {
            Expression::Parenthesized(inner) => inner.to_owned(),
            _ => unreachable!(),
        };
        
        let token2 = self.token4()?;
        match token2 {
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::LBrace => {
                        // {
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

        let block = self.parse_block_statement(token2)?;

        loc.end = block.loc.end;
        span.end = block.span.end;

        let body = block.body;

        let item = Function { loc, span, params, body, };

        Ok(item)
    }

    pub fn parse_function_declaration(&mut self, token: Token<'ast>) -> Result<FunctionDeclaration<'ast>, Error> {
        let (mut loc, mut span) = match token {
            Token::Keyword(kw) => {
                assert_eq!(kw.kind, KeywordKind::Function);
                (kw.loc, kw.span)
            },
            _ => unreachable!(),
        };

        let is_async = false;
        let mut is_generator: bool = false;
        let name: Identifier<'ast>;
        
        let mut token2 = self.token4()?;
        match token2 {
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::Mul => {
                        is_generator = true;
                        token2 = self.token4()?;
                    },
                    _ => { },
                }
            },
            _ => { }
        }

        match token2 {
            Token::Identifier(ident) => {
                name = ident;
                token2 = self.token4()?;
            },
            _ => {
                return Err(self.unexpected_token(token2));
            }
        }
        
        let func = self.parse_function(token2)?;
        loc.end = func.loc.end;
        span.end = func.span.end;

        let item = FunctionDeclaration { loc, span, is_async, is_generator, name, func, };
        
        Ok(item)
    }

    pub fn parse_function_expression(&mut self, token: Token<'ast>) -> Result<FunctionExpression<'ast>, Error> {
        // AsyncFunctionDeclaration       EXPR
        // AsyncGeneratorDeclaration      EXPR
        let (mut loc, mut span) = match token {
            Token::Keyword(kw) => {
                assert_eq!(kw.kind, KeywordKind::Function);
                (kw.loc, kw.span)
            },
            _ => unreachable!(),
        };

        let is_async = false;
        let mut is_generator: bool = false;
        let mut name: Option<Identifier<'ast>> = None;
        
        let mut token2 = self.token4()?;
        match token2 {
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::Mul => {
                        is_generator = true;
                        token2 = self.token4()?;
                    },
                    _ => { },
                }
            },
            _ => { }
        }

        match token2 {
            Token::Identifier(ident) => {
                name = Some(ident);
                token2 = self.token4()?;
            },
            _ => { }
        }
        
        let func = self.parse_function(token2)?;
        loc.end = func.loc.end;
        span.end = func.span.end;

        let item = FunctionExpression { loc, span, is_async, is_generator, name, func, };
        
        Ok(item)
    }

    pub fn parse_async_expression(&mut self, token: Token<'ast>) -> Result<Expression<'ast>, Error> {
        // AsyncFunctionDeclaration       EXPR
        // AsyncGeneratorDeclaration      EXPR
        // AsyncArrowFunctionExpression   EXPR
        let (mut loc, mut span) = match token {
            Token::Keyword(kw) => {
                assert_eq!(kw.kind, KeywordKind::Async);
                (kw.loc, kw.span)
            },
            _ => unreachable!(),
        };

        let token2 = self.token2()?;
        match token2 {
            Token::LineTerminator => {
                return Err(self.unexpected_token(token2));
            },
            _ => { }
        }

        let expr = self.parse_expression(token2, -1i8)?;

        match expr {
            Expression::Function(function) => {
                let mut f = function.to_owned();
                f.loc.start = loc.start;
                f.span.start = span.start;
                f.is_async = true;
                Ok(Expression::Function(self.alloc(f)))
            },
            Expression::ArrowFunction(arrow_function) => {
                let mut f = arrow_function.to_owned();
                f.loc.start = loc.start;
                f.span.start = span.start;
                f.is_async = true;
                Ok(Expression::ArrowFunction(self.alloc(f)))
            },
            _ => {
                return Err(self.unexpected_token(token2));
            }
        }
    }

    pub fn parse_arrow_function_expression(&mut self, params: Expression<'ast>) -> Result<Expression<'ast>, Error> {
        let mut loc = params.loc();
        let mut span = params.span();
        
        let token = self.token2()?;
        match token {
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::LBrace => {
                        // {
                        let block = self.parse_block_statement(token)?;
                        
                        loc.end = block.loc.end;
                        span.end = block.span.end;

                        let is_async = false;
                        let body = ConciseBody::Stmt(block.body);

                        let item = ArrowFunctionExpression { loc, span, is_async, params, body, };
                        return Ok(Expression::ArrowFunction(self.alloc(item)));
                    },
                    _ => {

                    }
                }
            },
            _ => {

            }
        }

        // FIXME: 或许需要把优先级设定为 0 ？这样 `逗号表达式` 将不会被允许作为 函数的 Body.
        let precedence = 0; // -1 or 0
        let expr = self.parse_expression(token, precedence)?;
        loc.end = expr.loc().end;
        span.end = expr.span().end;

        let is_async = false;
        let body = ConciseBody::Expr(expr);
        
        let item = ArrowFunctionExpression { loc, span, is_async, params, body, };
        
        Ok(Expression::ArrowFunction(self.alloc(item)))
    }

}