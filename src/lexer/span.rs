use std::fmt;


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct LineColumn {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Span {
    pub start: LineColumn,
    pub end: LineColumn,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Loc {
    pub start: usize,
    pub end: usize,
}

impl fmt::Debug for LineColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{:?}", self.line, self.column)
    }
}

impl fmt::Debug for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}..{:?}", self.start, self.end)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}..{:?}", self.start, self.end)
    }
}