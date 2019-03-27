#![allow(unused_imports)]


#[macro_use]
extern crate log;
extern crate env_logger;
extern crate ecmascript;


use std::mem;
use std::env;
use std::io::{ self, Read, };

use std::rc::{ Rc, };
use std::cell::{ Cell, RefCell, };


fn main() {
    env::set_var("RUST_LOG", "ecmascript=trace,esc=trace");
    env_logger::init();
    
    // println!("{:?}", mem::size_of::<Rc<RefCell<ecmascript::vm::value::String>>>() );
    // println!("{:?}", mem::size_of::<ecmascript::vm::value::String>() );
    // println!("{:?}", mem::size_of::<ecmascript::vm::value::Symbol>() );
    // println!("{:?}", mem::size_of::<ecmascript::vm::value::Object>() );
    // println!("{:?}", mem::size_of::<ecmascript::vm::value::Number>() );
    // println!("{:?}", mem::size_of::<ecmascript::vm::value::Null>() );

    let mut source = String::new();
    io::stdin().read_to_string(&mut source).unwrap();

    // ecmascript::lexer::tokenize(&source);
    ecmascript::parser::parse(&source);
}