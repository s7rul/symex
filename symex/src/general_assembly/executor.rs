//! General assembly executor

use std::collections::HashMap;

use tracing::{debug, trace};

use crate::{
    general_assembly::path_selection::Path,
    smt::{DExpr, SolverError},
    util::{ExpressionType, Variable},
};

use super::{
    instruction::{Instruction, Operand, Operation},
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

struct AddWithCarryResult {
    carry_out: DExpr,
    overflow: DExpr,
    result: DExpr,
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

    fn get_memmory(&mut self, address: &DExpr, bits: u32) -> Result<DExpr> {
        trace!("Getting memmory addr: {:?}", address);
        match address.get_constant() {
            Some(const_addr) => {
                if self.project.address_in_range(const_addr) {
                    if bits == self.project.get_word_size() {
                        Ok(self.get_dexpr_from_dataword(self.project.get_word(const_addr)?))
                    } else {
                        todo!()
                    }
                } else {
                    let data = self.state.memory.read(address, bits)?;
                    Ok(data)
                }
            }
            None => {
                let data = self.state.memory.read(address, bits)?;
                Ok(data)
            }
        }
    }

    fn set_memmory(&mut self, data: DExpr, address: &DExpr, bits: u32) -> Result<()> {
        trace!("Setting memmory addr: {:?}", address);
        match address.get_constant() {
            Some(const_addr) => {
                if self.project.address_in_range(const_addr) {
                    Err(super::GAError::WritingToStaticMemoryProhibited)
                } else {
                    self.state
                        .memory
                        .write(address, data.resize_unsigned(bits))?;
                    Ok(())
                }
            }
            None => {
                self.state
                    .memory
                    .write(address, data.resize_unsigned(bits))?;
                Ok(())
            }
        }
    }

    fn get_operand_value(
        &mut self,
        operand: &Operand,
        local: &HashMap<String, DExpr>,
    ) -> Result<DExpr> {
        match operand {
            Operand::Register(name) => match self.state.get_register(name.to_owned()) {
                Some(v) => Ok(v),
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
                    Ok(value)
                }
            },
            Operand::Immidiate(v) => Ok(self.get_dexpr_from_dataword(v.to_owned())),
            Operand::Address(address, width) => {
                let address = &self.get_dexpr_from_dataword(*address);
                self.get_memmory(address, *width)
            }
            Operand::AddressWithOffset {
                address: _,
                offset_reg: _,
                width: _,
            } => todo!(),
            Operand::Local(k) => Ok((local.get(k).unwrap()).to_owned()),
            Operand::AddressInLocal(local_name, width) => {
                let address =
                    self.get_operand_value(&Operand::Local(local_name.to_owned()), local)?;
                self.get_memmory(&address, *width)
            }
        }
    }

    fn set_operand_value(
        &mut self,
        operand: &Operand,
        value: DExpr,
        local: &mut HashMap<String, DExpr>,
    ) -> Result<()> {
        match operand {
            Operand::Register(v) => {
                trace!("Setting register {} to {:?}", v, value);
                self.state.set_register(v.to_owned(), value)
            }
            Operand::Immidiate(_) => panic!(), // not prohibited change to error later
            Operand::AddressInLocal(local_name, width) => {
                let address =
                    self.get_operand_value(&Operand::Local(local_name.to_owned()), local)?;
                self.set_memmory(value, &address, *width)?;
            }
            Operand::Address(address, width) => {
                let address = self.get_dexpr_from_dataword(*address);
                self.set_memmory(value, &address, *width)?;
            }
            Operand::AddressWithOffset {
                address: _,
                offset_reg: _,
                width: _,
            } => todo!(),
            Operand::Local(k) => {
                local.insert(k.to_owned(), value);
            }
        }
        Ok(())
    }

    fn add_with_carry(&mut self, op1: &DExpr, op2: &DExpr, carry_in: &DExpr) -> AddWithCarryResult {
        let carry_in = carry_in.resize_unsigned(1);
        let op2 = op2.add(&carry_in.zero_ext(self.project.get_word_size()));
        let result = op1.add(&op2);
        let carry = op1.uaddo(&op2);
        let overflow = carry_in.xor(&carry);
        AddWithCarryResult {
            carry_out: carry,
            overflow,
            result,
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
            self.executer_operation(operation, &mut local)?;
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
            Operation::Nop => (), // nop so do nothig
            Operation::Move {
                destination,
                source,
            } => {
                let value = self.get_operand_value(source, local)?;
                self.set_operand_value(destination, value, local)?;
            }
            Operation::Add {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, &local)?;
                let op2 = self.get_operand_value(operand2, &local)?;
                let result = op1.add(&op2);
                self.set_operand_value(destination, result, local)?;
            }
            Operation::Sub {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, &local)?;
                let op2 = self.get_operand_value(operand2, &local)?;
                let result = op1.sub(&op2);
                self.set_operand_value(destination, result, local)?;
            }
            Operation::Mul {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, &local)?;
                let op2 = self.get_operand_value(operand2, &local)?;
                let result = op1.mul(&op2);
                self.set_operand_value(destination, result, local)?;
            }
            Operation::And {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, &local)?;
                let op2 = self.get_operand_value(operand2, &local)?;
                let result = op1.and(&op2);
                self.set_operand_value(destination, result, local)?;
            }
            Operation::Or {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, &local)?;
                let op2 = self.get_operand_value(operand2, &local)?;
                let result = op1.or(&op2);
                self.set_operand_value(destination, result, local)?;
            }
            Operation::Xor {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, &local)?;
                let op2 = self.get_operand_value(operand2, &local)?;
                let result = op1.xor(&op2);
                self.set_operand_value(destination, result, local)?;
            }
            Operation::Not {
                destination,
                operand,
            } => {
                let op = self.get_operand_value(operand, local)?;

                let result = op.not();
                self.set_operand_value(destination, result, local)?;
            }
            Operation::Sl {
                destination,
                operand,
                shift,
            } => {
                let value = self.get_operand_value(operand, &local)?;
                let shift_amount = self.get_operand_value(shift, &local)?;
                let result = value.sll(&shift_amount);
                self.set_operand_value(destination, result, local)?;
            }
            Operation::Srl {
                destination,
                operand,
                shift,
            } => {
                let value = self.get_operand_value(operand, &local)?;
                let shift_amount = self.get_operand_value(shift, &local)?;
                let result = value.srl(&shift_amount);
                self.set_operand_value(destination, result, local)?;
            }
            Operation::Sra {
                destination,
                operand,
                shift,
            } => {
                let value = self.get_operand_value(operand, &local)?;
                let shift_amount = self.get_operand_value(shift, &local)?;
                let result = value.sra(&shift_amount);
                self.set_operand_value(destination, result, local)?;
            }
            Operation::Sror {
                destination,
                operand,
                shift,
            } => {
                let word_size = self.state.ctx.from_u64(
                    self.project.get_word_size() as u64,
                    self.project.get_word_size(),
                );
                let value = self.get_operand_value(operand, &local)?;
                let shift = self.get_operand_value(shift, &local)?;
                let result = value.srl(&shift).or(&value.srl(&word_size).sub(&shift));
                self.set_operand_value(destination, result, local)?;
            }
            Operation::ConditionalJump {
                destination,
                condition,
            } => {
                let c = self.state.get_expr(condition)?.simplify();
                trace!("conditional expr: {:?}", c);

                // if constant just jump
                if let Some(constant_c) = c.get_constant_bool() {
                    if constant_c {
                        let destination = self.get_operand_value(destination, &local)?;
                        self.state.set_register("PC".to_owned(), destination);
                    }
                    return Ok(());
                }

                let true_possible = self.state.constraints.is_sat_with_constraint(&c)?;
                let false_possible = self.state.constraints.is_sat_with_constraint(&c.not())?;
                trace!(
                    "true possible: {} false possible: {}",
                    true_possible,
                    false_possible
                );

                let destination: DExpr = match (true_possible, false_possible) {
                    (true, true) => {
                        self.fork(c.not())?;
                        self.state.constraints.assert(&c);
                        Ok(self.get_operand_value(destination, &local)?)
                    }
                    (true, false) => Ok(self.get_operand_value(destination, &local)?),
                    (false, true) => Ok(self.state.get_register("PC".to_owned()).unwrap()), // safe to asume PC exist
                    (false, false) => Err(SolverError::Unsat),
                }?;

                self.state.set_register("PC".to_owned(), destination);
            }
            Operation::SetNFlag(operand) => {
                let value = self.get_operand_value(operand, &local)?;
                let shift = self
                    .state
                    .ctx
                    .from_u64((self.project.get_word_size() - 1) as u64, 32);
                let result = value.srl(&shift).resize_unsigned(1);
                self.state.set_flag("N".to_owned(), result);
            }
            Operation::SetZFlag(operand) => {
                let value = self.get_operand_value(operand, &local)?;
                let result = value._eq(&self.state.ctx.zero(self.project.get_word_size()));
                self.state.set_flag("Z".to_owned(), result);
            }
            Operation::SetCFlag {
                operand1,
                operand2,
                sub,
                carry,
            } => {
                let op1 = self.get_operand_value(operand1, &local)?;
                let op2 = self.get_operand_value(operand2, &local)?;
                let one = self.state.ctx.from_u64(1, self.project.get_word_size());
                // not correct todo fix

                let result = match (sub, carry) {
                    (true, true) => {
                        let carry_in = self.state.get_flag("C".to_owned()).unwrap();
                        let op2 = op2.not().add(&one);
                        self.add_with_carry(&op1, &op2, &carry_in).carry_out
                    }
                    (true, false) => self.add_with_carry(&op1, &op2.not(), &one).carry_out,
                    (false, true) => {
                        let carry_in = self.state.get_flag("C".to_owned()).unwrap();
                        self.add_with_carry(&op1, &op2, &carry_in).carry_out
                    }
                    (false, false) => op1.uaddo(&op2),
                };

                self.state.set_flag("C".to_owned(), result);
            }
            Operation::SetVFlag {
                operand1,
                operand2,
                sub,
                carry,
            } => {
                let op1 = self.get_operand_value(operand1, &local)?;
                let op2 = self.get_operand_value(operand2, &local)?;
                let one = self.state.ctx.from_u64(1, self.project.get_word_size());

                let result = match (sub, carry) {
                    (true, true) => {
                        let carry_in = self.state.get_flag("C".to_owned()).unwrap();
                        let op2 = op2.not().add(&one);
                        self.add_with_carry(&op1, &op2, &carry_in).overflow
                    }
                    (true, false) => self.add_with_carry(&op1, &op2.not(), &one).overflow,
                    (false, true) => {
                        let carry_in = self.state.get_flag("C".to_owned()).unwrap();
                        self.add_with_carry(&op1, &op2, &carry_in).overflow
                    }
                    (false, false) => op1.saddo(&op2),
                };

                self.state.set_flag("V".to_owned(), result);
            }
            Operation::ForEach {
                operands: _,
                operations: _,
            } => {
                todo!()
            }
            Operation::ZeroExtend {
                destination,
                operand,
                bits,
            } => {
                let op = self.get_operand_value(operand, &local)?;
                let valid_bits = op.resize_unsigned(*bits);
                let result = valid_bits.zero_ext(self.project.get_word_size());
                self.set_operand_value(destination, result, local)?;
            }
            Operation::SignExtend {
                destination,
                operand,
                bits,
            } => {
                let op = self.get_operand_value(operand, &local)?;
                let valid_bits = op.resize_unsigned(*bits);
                let result = valid_bits.sign_ext(self.project.get_word_size());
                self.set_operand_value(destination, result, local)?;
            }
            Operation::Adc {
                destination,
                operand1,
                operand2,
            } => {
                let op1 = self.get_operand_value(operand1, local)?;
                let op2 = self.get_operand_value(operand2, local)?;
                let carry = self
                    .state
                    .get_flag("C".to_owned())
                    .unwrap()
                    .zero_ext(self.project.get_word_size());
                let result = self.add_with_carry(&op1, &op2, &carry).result;
                self.set_operand_value(destination, result, local)?;
            }
            // These need to be tested are way to complex to be trusted
            Operation::SetCFlagShiftLeft { operand, shift } => {
                let op = self
                    .get_operand_value(operand, local)?
                    .zero_ext(1 + self.project.get_word_size());
                let shift = self
                    .get_operand_value(shift, local)?
                    .zero_ext(1 + self.project.get_word_size());
                let result = op.sll(&shift);
                let carry = result
                    .srl(&self.state.ctx.from_u64(
                        self.project.get_word_size() as u64,
                        self.project.get_word_size() + 1,
                    ))
                    .resize_unsigned(1);
                self.state.set_flag("C".to_owned(), carry);
            }
            Operation::SetCFlagSrl { operand, shift } => {
                let op = self
                    .get_operand_value(operand, local)?
                    .zero_ext(1 + self.project.get_word_size())
                    .sll(&self.state.ctx.from_u64(1, 1 + self.project.get_word_size()));
                let shift = self
                    .get_operand_value(shift, local)?
                    .zero_ext(1 + self.project.get_word_size());
                let result = op.srl(&shift);
                let carry = result.resize_unsigned(1);
                self.state.set_flag("C".to_owned(), carry);
            }
            Operation::SetCFlagSra { operand, shift } => {
                let op = self
                    .get_operand_value(operand, local)?
                    .zero_ext(1 + self.project.get_word_size())
                    .sll(&self.state.ctx.from_u64(1, 1 + self.project.get_word_size()));
                let shift = self
                    .get_operand_value(shift, local)?
                    .zero_ext(1 + self.project.get_word_size());
                let result = op.sra(&shift);
                let carry = result.resize_unsigned(1);
                self.state.set_flag("C".to_owned(), carry);
            }
            Operation::SetCFlagRor(result) => {
                // this is wrong fix later
                todo!();
                let result = self.get_operand_value(result, local)?;
                let word_size_minus_one = self.state.ctx.from_u64(
                    self.project.get_word_size() as u64 - 1,
                    self.project.get_word_size(),
                );
                // result = srl(op, shift) OR sll(op, word_size - shift)
                let c = result.srl(&word_size_minus_one).resize_unsigned(1);
                self.state.set_flag("C".to_owned(), c);
            }
        }
        Ok(())
    }
}
