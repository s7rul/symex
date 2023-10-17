//! Translator for the armv6-m instruction set

use armv6_m_instruction_parser::{
    instructons::{Instruction, Operation},
    registers::Register,
};

use crate::general_assembly::{instruction::Operand, translator::Translator, DataWord};

type GAInstruction = crate::general_assembly::instruction::Instruction;
type GAOperation = crate::general_assembly::instruction::Operation;

impl Translator for Instruction {
    fn translate(&self) -> GAInstruction {
        match &self.operation {
            Operation::ADCReg { m, n, d } => todo!(),
            Operation::ADDImm { imm, n, d } => todo!(),
            Operation::ADDReg { m, n, d } => todo!(),
            Operation::ADDImmSP { d, imm } => GAInstruction {
                instruction_size: 16,
                operations: vec![GAOperation::Add {
                    destination: arm_register_to_ga_operand(d),
                    operand1: arm_register_to_ga_operand(&Register::SP),
                    operand2: Operand::Immidiate(DataWord::Word32(*imm)),
                }],
            },
            Operation::ADDRegSP { d, m } => todo!(),
            Operation::ADR { d, imm } => todo!(),
            Operation::ANDReg { m, dn } => todo!(),
            Operation::ASRImm { imm, m, d } => todo!(),
            Operation::ASRReg { m, dn } => todo!(),
            Operation::B { cond, imm } => todo!(),
            Operation::BICReg { m, dn } => todo!(),
            Operation::BKPT { imm } => todo!(),
            Operation::BL { imm } => GAInstruction {
                instruction_size: 32,
                operations: vec![
                    GAOperation::Move {
                        destination: Operand::Local("PC".to_owned()),
                        source: Operand::Register("PC".to_owned()),
                    },
                    GAOperation::Move {
                        destination: Operand::Register("LR".to_owned()),
                        source: Operand::Local("PC".to_owned()),
                    },
                    GAOperation::Add {
                        destination: Operand::Local("newPC".to_owned()),
                        operand1: Operand::Local("PC".to_owned()),
                        operand2: Operand::Immidiate(DataWord::Word32(*imm)),
                    },
                    GAOperation::Jump { destination: Operand::Local("newPC".to_owned()) }
                ],
            },
            Operation::BLXReg { m } => todo!(),
            Operation::BX { m } => todo!(),
            Operation::CMNReg { m, n } => todo!(),
            Operation::CMPImm { n, imm } => todo!(),
            Operation::CMPReg { m, n } => todo!(),
            Operation::CPS { im } => todo!(),
            Operation::CPY => todo!(),
            Operation::DMB { option } => todo!(),
            Operation::DSB { option } => todo!(),
            Operation::EORReg { m, dn } => todo!(),
            Operation::ISB { option } => todo!(),
            Operation::LDM { n, reg_list } => todo!(),
            Operation::LDRImm { imm, n, t } => todo!(),
            Operation::LDRLiteral { t, imm } => todo!(),
            Operation::LDRReg { m, n, t } => todo!(),
            Operation::LDRBImm { imm, n, t } => todo!(),
            Operation::LDRBReg { m, n, t } => todo!(),
            Operation::LDRHImm { imm, n, t } => todo!(),
            Operation::LDRHReg { m, n, t } => todo!(),
            Operation::LDRSBReg { m, n, t } => todo!(),
            Operation::LDRSH { m, n, t } => todo!(),
            Operation::LSLImm { imm, m, d } => todo!(),
            Operation::LSLReg { m, dn } => todo!(),
            Operation::LSRImm { imm, m, d } => todo!(),
            Operation::LSRReg { m, dn } => todo!(),
            Operation::MOVImm { d, imm } => todo!(),
            Operation::MOVReg { m, d, set_flags } => {
                let destination = arm_register_to_ga_operand(d);
                let source = arm_register_to_ga_operand(m);
                if !set_flags {
                    GAInstruction {
                        instruction_size: 16,
                        operations: vec![
                            GAOperation::Move {
                                destination,
                                source: source.clone(),
                            },
                            GAOperation::SetNFlag(source.clone()),
                            GAOperation::SetZFlag(source),
                        ],
                    }
                } else {
                    todo!()
                }
            }
            Operation::MRS { d, sysm } => todo!(),
            Operation::MSRReg { n, sysm } => todo!(),
            Operation::MUL { n, dm } => todo!(),
            Operation::MVNReg { m, d } => todo!(),
            Operation::NOP => todo!(),
            Operation::ORRReg { m, dn } => todo!(),
            Operation::POP { reg_list } => todo!(),
            Operation::PUSH { reg_list } => GAInstruction {
                instruction_size: 16,
                operations: vec![GAOperation::ForEach {
                    operands: arm_reg_list_to_ga_op_list(reg_list),
                    operations: vec![], // TODO add instuctions here
                }],
            },
            Operation::REV { m, d } => todo!(),
            Operation::REV16 { m, d } => todo!(),
            Operation::REVSH { m, d } => todo!(),
            Operation::RORReg { m, dn } => todo!(),
            Operation::RSBImm { n, d } => todo!(),
            Operation::SBCReg { m, dn } => todo!(),
            Operation::SEV => todo!(),
            Operation::STM { n, reg_list } => todo!(),
            Operation::STRImm { imm, n, t } => todo!(),
            Operation::STRReg { m, n, t } => todo!(),
            Operation::STRBImm { imm, n, t } => todo!(),
            Operation::STRBReg { m, n, t } => todo!(),
            Operation::STRHImm { imm, n, t } => todo!(),
            Operation::STRHReg { m, n, t } => todo!(),
            Operation::SUBImm { imm, n, d } => todo!(),
            Operation::SUBReg { m, n, d } => todo!(),
            Operation::SUBImmSP { imm } => todo!(),
            Operation::SVC { imm } => todo!(),
            Operation::SXTB { m, d } => todo!(),
            Operation::SXTH { m, d } => todo!(),
            Operation::TSTReg { m, n } => todo!(),
            Operation::UDFT1 { imm } => todo!(),
            Operation::UDFT2 { imm } => todo!(),
            Operation::UXTB { m, d } => todo!(),
            Operation::UXTH { m, d } => todo!(),
            Operation::WFE => todo!(),
            Operation::WFI => todo!(),
            Operation::YIELD => todo!(),
        }
    }
}

fn arm_register_to_ga_operand(reg: &Register) -> Operand {
    Operand::Register(match reg {
        Register::R0 => "R0".to_owned(),
        Register::R1 => "R1".to_owned(),
        Register::R2 => "R2".to_owned(),
        Register::R3 => "R3".to_owned(),
        Register::R4 => "R4".to_owned(),
        Register::R5 => "R5".to_owned(),
        Register::R6 => "R6".to_owned(),
        Register::R7 => "R7".to_owned(),
        Register::R8 => "R8".to_owned(),
        Register::R9 => "R9".to_owned(),
        Register::R10 => "R10".to_owned(),
        Register::R11 => "R11".to_owned(),
        Register::R12 => "R12".to_owned(),
        Register::SP => "SR".to_owned(),
        Register::LR => "LR".to_owned(),
        Register::PC => "PC".to_owned(),
    })
}

fn arm_reg_list_to_ga_op_list(reg_list: &Vec<Register>) -> Vec<Operand> {
    let mut ret = vec![];
    for reg in reg_list {
        ret.push(arm_register_to_ga_operand(reg));
    }
    ret
}
