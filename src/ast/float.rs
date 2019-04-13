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
