

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct LineColumn {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Span {
    pub start: LineColumn,
    pub end: LineColumn,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Loc {
    pub start: usize,
    pub end: usize,
}

