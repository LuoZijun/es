// Rational, BigRational
pub use num::{ BigInt, BigUint, complex::Complex32, complex::Complex64, };
use num::{ Zero, One, Float, ToPrimitive, FromPrimitive, bigint::ToBigInt, bigint::ToBigUint, };


use crate::rc_ref::RcRef;

use std::fmt;
use std::ptr::NonNull;
use std::collections::HashMap;


// #[derive(Clone, PartialEq, PartialOrd, Copy)]
// pub struct Float<T>(T);

// impl<T: fmt::Debug> fmt::Debug for Float<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self.0)
//     }
// }
// impl<T: fmt::Display> fmt::Display for Float<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }
// impl Eq for Float<f32> { }
// impl Eq for Float<f64> { }


// pub type F32 = Float<f32>;
// pub type F64 = Float<f64>;
pub type StringRef = RcRef<String>;
pub type SymbolRef = RcRef<Symbol>;
pub type BigIntRef = RcRef<BigInt>;
pub type BigUintRef = RcRef<BigUint>;
pub type Complex32Ref = RcRef<Complex32>;
pub type Complex64Ref = RcRef<Complex64>;




impl PartialEq for StringRef {
    fn eq(&self, other: &StringRef) -> bool {
        *self.borrow() == *other.borrow()
    }
}
impl fmt::Display for StringRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.borrow())
    }
}

impl PartialEq for SymbolRef {
    fn eq(&self, other: &SymbolRef) -> bool {
        *self.borrow() == *other.borrow()
    }
}
impl fmt::Display for SymbolRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.borrow())
    }
}

impl PartialEq for BigIntRef {
    fn eq(&self, other: &BigIntRef) -> bool {
        *self.borrow() == *other.borrow()
    }
}
impl fmt::Display for BigIntRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.borrow())
    }
}

impl PartialEq for BigUintRef {
    fn eq(&self, other: &BigUintRef) -> bool {
        *self.borrow() == *other.borrow()
    }
}
impl fmt::Display for BigUintRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.borrow())
    }
}

impl PartialEq for Complex32Ref {
    fn eq(&self, other: &Complex32Ref) -> bool {
        *self.borrow() == *other.borrow()
    }
}
impl fmt::Display for Complex32Ref {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.borrow())
    }
}

impl PartialEq for Complex64Ref {
    fn eq(&self, other: &Complex64Ref) -> bool {
        *self.borrow() == *other.borrow()
    }
}
impl fmt::Display for Complex64Ref {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.borrow())
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Symbol {
    is_public: bool,
    id: usize,
}

impl Symbol {
    #[inline]
    pub fn new(is_public: bool, id: usize) -> Self {
        Self { is_public, id }
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    #[inline]
    pub fn is_public(&self) -> bool {
        self.is_public
    }
}



/*
Prop:
    filename: String,
    line_number: usize,
    column_number: usize,

Prototype:
    name: String
    message: String
    toString: Function
*/
// https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Errors
// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-native-error-types-used-in-this-standard
pub enum NativeError {
    Error(String),
    SyntaxError(String),
    /// This exception is not currently used within this specification.
    /// This object remains for compatibility with previous editions of this specification.
    EvalError {
        line_number: usize,
        column_number: usize,
        native_error: Box<NativeError>,
    },
    RangeError(String),
    ReferenceError(String),
    TypeError(String),
    URIError(String),

    // non-standard
    InternalError(String),
}

impl fmt::Debug for NativeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for NativeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::NativeError::*;

        match self {
            Error(message) => write!(f, "Error: {}", message),
            SyntaxError(message) => write!(f, "SyntaxError: {}", message),
            EvalError { line_number, column_number, native_error } => write!(f, "EvalError: {}", native_error),
            RangeError(message) => write!(f, "RangeError: {}", message),
            ReferenceError(message) => write!(f, "ReferenceError: {}", message),
            TypeError(message) => write!(f, "TypeError: {}", message),
            URIError(message) => write!(f, "URIError: {}", message),
            InternalError(message) => write!(f, "InternalError: {}", message),
        }
    }
}

impl NativeError {
    pub fn error<T: Into<String>>(message: T) -> Self {
        NativeError::Error(message.into())
    }

    pub fn syntax_error<T: Into<String>>(message: T) -> Self {
        NativeError::SyntaxError(message.into())
    }

    pub fn eval_error<T: Into<String>>(message: T) -> Self {
        // NativeError::EvalError(message.into())
        unimplemented!()
    }

    pub fn range_error<T: Into<String>>(message: T) -> Self {
        NativeError::RangeError(message.into())
    }

    pub fn reference_error<T: Into<String>>(message: T) -> Self {
        NativeError::ReferenceError(message.into())
    }
    
    pub fn type_error<T: Into<String>>(message: T) -> Self {
        NativeError::TypeError(message.into())
    }

    pub fn uri_error<T: Into<String>>(message: T) -> Self {
        NativeError::URIError(message.into())
    }

    pub fn internal_error<T: Into<String>>(message: T) -> Self {
        NativeError::InternalError(message.into())
    }
}


// 16 Bytes
// Kind: 8 Bytes , Data: 8 Bytes
#[derive(PartialEq, Clone)]
pub enum Value {
    Undefined,
    Null,
    String(StringRef),
    Object(ObjectRef),
    Boolean(bool),
    I64(i64),
    F64(f64),
    // https://github.com/jkarns275/f128
    // F128(f128),
    // decimal floating: https://docs.rs/decimal
    // D64(d64),
    // D128(d128),
    // Rational(BigRational),
    J64(Complex64Ref),
    BigInt(BigIntRef),
    Symbol(SymbolRef),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;

        match *self {
            Undefined => write!(f, "undefined"),
            Null => write!(f, "null"),
            String(ref inner) => fmt::Debug::fmt(inner, f),
            Object(ref inner) => fmt::Debug::fmt(inner, f),
            Boolean(ref inner) => fmt::Debug::fmt(inner, f),
            I64(ref inner) => fmt::Debug::fmt(inner, f),
            F64(ref inner) => fmt::Debug::fmt(inner, f),
            J64(ref inner) => fmt::Debug::fmt(inner, f),
            BigInt(ref inner) => fmt::Debug::fmt(inner, f),
            Symbol(ref inner) => fmt::Debug::fmt(inner, f),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;

        match *self {
            Undefined => write!(f, "undefined"),
            Null => write!(f, "null"),
            String(ref inner) => fmt::Display::fmt(inner, f),
            Object(ref inner) => fmt::Display::fmt(inner, f),
            Boolean(ref inner) => fmt::Display::fmt(inner, f),
            I64(ref inner) => fmt::Display::fmt(inner, f),
            F64(ref inner) => fmt::Display::fmt(inner, f),
            J64(ref inner) => fmt::Display::fmt(inner, f),
            BigInt(ref inner) => fmt::Display::fmt(inner, f),
            Symbol(ref inner) => fmt::Display::fmt(inner, f),
        }
    }
}

impl Value {
    pub const ZERO: Value = Value::I64(0i64);
    pub const ONE: Value  = Value::I64(1i64);
    pub const NAN: Value  = Value::F64(std::f64::NAN);
    pub const INFINITY: Value     = Value::F64(std::f64::INFINITY);
    pub const NEG_INFINITY: Value = Value::F64(std::f64::NEG_INFINITY);
    pub const NEG_ZERO: Value     = Value::F64(-0f64);

    #[inline]
    pub fn typeof_(&self) -> &'static str {
        use self::Value::*;

        match *self {
            Undefined => "undefined",
            Null => "null",
            String(_) => "string",
            Object(_) => "object",
            Boolean(_) => "boolean",
            I64(_) => "number",
            F64(_) => "number",
            J64(_) => "complex",
            BigInt(_) => "bigint",
            Symbol(_) => "symbol",
        }
    }
    
    #[inline]
    pub fn zero<T: Zero + Into<Value>>() -> T {
        T::zero()
    }

    #[inline]
    pub fn one<T: One + Into<Value>>() -> T {
        T::one()
    }

    #[inline]
    pub fn nan<T: Float + Into<Value>>() -> T {
        T::nan()
    }

    #[inline]
    pub fn infinity<T: Float + Into<Value>>() -> T {
        T::infinity()
    }

    #[inline]
    pub fn neg_infinity<T: Float + Into<Value>>() -> T {
        T::neg_infinity()
    }

    #[inline]
    pub fn neg_zero<T: Float + Into<Value>>() -> T {
        T::neg_zero()
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        use self::Value::*;

        match self {
            Undefined => false,
            Null => false,
            String(_) => false,
            Object(_) => false,
            Boolean(_) => false,
            I64(inner) => inner.is_zero(),
            F64(inner) => inner.is_zero(),
            J64(inner) => inner.borrow().is_zero(),
            BigInt(inner) => inner.borrow().is_zero(),
            Symbol(_) => false,
        }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        use self::Value::*;
        
        match self {
            Undefined => false,
            Null => false,
            String(_) => false,
            Object(_) => false,
            Boolean(_) => false,
            I64(inner) => inner.is_one(),
            F64(inner) => inner.is_one(),
            J64(inner) => inner.borrow().is_one(),
            BigInt(inner) => inner.borrow().is_one(),
            Symbol(_) => false,
        }
    }

    /// Returns true if this value is NaN and false otherwise.
    #[inline]
    pub fn is_nan(&self) -> bool {
        use self::Value::*;
        
        match self {
            Undefined => false,
            Null => false,
            String(_) => false,
            Object(_) => false,
            Boolean(_) => false,
            I64(inner) => false,
            F64(inner) => inner.is_nan(),
            J64(inner) => inner.borrow().is_nan(),
            BigInt(inner) => false,
            Symbol(_) => false,
        }
    }

    /// Returns true if this value is positive infinity or negative infinity and false otherwise.
    #[inline]
    pub fn is_infinite(&self) -> bool {
        use self::Value::*;
        
        match self {
            Undefined => false,
            Null => false,
            String(_) => false,
            Object(_) => false,
            Boolean(_) => false,
            I64(inner) => false,
            F64(inner) => inner.is_infinite(),
            J64(inner) => inner.borrow().is_infinite(),
            BigInt(inner) => false,
            Symbol(_) => false,
        }
    }

    /// Returns true if this number is neither infinite nor NaN.
    #[inline]
    pub fn is_finite(&self) -> bool {
        use self::Value::*;
        
        match self {
            Undefined => false,
            Null => false,
            String(_) => false,
            Object(_) => false,
            Boolean(_) => false,
            I64(inner) => false,
            F64(inner) => inner.is_finite(),
            J64(inner) => inner.borrow().is_finite(),
            BigInt(inner) => false,
            Symbol(_) => false,
        }
    }

    

    // NOTE: 
    //      .clone()      值或引用值拷贝
    //      .copy()       值或引用拷贝
    //      .deepcopy()   同 .copy() 的区别在于针对 Object 深拷贝时，会拷贝 Object 内部的元素。
    #[inline]
    pub fn copy(&self) -> Value {
        match *self {
            // Native Copy
            Value::Undefined => Value::Undefined,
            Value::Null => Value::Null,
            Value::Boolean(inner) => inner.into(),
            Value::I64(inner) => inner.into(),
            Value::F64(inner) => inner.into(),

            // 堆拷贝
            Value::String(ref inner) => inner.borrow().clone().into(),
            Value::Symbol(ref inner) => inner.borrow().clone().into(),
            Value::J64(ref inner) => inner.borrow().clone().into(),
            Value::BigInt(ref inner) => inner.borrow().clone().into(),
            // TODO: 需要手动实现 copy 和 deepcopy 方法
            Value::Object(ref inner) => inner.borrow().clone().into(),
        }
    }

    #[inline]
    pub fn deepcopy(&self) -> Value {
        match *self {
            Value::Undefined
            | Value::Null
            | Value::String(_)
            | Value::Boolean(_)
            | Value::I64(_)
            | Value::F64(_)
            | Value::J64(_)
            | Value::BigInt(_)
            | Value::Symbol(_) => self.copy(),
            // TODO: 需要手动实现 deepcopy 方法
            Value::Object(ref inner) => inner.borrow().clone().into(),
        }
    }

    #[inline]
    pub fn is_undefined(&self) -> bool {
        use self::Value::*;

        match *self {
            Undefined => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        use self::Value::*;

        match *self {
            Null => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_string(&self) -> bool {
        use self::Value::*;

        match *self {
            String(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_object(&self) -> bool {
        use self::Value::*;

        match *self {
            Object(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_boolean(&self) -> bool {
        use self::Value::*;

        match *self {
            Boolean(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_number(&self) -> bool {
        use self::Value::*;

        match *self {
            I64(_) => true,
            F64(_) => true,
            J64(_) => true,
            BigInt(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_int(&self) -> bool {
        use self::Value::*;

        match *self {
            I64(_) => true,
            BigInt(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        use self::Value::*;

        match *self {
            F64(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_symbol(&self) -> bool {
        use self::Value::*;

        match *self {
            Symbol(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_bigint(&self) -> bool {
        use self::Value::*;

        match *self {
            BigInt(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_complex(&self) -> bool {
        use self::Value::*;

        match *self {
            J64(_) => true,
            _ => false,
        }
    }
}


impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(RcRef::new(s))
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(RcRef::new(s.to_string()))
    }
}


impl From<Object> for Value {
    fn from(s: Object) -> Self {
        Value::Object(RcRef::new(s))
    }
}

impl From<bool> for Value {
    fn from(s: bool) -> Self {
        Value::Boolean(s)
    }
}

impl From<u8> for Value {
    fn from(s: u8) -> Self {
        Value::I64(s as i64)
    }
}
impl From<u16> for Value {
    fn from(s: u16) -> Self {
        Value::I64(s as i64)
    }
}
impl From<u32> for Value {
    fn from(s: u32) -> Self {
        Value::I64(s as i64)
    }
}
impl From<u64> for Value {
    fn from(s: u64) -> Self {
        if s > std::i64::MAX as u64 {
            BigInt::from_u64(s).unwrap().into()
        } else {
            Value::I64(s as i64)
        }
    }
}
impl From<usize> for Value {
    fn from(s: usize) -> Self {
        if s as i128 > std::i64::MAX as i128 {
            BigInt::from_usize(s).unwrap().into()
        } else {
            Value::I64(s as i64)
        }
    }
}

impl From<i8> for Value {
    fn from(s: i8) -> Self {
        Value::I64(s as i64)
    }
}
impl From<i16> for Value {
    fn from(s: i16) -> Self {
        Value::I64(s as i64)
    }
}
impl From<i32> for Value {
    fn from(s: i32) -> Self {
        Value::I64(s as i64)
    }
}
impl From<i64> for Value {
    fn from(s: i64) -> Self {
        Value::I64(s)
    }
}
impl From<isize> for Value {
    fn from(s: isize) -> Self {
        Value::I64(s as i64)
    }
}

impl From<f32> for Value {
    fn from(s: f32) -> Self {
        Value::F64(s as f64)
    }
}
impl From<f64> for Value {
    fn from(s: f64) -> Self {
        Value::F64(s)
    }
}

impl From<Symbol> for Value {
    fn from(s: Symbol) -> Self {
        Value::Symbol(RcRef::new(s))
    }
}

impl From<BigInt> for Value {
    fn from(s: BigInt) -> Self {
        Value::BigInt(RcRef::new(s))
    }
}
impl From<BigUint> for Value {
    fn from(s: BigUint) -> Self {
        s.to_bigint().unwrap().into()
    }
}

impl From<Complex64> for Value {
    fn from(s: Complex64) -> Self {
        Value::J64(RcRef::new(s))
    }
}




pub type NativeFunction = fn() -> Value;





pub type ObjectRef = RcRef<Object>;

impl fmt::Display for ObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone)]
pub enum ObjectKind {
    // Normal,     // hasConstructor() == false && isCallable() == false
    Function,      // isCallable() == true && hasConstructor() == true || false
    ArrowFunction, // isCallable() == true && hasConstructor() == false
    Instance,      // hasConstructor() == true
}




pub struct Function {
    inner: ObjectRef,
}

// hasConstructor == true
pub struct Class {
    inner: ObjectRef,
}

// hasConstructor
pub struct Instance {
    inner: ObjectRef,
}

const PROP_PROTOTYPE: &'static str = "prototype";
const PROP_NAME: &'static str      = "name";
const PROP_LENGTH: &'static str    = "length";

// IsConstructor
// IsCallable

#[derive(Debug, Clone)]
pub struct BaseObject {
    pub properties: HashMap<PropertyKey, Property>,
    pub is_frozen: bool,
    pub is_sealed: bool,
    pub is_extensible: bool,
    pub kind: ObjectKind,
}


#[derive(Debug, Clone)]
pub struct Object {
    pub properties: HashMap<PropertyKey, Property>,
    pub is_frozen: bool,
    pub is_sealed: bool,
    pub is_extensible: bool,
    pub kind: ObjectKind,
}

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        false
    }
}
impl PartialEq for ObjectRef {
    fn eq(&self, other: &ObjectRef) -> bool {
        false
    }
}
impl Eq for Object { }
impl Eq for ObjectRef { }


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PropertyKey {
    String(String),
    Symbol(Symbol),
}


#[derive(Debug, Clone)]
pub struct Property {
    pub descriptor: PropertyDescriptor,
    pub enumerable: bool,
    pub configurable: bool,
}

#[derive(Debug, Clone)]
pub enum PropertyDescriptor {
    Data(DataPropertyDescriptor),
    Accessor(AccessorPropertyDescriptor),
}

#[derive(Debug, Clone)]
pub struct DataPropertyDescriptor {
    pub value: Value, // NOTE: Any ECMAScript Value
    pub writable: bool,
}

#[derive(Debug, Clone)]
pub struct AccessorPropertyDescriptor {
    pub get: Value,  // WARN: Value::Undefined || Value::Object<+ Callable>
    pub set: Value,  // WARN: Value::Undefined || Value::Object<+ Callable>
}

impl Object {
    pub fn isCallable(&self) -> bool {
        unimplemented!()
    }

    pub fn IsConstructor(&self) -> bool {
        unimplemented!()
    }

    pub fn call(&mut self) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn new(&mut self) -> Result<(), ()> {
        unimplemented!()
    }

}
