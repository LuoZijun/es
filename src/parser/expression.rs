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

};



impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self, spanned_token: SpannedToken) -> Result<SpannedExpression, Error> {
        let spanned_expression = match spanned_token.item {
            Token::Identifier(ident) => {
                let start = spanned_token.start;
                let end = spanned_token.end;
                let item = Expression::Identifier( Box::new(ident) );

                Span { start, end, item }
            },
            Token::LiteralNull
            | Token::LiteralBoolean(_)
            | Token::LiteralString(_) 
            | Token::LiteralDecimalNumeric(_)
            | Token::LiteralFloatNumeric(_) => {
                self.parse_primitive_literal(spanned_token)?
            },
            Token::Punctuator(Punctuator::BackTick) => {
                self.parse_template_literal(spanned_token)?
            },
            Token::Punctuator(Punctuator::Div) => {
                self.parse_regular_expression_literal(spanned_token)?
            },

            // prefix expression
            Token::Punctuator(Punctuator::Increment) => {
                unimplemented!()
            },
            Token::Punctuator(Punctuator::Decrement) => {
                unimplemented!()
            },
            Token::Punctuator(Punctuator::Not) => {
                unimplemented!()
            },
            Token::Punctuator(Punctuator::Add) => {
                unimplemented!()
            },
            Token::Punctuator(Punctuator::Sub) => {
                unimplemented!()
            },
            Token::Punctuator(Punctuator::BitNot) => {
                // ~
                unimplemented!()
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
            | Token::Punctuator(Punctuator::Comma)
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

    pub fn parse_object_binding_pattern(&mut self) -> Result<ObjectBindingPattern, Error> {
        unimplemented!()
    }

    pub fn parse_array_binding_pattern(&mut self) -> Result<ObjectBindingPattern, Error> {
        unimplemented!()
    }
}