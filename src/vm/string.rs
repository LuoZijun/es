use std::fmt;
use std::cmp;
use std::hash;

#[cfg(feature = "nightly")]
use std::alloc::{ alloc, dealloc, Layout, };

// 
// Feature: alloc_layout_extra
// 

// 8 Bytes ( 64 Bits )
/// ECMAScript String
#[cfg(feature = "nightly")]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct String {
    inner: Box<ESStringInner>,
}

#[cfg(not(feature = "nightly"))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct String {
    inner: Box<std::string::String>,
}


impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(feature = "nightly")]
impl String {
    pub fn from_utf8_unchecked(bytes: &[u8]) -> Self {
        String { inner: Box::new(ESStringInner::new(bytes)) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
    
    pub fn into_string(self) -> std::string::String {
        self.inner.into_string()
    }
}

#[cfg(feature = "nightly")]
pub struct ESStringInner {
    layout: Layout,
    ptr: *mut u8,
}

#[cfg(feature = "nightly")]
impl ESStringInner {
    pub fn new(bytes: &[u8]) -> Self {
        unsafe {
            let len = bytes.len();
            let layout = Layout::array::<u8>(len).unwrap();
            let ptr = alloc(layout);

            for idx in 0..len {
                *(ptr.offset(idx as isize)) = bytes[idx];
            }

            Self { layout, ptr }
        }
    }

    pub fn from_string(s: std::string::String) -> Self {
        Self::new(s.as_bytes())
    }

    pub fn from_str(s: &str) -> Self {
        Self::new(s.as_bytes())
    }

    pub fn from_chars(s: &[char]) -> Self {
        Self::from_string(s.iter().collect::<std::string::String>())
    }

    pub fn len(&self) -> usize {
        self.layout.size()
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.ptr
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr as *const u8
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len()) }
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }
    
    pub fn into_string(self) -> std::string::String {
        unsafe { std::string::String::from_raw_parts(self.ptr, self.len(), self.len()) }
    }
}

#[cfg(feature = "nightly")]
impl Drop for ESStringInner {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}

#[cfg(feature = "nightly")]
impl Clone for ESStringInner {
    fn clone(&self) -> Self {
        Self::new(self.as_bytes())
    }
}

#[cfg(feature = "nightly")]
impl PartialEq for ESStringInner {
    fn eq(&self, other: &ESStringInner) -> bool {
        self.as_bytes().eq(other.as_bytes())
    }
}

#[cfg(feature = "nightly")]
impl Eq for ESStringInner {}

#[cfg(feature = "nightly")]
impl PartialOrd for ESStringInner {
    fn partial_cmp(&self, other: &ESStringInner) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "nightly")]
impl Ord for ESStringInner {
    fn cmp(&self, other: &ESStringInner) -> cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

#[cfg(feature = "nightly")]
impl hash::Hash for ESStringInner {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
    }
}

#[cfg(feature = "nightly")]
impl fmt::Debug for ESStringInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

#[cfg(feature = "nightly")]
impl fmt::Display for ESStringInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
