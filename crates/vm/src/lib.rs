#![feature(allocator_api, alloc_layout_extra)]
#![allow(
    unused_imports, unused_variables, unused_must_use, 
    non_snake_case, unreachable_code, dead_code, unused_mut,
    unused_macros,
)]

extern crate num;
// TODO: 每日构建版标准库里面的 HashMap 已经是 FxHashMap 了，所以这个只在 std 版本启用
extern crate rustc_hash;


pub mod error;
pub mod env;
pub mod symbol;
pub mod function;
pub mod object;
pub mod value;
pub mod vm;
