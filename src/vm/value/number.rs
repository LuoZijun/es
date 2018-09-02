use error::{ Error, ErrorKind, };
use vm::value::Value;
use vm::value::Undefined;
use vm::value::Null;
use vm::value::Boolean;
use vm::value::String;
use vm::value::Symbol;
use vm::value::Object;


use std::fmt;
use std::mem;
use std::cmp;
use std::ops;
use std::hash;
use std::string;
use std::marker::PhantomData;
use std::collections::HashMap;


#[derive(Debug, Clone, Copy)]
pub struct Number(pub f64);


impl Number {
    // std::f64::RADIX.pow(std::f64::MANTISSA_DIGITS) as f64;
    pub const MAX_SAFE_INTEGER: Number = Number(9007199254740991f64);  // 2**53-1
    pub const MIN_SAFE_INTEGER: Number = Number(-9007199254740991f64); // -(2**53-1)
    pub const MAX_VALUE: Number = Number(std::f64::MAX);
    pub const MIN_VALUE: Number = Number(std::f64::MIN);
    
    pub const NAN: Number = Number(std::f64::NAN);
    pub const EPSILON: Number = Number(std::f64::EPSILON);

    pub const NEGATIVE_INFINITY: Number = Number(std::f64::NEG_INFINITY);
    pub const POSITIVE_INFINITY: Number = Number(std::f64::INFINITY);

    
    pub fn toInteger(&self) -> Number {
        if self.0.is_nan() {
            0.0f64.into()
        } else if self.0.is_infinite() || self.0 == 0.0f64 || self.0 == -0.0f64 {
            self.0.into()
        } else {
            self.0.abs().floor().into()
        }
    }

    pub fn toInt8(&self) -> i8 {
        self.clone().into()
    }
    pub fn toInt16(&self) -> i16 {
        self.clone().into()
    }
    pub fn toInt32(&self) -> i32 {
        self.clone().into()
    }
    pub fn toUint8(&self) -> u8 {
        self.clone().into()
    }
    pub fn toUint16(&self) -> u16 {
        self.clone().into()
    }
    pub fn toUint32(&self) -> u32 {
        self.clone().into()
    }
    
    pub fn toUint8Clamp(&self) -> u8 {
        if self.0.is_nan() || self.0 <= 0.0f64 {
            0u8
        } else if self.0 >= 255.0f64 {
            255u8
        } else {
            let f = self.0.floor();
            let n = f + 0.5f64;
            if n < self.0 {
                (f + 1.0) as u8
            } else if self.0 < n {
                f as u8
            } else {
                if f % 2.0 == 0.0 {
                    f as u8
                } else {
                    (f + 1.0) as u8
                }
            }
        }
    }

    pub fn toExponential(&self, fraction_digits: &Value) -> Result<String, Error> {
        match fraction_digits.toNumber().map(|num| num.toInteger()) {
            Ok(f) => {
                if self.0.is_nan() {
                    return Ok(self.into());
                }

                let f: u32 = f.into();

                if f > 20 {
                    Err(Error::new(ErrorKind::RangeError, "toExponential() argument must be between 0 and 20"))
                } else {
                    Ok(format!("{:.*e}", f as usize, self.0).into())
                }

            }
            Err(e) => Err(e),
        }
    }

    pub fn toPrecision(&self, precision: &Value) -> Result<String, Error> {
        if precision.is_undefined() {
            return Ok(self.into());
        }

        match precision.toNumber().map(|num| num.toInteger()) {
            Ok(p) => {
                if self.0.is_nan() || self.0.is_infinite() {
                    return Ok(self.into());
                }

                let p: usize = p.into();

                if p < 1 || p > 21 {
                    return Err(Error::new(ErrorKind::RangeError, "toPrecision() argument must be between 1 and 21"));
                }
                
                let integer_part = self.0.trunc();
                let fractional_part = self.0 - integer_part;

                let integer_part_s = format!("{}", integer_part);

                if p > integer_part_s.len() {
                    Ok(format!("{:.*}", p - integer_part_s.len(), self.0).into())
                } else if p < integer_part_s.len() {
                    Ok( (&format!("{}", integer_part)[..p]).to_string().into() )
                } else {
                    Ok(format!("{}", integer_part).into())
                }
                
            },
            Err(e) => Err(e)
        }
    }

    pub fn toFixed(&self, fraction_digits: &Value) -> Result<String, Error> {
        match fraction_digits.toNumber().map(|num| num.toInteger()) {
            Ok(f) => {
                if self.0.is_nan() {
                    return Ok(self.into());
                }

                let f: u32 = f.into();

                if f > 20 {
                    return Err(Error::new(ErrorKind::RangeError, "toFixed() argument must be between 0 and 20"));
                } else {
                    Ok(format!("{:.*}", f as usize, self.0).into())
                }

            },
            Err(e) => Err(e)
        }
    }

    pub fn pow(&self, exponent: &Number) -> Number {
        // https://www.ecma-international.org/ecma-262/8.0/#sec-exp-operator
        use std::f64;

        if exponent.0.is_nan() {
            return f64::NAN.into();
        }

        if exponent.0 == 0.0 || exponent.0 == -0.0 {
            return 1.0.into();
        }

        if self.0.is_nan() && ( exponent.0 != 0.0 && exponent.0 != -0.0 ) {
            return f64::NAN.into();
        }

        if self.0.abs() > 1.0 {
            if exponent.0.is_infinite() && exponent.0.is_sign_positive() {
                return f64::INFINITY.into();
            }

            if exponent.0.is_infinite() && exponent.0.is_sign_negative() {
                return 0.0f64.into();
            }
        }

        if self.0.abs() == 1.0 {
            if exponent.0.is_infinite() && exponent.0.is_sign_positive() {
                return f64::NAN.into();
            }

            if exponent.0.is_infinite() && exponent.0.is_sign_negative() {
                return f64::NAN.into();
            }

        }

        if self.0.abs() < 1.0 {
            if exponent.0.is_infinite() && exponent.0.is_sign_positive() {
                return 0.0f64.into();
            }

            if exponent.0.is_infinite() && exponent.0.is_sign_negative() {
                return f64::INFINITY.into();
            }
        }


        if self.0.is_infinite() && self.0.is_sign_positive() {
            if exponent.0 > 0.0 {
                return f64::INFINITY.into();
            }

            if exponent.0 < 0.0 {
                return 0.0f64.into();
            }
        }
    
        if self.0.is_infinite() && self.0.is_sign_negative() {
            if exponent.0 > 0.0 {
                if exponent.0 % 2.0 != 0.0 {
                    return f64::NEG_INFINITY.into();
                } else {
                    return f64::INFINITY.into();
                }
            }

            if exponent.0 < 0.0 {
                if exponent.0 % 2.0 != 0.0 {
                    return (-0.0f64).into();
                } else {
                    return 0.0.into();
                }
            }
        }

        if self.0 == 0.0 && self.0.is_sign_positive() {
            if exponent.0 > 0.0 {
                return 0.0.into();
            }

            if exponent.0 < 0.0 {
                return f64::INFINITY.into();
            }
        }

        if self.0 == 0.0 && self.0.is_sign_negative() {
            if exponent.0 > 0.0 {
                if exponent.0 % 2.0 != 0.0 {
                    return (-0.0).into();
                } else {
                    return 0.0.into();
                }
            }

            if exponent.0 < 0.0 {
                if exponent.0 % 2.0 != 0.0 {
                    return f64::NEG_INFINITY.into();
                } else {
                    return f64::INFINITY.into();
                }
            }
        }

        // If base < 0 and base is finite and exponent is finite and exponent is not an integer, the result is NaN.
        let integer_part = exponent.0.trunc();
        let fractional_part = exponent.0 - integer_part;
        
        assert_eq!(fractional_part >= 0.0, true);

        let exponent_is_integer = fractional_part == 0.0f64;

        if self.0 < 0.0f64 && self.0.is_finite() 
            && exponent.0.is_finite()
            && exponent_is_integer == false {
                return f64::NAN.into();
        }

        unreachable!()
    }

    pub fn unary_plus(&self) -> Number {
        if self.0.is_sign_negative() {
            (-self.0).into()
        } else {
            self.0.into()
        }
    }
}

impl ops::Add for Number {
    type Output = Number;
    
    fn add(self, other: Number) -> Self::Output {
        (self.0 + other.0).into()
    }
}
impl<'a> ops::Add for &'a Number {
    type Output = Number;
    
    fn add(self, other: &'a Number) -> Self::Output {
        (self.0 + other.0).into()
    }
}

impl ops::Sub for Number {
    type Output = Number;
    
    fn sub(self, other: Number) -> Self::Output {
        (self.0 - other.0).into()
    }
}
impl<'a> ops::Sub for &'a Number {
    type Output = Number;
    
    fn sub(self, other: &'a Number) -> Self::Output {
        (self.0 - other.0).into()
    }
}

impl ops::Mul for Number {
    type Output = Number;
    
    fn mul(self, other: Number) -> Self::Output {
        (self.0 * other.0).into()
    }
}
impl<'a> ops::Mul for &'a Number {
    type Output = Number;
    
    fn mul(self, other: &'a Number) -> Self::Output {
        (self.0 * other.0).into()
    }
}

impl ops::Div for Number {
    type Output = Number;
    
    fn div(self, other: Number) -> Self::Output {
        (self.0 / other.0).into()
    }
}
impl<'a> ops::Div for &'a Number {
    type Output = Number;
    
    fn div(self, other: &'a Number) -> Self::Output {
        (self.0 / other.0).into()
    }
}

impl ops::Rem for Number {
    type Output = Number;
    
    fn rem(self, other: Number) -> Self::Output {
        (self.0 % other.0).into()
    }
}
impl<'a> ops::Rem for &'a Number {
    type Output = Number;
    
    fn rem(self, other: &'a Number) -> Self::Output {
        (self.0 % other.0).into()
    }
}

impl ops::Shl for Number {
    type Output = Number;
    
    fn shl(self, rhs: Number) -> Self::Output {
        // NOTE:
        // Let shiftCount be the result of masking out all but the least significant 5 bits of rnum, that is, compute rnum & 0x1F.
        // Return the result of left shifting lnum by shiftCount bits. The result is a signed 32-bit integer. 
        ( self.toInt32() << rhs.toUint32() ).into()
    }
}
impl<'a> ops::Shl for &'a Number {
    type Output = Number;
    
    fn shl(self, rhs: &'a Number) -> Self::Output {
        ( self.toInt32() << rhs.toUint32() ).into()
    }
}

impl ops::Shr for Number {
    type Output = Number;
    
    fn shr(self, rhs: Number) -> Self::Output {
        // NOTE:
        // Let shiftCount be the result of masking out all but the least significant 5 bits of rnum, that is, compute rnum & 0x1F.
        // Return the result of left shifting lnum by shiftCount bits. The result is a signed 32-bit integer. 
        ( self.toInt32() >> rhs.toUint32() ).into()
    }
}
impl<'a> ops::Shr for &'a Number {
    type Output = Number;
    
    fn shr(self, rhs: &'a Number) -> Self::Output {
        ( self.toInt32() >> rhs.toUint32() ).into()
    }
}
// TODO: Unsigned Right Shift Operator ( >>> )

impl ops::Neg for Number {
    type Output = Number;
    
    fn neg(self) -> Self::Output {
        (-self.0).into()
    }
}


// impl ops::Not for Number {
//     type Output = Number;

//     fn not(self) -> Self::Output {
//         ( !(self.0) ).into()
//     }
// }

impl hash::Hash for Number {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // +NAN: 9221120237041090560
        // -NAN: 18444492273895866368
        //   -0: 9223372036854775808
        //   +0: 0
        // 
        // hash(+NAN) == hash(9221120237041090560)
        // hash(-NAN) == hash(9221120237041090560)
        // hash(+0)   == hash(0)
        // hash(-0)   == hash(0)
        let num = if self.0.is_nan() && self.0.is_sign_negative() {
            ::std::f64::NAN
        } else if self.0 == 0f64 && self.0.is_sign_negative() {
            0f64
        } else {
            self.0
        };

        num.to_bits().hash(state);
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        // +0   == -0
        // +NaN == +NaN
        // -NaN == -NaN
        // +NaN == -NaN
        if self.0.is_nan() {
            if other.0.is_nan() {
                true
            } else {
                false
            }
        } else {
            self.0 == other.0
        }
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Number) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Number) -> cmp::Ordering {
        if self.0  == other.0 {
            cmp::Ordering::Equal
        } else if self.0 > other.0 {
            cmp::Ordering::Greater
        } else if self.0 < other.0 {
            cmp::Ordering::Less
        } else {
            unreachable!()
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::f64::{ INFINITY, NEG_INFINITY, NAN };

        if self.0.is_nan() {
            write!(f, "NaN")
        } else if self.0.is_infinite() {
            if self.0.is_sign_negative() {
                write!(f, "-Infinity")
            } else {
                write!(f, "Infinity")
            }
        } else {
            write!(f, "{}", self.0)
        }
    }
}


impl From<u8> for Number {
    fn from(n: u8) -> Self {
        Number(n as f64)
    }
}
impl From<u16> for Number {
    fn from(n: u16) -> Self {
        Number(n as f64)
    }
}
impl From<u32> for Number {
    fn from(n: u32) -> Self {
        Number(n as f64)
    }
}
impl From<u64> for Number {
    fn from(n: u64) -> Self {
        Number(n as f64)
    }
}
impl From<i8> for Number {
    fn from(n: i8) -> Self {
        Number(n as f64)
    }
}
impl From<i16> for Number {
    fn from(n: i16) -> Self {
        Number(n as f64)
    }
}
impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Number(n as f64)
    }
}
impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Number(n as f64)
    }
}
impl From<isize> for Number {
    fn from(n: isize) -> Self {
        Number(n as f64)
    }
}
impl From<usize> for Number {
    fn from(n: usize) -> Self {
        Number(n as f64)
    }
}

impl From<f32> for Number {
    fn from(n: f32) -> Self {
        Number(n as f64)
    }
}
impl From<f64> for Number {
    fn from(n: f64) -> Self {
        Number(n)
    }
}


impl From<bool> for Number {
    fn from(b: bool) -> Self {
        if b == true {
            1.0f64.into()
        } else {
            0.0f64.into()
        }
    }
}

impl Into<bool> for Number {
    fn into(self) -> bool {
        if self.0 == 0.0f64 || self.0 == -0.0f64 {
            false
        } else if self.0.is_nan() {
            false
        } else {
            true
        }
    }
}

impl Into<i8> for Number {
    fn into(self) -> i8 {
        self.0 as i8
    }
}
impl Into<i16> for Number {
    fn into(self) -> i16 {
        self.0 as i16
    }
}
impl Into<i32> for Number {
    fn into(self) -> i32 {
        self.0 as i32
    }
}
impl Into<i64> for Number {
    fn into(self) -> i64 {
        self.0 as i64
    }
}
impl Into<isize> for Number {
    fn into(self) -> isize {
        self.0 as isize
    }
}


impl Into<u8> for Number {
    fn into(self) -> u8 {
        self.0 as u8
    }
}
impl Into<u16> for Number {
    fn into(self) -> u16 {
        self.0 as u16
    }
}
impl Into<u32> for Number {
    fn into(self) -> u32 {
        self.0 as u32
    }
}
impl Into<u64> for Number {
    fn into(self) -> u64 {
        self.0 as u64
    }
}
impl Into<f64> for Number {
    fn into(self) -> f64 {
        self.0
    }
}
impl Into<usize> for Number {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl From<Undefined> for Number {
    fn from(s: Undefined) -> Self {
        use std::f64;

        f64::NAN.into()
    }
}
impl From<Null> for Number {
    fn from(s: Null) -> Self {
        0.0f64.into()
    }
}
impl From<Boolean> for Number {
    fn from(s: Boolean) -> Self {
        let rust_bool: bool = s.into();
        rust_bool.into()
    }
}
impl From<String> for Number {
    fn from(s: String) -> Self {
        // NOTE: Should parse ECMAScript Number KEYWORDS
        //       -Infinity
        //       +Infinity
        //       Infinity
        //       -NaN
        //       +NaN
        //       NaN
        use std::f64::NAN;

        let rust_s: ::std::string::String = s.into();
        rust_s.parse::<f64>().unwrap_or(NAN).into()
    }
}
impl<'a> From<&'a String> for Number {
    fn from(s: &'a String) -> Self {
        // NOTE: Should parse ECMAScript Number KEYWORDS
        //       -Infinity
        //       +Infinity
        //       Infinity
        //       -NaN
        //       +NaN
        //       NaN
        use std::f64::NAN;

        let rust_s: ::std::string::String = s.to_string().into();
        rust_s.parse::<f64>().unwrap_or(NAN).into()
    }
}
impl From<Object> for Number {
    fn from(s: Object) -> Self {
        0.0f64.into()
    }
}
impl<'a> From<&'a Object> for Number {
    fn from(s: &'a Object) -> Self {
        0.0f64.into()
    }
}


#[test]
fn test_number_hash() {
    use std::collections::HashMap;
    use std::f64::NAN;

    assert_eq!(Number(NAN), Number(NAN));
    assert_eq!(Number(NAN), Number(-NAN));
    assert_eq!(Number(-NAN), Number(-NAN));

    assert_eq!(Number(0.0f64), Number(0.0f64));
    assert_eq!(Number(0.0f64), Number(-0.0f64));
    assert_eq!(Number(-0.0f64), Number(-0.0f64));

    let mut data: HashMap<Number, u8> = HashMap::new();

    // TEST +NAN, -NAN
    data.insert(Number(NAN), 0u8);

    assert_eq!(data.get(&Number(NAN)).unwrap(), &0u8);
    assert_eq!(data.get(&Number(-NAN)).unwrap(), &0u8);

    data.insert(Number(-NAN), 1u8);

    assert_eq!(data.get(&Number(NAN)).unwrap(), &1u8);
    assert_eq!(data.get(&Number(-NAN)).unwrap(), &1u8);

    // TEST +0.0, -0.0
    data.insert(Number(0.0f64), 0u8);

    assert_eq!(data.get(&Number(0.0f64)).unwrap(), &0u8);
    assert_eq!(data.get(&Number(-0.0f64)).unwrap(), &0u8);

    data.insert(Number(-0.0f64), 1u8);

    assert_eq!(data.get(&Number(0.0f64)).unwrap(), &1u8);
    assert_eq!(data.get(&Number(-0.0f64)).unwrap(), &1u8);
}


#[test]
fn test_to_integer() {
    use std::f64;

    assert_eq!(Number(f64::NAN).toInteger(), Number(0.0f64));
    assert_eq!(Number(-f64::NAN).toInteger(), Number(0.0f64));

    assert_eq!(Number(f64::INFINITY).toInteger(), Number(f64::INFINITY));
    assert_eq!(Number(f64::NEG_INFINITY).toInteger(), Number(f64::NEG_INFINITY));
        
    assert_eq!(Number(0.0f64).toInteger(), Number(0.0f64));
    assert_eq!(Number(-0.0f64).toInteger(), Number(-0.0f64));

    assert_eq!(Number(-100.0).toInteger(), Number(100.0f64));
    assert_eq!(Number(-100.1).toInteger(), Number(100.0f64));

    assert_eq!(Number(100.0).toInteger(), Number(100.0f64));
    assert_eq!(Number(100.1).toInteger(), Number(100.0f64));
}

#[test]
fn test_to_exponential() {
    assert_eq!(Number(12.2345678f64).toExponential(&Value::Number(21.0f64.into())).is_err(), true );

    assert_eq!(Number(12.2345678f64).toExponential(&Value::Number(0.0f64.into())), Ok("1e1".into()) );
    assert_eq!(Number(12.2345678f64).toExponential(&Value::Number(1.0f64.into())), Ok("1.2e1".into()) );
    assert_eq!(Number(12.2345678f64).toExponential(&Value::Number(2.0f64.into())), Ok("1.22e1".into()) );
    assert_eq!(Number(12.2345678f64).toExponential(&Value::Number(3.0f64.into())), Ok("1.223e1".into()) );
    assert_eq!(Number(12.2345678f64).toExponential(&Value::Number(4.0f64.into())), Ok("1.2235e1".into()) );
}


#[test]
fn test_to_precision() {
    assert_eq!(Number(12.2345678f64).toPrecision(&Value::Number(0.0f64.into())).is_err(), true );
    assert_eq!(Number(12.2345678f64).toPrecision(&Value::Number(22.0f64.into())).is_err(), true );
    
    assert_eq!(Number(12.2345678f64).toPrecision(&Value::Number(1.0f64.into())), Ok("1".into()) );
    assert_eq!(Number(12.2345678f64).toPrecision(&Value::Number(2.0f64.into())), Ok("12".into()) );
    assert_eq!(Number(12.2345678f64).toPrecision(&Value::Number(3.0f64.into())), Ok("12.2".into()) );
    assert_eq!(Number(12.2345678f64).toPrecision(&Value::Number(4.0f64.into())), Ok("12.23".into()) );
    assert_eq!(Number(12.2345678f64).toPrecision(&Value::Number(5.0f64.into())), Ok("12.235".into()) );
}

#[test]
fn test_to_fixed() {
    assert_eq!(Number(12.2345678f64).toFixed(&Value::Number(21.0f64.into())).is_err(), true);

    assert_eq!(Number(12.0f64).toFixed(&Value::Number(0.0f64.into())), Ok("12".into()) );
    assert_eq!(Number(1000000000000000000.0f64).toFixed(&Value::Number(0.0f64.into())), Ok("1000000000000000000".into()) );

    assert_eq!(Number(12.2345678f64).toFixed(&Value::Number(1.0f64.into())), Ok("12.2".into()) );
    assert_eq!(Number(12.2345678f64).toFixed(&Value::Number(2.0f64.into())), Ok("12.23".into()) );
    assert_eq!(Number(12.2345678f64).toFixed(&Value::Number(3.0f64.into())), Ok("12.235".into()) );
    assert_eq!(Number(12.2345678f64).toFixed(&Value::Number(4.0f64.into())), Ok("12.2346".into()) );
    assert_eq!(Number(12.2345678f64).toFixed(&Value::Number(19.0f64.into())), Ok("12.2345678000000006591".into()));

    // (1000000000000000128).toFixed(0) returns "1000000000000000128".
    assert_eq!(Number(1000000000000000128.0f64).toFixed(&Value::Number(0.0f64.into())), Ok("1000000000000000128".into()) );
    assert_eq!(Number(1000000000000000128.2f64).toFixed(&Value::Number(0.0f64.into())), Ok("1000000000000000128".into()) );
    assert_eq!(Number(1000000000000000128.22f64).toFixed(&Value::Number(1.0f64.into())), Ok("1000000000000000128.0".into()) );
    assert_eq!(Number(1000000000000000128.22f64).toFixed(&Value::Number(2.0f64.into())), Ok("1000000000000000128.00".into()) );
}

#[test]
fn test_pow() {
    // assert_eq!(Number(2.0f64).pow(&Number(2.00f64)), Number(4.928400000000001f64) );
}

