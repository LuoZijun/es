use error::{ ErrorKind, Error, };
use parser::parser::Parser;


use lexer::{ ESChar, Lexer, Token, SpannedToken, Punctuator, Keyword, };
use ast::IdentifierReference;
use ast::span::{ LineColumn, Span, };
use ast::float::Float;
use ast::statement::{ 
    SpannedStatement, Statement, StatementList,
    VariableStatement, LexicalDeclaration, LexicalDeclarationKind, LexicalBinding,

};
use ast::expression::{ SpannedExpression, Expression, };


impl<'a> Parser<'a> {
    pub fn parse_statement(&mut self, spanned_token: SpannedToken) -> Result<SpannedStatement, Error> {
        match spanned_token.item {
            Token::Punctuator(Punctuator::Semicolon) => {
                let start = spanned_token.start;
                let end = spanned_token.end;
                let item = Statement::Empty;
                let stmt = Span { start, end, item, };
                
                Ok(stmt)
            },
            Token::Keyword(Keyword::Debugger) => {
                let start = spanned_token.start;
                let end = spanned_token.end;
                let item = Statement::Debugger;
                let stmt = Span { start, end, item, };
                
                Ok(stmt)
            },
            Token::Keyword(Keyword::Var) => return self.parse_variable_statement(spanned_token),
            Token::Keyword(Keyword::Let) => return self.parse_lexical_declaration(spanned_token, LexicalDeclarationKind::Let),
            Token::Keyword(Keyword::Const) => return self.parse_lexical_declaration(spanned_token, LexicalDeclarationKind::Const),
            Token::Keyword(Keyword::Function)
            | Token::Keyword(Keyword::Class)
            | Token::Keyword(Keyword::Async) 
            => {
                unimplemented!()
            },
            _ => {
                unimplemented!()
            },
        }
    }

    fn parse_lexical_binding(&mut self, end: &mut LineColumn) -> Result<Vec<LexicalBinding>, Error> {
        let mut declarators: Vec<LexicalBinding> = vec![];

        loop {
            let token = self.next_token2_with_skip(&[
                Token::SingleLineComment,
                Token::MultiLineComment,
                Token::WhiteSpaces,
                Token::LineTerminator,
            ])?;

            match token.item {
                Token::Identifier(ident) => {
                    let token2 = self.next_token2_with_skip(&[
                        Token::SingleLineComment,
                        Token::MultiLineComment,
                        Token::WhiteSpaces,
                        Token::LineTerminator,
                    ])?;
                    
                    let mut lexical_binding = LexicalBinding {
                        name: Box::new(Expression::Identifier( Box::new(ident) )),
                        initializer: None,
                    };

                    match token2.item {
                        Token::Punctuator(Punctuator::Assign) => {
                            let token3 = self.next_token2_with_skip(&[
                                Token::SingleLineComment,
                                Token::MultiLineComment,
                                Token::WhiteSpaces,
                                Token::LineTerminator,
                            ])?;

                            let spanned_expr = self.parse_expression(token3)?;
                            lexical_binding.initializer = Some(Box::new(spanned_expr.item));

                            declarators.push(lexical_binding);
                        },
                        Token::Punctuator(Punctuator::Comma) => {
                            declarators.push(lexical_binding);
                            continue;
                        },
                        Token::Punctuator(Punctuator::Semicolon) => {
                            // ;
                            declarators.push(lexical_binding);
                            return Ok(declarators);
                        },
                        _ => {
                            return Err(self.unexpected_token(token2));
                        }
                    }
                },
                Token::Punctuator(Punctuator::LBracket) => {
                    // [
                    self.parse_array_binding_pattern();
                    unimplemented!()
                },
                Token::Punctuator(Punctuator::LBrace) => {
                    // {
                    self.parse_object_binding_pattern();
                    unimplemented!()
                },
                Token::Punctuator(Punctuator::Semicolon) => {
                    // ;
                    if declarators.len() == 0 {
                        return Err(self.unexpected_token(token));
                    }

                    return Ok(declarators);
                },
                _ => {
                    return Err(self.unexpected_token(token));
                }
            }
        }
    }

    pub fn parse_variable_statement(&mut self, spanned_token: SpannedToken) -> Result<SpannedStatement, Error> {
        // var
        let start = spanned_token.start;
        let mut end: LineColumn = LineColumn::default();
        let declarators = self.parse_lexical_binding(&mut end)?;
        let variable = VariableStatement { declarators, };

        let item = Statement::Variable(Box::new(variable));
        let stmt = Span { start, end, item, };

        Ok(stmt)
    }

    pub fn parse_lexical_declaration(&mut self,
                                     spanned_token: SpannedToken,
                                     kind: LexicalDeclarationKind) -> Result<SpannedStatement, Error> {
        // let/const
        let start = spanned_token.start;
        let mut end: LineColumn = LineColumn::default();
        let declarators = self.parse_lexical_binding(&mut end)?;
        let lexical_decl = LexicalDeclaration { kind, declarators, };
        
        let item = Statement::LetOrConst(Box::new(lexical_decl));
        let stmt = Span { start, end, item, };

        Ok(stmt)
    }
}
