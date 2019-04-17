#![cfg_attr(feature = "nightly", feature(test, repeat_generic_slice, duration_constants, const_fn))]


#![recursion_limit="128"]
#![allow(
    unused_imports, unused_variables, unused_must_use, 
    non_snake_case, unreachable_code, dead_code, unused_mut,
    unused_macros,
)]
#![allow(clippy::all)]

#[macro_use]
extern crate log;
extern crate unicode_xid;
extern crate toolshed;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate vlq;


pub mod rc_ref;


pub mod error;
pub mod version;
pub mod ast;

pub mod lexer;
// pub mod lexer0;
// pub mod lexer1;
// pub mod lexer2;

pub mod parser;
pub mod compiler;
pub mod vm;

