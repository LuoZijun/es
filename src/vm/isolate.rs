use error::Error;
use vm::value::Value;
use vm::value::symbol::{ SymbolRegistry, SymbolRegistryRef, };
use vm::scope::{ Scope, ScopeRef, };

use std::rc::{ Rc, };
use std::cell::{ Cell, Ref, RefMut, RefCell, };



#[derive(Debug)]
pub struct Isolate {
    scope_ref: Option<ScopeRef>, // Global
    symbol_register_ref: SymbolRegistryRef,
}

#[derive(Debug)]
pub struct IsolateRef {
    inner: Rc<RefCell<Isolate>>,
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

impl Clone for IsolateRef {
    fn clone(&self) -> Self {
        IsolateRef {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl IsolateRef {
    pub fn new(ctx: Isolate) -> Self {
        IsolateRef { inner: Rc::new(RefCell::new(ctx)) }
    }

    pub fn borrow(&self) -> Ref<Isolate> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<Isolate> {
        self.inner.borrow_mut()
    }
}


#[test]
fn test_isolate() {
    let mut isolate_ref = IsolateRef::new(Isolate::new());

    let global_scope_ref = ScopeRef::new(Scope::new(isolate_ref.clone(), None));
    
    isolate_ref.borrow_mut().set_global_scope(global_scope_ref);

}