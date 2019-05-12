use rustc_hash::FxHashMap;

use crate::value::{ Value, ValueKind, };
use crate::error::NativeError;
use crate::env::{ Environment, EnvironmentKind, RecordKind, };
use crate::object::{ Object, Property, PropertyKey, };
use crate::function::{ Function, FunctionCode, };


use std::ptr::NonNull;


pub type ByteCode = Vec<u8>;


#[derive(Debug)]
pub struct Module {
    codes: ByteCode,
}


// I/O Task
pub struct Task {

}


#[derive(Debug)]
pub struct Allocator {
    store: Vec<*mut u8>,
}

impl Allocator {
    pub fn new() -> Self {
        let store = Vec::new();
        Self { store }
    }

    pub fn alloc() {
        unimplemented!()
    }

    pub fn gc(&mut self) {

    }

    pub fn mark(&mut self) {

    }

    pub fn sweep(&mut self) {

    }
}

#[derive(Debug)]
pub struct Vm {
    allocator: Allocator,
    modules: FxHashMap<String, Module>,
    // scripts: ByteCode,
    // tasks: Vec<Task>,
    envs: Vec<Environment>,

    // object_prototype: NonNull<Object>,
    // function_prototype: NonNull<Object>,
}

impl Vm {
    pub fn new() -> Self {
        let allocator = Allocator::new();
        let modules = FxHashMap::default();
        let mut global_object = Object::global("global");
        let global_value = Value::Object(global_object.as_raw_non_null());

        let mut global_env = Environment::new(EnvironmentKind::Global);
        global_env.insert("global", RecordKind::Const, global_value);
        let envs = vec![ global_env ];

        Vm {
            allocator,
            modules,
            envs,
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const Vm {
        self as *const Vm
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut Vm {
        self as *mut Vm
    }

    #[inline]
    pub fn as_raw_non_null(&mut self) -> NonNull<Vm> {
        NonNull::new(self.as_mut_ptr()).unwrap()
    }

    pub fn environment(&self) -> &Environment {
        debug_assert!(self.envs.len() > 0);
        &self.envs.last().unwrap()
    }

    pub fn evaluate(&mut self, bytecode: &[u8]) -> Result<Value, NativeError> {
        unimplemented!()
    }
}
