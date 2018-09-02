

/// Unicode String
pub struct UString {
    inner: Vec<char>,
}

impl UString {
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}