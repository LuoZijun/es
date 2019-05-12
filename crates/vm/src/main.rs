#![feature(allocator_api, alloc_layout_extra)]
#![allow(
    unused_imports, unused_variables, unused_must_use, 
    non_snake_case, unreachable_code, dead_code, unused_mut,
    unused_macros,
)]


use std::ptr::{ NonNull, };
use std::io::{ self, Write, Read, };
use std::alloc::{ alloc, dealloc, Global, System, GlobalAlloc, Layout, Alloc, };
use std::rc::{ Rc, };
use std::cell::{ Cell, Ref, RefMut, RefCell, };

use std::mem;
use std::ops::Deref;


use rustc_hash::FxHashMap;


use vm::object::Object;
use vm::value::Value;
use vm::vm::Vm;


fn main() {
    let mut vm = Vm::new();
    println!("{:#?}", vm);
}


fn str_from_raw<'a>(ptr: *const u8, len: usize) -> &'a str {
    unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
    }
}