use error::{ ErrorKind, Error, };
use parser::parser::Parser;

use lexer::{ ESChar, Lexer, Token, SpannedToken, Punctuator, Keyword, };
use ast::span::{ LineColumn, Span, };
use ast::float::Float;
use ast::statement::{ 
    SpannedStatement, Statement, StatementList,
    VariableStatement, LexicalDeclaration, LexicalDeclarationKind, LexicalBinding,

};
use ast::expression::{
    SpannedExpression, Expression,
    ObjectBindingPattern, ArrayBindingPattern,
    StringLiteral, TemplateLiteral,

    PrefixOperator, InfixOperator, PostfixOperator,
    PrefixExpression, InfixExpression, PostfixExpression,

};


impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self, spanned_token: SpannedToken) -> Result<SpannedExpression, Error> {
        let mut spanned_expression_list: Vec<SpannedExpression> = vec![];
        let mut precedence = 0u8;

        let spanned_expression = match spanned_token.item {
            // Token::Punctuator(Punctuator::RParen)
            Token::Identifier(_)
            | Token::LiteralNull
            | Token::LiteralBoolean(_)
            | Token::LiteralString(_) 
            | Token::LiteralDecimalNumeric(_)
            | Token::LiteralFloatNumeric(_)
            | Token::Punctuator(Punctuator::BackTick)
            | Token::Punctuator(Punctuator::Div) => {
                self.parse_primitive_literal(spanned_token)?
            },
            
            // prefix expression
            Token::Keyword(Keyword::Await)
            | Token::Keyword(Keyword::Delete)
            | Token::Keyword(Keyword::Void)
            | Token::Keyword(Keyword::Typeof)
            | Token::Punctuator(Punctuator::Increment)
            | Token::Punctuator(Punctuator::Decrement)
            | Token::Punctuator(Punctuator::Not)
            | Token::Punctuator(Punctuator::Add)
            | Token::Punctuator(Punctuator::Sub)
            | Token::Punctuator(Punctuator::BitNot) => {
                self.parse_prefix_expression(spanned_token)?
            },

            // Token::Punctuator(Punctuator::Spread) => {
            //     unimplemented!()
            // },
            _ => {
                unimplemented!()
            },
        };

        let token2 = self.next_token2_with_skip(&[
            Token::SingleLineComment,
            Token::MultiLineComment,
            Token::WhiteSpaces,
            Token::LineTerminator,
        ])?;

        match token2.item {
            Token::Punctuator(Punctuator::Semicolon)
            | Token::Punctuator(Punctuator::Comma)   // 逗号是表达式序列，并不是一个表达式结束分界符号。
            | Token::Punctuator(Punctuator::RParen)
            | Token::Punctuator(Punctuator::RBracket)
            | Token::Punctuator(Punctuator::RBrace) => {
                // ; , ) ] }
                self.tokens.push(token2);
                return Ok(spanned_expression);
            },

            Token::Punctuator(Punctuator::DotMark) => {
                // .
                // member
                if spanned_expression.item.is_numeric_literal() {
                    return Err(self.unexpected_token(token2));
                }

                unimplemented!()
            },
            Token::Punctuator(Punctuator::LBracket) => {
                // [
                // member
                unimplemented!()
            },

            // infix expression
            Token::Punctuator(Punctuator::Pow)
            | Token::Punctuator(Punctuator::Mul)
            | Token::Punctuator(Punctuator::Div)
            | Token::Punctuator(Punctuator::Rem)
            | Token::Punctuator(Punctuator::Add)
            | Token::Punctuator(Punctuator::Sub)
            | Token::Punctuator(Punctuator::BitShl)
            | Token::Punctuator(Punctuator::BitShr)
            | Token::Punctuator(Punctuator::BitUShr)
            | Token::Punctuator(Punctuator::Gt)
            | Token::Punctuator(Punctuator::Lt)
            | Token::Punctuator(Punctuator::GtEq)
            | Token::Punctuator(Punctuator::LtEq)
            | Token::Punctuator(Punctuator::Eq)
            | Token::Punctuator(Punctuator::Neq)
            | Token::Punctuator(Punctuator::StrictEq)
            | Token::Punctuator(Punctuator::StrictNeq)
            | Token::Punctuator(Punctuator::BitAnd)
            | Token::Punctuator(Punctuator::BitXor)
            | Token::Punctuator(Punctuator::BitOr)
            | Token::Punctuator(Punctuator::And)
            | Token::Punctuator(Punctuator::Or)
            | Token::Keyword(Keyword::Instanceof)
            | Token::Keyword(Keyword::In) => {
                self.parse_infix_expression(spanned_expression, token2)
            },

            // postfix expression
            Token::Punctuator(Punctuator::Increment)
            | Token::Punctuator(Punctuator::Decrement) => {
                self.parse_postfix_expression(spanned_expression, token2)
            }
            _ => {
                unimplemented!()
            },
        }
    }

    pub fn parse_primitive_literal(&mut self, spanned_token: SpannedToken) -> Result<SpannedExpression, Error> {
        // null/true/false
        // string
        // number
        match spanned_token.item {
            Token::Identifier(ident) => {
                let start = spanned_token.start;
                let end = spanned_token.end;
                let item = Expression::Identifier( Box::new(ident) );

                Ok(Span { start, end, item })
            },
            Token::LiteralNull => {
                let start = spanned_token.start;
                let end = spanned_token.end;
                let item = Expression::NullLiteral;

                Ok(Span { start, end, item })
            },
            Token::LiteralBoolean(ref val) => {
                let start = spanned_token.start;
                let end = spanned_token.end;
                let item = Expression::BooleanLiteral(*val);
                
                Ok(Span { start, end, item })
            },
            Token::LiteralString(val) => {
                let start = spanned_token.start;
                let end = spanned_token.end;

                let start_offset = start.offset + start.column + 1;
                let end_offset = end.offset + end.column - 1;

                let raw = self.lexer.source[start_offset..end_offset].to_vec();

                let s = StringLiteral { raw: raw, cooked: val };
                let item = Expression::StringLiteral(Box::new(s));
                
                Ok(Span { start, end, item })
            },
            Token::LiteralDecimalNumeric(val) => {
                let start = spanned_token.start;
                let end = spanned_token.end;
                let item = Expression::NumericLiteral(val.into());
                
                Ok(Span { start, end, item })
            },
            Token::LiteralFloatNumeric(val) => {
                let start = spanned_token.start;
                let end = spanned_token.end;
                let item = Expression::NumericLiteral(val.into());
                
                Ok(Span { start, end, item })
            },
            Token::Punctuator(Punctuator::BackTick) => {
                self.parse_template_literal(spanned_token)
            },
            Token::Punctuator(Punctuator::Div) => {
                self.parse_regular_expression_literal(spanned_token)
            },
            _ => unreachable!(),
        }
    }
    
    pub fn parse_template_literal(&mut self, spanned_token: SpannedToken) -> Result<SpannedExpression, Error> {
        let mut strings: Vec<StringLiteral> = vec![];
        let mut bounds: Vec<Expression> = vec![];
        let start = spanned_token.start;

        #[allow(unused_assignments)]
        let mut end: LineColumn = spanned_token.end;

        let closing_delimiter = '`';
        let is_template = true;
        let allow_line_terminator = true;
        let unescape = true;

        loop {
            let ustring_start = self.lexer.line_column();

            #[allow(unused_assignments)]
            match self.lexer.read_string_literal(closing_delimiter,
                                                is_template,
                                                allow_line_terminator,
                                                unescape) {
                Some(ustring) => {
                    let line_column = self.lexer.line_column();
                    let idx = line_column.offset + line_column.column;
                    let last_two_char = [ self.lexer.source.get(idx-1), self.lexer.source.get(idx-2) ];
                    let has_inline_expr = last_two_char == [ Some(&'{'), Some(&'$') ];
                    
                    let ustring_end = self.lexer.line_column();

                    let start_offset = ustring_start.offset + ustring_start.column + 1;
                    let end_offset = ustring_end.offset + ustring_end.column - 1 - 1;

                    let raw = self.lexer.source[start_offset..end_offset].to_vec();

                    strings.push(StringLiteral { raw: raw, cooked: ustring });

                    if !has_inline_expr {
                        end = ustring_end;
                        break;
                    }

                    let token2 = self.next_token2_with_skip(&[
                        Token::SingleLineComment,
                        Token::MultiLineComment,
                        Token::WhiteSpaces,
                        Token::LineTerminator,
                    ])?;
                    let expr = self.parse_expression(token2)?;

                    let token3 = self.next_token2_with_skip(&[
                        Token::SingleLineComment,
                        Token::MultiLineComment,
                        Token::WhiteSpaces,
                        Token::LineTerminator,
                    ])?;

                    if token3.item != Token::Punctuator(Punctuator::RBrace) {
                        return Err(self.unexpected_token(token3));
                    }

                    end = expr.end;
                    bounds.push(expr.item);
                },
                None => {
                    return Err(self.unexpected_token(spanned_token));
                }
            }
        }

        let tpl = TemplateLiteral { strings, bounds };
        let item = Expression::TemplateLiteral(Box::new(tpl));
        
        Ok(Span { start, end: end, item })
    }

    pub fn parse_regular_expression_literal(&mut self, spanned_token: SpannedToken) -> Result<SpannedExpression, Error> {
        match self.lexer.read_regular_expression_literal() {
            Some(regexp) => {
                let start = spanned_token.start;
                let end = spanned_token.end;
                let item = Expression::RegularExpressionLiteral(Box::new(regexp));
                
                Ok(Span { start, end, item })
            },
            None => {
                return Err(self.unexpected_token(spanned_token));
            }
        }
    }

    pub fn parse_prefix_expression(&mut self, spanned_token: SpannedToken) -> Result<SpannedExpression, Error> {
        let op = match spanned_token.item {
            Token::Keyword(Keyword::Await) => {
                PrefixOperator::Await
            },
            Token::Keyword(Keyword::Delete) => {
                PrefixOperator::Delete
            },
            Token::Keyword(Keyword::Void) => {
                PrefixOperator::Void
            },
            Token::Keyword(Keyword::Typeof) => {
                PrefixOperator::TypeOf
            },
            Token::Punctuator(Punctuator::Increment) => {
                PrefixOperator::Increment
            },
            Token::Punctuator(Punctuator::Decrement) => {
                PrefixOperator::Decrement
            },
            Token::Punctuator(Punctuator::Not) => {
                PrefixOperator::Not
            },
            Token::Punctuator(Punctuator::Add) => {
                PrefixOperator::Add
            },
            Token::Punctuator(Punctuator::Sub) => {
                PrefixOperator::Sub
            },
            Token::Punctuator(Punctuator::BitNot) => {
                // ~
                PrefixOperator::BitNot
            },
            _ => unreachable!(),
        };

        let token2 = self.next_token2_with_skip(&[
            Token::SingleLineComment,
            Token::MultiLineComment,
            Token::WhiteSpaces,
            Token::LineTerminator,
        ])?;
        let spanned_expression = self.parse_expression(token2)?;

        let expr = PrefixExpression { operator: op, operand: Box::new(spanned_expression.item) };

        let start = spanned_token.start;
        let end = spanned_token.end;
        let item = Expression::Prefix(Box::new(expr));

        Ok(Span { start, end, item })
    }

    pub fn parse_infix_expression(&mut self, left_spanned_expression: SpannedExpression, spanned_token: SpannedToken) -> Result<SpannedExpression, Error> {
        let op = match spanned_token.item {
            Token::Punctuator(Punctuator::Pow) => {
                InfixOperator::Pow
            },
            Token::Punctuator(Punctuator::Mul) => {
                InfixOperator::Mul
            },
            Token::Punctuator(Punctuator::Div) => {
                InfixOperator::Div
            },
            Token::Punctuator(Punctuator::Rem) => {
                InfixOperator::Rem
            },
            Token::Punctuator(Punctuator::Add) => {
                InfixOperator::Add
            },
            Token::Punctuator(Punctuator::Sub) => {
                InfixOperator::Sub
            },
            Token::Punctuator(Punctuator::BitShl) => {
                InfixOperator::BitShl
            },
            Token::Punctuator(Punctuator::BitShr) => {
                InfixOperator::BitShr
            },
            Token::Punctuator(Punctuator::BitUShr) => {
                InfixOperator::BitUShr
            },
            Token::Punctuator(Punctuator::Gt) => {
                InfixOperator::Gt
            },
            Token::Punctuator(Punctuator::Lt) => {
                InfixOperator::Lt
            },
            Token::Punctuator(Punctuator::GtEq) => {
                InfixOperator::GtEq
            },
            Token::Punctuator(Punctuator::LtEq) => {
                InfixOperator::LtEq
            },
            Token::Punctuator(Punctuator::Eq) => {
                InfixOperator::Eq
            },
            Token::Punctuator(Punctuator::Neq) => {
                InfixOperator::Neq
            },
            Token::Punctuator(Punctuator::StrictEq) => {
                InfixOperator::StrictEq
            },
            Token::Punctuator(Punctuator::StrictNeq) => {
                InfixOperator::StrictNeq
            },
            Token::Punctuator(Punctuator::BitAnd) => {
                InfixOperator::BitAnd
            },
            Token::Punctuator(Punctuator::BitXor) => {
                InfixOperator::BitXor
            },
            Token::Punctuator(Punctuator::BitOr) => {
                InfixOperator::BitOr
            },
            Token::Punctuator(Punctuator::And) => {
                InfixOperator::And
            },
            Token::Punctuator(Punctuator::Or) => {
                InfixOperator::Or
            },
            Token::Keyword(Keyword::Instanceof) => {
                InfixOperator::InstanceOf
            },
            Token::Keyword(Keyword::In) => {
                InfixOperator::In
            },
            _ => unreachable!(),
        };
        
        let precedence = op.precedence();

        let token2 = self.next_token2_with_skip(&[
            Token::SingleLineComment,
            Token::MultiLineComment,
            Token::WhiteSpaces,
            Token::LineTerminator,
        ])?;
        let spanned_expression = self.parse_expression(token2)?;

        let left = Box::new(left_spanned_expression.item);
        let right = Box::new(spanned_expression.item);

        let expr = InfixExpression { operator: op, left, right  };
        
        let start = spanned_token.start;
        let end = spanned_token.end;
        let item = Expression::Infix(Box::new(expr));

        Ok(Span { start, end, item })
    }

    pub fn parse_postfix_expression(&mut self, left_spanned_expression: SpannedExpression, spanned_token: SpannedToken) -> Result<SpannedExpression, Error> {
        let op = match spanned_token.item {
            Token::Punctuator(Punctuator::Increment) => {
                PostfixOperator::Increment
            },
            Token::Punctuator(Punctuator::Decrement) => {
                PostfixOperator::Decrement
            },
            _ => unreachable!(),
        };

        let expr = PostfixExpression { operator: op, operand: Box::new(left_spanned_expression.item) };
        
        let start = spanned_token.start;
        let end = spanned_token.end;
        let item = Expression::Postfix(Box::new(expr));

        Ok(Span { start, end, item })
    }

    pub fn parse_object_binding_pattern(&mut self) -> Result<ObjectBindingPattern, Error> {
        unimplemented!()
    }

    pub fn parse_array_binding_pattern(&mut self) -> Result<ObjectBindingPattern, Error> {
        unimplemented!()
    }
}