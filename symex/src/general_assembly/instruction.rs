//! Describes a general assembly instruction.

pub struct Instruction {
    pub operations: Vec<Operation>,
}

pub enum Operation {
    Nop,
}
