use std::fmt;
use std::hash;


#[derive(Clone, Copy)]
pub struct Float(pub f64);

impl fmt::Debug for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Float) -> bool {
        let a = self.0;
        let b = other.0;

        if a.is_nan() {
            if b.is_nan() {
                true
            } else {
                false
            }
        } else {
            a == b
        }
    }
}

impl Eq for Float { }

impl hash::Hash for Float {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
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

impl Into<f64> for Float {
    fn into(self) -> f64 {
        self.0
    }
}

impl From<f64> for Float {
    fn from(n: f64) -> Self {
        Float(n)
    }
}

impl From<u64> for Float {
    fn from(n: u64) -> Self {
        Float(n as f64)
    }
}

impl From<i64> for Float {
    fn from(n: i64) -> Self {
        Float(n as f64)
    }
}
impl From<i32> for Float {
    fn from(n: i32) -> Self {
        Float(n as f64)
    }
}
impl From<isize> for Float {
    fn from(n: isize) -> Self {
        Float(n as f64)
    }
}


impl From<usize> for Float {
    fn from(n: usize) -> Self {
        Float(n as f64)
    }
}

impl From<u32> for Float {
    fn from(n: u32) -> Self {
        Float(n as f64)
    }
}

impl From<u16> for Float {
    fn from(n: u16) -> Self {
        Float(n as f64)
    }
}

impl From<u8> for Float {
    fn from(n: u8) -> Self {
        Float(n as f64)
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Numberic {
    I64(i64),
    F64(Float),
}

impl Numberic {
    pub const ZERO: Numberic = Numberic::I64(0);
}

impl fmt::Debug for Numberic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Numberic::I64(int) => write!(f, "{:?}", int),
            Numberic::F64(float) => write!(f, "{:?}", float),
        }
    }
}

impl fmt::Display for Numberic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Numberic::I64(int) => write!(f, "{}", int),
            Numberic::F64(float) => write!(f, "{}", float),
        }
    }
}

impl Into<Float> for Numberic {
    fn into(self: Numberic) -> Float {
        match self {
            Numberic::I64(n) => n.into(),
            Numberic::F64(float) => float,
        }
    }
}

impl Into<f64> for Numberic {
    fn into(self: Numberic) -> f64 {
        match self {
            Numberic::I64(n) => n as f64,
            Numberic::F64(float) => float.0,
        }
    }
}

impl From<u8> for Numberic {
    fn from(n: u8) -> Self {
        Numberic::I64(n as i64)
    }
}
impl From<u16> for Numberic {
    fn from(n: u16) -> Self {
        Numberic::I64(n as i64)
    }
}
impl From<u32> for Numberic {
    fn from(n: u32) -> Self {
        Numberic::I64(n as i64)
    }
}
impl From<u64> for Numberic {
    fn from(n: u64) -> Self {
        Numberic::I64(n as i64)
    }
}
impl From<usize> for Numberic {
    fn from(n: usize) -> Self {
        Numberic::I64(n as i64)
    }
}
impl From<i8> for Numberic {
    fn from(n: i8) -> Self {
        Numberic::I64(n as i64)
    }
}
impl From<i16> for Numberic {
    fn from(n: i16) -> Self {
        Numberic::I64(n as i64)
    }
}
impl From<i32> for Numberic {
    fn from(n: i32) -> Self {
        Numberic::I64(n as i64)
    }
}
impl From<i64> for Numberic {
    fn from(n: i64) -> Self {
        Numberic::I64(n)
    }
}
impl From<isize> for Numberic {
    fn from(n: isize) -> Self {
        Numberic::I64(n as i64)
    }
}
impl From<f32> for Numberic {
    fn from(n: f32) -> Self {
        Numberic::F64(Float(n as f64))
    }
}
impl From<f64> for Numberic {
    fn from(n: f64) -> Self {
        Numberic::F64(Float(n))
    }
}
