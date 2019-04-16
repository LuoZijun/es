
use std::fmt;
use std::slice;


/// Unicode String
pub struct UString {
    inner: Vec<char>,
}

impl UString {
    pub fn len_utf8(&self) -> usize {
        self.iter().map(|c| c.len_utf8()).sum()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> slice::Iter<'_, char> {
        self.inner.iter()
    }

    pub fn codePointAt(&self, index: usize) -> Option<u32> {
        self.inner.get(index).map(|c| *c as u32)
    }

    pub fn startsWith(&self, other: &Self) -> bool {
        self.inner.starts_with(&other.inner)
    }

    pub fn endsWith(&self, other: &Self) -> bool {
        self.inner.ends_with(&other.inner)
    }

    #[cfg(nightly)]
    pub fn repeat(&self, num: usize) -> Self {
        Self { inner: self.inner.repeat(num) }
    }

    #[cfg(not(nightly))]
    pub fn repeat(&mut self, num: usize) -> Self {
        let mut data = Vec::with_capacity(self.inner.len() * num);
        for _ in 0..num {
            data.extend_from_slice(&self.inner);
        }

        Self { inner: data }
    }

    pub fn indexOf(&self, s: &Self) -> Option<usize> {
        let mut idx = 0usize;

        while idx < self.len() {
            let data = &self.inner[idx..];
            if data.starts_with(&s.inner) {
                return Some(idx);
            }

            idx += 1;
        }

        return None;
    }

    pub fn toUpperCase(&self) -> Self {
        let acc = Vec::with_capacity(self.len());
        let acc: Vec<char> = self.iter().fold(acc, |mut acc, c| {
            c.to_uppercase().for_each(|c| acc.push(c));
            acc
        });

        Self { inner: acc }
    }

    pub fn toLowerCase(&self) -> Self {
        let acc = Vec::with_capacity(self.len());
        let acc: Vec<char> = self.iter().fold(acc, |mut acc, c| {
            c.to_uppercase().for_each(|c| acc.push(c));
            acc
        });
        
        Self { inner: acc }
    }

    pub fn trimStart(&self) -> Self {
        let mut idx = 0usize;
        
        while idx < self.len() {
            let c = self.inner[idx];
            
            if c.is_whitespace() {
                idx += 1;
            } else {
                break;
            }
        }

        Self { inner: self.inner[idx..].to_vec() }
    }

    pub fn trimLeft(&self) -> Self {
        self.trimStart()
    }

    pub fn trimEnd(&self) -> Self {
        let mut idx = self.len() - 1;
        
        loop {
            let c = self.inner[idx];
            
            if c.is_whitespace() {
                if idx == 0 {
                    break;
                }

                idx -= 1;
            } else {
                break;
            }
        }

        Self { inner: self.inner[..idx].to_vec() }
    }

    pub fn trimRight(&self) -> Self {
        self.trimEnd()
    }
    
    pub fn trim(&self) -> Self {
        self.trimStart().trimEnd()
    }

    pub fn slice(&self, start: usize, size: usize) -> Self {
        // https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/String/slice
        if size == 0 {
            return Self { inner: vec![] };
        }

        Self { inner: self.inner[start..size].to_vec() }
    }

    pub fn replace(&self, s: &Self, replace_with: &Self) -> Self {
        match self.indexOf(s) {
            Some(idx) => {
                let mut data = self.inner[..idx].to_vec();
                data.extend_from_slice(&replace_with.inner);
                data.extend_from_slice(&self.inner[idx + s.len()..]);
                Self { inner: data }
            },
            None => {
                Self { inner: self.inner.clone() }
            },
        }
    }
}

impl fmt::Debug for UString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner.iter().collect::<String>())
    }
}

impl fmt::Display for UString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner.iter().collect::<String>())
    }
}