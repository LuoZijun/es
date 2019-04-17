use crate::toolshed::{ Arena, };

use error::{ ErrorKind, Error, };
use version::ECMAScriptVersion;

use lexer::Lexer;
use lexer::token::Token;
use lexer::punctuator::PunctuatorKind;
use lexer::keyword::KeywordKind;
use lexer::LexerErrorKind;

use ast::numberic::{ Numberic, Float, };
use ast::statement::{ 
    SpannedStatement, Statement, StatementList,
    VariableStatement, LexicalDeclaration, LexicalDeclarationKind, LexicalBinding,

};
use ast::expression::{ SpannedExpression, Expression, };

use self::ParserErrorKind::*;


#[derive(Debug)]
pub enum ParserErrorKind {
    UnexpectedToken,
    UnexpectedEOF,
    Custom(&'static str),
}

pub struct Parser<'ast> {
    arena: &'ast Arena,
    lexer: Lexer<'ast>,
    pub token_pool: Vec<Token<'ast>>,

    pub body: Vec<SpannedStatement>,
    pub tokens: Vec<Token<'ast>>,
}

impl<'ast> Parser<'ast> {
    pub fn new(arena: &'ast Arena, source: &'ast [char], filename: &'ast str) -> Self {
        let lexer = Lexer::new(arena, source, filename);

        Self { arena, lexer, body: vec![], token_pool: vec![], tokens: vec![] }
    }
    
    pub fn error(&mut self) -> Error {
        unimplemented!()
    }

    pub fn error_line(&mut self) -> String {
        unimplemented!()
    }

    pub fn unexpected_token(&mut self, token: Token<'ast>) -> Error {
        unimplemented!()
    }

    pub fn token(&mut self) -> Result<Option<Token<'ast>>, Error> {
        self.lexer.consume()
    }

    pub fn token2(&mut self) -> Result<Token<'ast>, Error> {
        // NOTE: 不允许 EOF 的出现
        match self.lexer.consume() {
            Ok(Some(token)) => Ok(token),
            Ok(None) => Err(self.lexer.error(LexerErrorKind::UnexpectedEOF)),
            Err(e) => Err(e),
        }
    }

    pub fn process(&mut self) -> Result<SpannedStatement, Error> {
        // match token.item {
        //     Token::Keyword(Keyword::Var)
        //     | Token::Keyword(Keyword::Let)
        //     | Token::Keyword(Keyword::Const)
        //     | Token::Keyword(Keyword::Function)
        //     | Token::Keyword(Keyword::Class)
        //     | Token::Keyword(Keyword::Async) 

        //     | Token::Punctuator(Punctuator::Semicolon)
        //     | Token::Keyword(Keyword::Debugger)
        //      => {
        //         // Stmt
        //         return self.parse_statement(token);
        //     },

        //     Token::LiteralNull
        //     | Token::LiteralBoolean(_)
        //     | Token::LiteralString(_)
        //     | Token::LiteralDecimalNumeric(_)
        //     | Token::LiteralFloatNumeric(_)
        //     | Token::Identifier(_)

        //     | Token::Punctuator(Punctuator::BackTick)
        //     | Token::Punctuator(Punctuator::LParen)
        //     | Token::Punctuator(Punctuator::LBracket)
        //     | Token::Punctuator(Punctuator::LBrace)

        //     => {
        //         // expr
        //         let spanned_expr = self.parse_expression(token)?;
        //         let start = spanned_expr.start;
        //         let end = spanned_expr.end;
        //         let item = Statement::Expression(Box::new(spanned_expr.item));
        //         let stmt = Span { start, end, item, };
                
        //         Ok(stmt)
        //     },
        //     _ => {
        //         Err(self.unexpected_token(token))
        //     },
        // }
        unimplemented!()
    }

    pub fn parse(&mut self) -> Result<(), Error> {
        loop {
            match self.token()? {
                None => {
                    // NOTE: 只是 Token 流已正常结束 (EOF)
                    break;
                },
                Some(token) => {
                    match token {
                        Token::Keyword(_)
                        | Token::LiteralNull(_)
                        | Token::LiteralBoolean(_) => {
                            // NOTE: 在 Lexer 层，这些会被当作 Identifier 处理。
                            unreachable!()
                        },
                        Token::LiteralRegularExpression(_)
                        | Token::LiteralTemplate(_) => {
                            // NOTE: 考虑到这些 Token 边界有歧义，需要上下文来处理。
                            //       所以 Token 将会由 Parser 生成，而不是 Lexer.
                            unreachable!()
                        },
                        Token::LineTerminator => {
                            unimplemented!()
                        },
                        Token::Identifier(ident) => {
                            // NOTE: 当一个表达式的第一个 Token 是 Identifier 时，
                            //       如果这个 Identifier 能够被转换成 Keyword/LiteralNull/LiteralBoolean ,
                            //       那么它会被视作 Keyword/LiteralNull/LiteralBoolean Token.
                            unimplemented!()
                        },
                        Token::Punctuator(punct) => {
                            unimplemented!()
                        },
                        Token::LiteralString(lit_str) => {
                            unimplemented!()
                        },
                        Token::LiteralNumeric(lit_num) => {
                            unimplemented!()
                        },
                        Token::TemplateOpenning => {
                            // NOTE: 这将会指示 Parser 生成一个 LiteralTemplate Token.
                            unimplemented!()
                        },
                    }
                },
            }
        }
        
        Ok(())
    }
}

pub fn parse(source: &str, filename: &str) {
    let arena = Arena::new();

    let code = arena.alloc_vec(source.chars().collect::<Vec<char>>());
    let filename = arena.alloc_str(filename);

    let mut parser = Parser::new(&arena, &code, &filename);
    match parser.parse() {
        Ok(_) => {
            println!("TokenList:\n{:?}", parser.tokens);
            println!("StatementList:\n{:?}", parser.body);
            trace!("EOF.");
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
