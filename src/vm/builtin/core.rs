
// Undefined,
// Null,
// String,
// Bool,
// Symbol,
// Object,
// Number,
// BigInt,

// Function


use crate::error::Error;
use crate::vm::value::Value;


#[derive(Debug, Hash, Clone)]
pub struct Function {
    name: Value,      // anonymous
    args: Vec<Value>,
    // body: Vec<ByteCode>,
}
