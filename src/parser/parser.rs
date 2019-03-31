use error::{ ErrorKind, Error, };
use version::ECMAScriptVersion;
use lexer::{ ESChar, Lexer, Token, SpannedToken, Punctuator, Keyword, };


use ast::span::{ LineColumn, Span, };
use ast::float::Float;
use ast::statement::{ 
    SpannedStatement, Statement, StatementList,
    VariableStatement, LexicalDeclaration, LexicalDeclarationKind, LexicalBinding,

};
use ast::expression::{ SpannedExpression, Expression, };




pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub body: StatementList,
    pub filename: String,
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a [char]) -> Self {
        let lexer = Lexer::new(code);

        Self { lexer, body: vec![], filename: "@debugger".into() }
    }
    
    pub fn error(&mut self, e: Error, start: LineColumn) -> Error {
        unimplemented!()
    }

    pub fn error_line(&mut self, start: LineColumn) -> String {
        let mut idx = start.offset;
        for c in &self.lexer.source[start.offset..] {
            if c.is_es_line_terminator() {
                break;
            }

            idx += 1;
        }

        self.lexer.source[start.offset..idx].iter().collect::<String>()
    }

    pub fn unexpected_token(&mut self, spanned_token: SpannedToken) -> Error {
        let start = spanned_token.start;

        let mut e = Error::new(ErrorKind::LexicalError, "UnexpectedToken");
        let line = self.error_line(start);
        
        e.set_stack(self.filename.as_ref(), start.line, start.column, Some(line));

        e
    }

    pub fn next_token(&mut self) -> Result<Option<SpannedToken>, Error> {
        let start = self.lexer.line_column();
        
        self.lexer.consume();

        let end = self.lexer.line_column();

        match self.lexer.token {
            Token::EndOfProgram => {
                Ok(None)
            },
            Token::UnexpectedEof => {
                let mut e = Error::new(ErrorKind::LexicalError, "UnexpectedEof");
                e.set_stack(self.filename.as_ref(), start.line, start.column, None);
                Err(e)
            },
            Token::UnexpectedToken => {
                let mut e = Error::new(ErrorKind::LexicalError, "UnexpectedToken");
                let line = self.error_line(start);

                e.set_stack(self.filename.as_ref(), start.line, start.column, Some(line));

                Err(e)
            },
            _ => {
                let spanned_token = Span { start, end, item: self.lexer.token.clone() };
                Ok(Some(spanned_token))
            },
        }
    }

    pub fn next_token2(&mut self) -> Result<SpannedToken, Error> {
        let start = self.lexer.line_column();

        match self.next_token()? {
            Some(spanned_token) => Ok(spanned_token),
            None => {
                let mut e = Error::new(ErrorKind::ParseError, "UnexpectedEof");
                e.set_stack(self.filename.as_ref(), start.line, start.column, None);
                Err(e)
            },
        }
    }
    
    pub fn process(&mut self, token: SpannedToken) -> Result<SpannedStatement, Error> {
        match token.item {
            Token::Keyword(Keyword::Var)
            | Token::Keyword(Keyword::Let)
            | Token::Keyword(Keyword::Const)
            | Token::Keyword(Keyword::Function)
            | Token::Keyword(Keyword::Class)
            | Token::Keyword(Keyword::Async) 

            | Token::Punctuator(Punctuator::Semicolon)
            | Token::Keyword(Keyword::Debugger)
             => {
                // Stmt
                return self.parse_statement(token);
            },

            Token::LiteralNull
            | Token::LiteralBoolean(_)
            | Token::LiteralString(_)
            | Token::LiteralDecimalNumeric(_)
            | Token::LiteralFloatNumeric(_)
            | Token::Identifier(_)

            | Token::Punctuator(Punctuator::BackTick)
            | Token::Punctuator(Punctuator::LParen)
            | Token::Punctuator(Punctuator::LBracket)
            | Token::Punctuator(Punctuator::LBrace)

            => {
                // expr
                let spanned_expr = self.parse_expression(token)?;
                let start = spanned_expr.start;
                let end = spanned_expr.end;
                let item = Statement::Expression(Box::new(spanned_expr.item));
                let stmt = Span { start, end, item, };
                
                Ok(stmt)
            },
            _ => {
                Err(self.unexpected_token(token))
            },
        }
    }


    pub fn parse(&mut self) -> Result<(), Error> {
        match self.next_token()? {
            Some(spanned_token) => {
                match spanned_token.item {
                    Token::SingleLineComment | Token::MultiLineComment
                    | Token::WhiteSpaces
                    | Token::LineTerminator => {

                    },
                    Token::HashBang => {

                    },
                    _ => {
                        self.process(spanned_token)?;
                    }
                }
            },
            None => {
                return Ok(());
            }
        }

        loop {
            match self.next_token()? {
                Some(spanned_token) => {
                    match spanned_token.item {
                        Token::SingleLineComment | Token::MultiLineComment
                        | Token::WhiteSpaces
                        | Token::LineTerminator => {
                            continue;
                        },
                        _ => {
                            self.process(spanned_token)?;
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }

        Ok(())
    }
}


pub fn parse(source: &str) {
    let mut code = source.chars().collect::<Vec<char>>();
    code.push('\0'); // EOF

    let mut parser = Parser::new(&code);

    match parser.parse() {
        Ok(_) => {
            trace!("EOF.");
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
