//! Describes a general assembly instruction.

use super::DataWord;

pub struct Instruction {
    pub operations: Vec<Operation>,
}

pub enum Operation {
    Nop,
}

pub enum Operand {
    Register(String),
    Immidiate(DataWord),
}
