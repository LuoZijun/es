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


fn get_token<'ast>(parser: &mut Parser<'ast>) -> Result<Token<'ast>, Error> {
    loop {
        let mut token2 = parser.token2()?;
        match token2 {
            Token::LineTerminator => continue,
            Token::Identifier(ident) => {
                match ident.to_keyword_or_literal() {
                    Some(token2) => return Ok(token2),
                    None => return Ok(token2),
                }
            },
            _ => return Ok(token2),
        }
    }
}

impl<'ast> Parser<'ast> {
    pub fn parse_class(&mut self, mut token: Token<'ast>) -> Result<Class<'ast>, Error> {
        // Class Heritage
        let mut heritage: Option<Expression<'ast>> = None;
        let mut loc: Loc = Loc::default();
        let mut span: Span = Span::default();

        match token {
            Token::Keyword(kw) => {
                match kw.kind {
                    KeywordKind::Extends => {
                        loc = kw.loc;
                        span = kw.span;

                        let token2 = get_token(self)?;
                        let expr = self.parse_expression(token2, -1)?;

                        heritage = Some(expr);
                        token = get_token(self)?;
                    },
                    _ => {

                    }
                }
            },
            _ => {

            }
        }

        // Class Body
        let mut body: Vec<ClassMethodDefinition<'ast>> = vec![];
        match token {
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::LBrace => {
                        // {
                        if heritage.is_none() {
                            loc = punct.loc;
                            span = punct.span;
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

        // class method
        loop {
            let token2 = get_token(self)?;
            match token2 {
                Token::Punctuator(punct) => {
                    match punct.kind {
                        PunctuatorKind::RBrace => {
                            // }
                            loc.end = punct.loc.end;
                            span.end = punct.span.end;
                            break;
                        },
                        _ => { }
                    }
                },
                _ => { }
            }

            let class_method = self.parse_class_element(token2)?;
            body.push(class_method);
        }
        
        let item = Class { loc, span, heritage, body: self.arena.alloc_vec(body) };

        Ok(item)
    }

    pub fn parse_class_declaration(&mut self, token: Token<'ast>) -> Result<ClassDeclaration<'ast>, Error> {
        let (mut loc, mut span) = match token {
            Token::Keyword(kw) => {
                assert_eq!(kw.kind, KeywordKind::Class);
                (kw.loc, kw.span)
            },
            _ => unreachable!(),
        };

        let name: Identifier<'ast>;

        // class name
        let mut token2 = get_token(self)?;
        match token2 {
            Token::Identifier(ident) => {
                name = ident;
            },
            _ => {
                return Err(self.unexpected_token(token2));
            }
        }

        let token2 = self.token2()?;
        let class = self.parse_class(token2)?;
        loc.end = class.loc.end;
        span.end = class.span.end;
        let item = ClassDeclaration { loc, span, name, class, };

        Ok(item)
    }

    pub fn parse_class_expression(&mut self, token: Token<'ast>) -> Result<ClassExpression<'ast>, Error> {
        let (mut loc, mut span) = match token {
            Token::Keyword(kw) => {
                assert_eq!(kw.kind, KeywordKind::Class);
                (kw.loc, kw.span)
            },
            _ => unreachable!(),
        };
        
        let mut name: Option<Identifier<'ast>> = None;
        
        // class name
        let mut token2 = get_token(self)?;
        match token2 {
            Token::Identifier(ident) => {
                name = Some(ident);
                token2 = get_token(self)?;
            },
            _ => {

            }
        }

        let class = self.parse_class(token2)?;
        loc.end = class.loc.end;
        span.end = class.span.end;
        let item = ClassExpression { loc, span, name, class, };

        Ok(item)
    }

    pub fn parse_class_element(&mut self, mut token: Token<'ast>) -> Result<ClassMethodDefinition<'ast>, Error> {
        // MethodDefinition
        // static MethodDefinition
        let mut is_static: bool = false;
        
        match token {
            Token::Identifier(_)
            | Token::LiteralString(_)
            | Token::LiteralNumeric(_) => {
                
            },
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::Mul
                    | PunctuatorKind::LBracket => {
                        // *
                        // ComputedPropertyName
                        // [
                    },
                    _ => unreachable!(),
                }
            },
            Token::Keyword(kw) => {
                match kw.kind {
                    KeywordKind::Async => {
                        // async
                    },
                    KeywordKind::Static => {
                        // static
                        is_static = true;
                        loop {
                            let token2 = self.token2()?;
                            match token2 {
                                Token::LineTerminator => continue,
                                _ => {
                                    token = token2;
                                    break;
                                }
                            }
                        }
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }

        let method = self.parse_method_definition(token)?;
        let item = ClassMethodDefinition { is_static, method, };
        Ok(item)
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
                        let token2 = self.token2()?;
                        let fallback = match token2 {
                            Token::Identifier(_)
                            | Token::LiteralString(_)
                            | Token::LiteralNumeric(_) => {
                                false
                            },
                            Token::Punctuator(punct) => {
                                match punct.kind {
                                    PunctuatorKind::LBracket => {
                                        // ComputedPropertyName
                                        // [
                                        false
                                    },
                                    _ => true,
                                }
                            },
                            _ => true
                        };
                        
                        if fallback {
                            is_setter = false;
                            is_getter = false;
                            self.token.push(token2);
                        } else {
                            token = token2;
                        }
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
