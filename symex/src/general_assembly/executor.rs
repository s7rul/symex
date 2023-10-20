//! General assembly executor

use std::collections::HashMap;

use tracing::{debug, trace};

use crate::{
    general_assembly::path_selection::Path,
    smt::{DContext, DExpr, SolverError},
    util::{ExpressionType, Variable},
};

use super::{
    instruction::{Condition, Instruction, Operand, Operation},
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
        loop {
            let instruction = match self.state.get_next_instruction()? {
                Some(v) => v,
                None => {
                    debug!("Symbolic execution ended succesfully");
                    for (reg_name, reg_value) in &self.state.registers {
                        debug!("{}: {:?}", reg_name, reg_value.clone().simplify())
                    }
                    return Ok(PathResult::Success(None));
                }
            };
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

    fn fork_and_jump(&mut self, new_pc: DExpr, constraint: Option<DExpr>) -> Result<()> {
        trace!(
            "Save backtracking path: constrint={:?}, new_pc={:?}",
            constraint,
            new_pc
        );

        let mut state = self.state.clone();
        state.set_register("PC".to_owned(), new_pc);
        let path = Path::new(state, constraint);
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
                    self.state.marked_symbolic.push(Variable {
                        name: Some(name.to_owned()),
                        value: value.clone(),
                        ty: ExpressionType::Integer(self.project.get_word_size() as usize),
                    });
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
            new_pc.add(
                &self
                    .state
                    .ctx
                    .from_u64((i.instruction_size / 8) as u64, self.project.get_ptr_size()),
            ),
        );

        // initiate local variable storage
        let mut local: HashMap<String, DExpr> = HashMap::new();
        for operation in &i.operations {
            self.executer_operation(operation, &mut local);
        }

        Ok(())
    }

    fn executer_operation(
        &mut self,
        operation: &Operation,
        local: &mut HashMap<String, DExpr>,
    ) -> Result<()> {
        trace!("Executing operation: {:?}", operation);
        match operation {
            crate::general_assembly::instruction::Operation::Nop => (), // nop so do nothig
            crate::general_assembly::instruction::Operation::Move {
                destination,
                source,
            } => {
                let value = self.get_operand_value(source, local);
                self.set_operand_value(destination, value, local);
            }
            crate::general_assembly::instruction::Operation::Add {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, &local);
                let op2 = self.get_operand_value(operand2, &local);
                let result = op1.add(&op2);
                self.set_operand_value(destination, result, local);
            }
            crate::general_assembly::instruction::Operation::Sub {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, &local);
                let op2 = self.get_operand_value(operand2, &local);
                let result = op1.sub(&op2);
                self.set_operand_value(destination, result, local);
            }
            crate::general_assembly::instruction::Operation::And {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, &local);
                let op2 = self.get_operand_value(operand2, &local);
                let result = op1.and(&op2);
                self.set_operand_value(destination, result, local);
            }
            crate::general_assembly::instruction::Operation::Or {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, &local);
                let op2 = self.get_operand_value(operand2, &local);
                let result = op1.or(&op2);
                self.set_operand_value(destination, result, local);
            }
            crate::general_assembly::instruction::Operation::Xor {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, &local);
                let op2 = self.get_operand_value(operand2, &local);
                let result = op1.xor(&op2);
                self.set_operand_value(destination, result, local);
            }
            crate::general_assembly::instruction::Operation::Sl {
                destination,
                operand,
                shift,
            } => {
                let value = self.get_operand_value(operand, &local);
                let shift_amount = self.get_operand_value(shift, &local);
                let result = value.sll(&shift_amount);
                self.set_operand_value(destination, result, local);
            }
            crate::general_assembly::instruction::Operation::Srl {
                destination,
                operand,
                shift,
            } => {
                let value = self.get_operand_value(operand, &local);
                let shift_amount = self.get_operand_value(shift, &local);
                let result = value.srl(&shift_amount);
                self.set_operand_value(destination, result, local);
            }
            crate::general_assembly::instruction::Operation::Sra {
                destination,
                operand,
                shift,
            } => {
                let value = self.get_operand_value(operand, &local);
                let shift_amount = self.get_operand_value(shift, &local);
                let result = value.sra(&shift_amount);
                self.set_operand_value(destination, result, local);
            }
            crate::general_assembly::instruction::Operation::Jump { destination } => todo!(),
            crate::general_assembly::instruction::Operation::ConditionalJump {
                destination,
                condition,
            } => {
                let c = self.state.get_expr(condition)?.simplify();

                // if constant just jump
                if let Some(constant_c) = c.get_constant_bool() {
                    if constant_c {
                        let destination = self.get_operand_value(destination, &local);
                        self.state.set_register("PC".to_owned(), destination);
                    }
                    return Ok(());
                }

                let true_possible = self.state.constraints.is_sat_with_constraint(&c)?;
                let false_possible = self.state.constraints.is_sat_with_constraint(&c.not())?;

                let destination: DExpr = match (true_possible, false_possible) {
                    (true, true) => {
                        self.fork(c.not());
                        self.state.constraints.assert(&c);
                        Ok(self.get_operand_value(destination, &local))
                    }
                    (true, false) => Ok(self.get_operand_value(destination, &local)),
                    (false, true) => Ok(self.state.get_register("PC".to_owned()).unwrap()), // safe to asume PC exist
                    (false, false) => Err(SolverError::Unsat),
                }?;

                self.state.set_register("PC".to_owned(), destination);
            }
            crate::general_assembly::instruction::Operation::SetNFlag(operand) => {
                let value = self.get_operand_value(operand, &local);
                let shift = self
                    .state
                    .ctx
                    .from_u64((self.project.get_word_size() - 1) as u64, 32);
                let result = value.srl(&shift).resize_unsigned(1);
                self.state.set_flag("N".to_owned(), result);
            }
            crate::general_assembly::instruction::Operation::SetZFlag(operand) => {
                let value = self.get_operand_value(operand, &local);
                let result = value._eq(&self.state.ctx.zero(self.project.get_word_size()));
                self.state.set_flag("Z".to_owned(), result);
            }
            crate::general_assembly::instruction::Operation::SetCFlag {
                operand1,
                operand2,
                sub,
            } => {
                let op1 = self.get_operand_value(operand1, &local);
                let op2 = self.get_operand_value(operand2, &local);

                let result = if *sub {
                    op1.usubo(&op2)
                } else {
                    op1.uaddo(&op2)
                };

                self.state.set_flag("C".to_owned(), result);
            }
            crate::general_assembly::instruction::Operation::SetVFlag {
                operand1,
                operand2,
                sub,
            } => {
                let op1 = self.get_operand_value(operand1, &local);
                let op2 = self.get_operand_value(operand2, &local);

                let result = if *sub {
                    op1.ssubo(&op2)
                } else {
                    op1.saddo(&op2)
                };

                self.state.set_flag("C".to_owned(), result);
            }
            crate::general_assembly::instruction::Operation::ForEach {
                operands,
                operations,
            } => {
                todo!()
            }
            crate::general_assembly::instruction::Operation::ZeroExtend {
                destination,
                operand,
                bits,
            } => {
                let op = self.get_operand_value(operand, &local);
                let valid_bits = op.resize_unsigned(*bits);
                let result = valid_bits.zero_ext(self.project.get_word_size());
                self.set_operand_value(destination, result, local);
            }
        }
        Ok(())
    }
}
