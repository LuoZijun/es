use error::{ ErrorKind, Error, };
use parser::parser::Parser;


use lexer::{ ESChar, Lexer, Token, SpannedToken, Punctuator, Keyword, };
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
        let token2 = self.next_token2()?;

        match token2.item {
            Token::Identifier(ref ident) => {

            },
            Token::Punctuator(Punctuator::LBracket) => {
                // [

            },
            Token::Punctuator(Punctuator::LBrace) => {
                // {

            },
            Token::Punctuator(Punctuator::Semicolon) => {
                // ;
                unimplemented!()
            },
            _ => {

            }
        }

        unimplemented!()
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
