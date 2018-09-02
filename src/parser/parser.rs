use version::ECMAScriptVersion;
use lexer::{ ESChar, Lexer, Token, SpannedToken, Punctuator, Keyword, };

// use crate::ast::Node;
use ast::span::{ LineColumn, Span, };
use ast::float::Float;
use ast::statement::{ 
    Statement, StatementList,
    LexicalDeclaration, LexicalDeclarationKind, LexicalBinding,

};
use ast::expression::{ Expression, };



#[derive(Debug)]
pub enum ParseError {
    UnexpectedEndOfProgram,
    UnexpectedToken {
        source: String,
        line_column: LineColumn,
    },
}


pub struct Parser<'a> {
    start: LineColumn,
    lexer: Lexer<'a>,
    body: StatementList,
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a [char]) -> Self {
        let lexer = Lexer::new(code);
        let start = LineColumn::default();
        Self { start, lexer, body: vec![] }
    }
    
    fn next_token(&mut self) -> Result<SpannedToken, ParseError> {
        self.lexer.consume();
        unimplemented!()
    }

    pub fn process(&mut self) -> Result<(), ParseError>{
        let token = self.lexer.token.clone();

        match token {
            Token::WhiteSpaces => Ok(()),
            Token::LineTerminator => Ok(()),
            Token::Keyword(kw) => {
                match kw {
                    Keyword::Let => {
                        // let
                        self.next_token();

                    },
                    Keyword::Const => {
                        // const

                    },
                    Keyword::Var => {
                        // var

                    },
                    _ => {
                        
                    }
                }

                unimplemented!()
            },
            Token::Punctuator(punct) => {
                match punct {
                    Punctuator::BackTick => {
                        // `

                    },
                    Punctuator::Semicolon => {
                        // :

                    },
                    Punctuator::LParen => {
                        // (
                    },
                    Punctuator::LBracket => {
                        // [

                    },
                    Punctuator::LBrace => {
                        // {

                    },
                    _ => {

                    }
                }
                
                unimplemented!()
            },
            Token::Identifier(ident) => {
                // let i: Vec<char> = *ident;
                unimplemented!()
            },
            Token::LiteralNull => {
                unimplemented!()
            },
            Token::LiteralBoolean(val) => {
                unimplemented!()
            },
            Token::LiteralString(val) => {
                unimplemented!()
            },
            Token::LiteralDecimalNumeric(val) => {
                unimplemented!()
            },
            Token::LiteralFloatNumeric(val) => {
                unimplemented!()
            },
            _ => unreachable!(),
        }
    }

    fn error(&mut self) -> Result<(), ParseError> {
        match self.lexer.token {
            Token::UnexpectedEof => {
                return Err(ParseError::UnexpectedEndOfProgram);
            },
            Token::UnexpectedToken => {
                let mut idx = self.start.offset;
                let mut line_str: Vec<char> = vec![];
                
                for c in &self.lexer.source[self.start.offset..] {
                    if c.is_es_line_terminator() {
                        break;
                    }

                    idx += 1;
                }

                let line = self.lexer.source[self.start.offset..idx].iter().collect::<String>();

                return Err(ParseError::UnexpectedToken {
                    source: line,
                    line_column: self.start,
                });
            },
            _ => unreachable!(),
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        loop {
            self.start = self.lexer.line_column();

            self.lexer.consume();
            
            let end = self.lexer.line_column();

            match self.lexer.token {
                Token::EndOfProgram => {
                    return Ok(());
                },
                Token::UnexpectedEof | Token::UnexpectedToken => {
                    return self.error();
                },
                _ => {
                    self.process()?;
                },
            }
        }
    }

    fn parse_statement(&mut self) {

    }
    
    fn parse_declaration(&mut self) {

    }

    fn parse_expression(&mut self) {

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
            error!("{:?}", e);
        }
    }
}
