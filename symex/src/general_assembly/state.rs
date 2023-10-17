//! Holds the state in general assembly execution.

use std::{collections::HashMap, rc::Rc};

use tracing::debug;

use crate::{
    general_assembly::{GAError, Result},
    memory::{MemoryError, ObjectMemory},
    smt::{DContext, DExpr, DSolver},
    util::Variable,
};

use super::{instruction::Instruction, project::Project};

#[derive(Clone, Debug)]
pub struct GAState {
    project: &'static Project,
    pub ctx: &'static DContext,
    pub constraints: DSolver,
    pub marked_symbolic: Vec<Variable>,
    memory: ObjectMemory,
    pub cycle_count: u128,
    registers: HashMap<String, DExpr>,
    pc_register: u64, // this register is special
}

impl GAState {
    pub fn new(
        ctx: &'static DContext,
        project: &'static Project,
        constraints: DSolver,
        function: &str,
    ) -> Result<Self> {
        let pc_reg = match project.get_symbol_address(function) {
            Some(a) => a,
            None => return Err(GAError::EntryFunctionNotFound(function.to_owned())),
        };
        debug!("Found function.");
        let ptr_size = project.get_ptr_size();

        let memory = ObjectMemory::new(ctx, ptr_size, constraints.clone());

        Ok(GAState {
            project,
            ctx,
            constraints,
            marked_symbolic: Vec::new(),
            memory,
            cycle_count: 0,
            registers: HashMap::new(),
            pc_register: pc_reg,
        })
    }

    pub fn set_register(&mut self, register: String, expr: DExpr) {
        // crude solution should prbobly change
        if register == "PC" {
            // A branch has occured if conditional forking state should occur
            // can't know if it is a conditional branch here thou.
            let value = match expr.get_constant() {
                Some(v) => v,
                None => todo!("handle branch to symbolic address"),
            };

            self.registers.insert(register, expr);
        }
    }

    pub fn get_register(&self, register: String) -> Option<&DExpr> {
        self.registers.get(&register)
    }

    pub fn get_next_instruction(&self) -> Result<Instruction> {
        Ok(self.project.get_instruction(self.pc_register)?)
    }

    fn read_word_from_memory_no_static(&self, address: &DExpr) -> Result<DExpr> {
        Ok(self.memory.read(address, self.project.get_word_size())?)
    }

    fn write_word_from_memory_no_static(&mut self, address: &DExpr, value: DExpr) -> Result<()> {
        Ok(self.memory.write(address, value)?)
    }

    pub fn read_word_from_memory(&self, address: &DExpr) -> Result<DExpr> {
        match address.get_constant() {
            Some(address_const) => {
                if self.project.address_in_range(address_const) {
                    // read from static memmory in project
                    let value = match self.project.get_word(address_const)? {
                        crate::general_assembly::DataWord::Word64(data) => {
                            self.ctx.from_u64(data, 64)
                        }
                        crate::general_assembly::DataWord::Word32(data) => {
                            self.ctx.from_u64(data as u64, 32)
                        }
                        crate::general_assembly::DataWord::Word16(data) => {
                            self.ctx.from_u64(data as u64, 16)
                        }
                        crate::general_assembly::DataWord::Word8(data) => {
                            self.ctx.from_u64(data as u64, 8)
                        }
                    };
                    Ok(value)
                } else {
                    self.read_word_from_memory_no_static(address)
                }
            }

            // For non constant addresses always read non_static memmory
            None => self.read_word_from_memory_no_static(address),
        }
    }

    pub fn write_word_to_memory(&mut self, address: &DExpr, value: DExpr) -> Result<()> {
        match address.get_constant() {
            Some(address_const) => {
                if self.project.address_in_range(address_const) {
                    Err(GAError::WritingToStaticMemoryProhibited)
                } else {
                    self.write_word_from_memory_no_static(address, value)
                }
            }

            // For non constant addresses always read non_static memmory
            None => self.write_word_from_memory_no_static(address, value),
        }
    }
}
