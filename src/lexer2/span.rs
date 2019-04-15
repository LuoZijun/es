use std::fmt;
use std::hash;


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct LineColumn {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Default for LineColumn {
    fn default() -> Self {
        LineColumn { offset: 0, line: 0, column: 0 }
    }
}
impl fmt::Debug for LineColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
impl fmt::Display for LineColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Eq, Clone)]
pub struct Span<T: fmt::Debug + PartialEq + Clone> {
    pub start: LineColumn,
    pub end: LineColumn,
    pub item: T,
}

impl<T: fmt::Debug + PartialEq + Clone> PartialEq for Span<T> {
    fn eq(&self, other: &Span<T>) -> bool {
        self.item == other.item
    }
}

