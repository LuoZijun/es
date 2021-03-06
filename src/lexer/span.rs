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
        write!(f, "{:?}:{:?}", self.line + 1, self.column)
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

impl Default for LineColumn {
    fn default() -> Self {
        LineColumn { offset: 0, line: 0, column: 0, }
    }
}

impl Default for Span {
    fn default() -> Self {
        Span { start: LineColumn::default(), end: LineColumn::default(), }
    }
}

impl Default for Loc {
    fn default() -> Self {
        Loc { start: 0, end: 0, }
    }
}