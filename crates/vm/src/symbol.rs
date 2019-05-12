use std::fmt;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Symbol(usize);

impl Symbol {
    #[cfg(target_pointer_width = "32")]
    pub const WIDTH: u32 = 32;
    #[cfg(target_pointer_width = "64")]
    pub const WIDTH: u32 = 64;
    
    #[cfg(target_pointer_width = "32")]
    pub const MAX: usize = 2147483647;          // 2 ** 31 - 1
    #[cfg(target_pointer_width = "64")]
    pub const MAX: usize = 9223372036854775807; // 2 ** 63 - 1

    #[inline]
    pub fn new(is_public: bool, id: usize) -> Self {
        if id > Self::MAX {
            panic!("Symbol id too large (actual: {}, maximum: {}).", id, Self::MAX);
        }
        
        match is_public {
            true => Self(std::usize::MAX | id),
            false => Self(Self::MAX | id),
        }
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.0 & Self::MAX
    }

    #[inline]
    pub fn is_public(&self) -> bool {
        (self.0 >> (Self::WIDTH - 1)) == 1
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
