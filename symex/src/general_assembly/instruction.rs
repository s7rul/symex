//! Describes a general assembly instruction.

use super::DataWord;

#[derive(Debug)]
pub struct Instruction {
    pub instruction_size: u32,
    pub operations: Vec<Operation>,
}

#[derive(Debug)]
pub enum Operation {
    /// No operation
    Nop,

    /// Moves the value in the source to the destination.
    /// If source is an address it is loaded from memmory
    /// and if destination is an address it is stored into memmory.
    Move {
        destination: Operand,
        source: Operand,
    },

    /// Adds
    /// destination = operand1 + operand2
    Add {
        destination: Operand,
        operand1: Operand,
        operand2: Operand,
    },

    /// Subtracts
    /// destination = operand1 - operand2
    Sub {
        destination: Operand,
        operand1: Operand,
        operand2: Operand,
    },

    /// And
    /// destination = operand1 & operand2
    And {
        destination: Operand,
        operand1: Operand,
        operand2: Operand,
    },

    /// Or
    /// destination = operand1 | operand2
    Or {
        destination: Operand,
        operand1: Operand,
        operand2: Operand,
    },

    /// Xor
    /// destination = operand1 ^ operand2
    Xor {
        destination: Operand,
        operand1: Operand,
        operand2: Operand,
    },

    /// Shift left
    /// destination = operand << shift
    Sl {
        destination: Operand,
        operand: Operand,
        shift: Operand,
    },

    /// Shift rigt logical
    /// destination = operand >> shift
    Srl {
        destination: Operand,
        operand: Operand,
        shift: Operand,
    },

    /// Shift rigt arithmetic
    /// destination = operand >> shift
    Sra {
        destination: Operand,
        operand: Operand,
        shift: Operand,
    },

    /// Zero extend
    /// Zero exstends bits bits from operand and stores it in destination.
    ZeroExtend {
        destination: Operand,
        operand: Operand,
        bits: u32,
    },

    /// Conditional jump
    ConditionalJump {
        destination: Operand,
        condition: Condition,
    },

    /// Set the negative flag
    SetNFlag(Operand),

    /// Set the zero flag
    SetZFlag(Operand),

    /// Set the carry flag
    SetCFlag {
        operand1: Operand,
        operand2: Operand,
        sub: bool,
    },

    /// Set overfolow flag
    SetVFlag {
        operand1: Operand,
        operand2: Operand,
        sub: bool,
    },

    /// Do all the operations in operations for each operand.
    /// The current operand is stored in local as CurrentOperand.
    ForEach {
        operands: Vec<Operand>,
        operations: Vec<Operation>,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Condition {
    /// Equal
    EQ,

    /// Not Equal
    NE,
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    None,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Register(String),
    Immidiate(DataWord),
    AddressInLocal(String, u32),
    Address(DataWord, u32),
    AddressWithOffset {
        address: DataWord,
        offset_reg: String,
        width: u32,
    },
    Local(String),
}
