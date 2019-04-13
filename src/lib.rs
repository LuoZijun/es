#![feature(repeat_generic_slice, duration_constants, const_fn, test)]
#![recursion_limit="128"]
#![allow(
    unused_imports, unused_variables, unused_must_use, 
    non_snake_case, unreachable_code, dead_code, unused_mut,
    unused_macros,
)]
#![allow(clippy::all)]


#[cfg(test)]
extern crate test;

#[macro_use]
extern crate log;
extern crate unicode_xid;
extern crate toolshed;

pub mod rc_ref;


pub mod error;
pub mod version;
pub mod ast;

pub mod lexer;
// pub mod lexer0;
// pub mod lexer1;

// pub mod parser;
pub mod compiler;
pub mod vm;

