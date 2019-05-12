use rustc_hash::FxHashMap;
use num::{ Zero, One, Float, ToPrimitive, FromPrimitive, bigint::ToBigInt, bigint::ToBigUint, };
pub use num::{ BigInt, BigUint, complex::Complex32, complex::Complex64, Rational, BigRational, };

use crate::error::NativeError;
use crate::symbol::Symbol;
use crate::function::{ Function, FunctionCode, NativeFunction, };
use crate::object::{ Object, PropertyKey, Property, };

use std::fmt;
use std::cmp;
use std::ptr::NonNull;


#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ValueKind {
    // Copy Value
    Undefined = 0u8,
    Null,
    I64,
    F64,
    Boolean,
    Symbol,
    // Copy Ref
    // Heap RcRef Value
    String,
    BigInt,
    Complex,

    // Heap RcGcRef Value
    // Note: Rc<Gc<Value>>
    //       当 Rc 计数为 0 时，触发 GC.
    Function,
    Object,
}

// TODO: 考虑 Tagged Pointer
// https://blog.devtang.com/2014/05/30/understand-tagged-pointer/
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Undefined,
    Null,
    I64(i64),
    F64(f64),
    Symbol(Symbol),
    Boolean(bool),

    String(NonNull<String>),
    BigInt(NonNull<BigInt>),

    Function(NonNull<Function>),
    Object(NonNull<Object>),
}


pub trait Cast<T> {
    // Safe
    fn cast(self) -> Result<T, NativeError>;
    // Unsafe
    fn bitcast(self) -> T;
}

pub trait Copy: Clone {
    // shallow copy
    fn copy(&self) -> Self;
    fn deepcopy(&self) -> Self;
}


impl Cast<i64> for Value {
    // Safe
    fn cast(self) -> Result<i64, NativeError> {
        unimplemented!()
    }

    fn bitcast(self) -> i64 {
        unimplemented!()
    }
}



