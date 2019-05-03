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
use crate::ast::function::{ FunctionExpression, Function, ArrowFunctionExpression, ConciseBody, };

// 运算符优先级
// https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#Table

// LeftHandSideExpression [?Yield, ?Await] [no LineTerminator here] ++
// LeftHandSideExpression [?Yield, ?Await] [no LineTerminator here] --
// continue [no LineTerminator here] LabelIdentifier [?Yield, ?Await];
// break [no LineTerminator here] LabelIdentifier [?Yield, ?Await];
// return [no LineTerminator here] Expression [+In, ?Yield, ?Await];
// throw [no LineTerminator here] Expression [+In, ?Yield, ?Await];
// ArrowParameters [?Yield, ?Await] [no LineTerminator here] => ConciseBody [?In]
// yield [no LineTerminator here] AssignmentExpression [?In, +Yield, ?Await]
// yield [no LineTerminator here] *AssignmentExpression [?In, +Yield, ?Await]

#[inline]
pub fn punctuator_to_infix_op(punct: PunctuatorKind) -> InfixOperator {
    match punct {
        PunctuatorKind::Add => InfixOperator::Add,
        PunctuatorKind::Sub => InfixOperator::Sub,
        PunctuatorKind::Mul => InfixOperator::Mul,
        PunctuatorKind::Div => InfixOperator::Div,
        PunctuatorKind::Rem => InfixOperator::Rem,
        PunctuatorKind::Pow => InfixOperator::Pow,
        PunctuatorKind::BitShl => InfixOperator::BitShl,
        PunctuatorKind::BitShr => InfixOperator::BitShr,
        PunctuatorKind::BitUShr => InfixOperator::BitUShr,
        PunctuatorKind::And => InfixOperator::And,
        PunctuatorKind::Or => InfixOperator::Or,
        PunctuatorKind::BitAnd => InfixOperator::BitAnd,
        PunctuatorKind::BitXor => InfixOperator::BitXor,
        PunctuatorKind::BitOr => InfixOperator::BitOr,

        PunctuatorKind::Gt => InfixOperator::Gt,
        PunctuatorKind::Lt => InfixOperator::Lt,
        PunctuatorKind::GtEq => InfixOperator::GtEq,
        PunctuatorKind::LtEq => InfixOperator::LtEq,
        PunctuatorKind::Eq => InfixOperator::Eq,
        PunctuatorKind::Neq => InfixOperator::Neq,
        PunctuatorKind::StrictEq => InfixOperator::StrictEq,
        PunctuatorKind::StrictNeq => InfixOperator::StrictNeq,
        _ => unreachable!(),
    }
}

#[inline]
pub fn punctuator_to_assignment_op(punct: PunctuatorKind) -> AssignmentOperator {
    match punct {
        PunctuatorKind::Assign => AssignmentOperator::Assign,
        PunctuatorKind::AddAssign => AssignmentOperator::AddAssign,
        PunctuatorKind::SubAssign => AssignmentOperator::SubAssign,
        PunctuatorKind::MulAssign => AssignmentOperator::MulAssign,
        PunctuatorKind::DivAssign => AssignmentOperator::DivAssign,
        PunctuatorKind::RemAssign => AssignmentOperator::RemAssign,
        PunctuatorKind::PowAssign => AssignmentOperator::PowAssign,

        PunctuatorKind::BitAndAssign => AssignmentOperator::BitAndAssign,
        PunctuatorKind::BitOrAssign => AssignmentOperator::BitOrAssign,
        PunctuatorKind::BitXorAssign => AssignmentOperator::BitXorAssign,
        PunctuatorKind::BitShlAssign => AssignmentOperator::BitShlAssign,
        PunctuatorKind::BitShrAssign => AssignmentOperator::BitShrAssign,
        PunctuatorKind::BitUShrAssign => AssignmentOperator::BitUShrAssign,
        _ => unreachable!(),
    }
}

#[inline]
pub fn keyword_to_infix_op(keyword: KeywordKind) -> InfixOperator {
    match keyword {
        KeywordKind::In => InfixOperator::In,
        KeywordKind::InstanceOf => InfixOperator::InstanceOf,
        _ => unreachable!(),
    }
}

impl<'ast> Parser<'ast> {
    pub fn parse_expression(&mut self, token: Token<'ast>, precedence: i8) -> Result<Expression<'ast>, Error> {
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
            Token::Identifier(ident)   => {
                match ident.to_keyword_or_literal() {
                    Some(token) => return self.parse_expression(token, precedence),
                    None => Expression::Identifier(self.arena.alloc(ident)),
                }
            },
            Token::TemplateOpenning    => {
                let item = self.parse_literal_template()?;
                Expression::Template(self.arena.alloc(item))
            },
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::LParen => {
                        // ParenthesizedExpression
                        // (
                        let mut loc = punct.loc;
                        let mut span = punct.span;
                        let mut items: Vec<Expression<'ast>> = Vec::new();
                        let op_precedence = 20i8;

                        let mut is_first: bool = true;
                        loop {
                            let mut token2 = self.token2()?;
                            match token2 {
                                Token::LineTerminator => continue,
                                Token::Punctuator(punct) => {
                                    match punct.kind {
                                        PunctuatorKind::RParen => {
                                            // )
                                            loc.end = punct.loc.end;
                                            span.end = punct.span.end;
                                            break;
                                        },
                                        _ => {
                                            if !is_first {
                                                return Err(self.unexpected_token(token2));
                                            }

                                            let item = self.parse_expression(token2, precedence)?;
                                            items.push(item);
                                            
                                            is_first = false;
                                        }
                                    }
                                },
                                _ => {
                                    if !is_first {
                                        return Err(self.unexpected_token(token2));
                                    }

                                    let item = self.parse_expression(token2, precedence)?;
                                    items.push(item);
                                    
                                    is_first = false;
                                }
                            }
                        }
                        
                        let elems = self.arena.alloc_vec(items);
                        let item = ParenthesizedExpression { loc, span, items: elems };
                        Expression::Parenthesized(self.alloc(item))
                    },
                    PunctuatorKind::Div => {
                        let item = self.parse_literal_regular_expression()?;
                        Expression::RegularExpression(self.arena.alloc(item))
                    },
                    PunctuatorKind::DotDotDot => {
                        // Spread, 展开运算符
                        // ... 
                        let mut loc = punct.loc;
                        let mut span = punct.span;

                        let precedence = 1i8;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;

                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = SpreadExpression { loc, span, item: operand, };
                        Expression::Spread(self.arena.alloc(item))
                    },
                    PunctuatorKind::Add => {
                        // unary operator
                        // +
                        let mut loc = punct.loc;
                        let mut span = punct.span;

                        let precedence = 16i8;
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

                        let precedence = 16i8;
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

                        let precedence = 16i8;
                        let operator = PrefixOperator::Increment;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;

                        if !operand.is_member_expression() && !operand.is_identifier() {
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

                        let precedence = 16i8;
                        let operator = PrefixOperator::Decrement;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;
                        
                        if !operand.is_member_expression() && !operand.is_identifier() {
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

                        let precedence = 16i8;
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

                        let precedence = 16i8;
                        let operator = PrefixOperator::BitNot;
                        let token2 = self.token2()?;
                        let operand = self.parse_expression(token2, precedence)?;
                        
                        loc.end = operand.loc().end;
                        span.end = operand.span().end;

                        let item = PrefixExpression { loc, span, operator, operand, };
                        Expression::Prefix(self.arena.alloc(item))
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

                        let precedence = 16i8;
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

                        let precedence = 16i8;
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

                        let precedence = 16i8;
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

                        let precedence = 16i8;
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
                    KeywordKind::Function => {
                        // Function or Generator EXPR
                        let function_expr = self.parse_function_expression(token)?;
                        Expression::Function(self.alloc(function_expr))
                    },
                    KeywordKind::Class => {
                        // Class EXPR
                        let class_expr = self.parse_class_expression(token)?;
                        Expression::Class(self.alloc(class_expr))
                    },
                    KeywordKind::Async => {
                        // AsyncFunctionDeclaration       EXPR
                        // AsyncGeneratorDeclaration      EXPR
                        // AsyncArrowFunctionExpression   EXPR
                        self.parse_async_expression(token)?
                    },
                    KeywordKind::New => {
                        self.parse_new_expression(token)?
                    },
                    KeywordKind::Yield => {
                        let op_precedence = 2;

                        let mut loc = kw.loc;
                        let mut span = kw.span;

                        let mut token2 = self.token2()?;
                        let star = match token2 {
                            // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-YieldExpression
                            // no LineTerminator here
                            Token::Punctuator(punct) => {
                                if punct.kind == PunctuatorKind::Mul {
                                    token2 = self.token2()?;
                                    true
                                } else {
                                    false
                                }
                            },
                            _ => false
                        };

                        let expr = self.parse_expression(token2, op_precedence)?;
                        loc.end = expr.loc().end;
                        span.end = expr.span().end;

                        let item = YieldExpression{ loc, span, star, item: expr };
                        Expression::Yield(self.alloc(item))
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
        
        // if left_expr.precedence() >= precedence {
        //         return Ok(left_expr);
        // }

        loop {
            let token2 = match self.token2() {
                Ok(token2) => {
                    match token2 {
                        Token::Identifier(ident)   => {
                            match ident.to_keyword_or_literal() {
                                Some(token) => token,
                                None => token2,
                            }
                        },
                        _ => token2
                    }
                },
                Err(_) => return Ok(left_expr),
            };

            match token2 {
                Token::LineTerminator => {
                    continue;
                },
                Token::Punctuator(punct) => {
                    match punct.kind {
                        PunctuatorKind::Semicolon => {
                            // END.
                            self.token.push(token2);
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
                        PunctuatorKind::LBrace | PunctuatorKind::RBrace => {
                            // }
                            self.token.push(token2);
                            break;
                        },
                        PunctuatorKind::Colon => {
                            // :
                            self.token.push(token2);
                            break;
                        },
                        PunctuatorKind::FatArrow => {
                            // Lookahead `=>`
                            // AsyncArrowFunctionExpression
                            // a =>
                            if !left_expr.is_identifier() && !left_expr.is_parenthesized_expression() {
                                return Err(self.unexpected_token(token2));
                            }

                            left_expr = self.parse_arrow_function_expression(left_expr)?;
                        },
                        PunctuatorKind::Comma => {
                            // ,
                            // CommaExpression
                            let mut loc = left_expr.loc();
                            let mut span = left_expr.span();
                            let mut items: Vec<Expression<'ast>> = vec![ left_expr ];
                            let op_precedence = 0i8;

                            let mut token3 = token2;
                            loop {
                                match token3 {
                                    Token::LineTerminator => continue,
                                    Token::Punctuator(punct) => {
                                        match punct.kind {
                                            PunctuatorKind::Comma => {
                                                // FIXME: 需要处理优先级问题
                                                // if precedence >= op_precedence {
                                                if left_expr.precedence() >= op_precedence {
                                                    self.token.push(token3);
                                                    return Ok(left_expr);
                                                }
                                                let token4 = self.token2()?;
                                                let item = self.parse_expression(token4, op_precedence)?;
                                                items.push(item);

                                                match self.token()? {
                                                    None => {
                                                        break;
                                                    },
                                                    Some(new_token) => {
                                                        token3 = new_token;
                                                    }
                                                }
                                            },
                                            _ => {
                                                self.token.push(token3);
                                                break;
                                            }
                                        }
                                    },
                                    _ => {
                                        self.token.push(token3);
                                        break;
                                    }
                                }
                            }
                            
                            let elems = self.arena.alloc_vec(items);
                            let item = CommaExpression { loc, span, items: elems };
                            left_expr = Expression::Comma(self.alloc(item));
                        },
                        PunctuatorKind::Question => {
                            // ?
                            // EXPR ? EXPR : EXPR
                            let op_precedence = 4i8;
                            
                            let mut loc = left_expr.loc();
                            let mut span = left_expr.span();

                            let token3 = self.token2()?;
                            let and_then = self.parse_expression(token3, op_precedence)?;
                            let mut token4 = self.token2()?;
                            
                            loop {
                                match token4 {
                                    Token::LineTerminator => {
                                        token4 = self.token2()?;
                                        continue;
                                    },
                                    Token::Punctuator(punct) => {
                                        if punct.kind == PunctuatorKind::Colon {
                                            // :
                                            token4 = self.token2()?;
                                            break;
                                        } else {
                                            return Err(self.unexpected_token(token4));
                                        }
                                    },
                                    _ => {
                                        return Err(self.unexpected_token(token4));
                                    }
                                }
                            }

                            let or_else = self.parse_expression(token4, op_precedence)?;
                            
                            loc.end = or_else.loc().end;
                            span.end = or_else.span().end;

                            let item = ConditionalExpression { loc, span, condition: left_expr, and_then, or_else };
                            left_expr = Expression::Conditional(self.alloc(item));
                        },
                        PunctuatorKind::Increment => {
                            // 后置 递增
                            if !left_expr.is_member_expression() && !left_expr.is_identifier() {
                                return Err(self.unexpected_token(token2));
                            }

                            let mut loc = left_expr.loc();
                            let mut span = left_expr.span();

                            let precedence = 17i8;
                            let operator = PostfixOperator::Increment;
                            let operand = left_expr;
                            
                            loc.end = punct.loc.end;
                            span.end = punct.span.end;

                            let item = PostfixExpression { loc, span, operator, operand, };
                            left_expr = Expression::Postfix(self.arena.alloc(item));
                        },
                        PunctuatorKind::Decrement => {
                            // 后置 递减
                            if !left_expr.is_member_expression() && !left_expr.is_identifier() {
                                return Err(self.unexpected_token(token2));
                            }

                            let mut loc = left_expr.loc();
                            let mut span = left_expr.span();

                            let precedence = 17i8;
                            let operator = PostfixOperator::Decrement;
                            let operand = left_expr;
                            
                            loc.end = punct.loc.end;
                            span.end = punct.span.end;

                            let item = PostfixExpression { loc, span, operator, operand, };
                            left_expr = Expression::Postfix(self.arena.alloc(item));
                        },
                        PunctuatorKind::Dot => {
                            // MemberAccessor
                            // .
                            if left_expr.is_numeric_literal() {
                                return Err(self.unexpected_token(token2));
                            }

                            left_expr = self.parse_member_expression(left_expr, token2)?;
                        },
                        PunctuatorKind::LBracket => {
                            // MemberAccessor
                            // [
                            left_expr = self.parse_member_expression(left_expr, token2)?;
                        },
                        PunctuatorKind::LParen => {
                            // Call
                            // (
                            let op_precedence = 19i8;
                            // if precedence >= op_precedence {
                            //     self.token.push(token2);
                            //     return Ok(left_expr);
                            // }

                            let mut loc = left_expr.loc();
                            let mut span = left_expr.span();
                            
                            let callee = left_expr;
                            let arguments = match self.parse_expression(token2, op_precedence)? {
                                Expression::Parenthesized(inner) => *inner,
                                _ => {
                                    return Err(self.unexpected_token(token2))
                                },
                            };

                            let item = CallExpression { loc, span, callee, arguments, };
                            left_expr = Expression::Call(self.arena.alloc(item));
                        },

                        // Infix expr
                        PunctuatorKind::Add | PunctuatorKind::Sub | PunctuatorKind::Mul | PunctuatorKind::Div
                        | PunctuatorKind::Rem | PunctuatorKind::Pow
                        | PunctuatorKind::BitShl | PunctuatorKind::BitShr | PunctuatorKind::BitUShr
                        | PunctuatorKind::And | PunctuatorKind::Or
                        | PunctuatorKind::BitAnd | PunctuatorKind::BitXor | PunctuatorKind::BitOr
                        | PunctuatorKind::Gt | PunctuatorKind::Lt | PunctuatorKind::GtEq | PunctuatorKind::LtEq
                        | PunctuatorKind::Eq | PunctuatorKind::Neq | PunctuatorKind::StrictEq | PunctuatorKind::StrictNeq => {
                            let operator = punctuator_to_infix_op(punct.kind);
                            let op_precedence = operator.precedence();

                            let mut loc = left_expr.loc();
                            let mut span = left_expr.span();

                            if op_precedence <= precedence {
                                self.token.push(token2);
                                return Ok(left_expr);
                            }

                            let token3 = self.token2()?;
                            let right_expr = self.parse_expression(token3, op_precedence)?;

                            loc.end = right_expr.loc().end;
                            span.end = right_expr.span().end;

                            let item = InfixExpression { loc, span, operator, left: left_expr, right: right_expr };

                            left_expr = Expression::Infix(self.arena.alloc(item));
                        },
                        // AssignmentExpression
                        PunctuatorKind::Assign | PunctuatorKind::AddAssign | PunctuatorKind::SubAssign | PunctuatorKind::MulAssign 
                        | PunctuatorKind::DivAssign | PunctuatorKind::RemAssign | PunctuatorKind::PowAssign 
                        | PunctuatorKind::BitAndAssign | PunctuatorKind::BitOrAssign | PunctuatorKind::BitXorAssign 
                        | PunctuatorKind::BitShlAssign | PunctuatorKind::BitShrAssign | PunctuatorKind::BitUShrAssign => {
                            let op_precedence = 3i8;

                            let operator = punctuator_to_assignment_op(punct.kind);
                            let token3 = self.token2()?;
                            let right_expr = self.parse_expression(token3, op_precedence)?;
                            
                            let mut loc = left_expr.loc();
                            let mut span = left_expr.span();
                            loc.end = right_expr.loc().end;
                            span.end = right_expr.span().end;
                            let item = AssignmentExpression { loc, span, left: left_expr, operator, right: right_expr };
                            left_expr = Expression::Assignment(self.alloc(item));
                        },
                        _ => {
                            return Err(self.unexpected_token(token2));
                        },
                    }
                },
                Token::Keyword(kw) => {
                    match kw.kind {
                        KeywordKind::In | KeywordKind::InstanceOf => {
                            let operator = keyword_to_infix_op(kw.kind);
                            let op_precedence = operator.precedence();

                            let mut loc = left_expr.loc();
                            let mut span = left_expr.span();

                            if op_precedence <= precedence {
                                self.token.push(token2);
                                return Ok(left_expr);
                            }

                            let token3 = self.token2()?;
                            let right_expr = self.parse_expression(token3, op_precedence)?;

                            loc.end = right_expr.loc().end;
                            span.end = right_expr.span().end;

                            let item = InfixExpression { loc, span, operator, left: left_expr, right: right_expr };

                            left_expr = Expression::Infix(self.arena.alloc(item));
                        },
                        _ => {
                            return Err(self.unexpected_token(token2));
                        },
                    }
                },
                Token::TemplateOpenning    => {
                    // TaggedTemplate
                    let template = self.parse_literal_template()?;

                    let mut loc = left_expr.loc();
                    let mut span = left_expr.span();
                    loc.end = template.loc.end;
                    span.end = template.span.end;

                    let item = TaggedTemplateExpression { loc, span, tag: left_expr, template };
                    left_expr = Expression::TaggedTemplate(self.arena.alloc(item));
                },
                _ => {
                    return Err(self.unexpected_token(token2));
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
            let expr = self.parse_expression(next_token, -1i8)?;

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

    pub fn parse_member_expression(&mut self, mut left_expr: Expression<'ast>, mut token: Token<'ast>) -> Result<Expression<'ast>, Error> {
        let op_precedence = 19;

        loop {
            match token {
                Token::LineTerminator => {
                    // continue;
                },
                Token::Punctuator(punct) => {
                    match punct.kind {
                        PunctuatorKind::Dot => {
                            // .
                            let token2 = self.token2()?;
                            
                            match token2 {
                                Token::LineTerminator => {
                                    // continue
                                },
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

                            let right_expr = self.parse_expression(token2, -1i8)?;

                            loop {
                                let end_token = self.token2()?;
                                match end_token {
                                    Token::LineTerminator => continue,
                                    Token::Punctuator(punct) => match punct.kind {
                                        PunctuatorKind::RBracket => {
                                            break;
                                        },
                                        _ => {
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
                            let computed = true;
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

    pub fn parse_new_expression(&mut self, token: Token<'ast>) -> Result<Expression<'ast>, Error> {
        let (mut loc, mut span) = match token {
            Token::Keyword(kw) => {
                assert_eq!(kw.kind, KeywordKind::New);
                (kw.loc, kw.span)
            },
            _ => unreachable!(),
        };

        loop {
            let token2 = self.token2()?;
            match token2 {
                Token::LineTerminator => {
                    continue;
                },
                Token::Punctuator(punct) => {
                    match punct.kind {
                        PunctuatorKind::Dot => {
                            // new . target
                            let token3 = self.token2()?;
                            const TARGET: &[char] = &['t', 'a', 'r', 'g', 'e', 't'];
                            match token3 {
                                Token::Identifier(ident) => {
                                    if ident.raw == TARGET {
                                        loc.end = ident.loc.end;
                                        span.end = ident.span.end;

                                        let item = NewTargetExpression { loc, span, };
                                        
                                        return Ok(Expression::NewTarget(self.alloc(item)));
                                    } else {
                                        return Err(self.unexpected_token(token3));
                                    }
                                },
                                _ => {
                                    return Err(self.unexpected_token(token3));
                                }
                            }
                        },
                        _ => {
                            self.token.push(token2);
                            break;
                        }
                    }
                },
                _ => {
                    self.token.push(token2);
                    break;
                }
            }
        }

        let op_precedence = 18i8;

        let token2 = self.token2()?;
        let callee = self.parse_expression(token2, op_precedence)?;
        let arguments = None;

        loc.end = callee.loc().end;
        span.end = callee.span().end;

        let item = NewExpression { loc, span, callee, arguments, };
        
        Ok(Expression::New(self.alloc(item)))
    }

    pub fn parse_object_binding_pattern(&mut self) -> Result<Expression<'ast>, Error> {
        unimplemented!()
    }

    pub fn parse_array_binding_pattern(&mut self) -> Result<Expression<'ast>, Error> {
        unimplemented!()
    }

    pub fn parse_brace_expression(&mut self, token: Token<'ast>) -> Result<Expression<'ast>, Error> {
        // {}
        // LiteralObject
        // ObjectBindingPattern
        let (mut loc, mut span) = match token {
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::LBrace => {
                        (punct.loc, punct.span)
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        };

        unimplemented!()
    }

    pub fn parse_bracket_expression(&mut self, token: Token<'ast>) -> Result<Expression<'ast>, Error> {
        // []
        // LiteralArray
        // ArrayBindingPattern
        let (mut loc, mut span) = match token {
            Token::Punctuator(punct) => {
                match punct.kind {
                    PunctuatorKind::LBracket => {
                        (punct.loc, punct.span)
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        };

        unimplemented!()
    }
}