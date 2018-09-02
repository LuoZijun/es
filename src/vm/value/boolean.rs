
use vm::value::Value;
use vm::value::Undefined;
use vm::value::Null;
use vm::value::Number;
use vm::value::String;
use vm::value::Symbol;
use vm::value::Object;


use std::fmt;
use std::string;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Boolean(pub bool);


impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            true => write!(f, "true"),
            false => write!(f, "false"),
        }
    }
}


impl From<bool> for Boolean {
    fn from(val: bool) -> Self {
        Boolean(val)
    }
}

impl Into<bool> for Boolean {
    fn into(self) -> bool {
        self.0
    }
}

impl From<Undefined> for Boolean {
    fn from(val: Undefined) -> Self {
        Boolean(false)
    }
}
impl From<Null> for Boolean {
    fn from(val: Null) -> Self {
        Boolean(false)
    }
}
impl From<String> for Boolean {
    fn from(val: String) -> Self {
        Boolean(val.0.len() != 0)
    }
}
impl<'a> From<&'a String> for Boolean {
    fn from(val: &'a String) -> Self {
        Boolean(val.0.len() != 0)
    }
}
impl From<Number> for Boolean {
    fn from(val: Number) -> Self {
        Boolean(val.into())
    }
}
impl From<Object> for Boolean {
    fn from(val: Object) -> Self {
        Boolean(true)
    }
}
impl<'a> From<&'a Object> for Boolean {
    fn from(val: &'a Object) -> Self {
        Boolean(true)
    }
}
impl From<Symbol> for Boolean {
    fn from(val: Symbol) -> Self {
        Boolean(true)
    }
}
impl From<&Symbol> for Boolean {
    fn from(val: &Symbol) -> Self {
        Boolean(true)
    }
}
