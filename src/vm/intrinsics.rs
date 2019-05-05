use num::{ ToPrimitive, bigint::ToBigInt, bigint::ToBigUint, bigint::Sign, };

use crate::rc_ref::RcRef;
use crate::compiler::value::{ Value, BigInt, BigUint, Complex32, Complex64 };
use crate::compiler::value::{ NativeError, };

use std::ops::{ Neg, Add, };

// 特殊操作也许在 Scope 层处理比较好，这里先暂时列一下:
// in
// delete
// void
// new
// call
// 
// Lexical Environment
// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-lexical-environments
// 
//      Environment Record
//          Declarative Environment Records
//          Object Environment Records
//          Function Environment Records
//          Global Environment Records
//          Module Environment Records
// 

pub trait Environment {
    fn HasBinding(name: &str) -> bool;
    
}

// pub fn toUint8Clamp(&self) -> Value {
//     if self.0.is_nan() || self.0 <= 0.0f64 {
//         0u8
//     } else if self.0 >= 255.0f64 {
//         255u8
//     } else {
//         let f = self.0.floor();
//         let n = f + 0.5f64;
//         if n < self.0 {
//             (f + 1.0) as u8
//         } else if self.0 < n {
//             f as u8
//         } else {
//             if f % 2.0 == 0.0 {
//                 f as u8
//             } else {
//                 (f + 1.0) as u8
//             }
//         }
//     }
// }

#[derive(Debug, Clone)]
pub enum Uint {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    BigUint(BigUint),
}

impl Uint {
    pub fn toUint8(&self) -> u8 {
        match *self {
            Uint::U8(n) => n,
            Uint::U16(n) => n as u8,
            Uint::U32(n) => n as u8,
            Uint::U64(n) => n as u8,
            Uint::BigUint(ref n) => {
                let bytes = n.to_bytes_be();
                if bytes.len() == 0 {
                    0u8
                } else {
                    bytes[bytes.len()-1]
                }
            },
        }
    }

    pub fn toUint8Clamped(&self) -> u8 {
        match *self {
            Uint::U8(n) => n,
            Uint::U16(n) => if n > 255u16 { 255u8 } else { n as u8 },
            Uint::U32(n) => if n > 255u32 { 255u8 } else { n as u8 },
            Uint::U64(n) => if n > 255u64 { 255u8 } else { n as u8 },
            Uint::BigUint(ref n) => {
                let max_u8: BigUint = 255u32.into();
                let zero: BigUint = 0u8.into();
                if n == &zero {
                    0u8
                } else if n > &max_u8 {
                    255u8
                } else {
                    let bytes = n.to_bytes_be();
                    if bytes.len() == 0 {
                        0u8
                    } else {
                        bytes[bytes.len()-1]
                    }
                }
            },
        }
    }

    pub fn toUint16(&self) -> u16 {
        unimplemented!()
    }

    pub fn toUint32(&self) -> u32 {
        unimplemented!()
    }

    pub fn toUint64(&self) -> u64 {
        unimplemented!()
    }

    pub fn toBigUint(&self) -> BigUint {
        unimplemented!()
    }
}


#[inline]
pub fn toInt(val: Value) -> Result<Value, NativeError> {
    match val {
        Value::Undefined
        | Value::Null
        | Value::Object(_)
        | Value::Boolean(_)
        | Value::Symbol(_) => toInt(toNumber(val)?),
        Value::String(s) => {
            let s = s.borrow();
            match s.parse::<i64>() {
                Ok(n) => toInt(n.into()),
                Err(e) => match s.parse::<BigInt>() {
                    Ok(n) => toInt(n.into()),
                    Err(e) => Err(NativeError::type_error( format!("Cannot convert a String {:?} value to a Int", s) ))
                }
            }
        },
        Value::I64(_) => Ok(val),
        Value::BigInt(_) => Ok(val),
        Value::F64(inner) => Ok(Value::I64(inner as i64)),
        Value::J64(_) => Err(NativeError::type_error("Cannot convert a Complex value to a Int")),
    }
}

#[inline]
pub fn toUint(val: Value) -> Result<Uint, NativeError> {
    match val {
        Value::Undefined
        | Value::Null
        | Value::String(_)
        | Value::Object(_)
        | Value::Boolean(_)
        | Value::Symbol(_) => toUint(toNumber(val)?),
        Value::I64(inner) => Ok(Uint::U64(inner as u64)),
        Value::BigInt(ref inner) => {
            let inner = inner.borrow();
            match inner.sign() {
                Sign::Minus => {
                    let (_sign, bytes) = inner.to_bytes_be();
                    let uint = BigUint::from_bytes_be(&bytes);
                    Ok(Uint::BigUint(uint))
                    // Clamped
                    // Ok(Uint::U8(0u8))
                },
                Sign::NoSign => Ok(Uint::U8(0u8)),
                Sign::Plus => {
                    let uint = inner.to_biguint().unwrap();
                    let max_u64: BigUint = std::u64::MAX.into();
                    if uint > max_u64 {
                        Ok(Uint::BigUint(uint))
                    } else {
                        Ok(Uint::U64(inner.to_u64().unwrap()))
                    }
                }
            }
        },
        Value::F64(inner) => Ok(Uint::U64(inner as u64)),
        Value::J64(_) => Err(NativeError::type_error("Cannot convert a Complex value to a Uint")),
    }
}

#[inline]
pub fn toFloat(val: Value) -> Result<Value, NativeError> {
    match val {
        Value::Undefined
        | Value::Null
        | Value::Object(_)
        | Value::Boolean(_)
        | Value::Symbol(_) => toFloat(toNumber(val)?),
        Value::String(s) => {
            let s = s.borrow();
            match s.parse::<f64>() {
                Ok(n) => toFloat(n.into()),
                Err(_e) => Err(NativeError::type_error( format!("Cannot convert a String {:?} value to a Float", s) )),
            }
        },

        Value::I64(inner) => Ok(Value::F64(inner as f64)),
        Value::F64(_) => Ok(val),
        Value::BigInt(_) => Err(NativeError::type_error("Cannot convert a BigInt value to a Float")),
        Value::J64(_) => Err(NativeError::type_error("Cannot convert a Complex value to a Float")),
    }
}

#[inline]
pub fn toNumber(val: Value) -> Result<Value, NativeError> {
    match val {
        Value::Undefined => {
            // Ok(Value::NAN)
            Err(NativeError::type_error("Cannot convert a Undefined value to a Number"))
        },
        Value::Null => {
            // Ok(Value::ZERO)
            Err(NativeError::type_error("Cannot convert a Null value to a Number"))
        },
        Value::String(ref s) => {
            match toInt(val.clone()) {
                Ok(num) => Ok(num),
                Err(_) => match toFloat(val) {
                    Ok(num) => Ok(num),
                    Err(_) => {
                        // Ok(Value::NAN)
                        Err(NativeError::type_error("Cannot convert a String value to a Number"))
                    },
                }
            }
        },
        Value::Object(_) => {
            // Ok(Value::NAN)
            Err(NativeError::type_error("Cannot convert a Object value to a Number"))
        },
        Value::Boolean(inner) => match inner {
            true  => Ok(Value::ONE),
            false => Ok(Value::ZERO),
        },
        Value::Symbol(_) => Err(NativeError::type_error("Cannot convert a Symbol value to a Number")),
        Value::I64(_)
        | Value::F64(_)
        | Value::BigInt(_)
        | Value::J64(_) =>Ok(val),
    }
}

#[inline]
pub fn toString(val: Value) -> Value {
    match val {
        Value::String(_) => val,
        Value::Symbol(_)
        | Value::Object(_) => {
            // prototype toString()
            unimplemented!()
        },
        _ => format!("{}", val).into(),
    }
}



#[inline]
pub fn load(env: impl Environment, val: Value) -> Result<Value, NativeError> {
    unimplemented!()
}

#[inline]
pub fn store(env: impl Environment, val: Value) -> Result<Value, NativeError> {
    unimplemented!()
}

// new 
#[inline]
pub fn new(val: Value, args: Option<Vec<Value>>) -> Value {
    unimplemented!()
}

// call
#[inline]
pub fn call(val: Value, args: Vec<Value>) -> Value {
    unimplemented!()
}


// unary positive operator +.
#[inline]
pub fn positive(val: Value) -> Result<Value, NativeError> {
    match val {
        Value::Undefined
        | Value::Null
        | Value::String(_)
        | Value::Object(_)
        | Value::Boolean(_)
        | Value::Symbol(_) => positive(toNumber(val)?),
        
        Value::I64(_)
        | Value::BigInt(_)
        | Value::F64(_)
        | Value::J64(_) => Ok(val.copy()),
    }
}
// unary negation operator -.
#[inline]
pub fn negative(val: Value) -> Result<Value, NativeError> {
    match val {
        Value::Undefined
        | Value::Null
        | Value::String(_)
        | Value::Object(_)
        | Value::Boolean(_)
        | Value::Symbol(_) => negative(toNumber(val)?),

        Value::I64(inner) => Ok(Value::I64(-inner)),
        Value::BigInt(inner) => Ok(inner.borrow().clone().neg().into()),
        Value::F64(inner) => Ok(Value::F64(-inner)),
        Value::J64(inner) => Ok(inner.borrow().neg().into()),
    }
}

// ++
#[inline]
pub fn increment(val: Value) -> Result<Value, NativeError> {
    add(val, Value::ONE)
}
// --
#[inline]
pub fn decrement(val: Value) -> Result<Value, NativeError> {
    sub(val, Value::ONE)
}

// delete
#[inline]
pub fn delete(val: Value) -> Value {
    unimplemented!()
}
// void
#[inline]
pub fn void(_val: Value) -> Value {
    Value::Undefined
}

// typeof
#[inline]
pub fn typeof_(val: Value) -> Value {
    val.typeof_().into()
}
// instanceof
#[inline]
pub fn instanceof(val: Value, cls: Value) -> Value {
    unimplemented!()
}
// in
#[inline]
pub fn in_(val: Value, cls: Value) -> Value {
    unimplemented!()
}


// !
#[inline]
pub fn logical_not(val: Value) -> Value {
    unimplemented!()
}
// &&
#[inline]
pub fn logical_and(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// ||
#[inline]
pub fn logical_or(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}

// ~
#[inline]
pub fn bit_not(val: Value) -> Value {
    unimplemented!()
}
// &
#[inline]
pub fn bit_and(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// |
#[inline]
pub fn bit_or(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// ^
#[inline]
pub fn bit_xor(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// <<
#[inline]
pub fn bit_shl(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// >>
#[inline]
pub fn bit_shr(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// >>>
#[inline]
pub fn bit_ushr(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}


// +
#[inline]
pub fn add(lhs: Value, rhs: Value) -> Result<Value, NativeError> {
    let is_string = lhs.is_string() || rhs.is_string();
    if is_string {
        // String
        let a = if !lhs.is_string() {
            toString(lhs)
        } else {
            lhs
        };
        let b = if !rhs.is_string() {
            toString(rhs)
        } else {
            rhs
        };
        
        assert!(a.is_string());
        assert!(b.is_string());

        match a {
            Value::String(ref a_string_ref) => match b {
                Value::String(ref b_string_ref) => {
                    let a_ref = a_string_ref.borrow();
                    let b_ref = b_string_ref.borrow();

                    let a_str: &str = a_ref.as_str();
                    let b_str: &str = b_ref.as_str();

                    let mut output: String = String::with_capacity(a_str.len() + b_str.len());
                    output.push_str(a_str);
                    output.push_str(b_str);

                    return Ok(output.into())
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }

    } else {
        // Number
        match (toNumber(lhs)?, toNumber(rhs)?) {

            (Value::I64(a), Value::I64(b)) => Ok((a + b).into()),
            (Value::I64(a), Value::F64(b)) => Ok((a as f64 + b).into()),
            (Value::I64(a), Value::J64(ref b)) => unimplemented!(),
            (Value::I64(a), Value::BigInt(ref b)) => Ok((a.add( &*b.borrow() )).into()),

            (Value::F64(a), Value::I64(b)) => Ok((a + b as f64).into()),
            (Value::F64(a), Value::F64(b)) => Ok((a + b).into()),
            (Value::F64(a), Value::J64(b)) => Err(NativeError::type_error("Cannot convert a Float value to a Complex Number")),
            (Value::F64(a), Value::BigInt(b)) => Err(NativeError::type_error("Cannot convert a Float value to a BigInt Number")),

            (Value::J64(a), Value::I64(b)) => Err(NativeError::type_error("Cannot convert a Complex value to a Int Number")),
            (Value::J64(a), Value::F64(b)) => Err(NativeError::type_error("Cannot convert a Complex value to a Float Number")),
            (Value::J64(a), Value::J64(b)) => Ok((&*a.borrow()).add( &*b.borrow() ).into()),
            (Value::J64(a), Value::BigInt(b)) => Err(NativeError::type_error("Cannot convert a Complex value to a BigInt Number")),

            (Value::BigInt(ref a), Value::I64(b)) => Ok((&*a.borrow()).add(b).into()),
            (Value::BigInt(a), Value::F64(b)) => Err(NativeError::type_error("Cannot convert a Float value to a BigInt Number")),
            (Value::BigInt(a), Value::J64(b)) => Err(NativeError::type_error("Cannot convert a Complex value to a BigInt Number")),
            (Value::BigInt(ref a), Value::BigInt(ref b)) => Ok((&*a.borrow()).add( &*b.borrow() ).into()),
            _ => unreachable!(),
        }
    }

}
// -
#[inline]
pub fn sub(lhs: Value, rhs: Value) -> Result<Value, NativeError> {
    unimplemented!()
}
// *
#[inline]
pub fn mul(lhs: Value, rhs: Value) -> Result<Value, NativeError> {
    unimplemented!()
}
// /
#[inline]
pub fn div(lhs: Value, rhs: Value) -> Result<Value, NativeError> {
    unimplemented!()
}
// %
#[inline]
pub fn rem(lhs: Value, rhs: Value) -> Result<Value, NativeError> {
    unimplemented!()
}
// **
#[inline]
pub fn pow(lhs: Value, rhs: Value) -> Result<Value, NativeError> {
    unimplemented!()
}

// COMPARE OPERATORS
// >
#[inline]
pub fn gt(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// <
#[inline]
pub fn lt(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// >=
#[inline]
pub fn ge(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// <=
#[inline]
pub fn le(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// ==
#[inline]
pub fn eq(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// !=
#[inline]
pub fn neq(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// ===
#[inline]
pub fn strict_eq(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// !==
#[inline]
pub fn strict_neq(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}




// Assignment Operator

// =
#[inline]
pub fn assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// +=
#[inline]
pub fn add_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// -=
#[inline]
pub fn sub_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// *=
#[inline]
pub fn mul_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// /=
#[inline]
pub fn div_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// %=
#[inline]
pub fn rem_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// **=
#[inline]
pub fn pow_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}

// &=
#[inline]
pub fn bit_and_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// !=
#[inline]
pub fn bit_or_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// ^=
#[inline]
pub fn bit_xor_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// <<=
#[inline]
pub fn bit_shl_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// >>=
#[inline]
pub fn bit_shr_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}
// >>>=
#[inline]
pub fn bit_ushr_assign(lhs: Value, rhs: Value) -> Value {
    unimplemented!()
}

