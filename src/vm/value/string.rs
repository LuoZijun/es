use crate::error::{ Error, ErrorKind, };
use crate::vm::value::Value;
use crate::vm::value::Undefined;
use crate::vm::value::Null;
use crate::vm::value::Boolean;
use crate::vm::value::Number;
use crate::vm::value::Symbol;
use crate::vm::value::Object;
use crate::vm::value::object::PropertyKey;

use std::fmt;
use std::ops;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct String(pub Vec<char>);


impl String {
    pub fn fromCharCode(code_points: Vec<Value>) -> Result<Self, Error> {
        String::fromCodePoint(code_points)
    }

    pub fn fromCodePoint(code_points: Vec<Value>) -> Result<Self, Error> {
        let mut output: std::string::String = std::string::String::new();

        for val in code_points.iter() {
            match val.toNumber() {
                Ok(n) => output.push_str(&std::char::from_u32(n.into()).unwrap_or(' ').to_string()),
                Err(e) => return Err(e),
            }
        }

        Ok(output.into())
    }

    // String.prototype
    //  { [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: false }
    pub fn charAt(&self, pos: usize) -> Self {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn length(&self) -> Number {
        self.len().into()
    }

    pub fn trim(&self) -> String {
        unimplemented!()
    }
    
}

impl ops::Add for String {
    type Output = String;
    
    fn add(self, other: String) -> Self::Output {
        format!("{}{}", self, other).into()
    }
}
impl<'a> ops::Add for &'a String {
    type Output = String;
    
    fn add(self, other: &'a String) -> Self::Output {
        format!("{}{}", self, other).into()
    }
}


impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().collect::<std::string::String>().fmt(f)
    }
}


impl From<&str> for String {
    fn from(s: &str) -> Self {
        String(s.chars().collect())
    }
}
impl From<std::string::String> for String {
    fn from(s: std::string::String) -> Self {
        String(s.chars().collect())
    }
}
impl<'a> From<&'a std::string::String> for String {
    fn from(s: &'a std::string::String) -> Self {
        String(s.chars().collect())
    }
}


impl From<Undefined> for String {
    fn from(s: Undefined) -> Self {
        s.to_string().into()
    }
}
impl From<Null> for String {
    fn from(s: Null) -> Self {
        s.to_string().into()
    }
}
impl From<Boolean> for String {
    fn from(s: Boolean) -> Self {
        s.to_string().into()
    }
}
impl From<Number> for String {
    fn from(s: Number) -> Self {
        s.to_string().into()
    }
}
impl<'a> From<&'a Number> for String {
    fn from(s: &'a Number) -> Self {
        s.to_string().into()
    }
}
impl From<Object> for String {
    fn from(s: Object) -> Self {
        s.to_string().into()
    }
}
impl<'a> From<&'a Object> for String {
    fn from(s: &'a Object) -> Self {
        s.to_string().into()
    }
}
impl Into<std::string::String> for String {
    fn into(self) -> std::string::String {
        self.0.iter().collect::<std::string::String>()
    }
}

#[test]
fn test_add() {
    let b: String = "世界！".into();
    assert_eq!(b.len(), 3);

    {
        let a: String = "Hello, ".into();
        let b: String = "".into();
        let c = a + b;
        assert_eq!(c, "Hello, ".into());
    }

    {
        let a: String = "Hello, ".into();
        let b: String = "世界！".into();
        assert_eq!( a + b, "Hello, 世界！".into());
    }
}
