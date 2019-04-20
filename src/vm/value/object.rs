
use crate::vm::value::Value;
use crate::vm::value::Undefined;
use crate::vm::value::Boolean;
use crate::vm::value::String;
use crate::vm::value::Symbol;
// use builtin::Function;


use std::fmt;
use std::cmp;
use std::hash;
use std::string;
use std::vec::Vec;
use std::default::Default;
use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::BTreeSet;

// Fundamental Objects
// 19.1 Object Objects
// 19.2 Function Objects
// 19.3 Boolean Objects
// 19.4 Symbol Objects
// 19.5 Error Objects

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum ObjectKind {
    Normal,   // hasConstructor() == false && isCallable() == false
    Function, // isCallable() == true && hasConstructor() == true || false
    Instance, // hasConstructor() == true
}

#[derive(Debug, Clone)]
pub struct Object {
    pub properties: HashMap<PropertyKey, Property>,
    pub is_frozen: bool,
    pub is_sealed: bool,
    pub is_extensible: bool,
    pub kind: ObjectKind,
    // pub constructor: Option<Function>,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PropertyKey {
    String(String),
    Symbol(Symbol),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Property {
    pub descriptor: PropertyDescriptor,
    pub enumerable: bool,
    pub configurable: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PropertyDescriptor {
    Data(DataPropertyDescriptor),
    Accessor(AccessorPropertyDescriptor),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DataPropertyDescriptor {
    pub value: Box<Value>, // NOTE: Any ECMAScript Value
    pub writable: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AccessorPropertyDescriptor {
    pub get: Box<Value>,  // WARN: Value::Undefined || Value::Object<+ Callable>
    pub set: Box<Value>,  // WARN: Value::Undefined || Value::Object<+ Callable>
}



impl hash::Hash for Object {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (self as *const Object).hash(state);
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        (self as *const Object).eq( &(other as *const Object) )
    }
}

impl Eq for Object {}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Object) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Object {
    fn cmp(&self, other: &Object) -> cmp::Ordering {
        (self as *const Object).cmp( &(other as *const Object) )
    }
}


impl Object {
    pub fn get(&self) -> Option<Value> {

        unimplemented!()
    }

    pub fn set(&mut self, key: Value) -> bool {
        self.properties.insert(key.into(), Property::default());
        true
    }

    pub fn construct(&self) -> Option<Value> {
        unimplemented!()
    }
}


impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[object Object]")
    }
}


impl Default for DataPropertyDescriptor {
    fn default() -> Self {
        DataPropertyDescriptor {
            value: Box::new(Undefined.into()),
            writable: false
        }
    }
}

impl Default for AccessorPropertyDescriptor {
    fn default() -> Self {
        AccessorPropertyDescriptor {
            get: Box::new(Undefined.into()),
            set: Box::new(Undefined.into()),
        }
    }
}

impl Default for PropertyDescriptor {
    fn default() -> Self {
        PropertyDescriptor::Data(DataPropertyDescriptor::default())
    }
}

impl Default for Property {
    fn default() -> Self {
        Property {
            descriptor: PropertyDescriptor::default(),
            enumerable: false,
            configurable: false,
        }
    }
}


impl fmt::Display for PropertyKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PropertyKey::String(ref s) => s.fmt(f),
            PropertyKey::Symbol(ref s) => s.fmt(f),
        }
    }
}

impl fmt::Display for PropertyDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PropertyDescriptor::Data(ref data_prop_descptor) => data_prop_descptor.fmt(f),
            PropertyDescriptor::Accessor(ref accesor_prop_descptor) => accesor_prop_descptor.fmt(f),
        }
        
    }
}

impl fmt::Display for DataPropertyDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value: {}, writable: {}", self.value, self.writable)
    }
}

impl fmt::Display for AccessorPropertyDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "get: {}, set: {}", self.get, self.set)
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, enumerable: {}, configurable: {} }}", self.descriptor, self.enumerable, self.configurable)
    }
}


// impl From<Vec<Value>> for Object {
//     fn from(vec: Vec<Value>) -> Self {
//         unimplemented!()
//     }
// }

// impl From<VecDeque<Value>> for Object {
//     fn from(vec: Vec<Value>) -> Self {
//         unimplemented!()
//     }
// }

