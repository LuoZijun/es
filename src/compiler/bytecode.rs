
#[repr(u8)]
#[derive(Debug)]
pub enum Instruction {
    SourceMap,
    Push,
}

#[derive(Debug)]
pub struct ByteCode {
    kind: Instruction,
    offset: usize,
    size: usize,
}