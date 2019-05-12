use std::fmt;
use rustc_hash::FxHashMap;

use crate::error::NativeError;
use crate::value::{ Value, ValueKind, };
use crate::object::{ Object, PropertyKey, Property, };

use std::rc::Rc;
use std::cell::{ Ref, RefCell, };
use std::ptr::NonNull;

// Lexical Environment
// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-lexical-environments
// 
//      Environment Record
//          Global Environment Records
//          Module Environment Records
//          Function Environment Records
//          Block
//          Declarative Environment Records
//          Object Environment Records
// 
/*
Global:
    Module:
        Declarative
        Function
        Block
*/


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RecordKind {
    Var,
    Let,
    Const,
    FunctionDecl,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Record {
    pub kind: RecordKind,
    pub value: Value,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EnvironmentKind {
    Global,
    Module,
    Function,
    Declarative,
    // Object,   // deprecated

    // VAR 申明需要在上级非 Bloc 作用域进行
    Block,
}

#[derive(Debug, Clone)]
pub struct Environment {
    kind: EnvironmentKind,
    records: FxHashMap<String, Record>,
    parent: Option<NonNull<Environment>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(EnvironmentKind::Global)
    }
}

impl Environment {
    #[inline]
    pub fn new(kind: EnvironmentKind) -> Self {
        let records: FxHashMap<String, Record> = FxHashMap::default();
        let parent = None;

        Environment {
            kind,
            records,
            parent,
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const Environment {
        self as *const Environment
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut Environment {
        self as *mut Environment
    }

    #[inline]
    pub fn as_raw_non_null(&mut self) -> NonNull<Environment> {
        NonNull::new(self.as_mut_ptr()).unwrap()
    }

    #[inline]
    pub fn spawn(&mut self, kind: EnvironmentKind) -> Environment {
        let records: FxHashMap<String, Record> = FxHashMap::default();
        let parent = Some(self.as_raw_non_null());
        
        Environment {
            kind,
            records,
            parent,
        }
    }

    #[inline]
    pub fn global(&self) -> &Environment {
        self.parent().map_or(self, |env_ptr| env_ptr.global())
    }

    #[inline]
    pub fn kind(&self) -> EnvironmentKind {
        self.kind
    }

    #[inline]
    pub fn parent(&self) -> Option<&Environment> {
        match self.parent {
            Some(ref env_ptr) => Some(unsafe { env_ptr.as_ref() }),
            None => None,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    #[inline]
    pub fn contains_key<K: AsRef<str>>(&self, k: K) -> bool {
        self.records.contains_key(k.as_ref())
    }

    #[inline]
    pub fn insert<K, V>(&mut self, k: K, kind: RecordKind, v: V) -> Result<(), NativeError>
    where 
        K: Into<String>,
        V: Into<Value>,
    {
        use self::RecordKind::*;

        let key = k.into();
        let value = v.into();

        if let Some(record) = self.records.get(key.as_str()) {
            match (record.kind, kind) {
                (Var, Let)
                | (Var, Const)
                | (FunctionDecl, Let)
                | (FunctionDecl, Const)
                | (Let, _)
                | (Const, _) => {
                    // NOTE: SyntaxError
                    return Err(NativeError::syntax_error(format!("Identifier {:?} has already been declared", key.as_str())));
                },
                _ => { }
            }
        }

        let _ = self.records.insert(key, Record { kind, value, });

        Ok(())
    }

    #[inline]
    pub fn update<K, V>(&mut self, k: K, v: V) -> Result<(), NativeError>
    where 
        K: Into<String>,
        V: Into<Value>,
    {
        use self::RecordKind::*;

        let key = k.into();
        let value = v.into();

        match self.records.get(key.as_str()) {
            Some(record) => {
                let kind = record.kind;
                if kind == RecordKind::Const {
                    return Err(NativeError::type_error("Assignment to constant variable."));
                }

                let _ = self.records.insert(key, Record { kind, value, });

                Ok(())
            },
            None => {
                // WARN: 严格模式下，不允许隐式地使用 `var` 。
                // RecordKind::Var
                return Err(NativeError::reference_error(format!("{:?} is not defined", key.as_str())));
            }
        }
    }

    #[inline]
    pub fn get<K: AsRef<str>>(&self, k: K) -> Result<&Value, NativeError> {
        // NOTE: 向上查找
        self.records.get(k.as_ref())
            .map(|record| &record.value)
            .ok_or(NativeError::reference_error(format!("{:?} is not defined", k.as_ref())))
    }

    #[inline]
    pub fn remove<K: AsRef<str>>(&mut self, k: K) -> Option<Value> {
        self.records.remove(k.as_ref())
            .map(|record| record.value)
    }
}


#[test]
fn test_environment() {
    let mut global = Environment::default();
    assert!(global.parent().is_none());

    let module = global.spawn(EnvironmentKind::Module);
    assert!(module.parent().is_some());
    assert!(module.parent().unwrap().kind() == EnvironmentKind::Global);
}
