use crate::rc_ref::RcRef;
use crate::vm::value::Value;
use crate::vm::value::Undefined;
use crate::vm::value::String;

use std::fmt;
use std::rc::{ Rc, };
use std::cell::{ Cell, Ref, RefMut, RefCell, };

// Well-Known Symbols: https://www.ecma-international.org/ecma-262/9.0/#sec-well-known-symbols
// pub const SYMBOLS: [Option<&[char]>; 12] = [
    
// ]
// pub const ASYNC_ITERATOR: Symbol       = Symbol(1000000000000u128);
// pub const HAS_INSTANCE: Symbol         = Symbol(1000000000000u128);
// pub const IS_CONCAT_SPREADABLE: Symbol = Symbol(1000000000001u128);
// pub const ITERATOR: Symbol             = Symbol(1000000000002u128);
// pub const MATCH: Symbol                = Symbol(1000000000003u128);
// pub const REPLACE: Symbol              = Symbol(1000000000004u128);
// pub const SEARCH: Symbol               = Symbol(1000000000005u128);
// pub const SPECIES: Symbol              = Symbol(1000000000006u128);
// pub const SPLIT: Symbol                = Symbol(1000000000007u128);
// pub const TO_PRIMITIVE: Symbol         = Symbol(1000000000008u128);
// pub const TO_STRING_TAG: Symbol        = Symbol(1000000000009u128);
// pub const UNSCOPABLES: Symbol          = Symbol(1000000000010u128);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SymbolKind {
    Public,
    Private,
}

pub type SymbolRegistryRef = RcRef<SymbolRegistry>;

#[derive(Debug)]
pub struct SymbolRegistry {
    public: Vec<Vec<char>>,
    private: Vec<Option<Vec<char>>>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    kind: SymbolKind,
    id: usize,
    registry_ref: SymbolRegistryRef,
}


impl SymbolRegistry {
    pub fn new() -> Self {
        let public = vec![];
        let private = vec![];

        // TODO: 添加默认的私有符号常量。
        Self { public, private, }
    }

    pub fn len(&self, kind: SymbolKind) -> usize {
        match kind {
            SymbolKind::Public => self.public.len(),
            SymbolKind::Private => self.private.len(),
        }
    }

    pub fn find_public_description(&self, description: &[char]) -> Option<usize> {
        self.public.iter().position(|item| &item[..] == description)
    }

    pub fn get_description(&self, kind: SymbolKind, id: usize) -> Option<&Vec<char>> {
        match kind {
            SymbolKind::Public => {
                Some(&self.public[id])
            },
            SymbolKind::Private => {
                match self.private[id] {
                    Some(ref desc) => Some(&desc),
                    None => None,
                }
            },
        }
    }
}



impl Symbol {
    // CONST
    
}

impl Symbol {
    pub fn new(kind: SymbolKind, description: Option<Vec<char>>, registry_ref: &SymbolRegistryRef) -> Self {
        match kind {
            SymbolKind::Public => {
                // Symbol.for(...)
                let mut registry = registry_ref.borrow_mut();

                let description = description.expect("Must have a description!");

                let id = match registry.find_public_description(&description) {
                    Some(pos) => pos,
                    None => {
                        registry.public.push(description);
                        registry.len(kind)
                    },
                };

                Self { kind: kind, id: id, registry_ref: registry_ref.clone() }
            },
            SymbolKind::Private => {
                // Symbol(...)
                let mut registry = registry_ref.borrow_mut();
                
                registry.private.push(description);

                let id = registry.len(kind);

                Self { kind: kind, id: id, registry_ref: registry_ref.clone() }
            },
        }
    }
    
    // Symbol.keyFor(symbol)
    pub fn key_for(symbol: &Symbol) -> Option<Vec<char>> {
        match symbol.kind {
            SymbolKind::Public => {
                symbol.description()
            },
            SymbolKind::Private => {
                None
            }
        }
    }

    pub fn description(&self) -> Option<Vec<char>> {
        match self.registry_ref.borrow().get_description(self.kind, self.id) {
            Some(desc) => Some(desc.clone()),
            None => None,
        }
    }
}


impl std::hash::Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.id.hash(state);
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        (self.kind == other.kind) && (self.id == other.id)
    }
}

impl Eq for Symbol { }

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.description() {
            None => write!(f, "Symbol()"),
            Some(desc) => write!(f, "Symbol({})", desc.iter().collect::<std::string::String>()),
        }
    }
}


// #[test]
// fn test_symbol() {
//     let mut registry = SymbolRegistry::new();
    
//     assert_ne!(Symbol::HAS_INSTANCE, Symbol::new(Value::Undefined(Undefined), false, &mut registry) );
//     assert_ne!(Symbol::HAS_INSTANCE, Symbol::new(Value::String("Symbol.hasInstance".into()), false, &mut registry) );

//     assert_ne!(
//         Symbol::new(Value::String("".into()), false, &mut registry),
//         Symbol::new(Value::String("".into()), true, &mut registry),
//     );
//     assert_ne!(
//         Symbol::new(Value::Undefined(Undefined), false, &mut registry),
//         Symbol::new(Value::Undefined(Undefined), true, &mut registry),
//     );

//     assert_eq!(
//         Symbol::new(Value::Undefined(Undefined), true, &mut registry).description(&mut registry),
//         Some("undefined".into())
//     );
//     assert_ne!(
//         Symbol::new(Value::Undefined(Undefined), false, &mut registry).description(&mut registry),
//         Some("undefined".into())
//     );
// }