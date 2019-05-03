
#[repr(u8)]
#[derive(Debug)]
pub enum Instruction {
    SourceMap,
    Push,
}

#[derive(Debug)]
pub struct ByteCode {
    pub kind: Instruction,
    pub offset: usize,
    pub size: usize,
}
