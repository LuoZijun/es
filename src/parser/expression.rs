use crate::toolshed::{ Arena, };

use crate::version::ECMAScriptVersion;
use crate::error::{ ErrorKind, Error, };

use crate::lexer::Lexer;
use crate::lexer::token::{ Token, LiteralString, LiteralRegularExpression, LiteralTemplate, };
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
};
use crate::ast::expression::{
    Expression, LiteralTemplateExpression,
    PrefixExpression, InfixExpression, PostfixExpression, AssignmentExpression,
    MemberExpression,
};

// 运算符优先级
// https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#Table


impl<'ast> Parser<'ast> {
    pub fn parse_expression(&mut self, token: Token<'ast>, precedence: u8) -> Result<Expression<'ast>, Error> {
        let mut left_expr = match token {
            Token::LiteralTemplate(_) => unreachable!(),
            Token::LiteralRegularExpression(_) => unreachable!(),

            Token::LineTerminator => {
                let token2 = self.token2()?;
                self.parse_expression(token2, precedence)?
            },
            Token::LiteralString(lit)  => Expression::String(self.arena.alloc(lit)),
            Token::LiteralNumeric(lit) => Expression::Numeric(self.arena.alloc(lit)),
            Token::LiteralNull(lit)    => Expression::Null(self.arena.alloc(lit)),
            Token::LiteralBoolean(lit) => Expression::Boolean(self.arena.alloc(lit)),
            Token::Identifier(ident)   => Expression::Identifier(self.arena.alloc(ident)),
            Token::TemplateOpenning    => {
                let item = self.parse_literal_template()?;
                Expression::Template(self.arena.alloc(item))
            },
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::Div => {
                        let item = self.parse_literal_regular_expression()?;
                        Expression::RegularExpression(self.arena.alloc(item))
                    },
                    PunctuatorKind::Add => {
                        // unary operator
                        // +
                        let mut loc = punct.loc;
                        let mut span = punct.span;

                        let precedence = 16u8;
                        let operator = PrefixOperator::Positive;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;
                        
                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = PrefixExpression { loc, span, operator, operand, };
                        Expression::Prefix(self.arena.alloc(item))
                    },
                    PunctuatorKind::Sub => {
                        // unary operator
                        // -
                        let mut loc = punct.loc;
                        let mut span = punct.span;

                        let precedence = 16u8;
                        let operator = PrefixOperator::Negative;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;
                        
                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = PrefixExpression { loc, span, operator, operand, };
                        Expression::Prefix(self.arena.alloc(item))
                    },
                    PunctuatorKind::Increment => {
                        // unary operator
                        // ++
                        let mut loc = punct.loc;
                        let mut span = punct.span;

                        let precedence = 16u8;
                        let operator = PrefixOperator::Increment;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;

                        if !operand.is_identifier() {
                            return Err(self.unexpected_token(token2));
                        }

                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = PrefixExpression { loc, span, operator, operand, };
                        Expression::Prefix(self.arena.alloc(item))
                    },
                    PunctuatorKind::Decrement => {
                        // unary operator
                        // --
                        let mut loc = punct.loc;
                        let mut span = punct.span;

                        let precedence = 16u8;
                        let operator = PrefixOperator::Decrement;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;
                        
                        if !operand.is_identifier() {
                            return Err(self.unexpected_token(token2));
                        }

                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = PrefixExpression { loc, span, operator, operand, };
                        Expression::Prefix(self.arena.alloc(item))
                    },
                    PunctuatorKind::Not => {
                        // unary operator
                        // !
                        let mut loc = punct.loc;
                        let mut span = punct.span;

                        let precedence = 16u8;
                        let operator = PrefixOperator::Not;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;
                        
                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = PrefixExpression { loc, span, operator, operand, };
                        Expression::Prefix(self.arena.alloc(item))
                    },
                    PunctuatorKind::BitNot => {
                        // unary operator
                        // ~
                        let mut loc = punct.loc;
                        let mut span = punct.span;

                        let precedence = 16u8;
                        let operator = PrefixOperator::BitNot;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;
                        
                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = PrefixExpression { loc, span, operator, operand, };
                        Expression::Prefix(self.arena.alloc(item))
                    },
                    PunctuatorKind::DotDotDot => {
                        // spread
                        // ...
                        unimplemented!()
                    },
                    _ => {
                        return Err(self.unexpected_token(token));
                    }
                }
            },
            Token::Keyword(kw) => {
                match kw.kind {
                    KeywordKind::Await => {
                        // unary operator
                        // NOTE: 只在 AsyncFunction 内部才有效
                        let mut loc = kw.loc;
                        let mut span = kw.span;

                        let precedence = 16u8;
                        let operator = PrefixOperator::Await;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;
                        
                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = PrefixExpression { loc, span, operator, operand, };
                        Expression::Prefix(self.arena.alloc(item))
                    },
                    KeywordKind::TypeOf => {
                        // unary operator
                        let mut loc = kw.loc;
                        let mut span = kw.span;

                        let precedence = 16u8;
                        let operator = PrefixOperator::TypeOf;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;
                        
                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = PrefixExpression { loc, span, operator, operand, };
                        Expression::Prefix(self.arena.alloc(item))
                    },
                    KeywordKind::Void => {
                        // unary operator
                        let mut loc = kw.loc;
                        let mut span = kw.span;

                        let precedence = 16u8;
                        let operator = PrefixOperator::Void;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;
                        
                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = PrefixExpression { loc, span, operator, operand, };
                        Expression::Prefix(self.arena.alloc(item))
                    },
                    KeywordKind::Delete => {
                        // unary operator
                        let mut loc = kw.loc;
                        let mut span = kw.span;

                        let precedence = 16u8;
                        let operator = PrefixOperator::Delete;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;
                        
                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = PrefixExpression { loc, span, operator, operand, };
                        Expression::Prefix(self.arena.alloc(item))
                    },

                    KeywordKind::This => Expression::This(self.arena.alloc(kw)),
                    KeywordKind::Super => Expression::Super(self.arena.alloc(kw)),
                    KeywordKind::Function => unimplemented!(),
                    KeywordKind::Async => {
                        // AsyncFunctionDeclaration       EXPR
                        // AsyncGeneratorDeclaration      EXPR
                        // AsyncArrowFunctionExpression   EXPR
                        // AsyncArrowGeneratorExpression  EXPR
                        unimplemented!()
                    },
                    KeywordKind::New => {
                        unimplemented!()
                    },
                    KeywordKind::Yield => {
                        // let expr = self.parse_expression(token, expr_precedence)?;
                        // Ok(Statement::Expression(self.alloc(expr)))
                        unimplemented!()
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
        };
        
        if left_expr.precedence() > precedence {
            return Ok(left_expr);
        }
        
        loop {
            let precedence = left_expr.precedence();

            let token2 = match self.token2() {
                Ok(token2) => token2,
                Err(_) => return Ok(left_expr),
            };

            match token2 {
                Token::Punctuator(punct) => {
                    match punct.kind {
                        PunctuatorKind::Semicolon => {
                            // END.
                            break;
                        },
                        PunctuatorKind::RParen => {
                            // )
                            self.token.push(token2);
                            break;
                        },
                        PunctuatorKind::RBracket => {
                            // ]
                            self.token.push(token2);
                            break;
                        },
                        PunctuatorKind::RBrace => {
                            // }
                            self.token.push(token2);
                            break;
                        },

                        PunctuatorKind::Comma => {
                            // ,
                            unimplemented!()
                        },
                        PunctuatorKind::Question => {
                            // ?
                            let op_precedence = 4;

                            unimplemented!()
                        },
                        PunctuatorKind::Increment => {
                            // 后置 递增
                            if !left_expr.is_member_expression() || !left_expr.is_identifier() {
                                return Err(self.unexpected_token(token2));
                            }

                            unimplemented!()
                        },
                        PunctuatorKind::Decrement => {
                            // 后置 递减
                            if !left_expr.is_member_expression() || !left_expr.is_identifier() {
                                return Err(self.unexpected_token(token2));
                            }

                            unimplemented!()
                        },
                        PunctuatorKind::Dot => {
                            // MemberAccessor
                            // .
                            left_expr = self.parse_member_expression(left_expr, token2)?;
                        },
                        PunctuatorKind::LBracket => {
                            // MemberAccessor
                            // [
                            left_expr = self.parse_member_expression(left_expr, token2)?;
                        },

                        PunctuatorKind::Add => {
                            unimplemented!()
                        },
                        PunctuatorKind::Sub => {
                            unimplemented!()
                        },
                        _ => {
                            unimplemented!()
                        },
                    }
                },
                Token::Keyword(kw) => {
                    match kw.kind {
                        KeywordKind::In => {
                            unimplemented!()
                        },
                        KeywordKind::InstanceOf => {
                            unimplemented!()
                        },
                        _ => {
                            unimplemented!()
                        },
                    }
                },
                _ => {
                    unimplemented!()
                }
            }
        }
        
        Ok(left_expr)
    }

    pub(super) fn parse_literal_regular_expression(&mut self) -> Result<LiteralRegularExpression<'ast>, Error> {
        // NOTE: Lexer 扩展
        match self.lexer.read_literal_regular_expression() {
            Ok(None) => Err(self.lexer.error(LexerErrorKind::UnexpectedEOF)),
            Ok(Some(token)) => {
                self.tokens.push(token);
                
                match token {
                    Token::LiteralRegularExpression(item) => Ok(item),
                    _ => unreachable!(),
                }
            },
            Err(e) => Err(e),
        }
    }

    pub(super) fn parse_literal_template(&mut self) -> Result<LiteralTemplateExpression<'ast>, Error> {
        // NOTE: Lexer 扩展
        let mut loc = self.lexer.loc();
        let mut span = self.lexer.span();

        let mut strings: Vec<LiteralString<'ast>> = vec![];
        let mut bounds: Vec<Expression<'ast>> = vec![];

        let mut bound_tokens: Vec<&'ast [Token<'ast>]> = vec![];

        loop {
            let (lit_str, is_end) = self.lexer.read_literal_template_string()?;
            
            if lit_str.raw.len() > 0 {
                strings.push(lit_str);
            }

            if is_end {
                break;
            }

            let mut token_stream_idx = self.tokens.len();

            // Read bound
            let next_token = self.token2()?;
            let expr = self.parse_expression(next_token, 0)?;

            let last_offset = self.lexer.offset();
            let last_line_offset = self.lexer.line_offset;
            let last_line_number = self.lexer.line;
            let last_line_column = self.lexer.column;

            let last_token = self.token2()?;
            let ok = match last_token {
                Token::Punctuator(punct) => punct.kind == PunctuatorKind::RBrace,
                _ => false,
            };
            
            if !ok {
                self.lexer.offset      = last_offset;
                self.lexer.line_offset = last_line_offset;
                self.lexer.line        = last_line_number;
                self.lexer.column      = last_line_column;

                return Err(self.lexer.error(LexerErrorKind::UnexpectedCharacter))
            }

            bounds.push(expr);


            let tokens = self.tokens[token_stream_idx..].to_vec();
            let tokens_len = tokens.len();

            bound_tokens.push(self.arena.alloc_vec(tokens));

            for _ in 0..tokens_len {
                let idx = self.tokens.len() - 1;
                self.tokens.remove(idx);
            }
        }
        
        let end_loc = self.lexer.loc();
        let end_span = self.lexer.span();

        loc.end = end_loc.end;
        span.end = end_span.end;

        let strings_ref = self.arena.alloc_vec(strings);

        // 生成新的 Token
        let raw = &self.lexer.source()[loc.start..loc.end];
        let bounds2 = self.arena.alloc_vec(bound_tokens);
        let new_token = Token::LiteralTemplate(LiteralTemplate { loc, span, raw, strings: strings_ref, bounds: bounds2 });
        self.tokens.push(new_token);
        
        Ok(LiteralTemplateExpression { loc, span, strings: strings_ref, bounds: self.arena.alloc_vec(bounds) })
    }

    pub fn parse_async_expression(&mut self, token: Token<'ast>) -> Result<Expression<'ast>, Error> {
        // AsyncFunctionDeclaration       EXPR
        // AsyncGeneratorDeclaration      EXPR
        // AsyncArrowFunctionExpression   EXPR
        // AsyncArrowGeneratorExpression  EXPR
        unimplemented!()
    }

    pub fn parse_prefix_expression(&mut self) -> Result<Expression<'ast>, Error> {
        unimplemented!()
    }

    pub fn parse_infix_expression(&mut self) -> Result<Expression<'ast>, Error> {
        unimplemented!()
    }

    pub fn parse_postfix_expression(&mut self) -> Result<Expression<'ast>, Error> {
        unimplemented!()
    }

    pub fn parse_member_expression(&mut self, mut left_expr: Expression<'ast>, mut token: Token<'ast>) -> Result<Expression<'ast>, Error> {
        let op_precedence = 19;

        loop {
            match token {
                Token::LineTerminator => {
                    continue;
                },
                Token::Punctuator(punct) => {
                    match punct.kind {
                        PunctuatorKind::Dot => {
                            // .
                            let token2 = self.token2()?;
                            
                            match token2 {
                                Token::LineTerminator => continue,
                                Token::Identifier(ident) => {
                                    let right_expr = Expression::Identifier(self.arena.alloc(ident));

                                    let mut loc = left_expr.loc();
                                    let mut span = left_expr.span();
                                    
                                    loc.end = right_expr.loc().end;
                                    span.end = right_expr.span().end;

                                    let left = left_expr;
                                    let right = right_expr;
                                    let computed = false;
                                    let item = MemberExpression { loc, span, left, right, computed, };

                                    left_expr = Expression::Member(self.alloc(item));
                                },
                                _ => return Err(self.unexpected_token(token2)),
                            }
                        },
                        PunctuatorKind::LBracket => {
                            // [
                            let token2 = self.token2()?;

                            let right_expr = self.parse_expression(token2, op_precedence)?;

                            loop {
                                let end_token = self.token2()?;
                                match end_token {
                                    Token::LineTerminator => continue,
                                    Token::Punctuator(punct) => match punct.kind {
                                        PunctuatorKind::RBracket => {
                                            break;
                                        },
                                        _ => {
                                            println!("{:?}", self.token);
                                            println!("{:?}", end_token);
                                            println!("{:?}", right_expr);
                                            return Err(self.unexpected_token(end_token));
                                        }
                                    },
                                    _ => {
                                        return Err(self.unexpected_token(end_token));
                                    }
                                }
                            }
                            
                            let mut loc = left_expr.loc();
                            let mut span = left_expr.span();
                            
                            loc.end = right_expr.loc().end;
                            span.end = right_expr.span().end;

                            let left = left_expr;
                            let right = right_expr;
                            let computed = false;
                            let item = MemberExpression { loc, span, left, right, computed, };

                            left_expr = Expression::Member(self.alloc(item));
                        },
                        _ => {
                            self.token.push(token);
                            break;
                        }
                    }
                },
                _ => {
                    self.token.push(token);
                    break;
                },
            }

            match self.token()? {
                Some(token3) => {
                    token = token3;
                },
                None => {
                    break;
                }
            }
        }

        Ok(left_expr)
    }

    pub fn parse_super_expression(&mut self) -> Result<Expression<'ast>, Error> {
        // super member
        // super call
        unimplemented!()
    }

    pub fn parse_new_expression(&mut self) -> Result<Expression<'ast>, Error> {
        unimplemented!()
    }

    // pub fn parse_object_binding_pattern(&mut self) -> Result<ObjectBindingPattern, Error> {
    //     unimplemented!()
    // }

    // pub fn parse_array_binding_pattern(&mut self) -> Result<ObjectBindingPattern, Error> {
    //     unimplemented!()
    // }

    // pub fn parse_infix_expression(&mut self,
    //                               left_spanned_expression: SpannedExpression,
    //                               spanned_token: SpannedToken) -> Result<SpannedExpression, Error> {
    //     let op = match spanned_token.item {
    //         Token::Punctuator(Punctuator::Pow) => {
    //             InfixOperator::Pow
    //         },
    //         Token::Punctuator(Punctuator::Mul) => {
    //             InfixOperator::Mul
    //         },
    //         Token::Punctuator(Punctuator::Div) => {
    //             InfixOperator::Div
    //         },
    //         Token::Punctuator(Punctuator::Rem) => {
    //             InfixOperator::Rem
    //         },
    //         Token::Punctuator(Punctuator::Add) => {
    //             InfixOperator::Add
    //         },
    //         Token::Punctuator(Punctuator::Sub) => {
    //             InfixOperator::Sub
    //         },
    //         Token::Punctuator(Punctuator::BitShl) => {
    //             InfixOperator::BitShl
    //         },
    //         Token::Punctuator(Punctuator::BitShr) => {
    //             InfixOperator::BitShr
    //         },
    //         Token::Punctuator(Punctuator::BitUShr) => {
    //             InfixOperator::BitUShr
    //         },
    //         Token::Punctuator(Punctuator::Gt) => {
    //             InfixOperator::Gt
    //         },
    //         Token::Punctuator(Punctuator::Lt) => {
    //             InfixOperator::Lt
    //         },
    //         Token::Punctuator(Punctuator::GtEq) => {
    //             InfixOperator::GtEq
    //         },
    //         Token::Punctuator(Punctuator::LtEq) => {
    //             InfixOperator::LtEq
    //         },
    //         Token::Punctuator(Punctuator::Eq) => {
    //             InfixOperator::Eq
    //         },
    //         Token::Punctuator(Punctuator::Neq) => {
    //             InfixOperator::Neq
    //         },
    //         Token::Punctuator(Punctuator::StrictEq) => {
    //             InfixOperator::StrictEq
    //         },
    //         Token::Punctuator(Punctuator::StrictNeq) => {
    //             InfixOperator::StrictNeq
    //         },
    //         Token::Punctuator(Punctuator::BitAnd) => {
    //             InfixOperator::BitAnd
    //         },
    //         Token::Punctuator(Punctuator::BitXor) => {
    //             InfixOperator::BitXor
    //         },
    //         Token::Punctuator(Punctuator::BitOr) => {
    //             InfixOperator::BitOr
    //         },
    //         Token::Punctuator(Punctuator::And) => {
    //             InfixOperator::And
    //         },
    //         Token::Punctuator(Punctuator::Or) => {
    //             InfixOperator::Or
    //         },
    //         Token::Keyword(Keyword::Instanceof) => {
    //             InfixOperator::InstanceOf
    //         },
    //         Token::Keyword(Keyword::In) => {
    //             InfixOperator::In
    //         },
    //         _ => unreachable!(),
    //     };
        
    //     let precedence = op.precedence();

    //     let token2 = self.next_token2_with_skip(&[
    //         Token::SingleLineComment,
    //         Token::MultiLineComment,
    //         Token::WhiteSpaces,
    //         Token::LineTerminator,
    //     ])?;
    //     let right_spanned_expression = self.parse_expression(token2)?;

    //     let left = Box::new(left_spanned_expression.item);
    //     let right = Box::new(right_spanned_expression.item);

    //     let expr = InfixExpression { operator: op, left, right  };
        
    //     let start = spanned_token.start;
    //     let end = spanned_token.end;
    //     let item = Expression::Infix(Box::new(expr));

    //     Ok(Span { start, end, item })
    // }

}