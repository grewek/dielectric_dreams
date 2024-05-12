use super::register::Register;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddressingMode {
    Atomic(Register),
    Memory(Register),
    MemoryInc(Register),
    MemoryDec(Register),
}
