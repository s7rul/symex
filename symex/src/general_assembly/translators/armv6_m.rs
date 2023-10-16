//! Translator for the armv6-m instruction set

use armv6_m_instruction_parser::instructons::Instruction;

use crate::general_assembly::translator::Translator;

impl Translator for Instruction {
    fn translate(&self) -> crate::general_assembly::instruction::Instruction {
        todo!()
    }
}
