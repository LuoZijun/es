use std::fmt;
use std::ptr::NonNull;
use rustc_hash::FxHashMap;


use crate::error::NativeError;
use crate::value::{ Value, ValueKind, };
use crate::symbol::Symbol;
use crate::function::{ Function, FunctionCode, NativeFunction, };
use crate::vm::{ Vm, };




const PROP_PROTOTYPE: &'static str = "prototype";


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PropertyKey {
    String(String),
    Symbol(Symbol),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub enumerable: bool,
    pub configurable: bool,
    
    // DataPropertyDescriptor
    pub value: Value, // NOTE: Any ECMAScript Value
    pub writable: bool,

    // AccessorPropertyDescriptor
    pub getter: Value,  // WARN: Value::Undefined || Value::Object<+ Callable>
    pub setter: Value,  // WARN: Value::Undefined || Value::Object<+ Callable>
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ObjectKind {
    Function,
    ArrowFunction,
    RegExp,
    Map,
    Set,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub properties: FxHashMap<PropertyKey, Property>,
    // pub prototype: NonNull<Object>,
    pub is_frozen: bool,
    pub is_sealed: bool,
    pub is_extensible: bool,
}

impl Object {
    #[inline]
    pub fn empty() -> Self {
        Self {
            properties: FxHashMap::default(),
            is_frozen: false,
            is_sealed: false,
            is_extensible: false,
        }
    }

    #[inline]
    pub fn new() -> Self {
        Self::empty()
    }

    #[inline]
    pub fn with_prototype(prototype: &Object) -> Self {
        let mut object = Self::empty();
        
        for (key, value) in &prototype.properties {
            object.insert(key, value.clone());
        }

        object
    }

    #[inline]
    pub fn as_ptr(&self) -> *const Object {
        self as *const Object
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut Object {
        self as *mut Object
    }

    #[inline]
    pub fn as_raw_non_null(&mut self) -> NonNull<Object> {
        NonNull::new(self.as_mut_ptr()).unwrap()
    }

    // Object Prototype:
    //     constructor: function Object()
    //     hasOwnProperty: function hasOwnProperty()
    //     isPrototypeOf: function isPrototypeOf()
    //     propertyIsEnumerable: function propertyIsEnumerable()
    //     toLocaleString: function toLocaleString()
    //     toSource: function toSource()
    //     toString: function toString()
    //     valueOf: function valueOf()
    #[inline]
    pub fn prototype() -> Self {
        // NOTE: 先有了 ObjectPrototype ，再有 Object
        let mut obj = Self::empty();

        let constructor = Property {
            enumerable: false,
            configurable: true,
            value: Value::Null,
            writable: true,
            getter: Value::Undefined,
            setter: Value::Undefined,
        };
        let toString = Property {
            enumerable: false,
            configurable: true,
            value: Value::Null,
            writable: true,
            getter: Value::Undefined,
            setter: Value::Undefined,
        };
        let toSource = Property {
            enumerable: false,
            configurable: true,
            value: Value::Null,
            writable: true,
            getter: Value::Undefined,
            setter: Value::Undefined,
        };

        obj.insert("constructor", constructor);
        obj.insert("toString", toString);
        obj.insert("toSource", toSource);

        obj
    }

    #[inline]
    pub fn is_extensible(&self) -> bool {
        self.is_extensible
    }

    #[inline]
    pub fn is_sealed(&self) -> bool {
        self.is_sealed
    }

    #[inline]
    pub fn is_frozen(&self) -> bool {
        self.is_frozen
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
    
    #[inline]
    pub fn keys(&self) -> Vec<&PropertyKey> {
        self.properties.keys().collect::<Vec<&PropertyKey>>()
    }

    #[inline]
    pub fn values(&self) -> Vec<&Property> {
        self.properties.values().collect::<Vec<&Property>>()
    }

    #[inline]
    pub fn get<K: Into<PropertyKey>>(&self, k: K) -> Option<&Property> {
        self.properties.get(&k.into())
    }

    #[inline]
    pub fn insert<K: Into<PropertyKey>, >(&mut self, k: K, v: Property) {
        self.properties.insert(k.into(), v);
    }

    #[inline]
    pub fn remove<K: Into<PropertyKey>>(&mut self, k: K) {
        unimplemented!()
    }


    #[inline]
    pub fn global<T: Into<String>>(name: T) -> Self {
        let mut object_prototype = Object::empty();
        let mut function_prototype = Object::empty();

        let mut global = Object::empty();

        // function Object() { }
        let mut object_function = Function::new(
            "Object",
            FunctionCode::NativeCode(|vm: NonNull<Vm>| -> Result<Value, NativeError> {
                unimplemented!()
            }),
            &mut function_prototype,
        );
        let mut object_function_value = Value::Function(NonNull::new(&mut object_function).unwrap());
        
        // function Function() { }
        let mut function_function = Function::new(
            "Function",
            FunctionCode::NativeCode(|vm: NonNull<Vm>| -> Result<Value, NativeError> {
                unimplemented!()
            }),
            &mut function_prototype,
        );
        let mut function_function_value = Value::Function(NonNull::new(&mut function_function).unwrap());

        global.insert("Object", Property {
            enumerable: false,
            configurable: true,
            value: object_function_value.clone(),
            writable: true,
            getter: Value::Undefined,
            setter: Value::Undefined,
        });
        global.insert("Function", Property {
            enumerable: false,
            configurable: true,
            value: function_function_value.clone(),
            writable: true,
            getter: Value::Undefined,
            setter: Value::Undefined,
        });
        
        // Update Object Prototype and Function Prototype
        function_prototype.insert("constructor", Property {
            enumerable: false,
            configurable: true,
            value: function_function_value.clone(),
            writable: true,
            getter: Value::Undefined,
            setter: Value::Undefined,
        });

        object_prototype.insert("constructor", Property {
            enumerable: false,
            configurable: true,
            value: object_function_value.clone(),
            writable: true,
            getter: Value::Undefined,
            setter: Value::Undefined,
        });
        
        let mut object_prototype_toString = Function::new(
            "toString",
            FunctionCode::NativeCode(|vm: NonNull<Vm>| -> Result<Value, NativeError> {
                unimplemented!()
            }),
            &mut function_prototype,
        );
        let mut object_prototype_toString_value = Value::Function(NonNull::new(&mut object_prototype_toString).unwrap());
        object_prototype.insert("toString", Property {
            enumerable: false,
            configurable: true,
            value: object_prototype_toString_value,
            writable: true,
            getter: Value::Undefined,
            setter: Value::Undefined,
        });

        let mut object_prototype_toSource = Function::new(
            "toSource",
            FunctionCode::NativeCode(|vm: NonNull<Vm>| -> Result<Value, NativeError> {
                unimplemented!()
            }),
            &mut function_prototype,
        );
        let mut object_prototype_toSource_value = Value::Function(NonNull::new(&mut object_prototype_toSource).unwrap());
        object_prototype.insert("toSource", Property {
            enumerable: false,
            configurable: true,
            value: object_prototype_toSource_value,
            writable: true,
            getter: Value::Undefined,
            setter: Value::Undefined,
        });

        // Update Global Object
        let global_value = Value::Object(global.as_raw_non_null());
        global.insert(name.into(), Property {
            enumerable: true,
            configurable: false,
            value: global_value,
            writable: false,
            getter: Value::Undefined,
            setter: Value::Undefined,
        });

        global
    }
}



impl From<&PropertyKey> for PropertyKey {
    fn from(s: &PropertyKey) -> Self {
        s.clone()
    }
}

impl From<&str> for PropertyKey {
    fn from(s: &str) -> Self {
        PropertyKey::String(s.to_string())
    }
}
impl From<String> for PropertyKey {
    fn from(s: String) -> Self {
        PropertyKey::String(s)
    }
}

// impl PartialEq for Object {
//     fn eq(&self, other: &Object) -> bool {
//         false
//     }
// }