//! Describes the translator trait.
//! A translator that translates between machine code and general assembly instructions.

use super::instruction::Instruction;

/// A translator
pub trait Translatable {
    fn translate(&self) -> Instruction;
}
