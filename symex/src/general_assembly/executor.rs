//! General assembly executor

use tracing::trace;

use crate::{general_assembly::path_selection::Path, smt::DExpr};

use super::{
    instruction::{Instruction, Operand},
    project::Project,
    state::GAState,
    vm::VM,
    Result,
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

    fn get_operand_value(&self, operand: Operand) -> &DExpr {
        match operand {
            Operand::Register(name) => match self.state.get_register(name) {
                Some(v) => v,
                None => todo!(),
            },
            Operand::Immidiate(_) => todo!(),
            Operand::Address(_) => todo!(),
            Operand::AddressWithOffset {
                address,
                offset_reg,
            } => todo!(),
            Operand::Local(_) => todo!(),
        }
    }

    fn execute_instruction(&mut self, i: &Instruction) -> Result<()> {
        // Always increment pc before doing anything
        let new_pc = self.state.get_register("PC".to_owned()).unwrap();
        self.state.set_register(
            "PC".to_owned(),
            new_pc.add(&self.state.ctx.from_u64((i.instruction_size / 8) as u64, 64)),
        );
        return Ok(());

        for operation in &i.operations {
            match operation {
                crate::general_assembly::instruction::Operation::Nop => (), // nop so do nothig
                crate::general_assembly::instruction::Operation::Move {
                    destination,
                    source,
                } => {}
                crate::general_assembly::instruction::Operation::Add {
                    destination,
                    operand1,
                    operand2,
                } => todo!(),
                crate::general_assembly::instruction::Operation::Sub {
                    destination,
                    operand1,
                    operand2,
                } => todo!(),
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
                } => todo!(),
                crate::general_assembly::instruction::Operation::Srl {
                    destionation,
                    operand,
                    shift,
                } => todo!(),
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
                crate::general_assembly::instruction::Operation::SetNFlag(_) => todo!(),
                crate::general_assembly::instruction::Operation::SetZFlag(_) => todo!(),
                crate::general_assembly::instruction::Operation::SetCFlag {
                    operand1,
                    operand2,
                } => todo!(),
                crate::general_assembly::instruction::Operation::SetVFlag {
                    operand1,
                    operand2,
                } => todo!(),
                crate::general_assembly::instruction::Operation::ForEach {
                    operands,
                    operations,
                } => todo!(),
            }
        }
        Ok(())
    }
}
