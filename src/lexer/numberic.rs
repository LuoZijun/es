use ast::float::{ Float,  };


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum NumbericErrorKind {
    Overflow,
    InvalidDigit,
    // invalid float literal
    InvalidFloat,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ParseNumbericError {
    kind: NumbericErrorKind,
    offset: usize,
}

impl ParseNumbericError {
    pub fn new(kind: NumbericErrorKind, offset: usize) -> Self {
        Self { kind, offset }
    }

    pub fn kind(&self) -> &NumbericErrorKind {
        &self.kind
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Numberic {
    I64(i64),
    F64(Float),
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

const ZERO: Numberic = Numberic::I64(0);
const BINARY: u32 = 2;
const OCTAL: u32  = 8;
const HEX: u32    = 16;


#[inline]
pub fn from_chars_radix(digits: &[char], radix: u32, pre_offset: usize) -> Result<u64, ParseNumbericError> {
    assert_eq!(radix == BINARY || radix == OCTAL || radix == HEX, true);

    let digits_len = digits.len() as u32;

    if digits_len == 0 {
        return Err(ParseNumbericError::new(NumbericErrorKind::InvalidDigit, pre_offset));
    }

    let mut offset: u32 = 0;
    let mut acc: u64 = 0;

    while offset < digits_len {
        let digit: u32 = digits[offset as usize]
                            .to_digit(radix)
                            .ok_or(ParseNumbericError::new(NumbericErrorKind::InvalidDigit, pre_offset + offset as usize))?;

        let idx = digits_len - 1 - offset;
        match radix.checked_pow(idx) {
            Some(n) => match digit.checked_mul(n) {
                Some(n) => {
                    // acc += digit * radix.pow(idx);
                    acc += n as u64;
                },
                None => {
                    return Err(ParseNumbericError::new(NumbericErrorKind::Overflow, pre_offset + offset as usize));
                },
            },
            None => {
                return Err(ParseNumbericError::new(NumbericErrorKind::Overflow, pre_offset + offset as usize));
            },
        }
        
        offset += 1;
    }

    Ok(acc)
}



#[inline]
pub fn parse(input: &[char]) -> Result<Numberic, ParseNumbericError> {
    let input_len = input.len();
    let mut idx = 0usize;

    if idx >= input_len {
        return Err(ParseNumbericError::new(NumbericErrorKind::InvalidDigit, idx));
    }

    let c = input[idx];
    match c {
        '0' => {
            idx += 1;
            match input.get(idx) {
                Some('b') | Some('B') => {
                    idx += 1;
                    
                    if idx >= input_len {
                        return Err(ParseNumbericError::new(NumbericErrorKind::InvalidDigit, idx));
                    }

                    let n = from_chars_radix(&input[idx..], BINARY, idx)?;

                    return Ok(n.into())
                },
                Some('o') | Some('O') => {
                    idx += 1;
                    
                    if idx >= input_len {
                        return Err(ParseNumbericError::new(NumbericErrorKind::InvalidDigit, idx));
                    }

                    let n = from_chars_radix(&input[idx..], OCTAL, idx)?;

                    return Ok(n.into())
                },
                Some('x') | Some('X') => {
                    idx += 1;
                    
                    if idx >= input_len {
                        return Err(ParseNumbericError::new(NumbericErrorKind::InvalidDigit, idx));
                    }

                    let n = from_chars_radix(&input[idx..], HEX, idx)?;

                    return Ok(n.into())
                },
                Some('.') => {
                    let s = input.iter().collect::<String>();
                    match s.parse::<f64>() {
                        Ok(float) => {
                            return Ok(Numberic::F64(float.into()));
                        },
                        Err(_) => {
                            return Err(ParseNumbericError::new(NumbericErrorKind::InvalidFloat, 0));
                        }
                    }
                },
                Some('e') | Some('E') => {
                    let s = input.iter().collect::<String>();
                    match s.parse::<f64>() {
                        Ok(float) => {
                            return Ok(Numberic::F64(float.into()));
                        },
                        Err(_) => {
                            return Err(ParseNumbericError::new(NumbericErrorKind::InvalidFloat, 0));
                        }
                    }
                },
                Some(c) => {
                    // c.is_es_decimal_digit()
                    // warn!("please don't add zero on numeric's head.");
                    return Err(ParseNumbericError::new(NumbericErrorKind::InvalidDigit, idx));
                },
                None => {
                    return Ok(ZERO);
                }
            }
        },
        '1' ... '9' => {
            let s = input.iter().collect::<String>();
            match s.parse::<f64>() {
                Ok(float) => {
                    let fract = float.fract();
                    // TODO: 计算 EPSILON ? std::f64::EPSILON
                    if fract < 0.0 || fract > 0.0 {
                        return Ok(Numberic::F64(float.into()));
                    } else {
                        // int
                        let trunc = float.trunc() as i64;
                        return Ok(Numberic::I64(trunc));
                    }
                },
                Err(_) => {
                    return Err(ParseNumbericError::new(NumbericErrorKind::InvalidFloat, 0));
                }
            }
        },
        '-' => {
            idx += 1;
            return parse(&input[idx..]).map(|n| {
                match n {
                    Numberic::I64(int) => Numberic::I64(-int),
                    Numberic::F64(float) => Numberic::F64( Float(-(float.0)) ),
                }
            });
        },
        '+' => {
            idx += 1;
            return parse(&input[idx..]);
        },
        _ => {
            return Err(ParseNumbericError::new(NumbericErrorKind::InvalidDigit, idx));
        }
    }
}


#[test]
fn test_from_chars_radix() {
    let f = |s: &str| -> Vec<char> {
        s.chars().collect::<Vec<char>>()
    };

    assert_eq!(from_chars_radix(&f("111"), 2, 0), Ok(7));
    assert_eq!(from_chars_radix(&f("1000"), 2, 0), Ok(8));

    assert_eq!(from_chars_radix(&f("123"), 8, 0), Ok(83));
    assert_eq!(from_chars_radix(&f("255"), 8, 0), Ok(173));

    assert_eq!(from_chars_radix(&f("69"), 16, 0), Ok(105));
    assert_eq!(from_chars_radix(&f("ff"), 16, 0), Ok(255));
}

#[test]
fn test_parse_float() {
    let f = |s: &str| -> Vec<char> {
        s.chars().collect::<Vec<char>>()
    };
    
    assert_eq!(parse(&f("0.234235834589")), Ok(Numberic::F64( 0.234235834589f64.into() )));
    assert_eq!(parse(&f("0.234235834589e1")), Ok(Numberic::F64( 0.234235834589e1f64.into() )));
}

#[test]
fn test_parse_int() {
    let f = |s: &str| -> Vec<char> {
        s.chars().collect::<Vec<char>>()
    };

    assert_eq!(parse(&f("0")), Ok(Numberic::I64(0)));
    assert_eq!(parse(&f("023424")).is_err(), true);
    assert_eq!(parse(&f("12342342")), Ok(Numberic::I64(12342342)));

    assert_eq!(parse(&f("0b111111")), Ok(Numberic::I64(63)));
    assert_eq!(parse(&f("0o124234126")), Ok(Numberic::I64(22100054)));
    assert_eq!(parse(&f("0x69")), Ok(Numberic::I64(105)));
}

#[bench]
fn bench_parse_ecmascript_float(b: &mut test::Bencher) {
    let input = "0x1232345".chars().collect::<Vec<char>>();

    b.bytes = input.len() as u64;
    b.iter(|| {
        let _ = parse(&input);
    });
}

