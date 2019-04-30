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
    Statement, StatementList,
    VariableStatement, LexicalDeclarationKind, LexicalBinding,
    BlockStatement,
};
use crate::ast::expression::{
    Expression, LiteralTemplateExpression,
    PrefixExpression, InfixExpression, PostfixExpression, AssignmentExpression,
    MemberExpression, NewTargetExpression, NewExpression,
    ConditionalExpression, YieldExpression, CommaExpression,
    TaggedTemplateExpression, SpreadExpression, ParenthesizedExpression,
    CallExpression, 
};
use crate::ast::function::{ FunctionExpression, Function, ArrowFunctionExpression, ConciseBody, FunctionBody, };
use crate::ast::class::{
    ClassDeclaration, ClassExpression, Class, ClassMethodDefinition, 
    MethodDefinition, Method, Getter, Setter,
};

const GET: &'static [char] = &['g', 'e', 't'];
const SET: &'static [char] = &['s', 'e', 't'];


impl<'ast> Parser<'ast> {
    pub fn parse_class(&mut self, token: Token<'ast>) -> Result<Expression<'ast>, Error> {
        unimplemented!()
    }

    pub fn parse_method_definition(&mut self, mut token: Token<'ast>) -> Result<MethodDefinition<'ast>, Error> {
        // Method
        // Getter
        // Setter
        // Start Token:
        //      Identifier
        //      Identifier('get')
        //      Identifier('set')
        //      PunctuatorKind::Mul  *
        //      KeywordKind::Async

        // PropertyName:
        //      IdentifierName
        //      StringLiteral
        //      NumericLiteral
        //      ComputedPropertyName

        let mut is_async = false;
        let mut is_generator: bool = false;
        let mut is_getter: bool = false;
        let mut is_setter: bool = false;
        let mut property_name: Expression<'ast>;
        let mut loc: Loc;
        let mut span: Span;

        match token {
            Token::Identifier(ident) => {
                loc = ident.loc;
                span = ident.span;
            },
            Token::LiteralString(lit_str) => {
                loc = lit_str.loc;
                span = lit_str.span;
            },
            Token::LiteralNumeric(lit_num) => {
                loc = lit_num.loc;
                span = lit_num.span;
            },
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::Mul => {
                        // *
                        is_generator = true;
                        loc = punct.loc;
                        span = punct.span;

                        token = self.token2()?;
                    },
                    PunctuatorKind::LBracket => {
                        // ComputedPropertyName
                        // [
                        loc = punct.loc;
                        span = punct.span;
                    },
                    _ => unreachable!(),
                }
            },
            Token::Keyword(kw) => {
                match kw.kind {
                    KeywordKind::Async => {
                        // async
                        is_async = true;
                        loc = kw.loc;
                        span = kw.span;

                        let token2 = self.token2()?;
                        match token2 {
                            Token::Identifier(_) 
                            | Token::LiteralString(_)
                            | Token::LiteralNumeric(_) => {
                                token = token2;
                            },
                            Token::Punctuator(punct) => {
                                match punct.kind {
                                    PunctuatorKind::Mul => {
                                        // *
                                        is_generator = true;
                                        token = self.token2()?;
                                    },
                                    PunctuatorKind::LBracket => {
                                        // ComputedPropertyName
                                        // [
                                        loc = punct.loc;
                                        span = punct.span;
                                        token = token2;
                                    },
                                    _ => return Err(self.unexpected_token(token2)),
                                }
                            },
                            _ => {
                                return Err(self.unexpected_token(token2));
                            }
                        }
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
        
        // Getter || Setter
        if !is_generator && !is_async {
            match token {
                Token::Identifier(ident) => {
                    if let Some(_) = ident.to_keyword_or_literal() {
                        return Err(self.unexpected_token(token));
                    }
                    
                    is_setter = ident.raw == SET;
                    is_getter = ident.raw == GET;

                    if is_setter || is_getter {
                        token = self.token2()?;
                    }
                },
                _ => { }
            }
        }

        // PropertyName
        match token {
            Token::Identifier(ident) => {
                if let Some(_) = ident.to_keyword_or_literal() {
                    return Err(self.unexpected_token(token));
                }

                property_name = Expression::Identifier(self.alloc(ident));
            },
            Token::LiteralString(lit_str) => {
                property_name = Expression::String(self.alloc(lit_str));
            },
            Token::LiteralNumeric(lit_num) => {
                property_name = Expression::Numeric(self.alloc(lit_num));
            },
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::LBracket => {
                        // ComputedPropertyName
                        // [
                        property_name = self.parse_expression(token, -1)?;
                        
                        loop {
                            let token3 = self.token2()?;
                            match token3 {
                                Token::LineTerminator => continue,
                                Token::Punctuator(punct) => {
                                    match punct.kind {
                                        PunctuatorKind::RBracket => {
                                            // ]
                                            break;
                                        },
                                        _ => {
                                            return Err(self.unexpected_token(token3));
                                        }
                                    }
                                },
                                _ => {
                                    return Err(self.unexpected_token(token3));
                                }
                            }
                        }
                    },
                    _ => {
                        return Err(self.unexpected_token(token));
                    }
                }
            },
            _ => {
                return Err(self.unexpected_token(token));
            }
        }

        if is_generator || is_async {
            assert_eq!(!is_getter, true);
            assert_eq!(!is_setter, true);
        }
        
        let parse_function_body = |parser: &mut Parser<'ast>| -> Result<BlockStatement<'ast>, Error> {
            let block;
            loop {
                let token3 = parser.token2()?;
                match token3 {
                    Token::LineTerminator => continue,
                    Token::Punctuator(punct) => {
                        match punct.kind {
                            PunctuatorKind::LBrace => {
                                // {
                                block = parser.parse_block_statement(token3)?;
                                break;
                            },
                            _ => {
                                return Err(parser.unexpected_token(token3));
                            }
                        }
                    },
                    _ => {
                        return Err(parser.unexpected_token(token3));
                    }
                }
            };

            Ok(block)
        };

        if !is_getter && !is_setter {
            // Method
            let token2 = self.token2()?;
            let params = match self.parse_expression(token2, -1)? {
                Expression::Parenthesized(inner) => inner.to_owned(),
                _ => return Err(self.unexpected_token(token2)),
            };
            
            let block = parse_function_body(self)?;

            loc.end = block.loc.end;
            span.end = block.span.end;
            let name = property_name;
            let body = block.body;

            let item = Method { loc, span, is_async, is_generator, name, params, body, };
            
            return Ok(MethodDefinition::Method(item));
        }

        if is_getter {
            // Getter
            let block = parse_function_body(self)?;
            
            loc.end = block.loc.end;
            span.end = block.span.end;
            let name = property_name;
            let body = block.body;

            let item = Getter { loc, span, name, body, };
            
            return Ok(MethodDefinition::Getter(item));
        }

        if is_setter {
            // Setter
            let token2 = self.token2()?;
            let params = match self.parse_expression(token2, -1)? {
                Expression::Parenthesized(inner) => inner.to_owned(),
                _ => return Err(self.unexpected_token(token2)),
            };

            let block = parse_function_body(self)?;
            
            loc.end = block.loc.end;
            span.end = block.span.end;
            let name = property_name;
            let body = block.body;

            let item = Setter { loc, span, name, params, body, };
            
            return Ok(MethodDefinition::Setter(item));
        }

        unreachable!()
    }
}
