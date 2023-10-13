//! Describes a general assembly instruction.

use super::DataWord;

pub struct Instruction {
    pub operations: Vec<Operation>,
}

pub enum Operation {
    /// No operation
    Nop,

    /// Moves the value in the source to the destination.
    /// If source is an address it is loaded from memmory
    /// and if destination is an address it is stored into memmory.
    Move{destination: Operand, source: Operand},

    /// Adds
    /// destination = operand1 + operand2
    Add{destination: Operand, operand1: Operand, operand2: Operand},

    /// Subtracts
    /// destination = operand1 - operand2
    Sub{destination: Operand, operand1: Operand, operand2: Operand},
}

pub enum Operand {
    Register(String),
    Immidiate(DataWord),
    Address(DataWord),
    AddressWithOffset{address: DataWord, offset_reg: String},
}
