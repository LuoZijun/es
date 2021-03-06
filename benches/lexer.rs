#![feature(test)]

extern crate test;
extern crate toolshed;
extern crate ecmascript;

use toolshed::Arena;

use ecmascript::lexer::htmlentity::HTMLEntity;
use ecmascript::lexer::Lexer;
use ecmascript::lexer::escape::{ unescape_string, };
use ecmascript::lexer::numberic::{ parse_numberic, };


// #[bench]
// fn bench_tokenization_v0(b: &mut test::Bencher) {
//     use ecmascript::lexer0::Lexer;
//     use ecmascript::lexer0::token::TokenKind;
    
//     let source = include_str!("../data/react-16.8.3.development.js");
//     let mut code = source.chars().collect::<Vec<char>>();
//     code.push('\0'); // EOF

//     b.bytes = source.len() as _;

//     b.iter(|| {
//         let mut lexer = Lexer::new(&code);
//         loop {
//             let token = lexer.consume();
//             let kind = token.item;

//             if kind == TokenKind::UnexpectedToken {
//                 break;
//             }
//             if kind == TokenKind::UnexpectedEof {
//                 break;
//             }
//             if kind == TokenKind::EndOfProgram {
//                 break;
//             }
//         }
//     })
// }

// #[bench]
// fn bench_tokenization_v1(b: &mut test::Bencher) {
//     use ecmascript::lexer1::Lexer;
//     use ecmascript::lexer1::LexError;

//     let source = include_str!("../data/react-16.8.3.development.js");
//     // let mut code = source.chars().collect::<Vec<char>>();
//     // code.push('\0'); // EOF
    
//     b.bytes = source.len() as _;

//     b.iter(|| {
//         let mut lexer = Lexer::new(&source);

//         loop {
//             match lexer.consume() {
//                 Ok(_token) => {
//                     // println!("{:?}", token);
//                 },
//                 Err(e) => {
//                     if e != LexError::EndOfProgram {
//                         panic!("{:?}", e);
//                     }
//                     break;
//                 }
//             }
//         }
//     })
// }

// #[bench]
// fn bench_tokenization_v2(b: &mut test::Bencher) {
//     use ecmascript::lexer2::Lexer;
//     use ecmascript::lexer2::Token;
    
//     let source = include_str!("../data/react-16.8.3.development.js");
//     // let source = include_str!("../data/colors.js");
//     let mut code = source.chars().collect::<Vec<char>>();
//     code.push('\0'); // EOF

//     b.bytes = source.len() as _;

//     b.iter(|| {
//         let mut lexer = Lexer::new(&code);

//         loop {
//             lexer.consume();
//             let token = &lexer.token;

//             match token {
//                 Token::UnexpectedEof | Token::UnexpectedToken => {
//                     panic!("{:?}", token);
//                 },
//                 Token::EndOfProgram => {
//                     break;
//                 },
//                 _ => {

//                 }
//             }
//         }
//     })
// }

#[bench]
fn bench_tokenization(b: &mut test::Bencher) {
    let source = include_str!("../data/react-16.8.3.development.js");

    let arena = Arena::new();
    let code = arena.alloc_vec(source.chars().collect::<Vec<char>>());
    let filename = arena.alloc_str("src/main.js");
    
    b.bytes = source.len() as u64;
    b.iter(|| {
        let mut lexer = Lexer::new(&arena, &code, &filename);
        loop {
            match lexer.consume() {
                Ok(Some(_token)) => {
                    
                },
                Ok(None) => {
                    // EOF
                    break;
                },
                Err(e) => {
                    panic!("{:?}", e);
                }
            }
        }
    });
}

#[bench]
fn bench_parse_to_ast(b: &mut test::Bencher) {
    let source = include_str!("../in.js");
    let filename = "src/main.js";

    let arena = Arena::new();

    let code = arena.alloc_vec(source.chars().collect::<Vec<char>>());
    let filename = arena.alloc_str(filename);

    b.bytes = source.len() as u64;
    b.iter(|| {
        let mut parser = ecmascript::parser::Parser::new(&arena, &code, &filename);
        let ret = parser.parse();
        assert!(ret.is_ok());
    });
}


#[bench]
fn bench_escape_html_with_str(b: &mut test::Bencher) {
    static BIG_STR: &'static str = include_str!("../data/moonstone-short.txt");
    
    b.bytes = BIG_STR.len() as u64;

    b.iter(|| {
        let _ = BIG_STR.escape_html();
    });
}

#[bench]
fn bench_escape_html_with_bytes(b: &mut test::Bencher) {
    static BIG_STR: &[u8] = include_bytes!("../data/moonstone-short.txt");
    
    b.bytes = BIG_STR.len() as u64;

    b.iter(|| {
        let _ = BIG_STR.escape_html();
    });
}

#[bench]
fn bench_escape_html_with_chars(b: &mut test::Bencher) {
    #[allow(non_snake_case)]
    let BIG_STR: Vec<char> = include_str!("../data/moonstone-short.txt").chars().collect::<Vec<char>>();
    
    b.bytes = BIG_STR.len() as u64;

    b.iter(|| {
        let _ = BIG_STR.escape_html();
    });
}

#[bench]
fn bench_unescape_html_with_chars(b: &mut test::Bencher) {
    #[allow(non_snake_case)]
    let BIG_STR: Vec<char> = include_str!("../data/moonstone-short.txt").chars().collect::<Vec<char>>();
    let escaped = BIG_STR.escape_html();

    let amt = escaped.iter().collect::<String>().len();

    b.bytes = amt as u64;
    b.iter(|| {
        let _ = escaped.unescape_html();
    });
}

#[bench]
fn bench_unescape_html_with_bytes(b: &mut test::Bencher) {
    static BIG_STR: &[u8] = include_bytes!("../data/moonstone-short.txt");
    let escaped = BIG_STR.escape_html();
    let escaped_slice = escaped.as_slice();

    b.bytes = escaped_slice.len() as u64;
    b.iter(|| {
        let _ = escaped_slice.unescape_html();
    });
}

#[bench]
fn bench_unescape_html_with_str(b: &mut test::Bencher) {
    #[allow(non_snake_case)]
    let BIG_STR: &str = include_str!("../data/moonstone-short.txt");
    let escaped = BIG_STR.escape_html();
    let escaped_str = escaped.as_str();

    b.bytes = escaped_str.len() as u64;
    b.iter(|| {
        let _ = escaped_str.unescape_html();
    });
}

#[bench]
fn bench_unescape_ecmascript_string(b: &mut test::Bencher) {
    let input: Vec<char> = r#"我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
我\u{69}\u0069\x69\n\b\v\t\r\n\a\send.\
"#.chars().collect::<Vec<char>>();

    b.bytes = input.len() as u64;
    b.iter(|| {
        let _ = unescape_string(&input);
    });
}

#[bench]
fn bench_parse_ecmascript_float(b: &mut test::Bencher) {
    let input = "0x1232345".chars().collect::<Vec<char>>();

    b.bytes = input.len() as u64;
    b.iter(|| {
        let _ = parse_numberic(&input);
    });
}