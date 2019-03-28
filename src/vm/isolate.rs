use error::Error;
use rc_ref::RcRef;
use vm::value::Value;
use vm::value::symbol::{ SymbolRegistry, SymbolRegistryRef, };
use vm::scope::{ Scope, ScopeRef, };

use std::rc::{ Rc, };
use std::cell::{ Cell, Ref, RefMut, RefCell, };


pub type IsolateRef = RcRef<Isolate>;
#[derive(Debug)]
pub struct Isolate {
    scope_ref: Option<ScopeRef>, // Global
    symbol_register_ref: SymbolRegistryRef,
}

impl Isolate {
    pub fn new() -> Self {
        let symbol_register_ref = SymbolRegistryRef::new(SymbolRegistry::new());
        
        Isolate {
            scope_ref: None,
            symbol_register_ref: symbol_register_ref,
        }
    }

    pub fn set_global_scope(&mut self, scope_ref: ScopeRef) {
        self.scope_ref = Some(scope_ref);
    }

    pub fn global(&self) -> &ScopeRef {
        match self.scope_ref {
            None => panic!("Should set global scope first!"),
            Some(ref scope_ref) => scope_ref,
        }
    }

    pub fn symbol_register_ref(&self) -> &SymbolRegistryRef {
        &self.symbol_register_ref
    }
    
    pub fn run_script(&self, script: &str) -> Result<Value, Error> {
        unimplemented!()
    }

    pub fn run_module(&self, module: &str) -> Result<Value, Error> {
        unimplemented!()
    }
}


#[test]
fn test_isolate() {
    let mut isolate_ref = IsolateRef::new(Isolate::new());

    let global_scope_ref = ScopeRef::new(Scope::new(isolate_ref.clone(), None));
    
    isolate_ref.borrow_mut().set_global_scope(global_scope_ref);

}