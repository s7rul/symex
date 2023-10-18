//! General assembly executor

use std::collections::HashMap;

use tracing::trace;

use crate::{
    general_assembly::path_selection::Path,
    smt::{DContext, DExpr},
};

use super::{
    instruction::{Instruction, Operand},
    project::Project,
    state::GAState,
    vm::VM,
    DataWord, Result,
};

pub struct GAExecutor<'vm> {
    pub vm: &'vm mut VM,
    pub state: GAState,
    pub project: &'static Project,
}

pub enum PathResult {
    Success(Option<DExpr>),
    Faliure,
    AssumptionUnsat,
    Suppress,
}

impl<'vm> GAExecutor<'vm> {
    pub fn from_state(state: GAState, vm: &'vm mut VM, project: &'static Project) -> Self {
        Self { vm, state, project }
    }

    pub fn resume_execution(&mut self) -> Result<PathResult> {
        for _ in 0..5 {
            let instruction = self.state.get_next_instruction()?;
            trace!("executing instruction: {:?}", instruction);
            self.execute_instruction(&instruction)?;
        }
        todo!()
    }

    pub fn fork(&mut self, constraint: DExpr) -> Result<()> {
        trace!("Save backtracking path: constraint={:?}", constraint);
        let forked_state = self.state.clone();
        let path = Path::new(forked_state, Some(constraint));

        self.vm.paths.save_path(path);
        Ok(())
    }

    /// Resolve an address expression to a single value.
    ///
    /// If the address contain more than one possible address, then we create new paths for all
    /// but one of the addresses.
    fn resolve_address(&mut self, address: DExpr) -> Result<DExpr> {
        if let Some(_) = address.get_constant() {
            return Ok(address);
        }

        // Create new paths for all but one of the addresses.
        let mut addresses = self.state.memory.resolve_addresses(&address, 50)?;
        for address in addresses.iter().skip(1) {
            let constraint = address._eq(&address);
            self.fork(constraint)?;
        }

        // If we received more than one possible address, then constrain our current address.
        if addresses.len() > 1 {
            let constraint = address._eq(&addresses[0]);
            self.state.constraints.assert(&constraint);
        }

        match addresses.is_empty() {
            true => panic!("no address..."),
            false => Ok(addresses.swap_remove(0)),
        }
    }

    fn get_dexpr_from_dataword(&mut self, data: DataWord) -> DExpr {
        match data {
            DataWord::Word64(v) => self.state.ctx.from_u64(v, 64),
            DataWord::Word32(v) => self.state.ctx.from_u64(v as u64, 32),
            DataWord::Word16(v) => self.state.ctx.from_u64(v as u64, 16),
            DataWord::Word8(v) => self.state.ctx.from_u64(v as u64, 8),
        }
    }

    fn get_operand_value(&mut self, operand: &Operand, local: &HashMap<String, DExpr>) -> DExpr {
        match operand {
            Operand::Register(name) => match self.state.get_register(name.to_owned()) {
                Some(v) => v,
                None => {
                    // If register not writen to asume it can be any value
                    let value = self
                        .state
                        .ctx
                        .unconstrained(self.project.get_word_size(), name);
                    self.state.set_register(name.to_owned(), value.clone());
                    value
                }
            },
            Operand::Immidiate(v) => self.get_dexpr_from_dataword(v.to_owned()),
            Operand::Address(_) => todo!(),
            Operand::AddressWithOffset {
                address,
                offset_reg,
            } => todo!(),
            Operand::Local(k) => (local.get(k).unwrap()).to_owned(),
            Operand::AddressInLocal(_) => todo!(),
        }
    }

    fn set_operand_value(
        &mut self,
        operand: &Operand,
        value: DExpr,
        local: &mut HashMap<String, DExpr>,
    ) {
        match operand {
            Operand::Register(v) => self.state.set_register(v.to_owned(), value),
            Operand::Immidiate(_) => panic!(), // not prohibited change to error later
            Operand::AddressInLocal(_) => todo!(),
            Operand::Address(_) => todo!(),
            Operand::AddressWithOffset {
                address,
                offset_reg,
            } => todo!(),
            Operand::Local(k) => {
                local.insert(k.to_owned(), value);
            }
        }
    }

    fn execute_instruction(&mut self, i: &Instruction) -> Result<()> {
        // Always increment pc before doing anything
        let new_pc = self.state.get_register("PC".to_owned()).unwrap();
        self.state.set_register(
            "PC".to_owned(),
            new_pc.add(&self.state.ctx.from_u64((i.instruction_size / 8) as u64, 64)),
        );

        // initiate local variable storage
        let mut local: HashMap<String, DExpr> = HashMap::new();

        for operation in &i.operations {
            trace!("Executing operation: {:?}", operation);
            match operation {
                crate::general_assembly::instruction::Operation::Nop => (), // nop so do nothig
                crate::general_assembly::instruction::Operation::Move {
                    destination,
                    source,
                } => {
                    let value = self.get_operand_value(source, &mut local);
                    self.set_operand_value(destination, value, &mut local);
                }
                crate::general_assembly::instruction::Operation::Add {
                    destination,
                    operand1,
                    operand2,
                } => todo!(),
                crate::general_assembly::instruction::Operation::Sub {
                    destination,
                    operand1,
                    operand2,
                } => {
                    let op1 = self.get_operand_value(operand1, &local);
                    let op2 = self.get_operand_value(operand2, &local);
                    let result = op1.sub(&op2);
                    self.set_operand_value(destination, result, &mut local);
                }
                crate::general_assembly::instruction::Operation::And {
                    destination,
                    operand1,
                    operand2,
                } => todo!(),
                crate::general_assembly::instruction::Operation::Or {
                    destination,
                    operand1,
                    operand2,
                } => todo!(),
                crate::general_assembly::instruction::Operation::Xor {
                    destination,
                    operand1,
                    operand2,
                } => todo!(),
                crate::general_assembly::instruction::Operation::Sl {
                    destination,
                    operand,
                    shift,
                } => {
                    let value = self.get_operand_value(operand, &local);
                    let shift_amount = self.get_operand_value(shift, &local);
                    let result = value.sll(&shift_amount);
                    self.set_operand_value(destination, result, &mut local);
                }
                crate::general_assembly::instruction::Operation::Srl {
                    destination,
                    operand,
                    shift,
                } => {
                    let value = self.get_operand_value(operand, &local);
                    let shift_amount = self.get_operand_value(shift, &local);
                    let result = value.srl(&shift_amount);
                    self.set_operand_value(destination, result, &mut local);
                }
                crate::general_assembly::instruction::Operation::Sra {
                    destination,
                    operand,
                    shift,
                } => todo!(),
                crate::general_assembly::instruction::Operation::Jump { destination } => todo!(),
                crate::general_assembly::instruction::Operation::ConditionalJump {
                    destination,
                    condition,
                } => todo!(),
                crate::general_assembly::instruction::Operation::SetNFlag(operand) => {
                    let value = self.get_operand_value(operand, &local);
                    let result = value._eq(&self.state.ctx.zero(self.project.get_word_size()));
                }
                crate::general_assembly::instruction::Operation::SetZFlag(_) => todo!(),
                crate::general_assembly::instruction::Operation::SetCFlag {
                    operand1,
                    operand2,
                    sub,
                } => todo!(),
                crate::general_assembly::instruction::Operation::SetVFlag {
                    operand1,
                    operand2,
                    sub,
                } => todo!(),
                crate::general_assembly::instruction::Operation::ForEach {
                    operands,
                    operations,
                } => todo!(),
                crate::general_assembly::instruction::Operation::ZeroExtend {
                    destination,
                    operand,
                    bits,
                } => {
                    let op = self.get_operand_value(operand, &local);
                    let result = op.zero_ext(*bits);
                }
            }
        }
        Ok(())
    }
}

#[test]
fn test_operation_zero_extend() {
    let context = Box::new(DContext::new());
    let context = Box::leak(context);

    let project = Box::new(Project::create_dummy(
        super::WordSize::Bit32,
        super::Endianness::Little,
        object::Architecture::Arm,
    ));
    let project = Box::leak(project);
}
