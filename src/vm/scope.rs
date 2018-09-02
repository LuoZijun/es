
use rc_ref::RcRef;
use vm::isolate::IsolateRef;
use vm::value::Value;


use std::rc::{ Rc, };
use std::cell::{ Cell, Ref, RefMut, RefCell, };
use std::collections::HashMap;


// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-lexical-environments
#[derive(Debug)]
pub enum ScopeLevel {
    Top,      // outside any function, global
    Function, // 
    Class,    // Object
    Block,    // Local
    // Catch,
    // Try
}



pub type ScopeRef = RcRef<Scope>;


#[derive(Debug)]
pub struct Scope {
    isolate_ref: IsolateRef,
    parent: Option<ScopeRef>,
    record: HashMap<std::string::String, Value>,
}

impl Scope {
    pub fn new(isolate_ref: IsolateRef, parent: Option<ScopeRef>) -> Self {
        Scope {
            isolate_ref: isolate_ref,
            parent: parent,
            record: HashMap::new(),
        }
    }
    
    pub fn is_root(&self) -> bool {
        // Global
        self.parent.is_none()
    }

    pub fn isolate_ref(&self) -> &IsolateRef {
        &self.isolate_ref
    }

    pub fn parent(&self) -> &Option<ScopeRef> {
        &self.parent
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.record.get(key)
    }

    pub fn set(&mut self, key: std::string::String, val: Value) -> Option<Value> {
        self.record.insert(key, val)
    }
}


impl ScopeRef {
    pub fn is_root(&self) -> bool {
        self.borrow().is_root()
    }

    pub fn fork(&self) -> ScopeRef {
        let isolate_ref = self.borrow().isolate_ref().clone();
        let parent = self.clone();
        
        ScopeRef::new(Scope::new(isolate_ref, Some(parent)))
    }
}
