use crate::value::{ Value, ValueKind, };
use crate::error::NativeError;
use crate::env::{ Environment, };
use crate::object::{ Object, Property, PropertyKey, };
use crate::vm::{ Vm, };


use std::fmt;
use std::ptr::NonNull;


pub type NativeFunction = fn(NonNull<Vm>) -> Result<Value, NativeError>;


pub fn isNaN(vm: NonNull<Vm>) -> Result<Value, NativeError> {
    unimplemented!()
}


#[derive(Debug, PartialEq)]
pub enum FunctionCode {
    NativeCode(NativeFunction),
    ByteCode(NonNull<Vec<u8>>),
}

impl Clone for FunctionCode {
    fn clone(&self) -> Self {
        match *self {
            FunctionCode::NativeCode(f) => FunctionCode::NativeCode(f),
            FunctionCode::ByteCode(f) => FunctionCode::ByteCode(f),
        }
    }
}

// IsConstructor
// IsCallable
#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    name: Option<String>,
    code: FunctionCode,
    // prototype
    object: NonNull<Object>,
}

impl Function {
    #[inline]
    pub fn new<N: Into<Option<S>>, S: Into<String>>(name: N, code: FunctionCode, object: &mut Object) -> Self {
        Self {
            name: name.into().map(|s| s.into()),
            code,
            object: object.as_raw_non_null()
        }
    }

    pub fn set_name<T: Into<String>>(&mut self, name: T) {
        let name: &String = &name.into();
        let value = unsafe {
            Value::String(NonNull::new( std::mem::transmute::<&String, *mut String>(name) ).unwrap())
        };
        self.object_mut().insert("name", Property {
            enumerable: false,
            configurable: false,
            value: value,
            writable: false,
            getter: Value::Undefined,
            setter: Value::Undefined,
        });
    }

    #[inline]
    pub fn name(&self) -> &Value {
        match &self.object().get("name") {
            Some(ref property) => &property.value,
            None => unreachable!(),
        }
    }

    #[inline]
    pub fn code(&self) -> &FunctionCode {
        &self.code
    }

    #[inline]
    pub fn object(&self) -> &Object {
        unsafe { self.object.as_ref() }
    }

    #[inline]
    pub fn object_mut(&mut self) -> &mut Object {
        unsafe { self.object.as_mut() }
    }
}


