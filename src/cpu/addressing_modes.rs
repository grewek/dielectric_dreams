#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddressingMode {
    Atomic,
    Memory,
    MemoryInc,
    MemoryDec,
}
