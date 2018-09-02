use error::{ Error, ErrorKind, };
use vm::value::Undefined;
use vm::value::Null;
use vm::value::Boolean;
use vm::value::String;
use vm::value::Number;
use vm::value::Symbol;
use vm::value::Object;
use vm::value::object::PropertyKey;


use std::hash;
use std::fmt;
use std::cmp;
use std::ops;
use std::mem;
use std::rc::{ Rc, };
use std::cell::{ Cell, Ref, RefMut, RefCell, };
use std::marker::PhantomData;
use std::collections::HashMap;


// pub type ValueRef = Rc<RefCell<Value>>;

#[derive(Debug)]
pub struct Value3 {
    kind: ValueKind,
    val: *mut u8,
}

#[derive(Debug)]
pub enum Value2 {
    Undefined,
    Null,
    Boolean(RcRefCell<Boolean>),
    String(RcRefCell<String>),
    Number(RcRefCell<Number>),
    Symbol(RcRefCell<Symbol>),
    Object(RcRefCell<Object>),
}

#[derive(Debug)]
pub struct RcRefCell<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Clone for RcRefCell<T> {
    fn clone(&self) -> Self {
        RcRefCell {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T> RcRefCell<T> {
    pub fn new(val: T) -> Self {
        RcRefCell { inner: Rc::new(RefCell::new(val)) }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}


#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum ValueKind {
    Undefined,
    Null,
    Boolean,
    String,
    Number,
    Symbol,
    Object,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Value {
    Undefined(Undefined),
    Null(Null),
    Boolean(Boolean),
    String(String),
    Number(Number),
    Symbol(Symbol),
    Object(Object),
}


impl Value {
    pub fn kind(&self) -> ValueKind {
        use self::Value::*;

        match *self {
            Undefined(_) => ValueKind::Undefined,
            Null(_)      => ValueKind::Null,
            Boolean(_)   => ValueKind::Boolean,
            String(_)    => ValueKind::String,
            Number(_)    => ValueKind::Number,
            Symbol(_)    => ValueKind::Symbol,
            Object(_)    => ValueKind::Object,
        }
    }

    pub fn is_primitive(&self) -> bool {
        use self::ValueKind::*;

        match self.kind() {
            Object => false,
            _ => true,
        }
    }

    pub fn is_undefined(&self) -> bool {
        use self::ValueKind::*;

        match self.kind() {
            Undefined => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        use self::ValueKind::*;

        match self.kind() {
            Null => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        use self::ValueKind::*;

        match self.kind() {
            Boolean => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        use self::ValueKind::*;

        match self.kind() {
            String => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        use self::ValueKind::*;

        match self.kind() {
            Number => true,
            _ => false,
        }
    }

    pub fn is_symbol(&self) -> bool {
        use self::ValueKind::*;

        match self.kind() {
            Symbol => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        use self::ValueKind::*;

        match self.kind() {
            Object => true,
            _ => false,
        }
    }

    pub fn toPrimitive(&self, preferred_type: &str) -> Result<Value, Error>{
        use self::Value::*;

        match self {
            Undefined(s)  => Ok(self.clone()),
            Null(s)       => Ok(self.clone()),
            Boolean(s)    => Ok(self.clone()),
            String(ref s) => Ok(self.clone()),
            Number(s)     => Ok(self.clone()),
            Symbol(ref s)     => Ok(self.clone()),
            Object(ref s) => {
                unimplemented!()
            },
        }
    }

    pub fn toBoolean(&self) -> Boolean {
        use self::Value::*;

        match *self {
            Undefined(s)  => s.into(),
            Null(s)       => s.into(),
            Boolean(s)    => s,
            String(ref s) => s.into(),
            Number(s)     => s.into(),
            Symbol(ref s)     => s.into(),
            Object(ref s) => s.into(),
        }
    }

    pub fn toNumber(&self) -> Result<Number, Error> {
        use self::Value::*;

        use std::f64::NAN;

        match *self {
            Undefined(s)  => Ok(s.into()),
            Null(s)       => Ok(s.into()),
            Boolean(s)    => Ok(s.into()),
            String(ref s) => Ok(s.into()),
            Number(s)     => Ok(s.into()),
            // FIXME: Return JSValue (TypeError)
            Symbol(_)     => Err(Error::new(ErrorKind::TypeError, "Cannot convert a Symbol value to a number")),
            Object(ref s) => Ok(s.into()),
        }
    }

    pub fn toString(&self) -> Result<String, Error> {
        use self::Value::*;

        match *self {
            Undefined(s)  => Ok(s.into()),
            Null(s)       => Ok(s.into()),
            Boolean(s)    => Ok(s.into()),
            String(ref s) => Ok(s.clone()),
            Number(s)     => Ok(s.into()),
            // FIXME: Return JSValue ( TypeError )
            Symbol(_)     => Err(Error::new(ErrorKind::TypeError, "Cannot convert a Symbol value to a string")),
            Object(ref s) => Ok(s.into()),
        }
        /*
        Object.toString:
            // ? ToPrimitive(argument, hint String)
            match self.toPrimitive("string") {
                Ok(primitive) => primitive.toString(),
                Err(_) => Err(())
            }
        */
    }

    pub fn toObject(&self) -> Result<Object, Object> {
        use self::Value::*;

        match *self {
            // FIXME: Return JSValue ( TypeError )
            Undefined(_)   => Err(panic!("Cannot convert a Undefined value to a object".to_string())),
            // FIXME: Return JSValue ( TypeError )
            Null(_)        => Err(panic!("Cannot convert a Null value to a object".to_string())),
            // Boolean(s)     => Ok(s.into()),
            // String(ref s)  => Ok(s.into()),
            // Number(n)      => Ok(n.into()),
            // Symbol(s)      => Ok(s.into()),
            // Object(ref s)  => Ok(s.into()),
            _ => unimplemented!()
        }
    }

    // pub fn max(&self) -> bool {
    //     true
    // }

    // pub fn eq(&self) -> bool {
    //     unimplemented!()
    // }
    pub fn ops_delete(&self) -> bool {
        false
    }
    pub fn ops_void(&self) -> bool {
        false
    }
    pub fn ops_typeof(&self) -> bool {
        false
    }
    
}


impl ops::Add for Value {
    type Output = Result<Value, Error>;

    fn add(self, other: Value) -> Self::Output {
        let is_string = self.is_string() || other.is_string();

        if is_string {
            match self.toString() {
                Ok(a) => match other.toString() {
                    Ok(b) => Ok((a + b).into()),
                    Err(e) => Err(e)
                },
                Err(e) => Err(e)
            }
        } else {
            // Number
            match self.toNumber() {
                Ok(a) => match other.toNumber() {
                    Ok(b) => Ok((a + b).into()),
                    Err(e) => Err(e)
                },
                Err(e) => Err(e)
            }
        }
    }
}

impl ops::Sub for Value {
    type Output = Result<Value, Error>;

    fn sub(self, other: Value) -> Self::Output {
        match self.toNumber() {
            Ok(a) => match other.toNumber() {
                Ok(b) => Ok((a - b).into()),
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }
}

impl ops::Mul for Value {
    type Output = Result<Value, Error>;

    fn mul(self, other: Value) -> Self::Output {
        match self.toNumber() {
            Ok(a) => match other.toNumber() {
                Ok(b) => Ok((a * b).into()),
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }
}

impl ops::Div for Value {
    type Output = Result<Value, Error>;

    fn div(self, other: Value) -> Self::Output {
        match self.toNumber() {
            Ok(a) => match other.toNumber() {
                Ok(b) => Ok((a / b).into()),
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }
}

impl ops::Rem for Value {
    type Output = Result<Value, Error>;

    fn rem(self, other: Value) -> Self::Output {
        match self.toNumber() {
            Ok(a) => match other.toNumber() {
                Ok(b) => Ok((a % b).into()),
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ValueKind::*;
        
        match *self {
            Undefined => write!(f, "undefined"),
            Null      => write!(f, "null"),
            Boolean   => write!(f, "boolean"),
            Number    => write!(f, "number"),
            String    => write!(f, "string"),
            Symbol    => write!(f, "symbol"),
            Object    => write!(f, "object"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;
        
        match *self {
            Undefined(ref val) => val.fmt(f),
            Null(ref val)      => val.fmt(f),
            Boolean(ref val)   => val.fmt(f),
            String(ref val)    => val.fmt(f),
            Number(ref val)    => val.fmt(f),
            Symbol(ref val)    => val.fmt(f),
            Object(ref val)    => val.fmt(f),
        }
    }
}


impl From<Undefined> for Value {
    fn from(val: Undefined) -> Self {
        Value::Undefined(val)
    }
}
impl From<Null> for Value {
    fn from(val: Null) -> Self {
        Value::Null(val)
    }
}
impl From<Boolean> for Value {
    fn from(val: Boolean) -> Self {
        Value::Boolean(val)
    }
}
impl From<Number> for Value {
    fn from(val: Number) -> Self {
        Value::Number(val)
    }
}
impl From<String> for Value {
    fn from(val: String) -> Self {
        Value::String(val)
    }
}
impl<'a> From<&'a String> for Value {
    fn from(val: &'a String) -> Self {
        Value::String(val.clone())
    }
}
impl From<Symbol> for Value {
    fn from(val: Symbol) -> Self {
        Value::Symbol(val)
    }
}
impl From<Object> for Value {
    fn from(val: Object) -> Self {
        Value::Object(val)
    }
}


impl From<std::string::String> for Value {
    fn from(val: std::string::String) -> Self {
        Value::String(val.into())
    }
}
impl<'a> From<&'a std::string::String> for Value {
    fn from(val: &'a std::string::String) -> Self {
        Value::String(val.into())
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Number(val.into())
    }
}
impl From<f32> for Value {
    fn from(val: f32) -> Self {
        Value::Number(val.into())
    }
}

impl From<usize> for Value {
    fn from(val: usize) -> Self {
        Value::Number(val.into())
    }
}
impl From<u64> for Value {
    fn from(val: u64) -> Self {
        Value::Number(val.into())
    }
}
impl From<u32> for Value {
    fn from(val: u32) -> Self {
        Value::Number(val.into())
    }
}
impl From<u16> for Value {
    fn from(val: u16) -> Self {
        Value::Number(val.into())
    }
}
impl From<u8> for Value {
    fn from(val: u8) -> Self {
        Value::Number(val.into())
    }
}

impl From<isize> for Value {
    fn from(val: isize) -> Self {
        Value::Number(val.into())
    }
}
impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Value::Number(val.into())
    }
}
impl From<i32> for Value {
    fn from(val: i32) -> Self {
        Value::Number(val.into())
    }
}
impl From<i16> for Value {
    fn from(val: i16) -> Self {
        Value::Number(val.into())
    }
}
impl From<i8> for Value {
    fn from(val: i8) -> Self {
        Value::Number(val.into())
    }
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Value::Boolean(val.into())
    }
}

impl Into<String> for ValueKind {
    fn into(self) -> String {
        self.to_string().into()
    }
}

impl Into<PropertyKey> for Value {
    fn into(self) -> PropertyKey {
        match self {
            Value::Symbol(val) => PropertyKey::Symbol(val),
            val @ _ => PropertyKey::String(val.toString().unwrap())
        }
    }
}


#[test]
fn test_value_add() {
    // assert_eq!( (Value::Number(100f64.into())    + Value::Symbol(Symbol::HAS_INSTANCE)).is_err(), true);
    // assert_eq!( (Value::String("Hello, ".into()) + Value::Symbol(Symbol::HAS_INSTANCE)).is_err(), true);

    assert_eq!(Value::String("Hello, ".into()) + Value::String("世界！".into()), Ok("Hello, 世界！".to_string().into()));
    assert_eq!(Value::String("Hello, ".into()) + Value::Number(100f64.into()), Ok("Hello, 100".to_string().into()));
    assert_eq!(Value::Number(200f64.into())    + Value::Number(100f64.into()), Ok(Value::Number(300f64.into())));
    assert_eq!(Value::Number(200f64.into())    + Value::Number(100.22f64.into()), Ok(Value::Number(300.22f64.into())));

    assert_eq!(Value::Null(Null)           + Value::Null(Null), Ok(Value::Number(0.0f64.into())));
    assert_eq!(Value::Undefined(Undefined) + Value::Undefined(Undefined), Ok(Value::Number(::std::f64::NAN.into())));
    assert_eq!(Value::Undefined(Undefined) + Value::Null(Null), Ok(Value::Number(::std::f64::NAN.into())));
}

#[test]
fn test_value_sub() {
    // assert_eq!( (Value::Number(100f64.into())    - Value::Symbol(Symbol::HAS_INSTANCE)).is_err(), true);
    // assert_eq!( (Value::String("Hello, ".into()) - Value::Symbol(Symbol::HAS_INSTANCE)).is_err(), true);
    
    assert_eq!( (Value::String("Hello, ".into()) - Value::Number(100f64.into())), Ok(Value::Number(::std::f64::NAN.into())));
    assert_eq!(Value::Number(200f64.into()) - Value::Number(100.22f64.into()), Ok(Value::Number(99.78f64.into())));
    assert_eq!(Value::Number(200f64.into()) - Value::Boolean(true.into()), Ok(Value::Number(199f64.into())));
    assert_eq!(Value::Number(200f64.into()) - Value::Boolean(false.into()), Ok(Value::Number(200f64.into())));

    assert_eq!(Value::Number(200f64.into()) - Value::Undefined(Undefined), Ok(Value::Number(::std::f64::NAN.into())));
    assert_eq!(Value::Number(200f64.into()) - Value::Null(Null), Ok(Value::Number(200f64.into())));
}

#[test]
fn test_value_mul() {
    // assert_eq!( (Value::Number(100f64.into())    * Value::Symbol(Symbol::HAS_INSTANCE)).is_err(), true);
    // assert_eq!( (Value::String("Hello, ".into()) * Value::Symbol(Symbol::HAS_INSTANCE)).is_err(), true);

    assert_eq!( (Value::String("Hello, ".into()) * Value::Number(100f64.into())), Ok(Value::Number(::std::f64::NAN.into())));
}

#[test]
fn test_value_div() {
    // assert_eq!( (Value::Number(100f64.into())    / Value::Symbol(Symbol::HAS_INSTANCE)).is_err(), true);
    // assert_eq!( (Value::String("Hello, ".into()) / Value::Symbol(Symbol::HAS_INSTANCE)).is_err(), true);

    assert_eq!( (Value::String("Hello, ".into()) / Value::Number(100f64.into())), Ok(Value::Number(::std::f64::NAN.into())));
}

#[test]
fn test_value_rem() {
    // assert_eq!( (Value::Number(100f64.into())    % Value::Symbol(Symbol::HAS_INSTANCE)).is_err(), true);
    // assert_eq!( (Value::String("Hello, ".into()) % Value::Symbol(Symbol::HAS_INSTANCE)).is_err(), true);

    assert_eq!( (Value::String("Hello, ".into()) % Value::Number(100f64.into())), Ok(Value::Number(::std::f64::NAN.into())));
}


