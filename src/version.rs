use std::fmt;

const MAX_U16: u16 = u16::max_value();


#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

// https://en.wikipedia.org/wiki/ECMAScript#Versions
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ECMAScriptVersion {
    pub major: u16,
    pub minor: u16,
    pub micro: u16,
}



impl ECMAScriptVersion {
    pub const ES5: ECMAScriptVersion    = ECMAScriptVersion { major: 5, minor: 0, micro: 0 };
    pub const ES2009: ECMAScriptVersion = Self::ES5;

    // ECMAScript 2011 (ES5): https://www.ecma-international.org/ecma-262/5.1/index.html
    pub const ES2011: ECMAScriptVersion = ECMAScriptVersion { major: 5, minor: 1, micro: 0 };
    // ECMAScript 2015 (ES6): https://www.ecma-international.org/ecma-262/6.0/index.html
    pub const ES2015: ECMAScriptVersion = ECMAScriptVersion { major: 6, minor: 0, micro: 0 };
    // ECMAScript 2016 (ES2016): https://www.ecma-international.org/ecma-262/7.0/index.html
    pub const ES2016: ECMAScriptVersion = ECMAScriptVersion { major: 7, minor: 0, micro: 0 };
    // ECMAScript 2017 (ES2017): https://www.ecma-international.org/ecma-262/8.0/index.html
    pub const ES2017: ECMAScriptVersion = ECMAScriptVersion { major: 8, minor: 0, micro: 0 };
    // ECMAScript 2018 (ES2018): https://www.ecma-international.org/ecma-262/9.0/index.html
    pub const ES2018: ECMAScriptVersion = ECMAScriptVersion { major: 9, minor: 0, micro: 0 };

    pub const ESNEXT: ECMAScriptVersion = ECMAScriptVersion { major: MAX_U16, minor: MAX_U16, micro: MAX_U16 };

    pub const LATEST: ECMAScriptVersion = Self::ES2018;
}

impl ECMAScriptVersion {
    pub fn published_at(&self) -> Date {
        match *self {
            Self::ES5    => Date { year: 2009, month: 12, day: 0 },
            Self::ES2011 => Date { year: 2011, month:  6, day: 0 },
            Self::ES2015 => Date { year: 2015, month:  6, day: 0 },
            Self::ES2016 => Date { year: 2016, month:  6, day: 0 },
            Self::ES2017 => Date { year: 2017, month:  6, day: 0 },
            Self::ES2018 => Date { year: 2018, month:  6, day: 0 },
            Self::ESNEXT => Date { year: MAX_U16, month:  12, day: 30 },
            _ => unreachable!(),
        }
    }
}

impl Default for ECMAScriptVersion {
    fn default() -> Self {
        Self::LATEST
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.year, self.month)
    }
}

impl fmt::Display for ECMAScriptVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::ES5    => write!(f, "ES{}(ES5.0)", self.published_at().year),
            Self::ES2011 => write!(f, "ES{}(ES5.1)", self.published_at().year),
            Self::ES2015 => write!(f, "ES{}(ES6.0)", self.published_at().year),
            Self::ES2016 => write!(f, "ES{}(ES7.0)", self.published_at().year),
            Self::ES2017 => write!(f, "ES{}(ES8.0)", self.published_at().year),
            Self::ES2018 => write!(f, "ES{}(ES9.0)", self.published_at().year),
            Self::ESNEXT => write!(f, "ESNEXT"),
            _ => unreachable!(),
        }
    }
}


pub trait Version {
    // fn age(self) -> ECMAScriptVersion;
    fn standard_since(&self) -> ECMAScriptVersion;
    fn deprecated_since(&self) -> Option<ECMAScriptVersion>;
    fn is_deprecated_at(&self, target: ECMAScriptVersion) -> bool {
        match self.deprecated_since() {
            Some(ver) => target > ver,
            None => false,
        }
    }
}
