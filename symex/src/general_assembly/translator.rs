//! Describes the translator trait.
//! A translator that translates between machine code and general assembly instructions.

use super::{instruction::Instruction, project::PCHook};

/// A translator
pub trait Translatable {
    /// Translate the given instruction into a GA instruction.
    fn translate(&self) -> Instruction;

    /// Add target specific or dependant pc hooks.
    fn add_pc_hooks(hooks: &mut Vec<(&str, PCHook)>);
}
