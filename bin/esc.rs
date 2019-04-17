#![allow(unused_imports)]


#[macro_use]
extern crate log;
extern crate env_logger;
extern crate ecmascript;

use std::mem;
use std::env;
use std::io::{ self, Read, };


fn main() {
    env::set_var("RUST_LOG", "ecmascript=trace,esc=trace");
    env_logger::init();

    let filename = "src/main.js";
    let mut source = String::new();
    io::stdin().read_to_string(&mut source).unwrap();
    
    // ecmascript::lexer::tokenize(&source, &filename);
    ecmascript::parser::parse(&source, &filename);
}