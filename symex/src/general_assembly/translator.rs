//! Describes the translator trait.
//! A translator that translates between machine code and general assembly instructions.

use super::instruction::Instruction;

pub trait Translator {
    fn translate(&self) -> Instruction;
}
