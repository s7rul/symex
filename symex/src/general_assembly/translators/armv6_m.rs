//! Translator for the armv6-m instruction set

use armv6_m_instruction_parser::{
    instructons::{Instruction, Operation},
    registers::{Register, SpecialRegister},
};

use crate::general_assembly::{
    instruction::{Condition, Operand},
    translator::Translator,
    DataWord,
};

type GAInstruction = crate::general_assembly::instruction::Instruction;
type GAOperation = crate::general_assembly::instruction::Operation;
type ArmCodition = armv6_m_instruction_parser::conditions::Condition;

impl Translator for Instruction {
    fn translate(&self) -> GAInstruction {
        match &self.operation {
            Operation::ADCReg { m, n, d } => {
                let dest = arm_register_to_ga_operand(d);
                let mreg = arm_register_to_ga_operand(m);
                let nreg = arm_register_to_ga_operand(n);

                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Adc {
                            destination: dest.clone(),
                            operand1: nreg.clone(),
                            operand2: mreg.clone(),
                        },
                        GAOperation::SetNFlag(dest.clone()),
                        GAOperation::SetZFlag(dest),
                        GAOperation::SetCFlag {
                            operand1: nreg.clone(),
                            operand2: mreg.clone(),
                            sub: false,
                            carry: true,
                        },
                        GAOperation::SetVFlag {
                            operand1: nreg,
                            operand2: mreg,
                            sub: false,
                            carry: true,
                        },
                    ],
                }
            }
            Operation::ADDImm { imm, n, d } => {
                let dest = arm_register_to_ga_operand(d);
                let imm = Operand::Immidiate(DataWord::Word32(*imm));
                let nreg = arm_register_to_ga_operand(n);
                let op_local = Operand::Local("op".to_owned());

                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Move {
                            destination: op_local.clone(),
                            source: nreg.clone(),
                        },
                        GAOperation::Add {
                            destination: dest.clone(),
                            operand1: nreg,
                            operand2: imm.clone(),
                        },
                        GAOperation::SetNFlag(dest.clone()),
                        GAOperation::SetZFlag(dest),
                        GAOperation::SetCFlag {
                            operand1: op_local.clone(),
                            operand2: imm.clone(),
                            sub: false,
                            carry: false,
                        },
                        GAOperation::SetVFlag {
                            operand1: op_local,
                            operand2: imm,
                            sub: false,
                            carry: false,
                        },
                    ],
                }
            }
            Operation::ADDReg { m, n, d } => {
                let dest = arm_register_to_ga_operand(d);
                let mreg = arm_register_to_ga_operand(m);
                let nreg = arm_register_to_ga_operand(n);
                let m_local = Operand::Local("m".to_owned());
                let n_local = Operand::Local("n".to_owned());

                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Move {
                            destination: m_local.clone(),
                            source: mreg.clone(),
                        },
                        GAOperation::Move {
                            destination: n_local.clone(),
                            source: nreg.clone(),
                        },
                        GAOperation::Add {
                            destination: dest.clone(),
                            operand1: nreg,
                            operand2: mreg,
                        },
                        GAOperation::SetNFlag(dest.clone()),
                        GAOperation::SetZFlag(dest),
                        GAOperation::SetCFlag {
                            operand1: n_local.clone(),
                            operand2: m_local.clone(),
                            sub: false,
                            carry: false,
                        },
                        GAOperation::SetVFlag {
                            operand1: n_local,
                            operand2: m_local,
                            sub: false,
                            carry: false,
                        },
                    ],
                }
            }
            Operation::ADDImmSP { d, imm } => GAInstruction {
                instruction_size: 16,
                operations: vec![GAOperation::Add {
                    destination: arm_register_to_ga_operand(d),
                    operand1: arm_register_to_ga_operand(&Register::SP),
                    operand2: Operand::Immidiate(DataWord::Word32(*imm)),
                }],
            },
            Operation::ADDRegSP { d, m } => GAInstruction {
                instruction_size: 16,
                operations: vec![GAOperation::Add {
                    destination: arm_register_to_ga_operand(d),
                    operand1: arm_register_to_ga_operand(&Register::SP),
                    operand2: arm_register_to_ga_operand(m),
                }],
            },
            Operation::ADR { d, imm } => {
                let imm = imm + 2;
                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Move {
                            destination: Operand::Local("addr".to_owned()),
                            source: arm_register_to_ga_operand(&Register::PC),
                        },
                        GAOperation::And {
                            destination: Operand::Local("addr".to_owned()),
                            operand1: Operand::Local("addr".to_owned()),
                            operand2: Operand::Immidiate(DataWord::Word32(!0b11)),
                        },
                        GAOperation::Add {
                            destination: arm_register_to_ga_operand(d),
                            operand1: Operand::Local("addr".to_owned()),
                            operand2: Operand::Immidiate(DataWord::Word32(imm)),
                        },
                    ],
                }
            }
            Operation::ANDReg { m, dn } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::And {
                        destination: arm_register_to_ga_operand(dn),
                        operand1: arm_register_to_ga_operand(dn),
                        operand2: arm_register_to_ga_operand(m),
                    },
                    GAOperation::SetNFlag(arm_register_to_ga_operand(dn)),
                    GAOperation::SetZFlag(arm_register_to_ga_operand(dn)),
                ],
            },
            Operation::ASRImm { imm, m, d } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Move {
                        destination: Operand::Local("m".to_owned()),
                        source: arm_register_to_ga_operand(m),
                    },
                    GAOperation::Sra {
                        destination: arm_register_to_ga_operand(d),
                        operand: arm_register_to_ga_operand(m),
                        shift: Operand::Immidiate(DataWord::Word32(*imm)),
                    },
                    GAOperation::SetNFlag(arm_register_to_ga_operand(d)),
                    GAOperation::SetZFlag(arm_register_to_ga_operand(d)),
                    GAOperation::SetCFlagSra {
                        operand: Operand::Local("m".to_owned()),
                        shift: Operand::Immidiate(DataWord::Word32(*imm)),
                    },
                ],
            },
            Operation::ASRReg { m, dn } => {
                let dnreg = arm_register_to_ga_operand(dn);
                let mreg = arm_register_to_ga_operand(m);
                let n_local = Operand::Local("n".to_owned());
                let shift_local = Operand::Local("shift".to_owned());

                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Move {
                            destination: n_local.clone(),
                            source: dnreg.clone(),
                        },
                        GAOperation::And {
                            destination: shift_local.clone(),
                            operand1: mreg,
                            operand2: Operand::Immidiate(DataWord::Word32(0xff)),
                        },
                        GAOperation::Sra {
                            destination: dnreg.clone(),
                            operand: dnreg.clone(),
                            shift: shift_local.clone(),
                        },
                        GAOperation::SetNFlag(dnreg.clone()),
                        GAOperation::SetZFlag(dnreg),
                        GAOperation::SetCFlagSra {
                            operand: n_local,
                            shift: shift_local,
                        },
                    ],
                }
            }
            Operation::B { cond, imm } => {
                let imm = imm + 2; // Beacause arm always adds as a 32 bit instruction.
                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Add {
                            destination: Operand::Local("new_pc".to_owned()),
                            operand1: Operand::Register("PC".to_owned()),
                            operand2: Operand::Immidiate(DataWord::Word32(imm)),
                        },
                        GAOperation::ConditionalJump {
                            destination: Operand::Local("new_pc".to_owned()),
                            condition: arm_cond_to_ga_cond(cond),
                        },
                    ],
                }
            }
            Operation::BICReg { m, dn } => {
                let reg_m = arm_register_to_ga_operand(m);
                let reg_dn = arm_register_to_ga_operand(dn);

                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Not {
                            destination: Operand::Local("mask".to_owned()),
                            operand: reg_m,
                        },
                        GAOperation::And {
                            destination: reg_dn.clone(),
                            operand1: reg_dn,
                            operand2: Operand::Local("mask".to_owned()),
                        },
                    ],
                }
            }
            Operation::BKPT { imm: _ } => GAInstruction {
                instruction_size: 16,
                operations: vec![],
            },
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
                    GAOperation::Move {
                        destination: Operand::Register("PC".to_owned()),
                        source: Operand::Local("newPC".to_owned()),
                    },
                ],
            },
            Operation::BLXReg { m } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Move {
                        destination: arm_register_to_ga_operand(&Register::LR),
                        source: arm_register_to_ga_operand(&Register::PC),
                    },
                    GAOperation::Move {
                        destination: arm_register_to_ga_operand(&Register::PC),
                        source: arm_register_to_ga_operand(m),
                    },
                ],
            },
            Operation::BX { m } => {
                let reg = arm_register_to_ga_operand(m);
                let destination = arm_register_to_ga_operand(&Register::PC);
                GAInstruction {
                    instruction_size: 16,
                    operations: vec![GAOperation::Move {
                        destination,
                        source: reg,
                    }],
                }
            }
            Operation::CMNReg { m, n } => {
                let m = arm_register_to_ga_operand(m);
                let n = arm_register_to_ga_operand(n);
                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Add {
                            destination: Operand::Local("result".to_owned()),
                            operand1: n.clone(),
                            operand2: m.clone(),
                        },
                        GAOperation::SetNFlag(Operand::Local("result".to_owned())),
                        GAOperation::SetZFlag(Operand::Local("result".to_owned())),
                        GAOperation::SetCFlag {
                            operand1: n.clone(),
                            operand2: m.clone(),
                            sub: false,
                            carry: false,
                        },
                        GAOperation::SetVFlag {
                            operand1: n,
                            operand2: m,
                            sub: false,
                            carry: false,
                        },
                    ],
                }
            }
            Operation::CMPImm { n, imm } => {
                let op_n = arm_register_to_ga_operand(n);
                let op_imm = Operand::Immidiate(DataWord::Word32(*imm));
                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Sub {
                            destination: Operand::Local("result".to_owned()),
                            operand1: op_n.clone(),
                            operand2: op_imm.clone(),
                        },
                        GAOperation::SetNFlag(Operand::Local("result".to_owned())),
                        GAOperation::SetZFlag(Operand::Local("result".to_owned())),
                        GAOperation::SetCFlag {
                            operand1: op_n.clone(),
                            operand2: op_imm.clone(),
                            sub: true,
                            carry: false,
                        },
                        GAOperation::SetVFlag {
                            operand1: op_n.clone(),
                            operand2: op_imm.clone(),
                            sub: true,
                            carry: false,
                        },
                    ],
                }
            }
            Operation::CMPReg { m, n } => {
                let op_n = arm_register_to_ga_operand(n);
                let op_m = arm_register_to_ga_operand(m);
                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Sub {
                            destination: Operand::Local("result".to_owned()),
                            operand1: op_n.clone(),
                            operand2: op_m.clone(),
                        },
                        GAOperation::SetNFlag(Operand::Local("result".to_owned())),
                        GAOperation::SetZFlag(Operand::Local("result".to_owned())),
                        GAOperation::SetCFlag {
                            operand1: op_n.clone(),
                            operand2: op_m.clone(),
                            sub: true,
                            carry: false,
                        },
                        GAOperation::SetVFlag {
                            operand1: op_n.clone(),
                            operand2: op_m.clone(),
                            sub: true,
                            carry: false,
                        },
                    ],
                }
            }
            Operation::CPS { im: _ } => {
                // change processor state do nothig for now but should probably be modeled
                // in armv6-m it is only used to enable disable interupts
                GAInstruction {
                    instruction_size: 16,
                    operations: vec![],
                }
            }
            Operation::CPY => {
                // this is not a real instruction is equvelatn to mov
                unreachable!()
            }
            Operation::DMB { option: _ } => {
                // data barier do nothig as data barier is not modeled yet
                GAInstruction {
                    instruction_size: 32,
                    operations: vec![],
                }
            }
            Operation::DSB { option: _ } => {
                // data barier do nothig as data barier is not modeled yet
                GAInstruction {
                    instruction_size: 32,
                    operations: vec![],
                }
            }
            Operation::EORReg { m, dn } => {
                let dn = arm_register_to_ga_operand(dn);
                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Xor {
                            destination: dn.clone(),
                            operand1: dn.clone(),
                            operand2: arm_register_to_ga_operand(m),
                        },
                        GAOperation::SetNFlag(dn.clone()),
                        GAOperation::SetZFlag(dn),
                    ],
                }
            }
            Operation::ISB { option: _ } => {
                // flushes pipeline do nothig as pipeline is not modeled
                GAInstruction {
                    instruction_size: 32,
                    operations: vec![],
                }
            }
            Operation::LDM { n, reg_list } => {
                let mut operations: Vec<GAOperation> = vec![GAOperation::Move {
                    destination: Operand::Local("Address".to_owned()),
                    source: arm_register_to_ga_operand(n),
                }];
                for reg in reg_list {
                    // write register to memory
                    operations.push(GAOperation::Move {
                        destination: Operand::AddressInLocal("Address".to_owned(), 32),
                        source: arm_register_to_ga_operand(reg),
                    });
                    // update address
                    operations.push(GAOperation::Add {
                        destination: Operand::Local("Address".to_owned()),
                        operand1: Operand::Local("Address".to_owned()),
                        operand2: Operand::Immidiate(DataWord::Word32(4)),
                    })
                }
                if reg_list.contains(n) {
                    // addre reg not in reg list writeback
                    operations.push(GAOperation::Move {
                        destination: arm_register_to_ga_operand(n),
                        source: Operand::Local("Address".to_owned()),
                    });
                }

                GAInstruction {
                    instruction_size: 16,
                    operations,
                }
            }
            Operation::LDRImm { imm, n, t } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Add {
                        destination: Operand::Local("addr".to_owned()),
                        operand1: arm_register_to_ga_operand(n),
                        operand2: Operand::Immidiate(DataWord::Word32(*imm)),
                    },
                    GAOperation::Move {
                        destination: arm_register_to_ga_operand(t),
                        source: Operand::AddressInLocal("addr".to_owned(), 32),
                    },
                ],
            },
            Operation::LDRLiteral { t, imm } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::And {
                        destination: Operand::Local("addr".to_owned()),
                        operand1: arm_register_to_ga_operand(&Register::PC),
                        operand2: Operand::Immidiate(DataWord::Word32(!0b11)),
                    },
                    GAOperation::Add {
                        destination: Operand::Local("addr".to_owned()),
                        operand1: Operand::Local("addr".to_owned()),
                        operand2: Operand::Immidiate(DataWord::Word32(*imm)),
                    },
                    GAOperation::Move {
                        destination: arm_register_to_ga_operand(t),
                        source: Operand::AddressInLocal("addr".to_owned(), 32),
                    },
                ],
            },
            Operation::LDRReg { m, n, t } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Add {
                        destination: Operand::Local("addr".to_owned()),
                        operand1: arm_register_to_ga_operand(n),
                        operand2: arm_register_to_ga_operand(m),
                    },
                    GAOperation::Move {
                        destination: arm_register_to_ga_operand(t),
                        source: Operand::AddressInLocal("addr".to_owned(), 32),
                    },
                ],
            },
            Operation::LDRBImm { imm, n, t } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Add {
                        destination: Operand::Local("addr".to_owned()),
                        operand1: arm_register_to_ga_operand(n),
                        operand2: Operand::Immidiate(DataWord::Word32(*imm)),
                    },
                    GAOperation::Move {
                        destination: arm_register_to_ga_operand(t),
                        source: Operand::AddressInLocal("addr".to_owned(), 8),
                    },
                    GAOperation::ZeroExtend {
                        destination: arm_register_to_ga_operand(t),
                        operand: arm_register_to_ga_operand(t),
                        bits: 8,
                    },
                ],
            },
            Operation::LDRBReg { m, n, t } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Add {
                        destination: Operand::Local("addr".to_owned()),
                        operand1: arm_register_to_ga_operand(n),
                        operand2: arm_register_to_ga_operand(m),
                    },
                    GAOperation::Move {
                        destination: arm_register_to_ga_operand(t),
                        source: Operand::AddressInLocal("addr".to_owned(), 8),
                    },
                    GAOperation::ZeroExtend {
                        destination: arm_register_to_ga_operand(t),
                        operand: arm_register_to_ga_operand(t),
                        bits: 8,
                    },
                ],
            },
            Operation::LDRHImm { imm, n, t } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Add {
                        destination: Operand::Local("addr".to_owned()),
                        operand1: arm_register_to_ga_operand(n),
                        operand2: Operand::Immidiate(DataWord::Word32(*imm)),
                    },
                    GAOperation::Move {
                        destination: arm_register_to_ga_operand(t),
                        source: Operand::AddressInLocal("addr".to_owned(), 16),
                    },
                    GAOperation::ZeroExtend {
                        destination: arm_register_to_ga_operand(t),
                        operand: arm_register_to_ga_operand(t),
                        bits: 16,
                    },
                ],
            },
            Operation::LDRHReg { m, n, t } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Add {
                        destination: Operand::Local("addr".to_owned()),
                        operand1: arm_register_to_ga_operand(n),
                        operand2: arm_register_to_ga_operand(m),
                    },
                    GAOperation::Move {
                        destination: arm_register_to_ga_operand(t),
                        source: Operand::AddressInLocal("addr".to_owned(), 16),
                    },
                    GAOperation::ZeroExtend {
                        destination: arm_register_to_ga_operand(t),
                        operand: arm_register_to_ga_operand(t),
                        bits: 16,
                    },
                ],
            },
            Operation::LDRSBReg { m, n, t } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Add {
                        destination: Operand::Local("addr".to_owned()),
                        operand1: arm_register_to_ga_operand(n),
                        operand2: arm_register_to_ga_operand(m),
                    },
                    GAOperation::Move {
                        destination: arm_register_to_ga_operand(t),
                        source: Operand::AddressInLocal("addr".to_owned(), 8),
                    },
                    GAOperation::SignExtend {
                        destination: arm_register_to_ga_operand(t),
                        operand: arm_register_to_ga_operand(t),
                        bits: 8,
                    },
                ],
            },
            Operation::LDRSH { m, n, t } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Add {
                        destination: Operand::Local("addr".to_owned()),
                        operand1: arm_register_to_ga_operand(n),
                        operand2: arm_register_to_ga_operand(m),
                    },
                    GAOperation::Move {
                        destination: arm_register_to_ga_operand(t),
                        source: Operand::AddressInLocal("addr".to_owned(), 16),
                    },
                    GAOperation::SignExtend {
                        destination: arm_register_to_ga_operand(t),
                        operand: arm_register_to_ga_operand(t),
                        bits: 8,
                    },
                ],
            },
            Operation::LSLImm { imm, m, d } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Sl {
                        destination: arm_register_to_ga_operand(d),
                        operand: arm_register_to_ga_operand(m),
                        shift: Operand::Immidiate(DataWord::Word32(*imm)),
                    },
                    GAOperation::SetNFlag(arm_register_to_ga_operand(d)),
                    GAOperation::SetZFlag(arm_register_to_ga_operand(d)),
                ],
            },
            Operation::LSLReg { m, dn } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::And {
                        destination: Operand::Local("shift".to_owned()),
                        operand1: arm_register_to_ga_operand(m),
                        operand2: Operand::Immidiate(DataWord::Word32(0xff)),
                    },
                    GAOperation::Sl {
                        destination: arm_register_to_ga_operand(dn),
                        operand: arm_register_to_ga_operand(dn),
                        shift: Operand::Local("shift".to_owned()),
                    },
                    GAOperation::SetNFlag(arm_register_to_ga_operand(dn)),
                    GAOperation::SetZFlag(arm_register_to_ga_operand(dn)),
                ],
            },
            Operation::LSRImm { imm, m, d } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::Srl {
                        destination: arm_register_to_ga_operand(d),
                        operand: arm_register_to_ga_operand(m),
                        shift: Operand::Immidiate(DataWord::Word32(*imm)),
                    },
                    GAOperation::SetNFlag(arm_register_to_ga_operand(d)),
                    GAOperation::SetZFlag(arm_register_to_ga_operand(d)),
                ],
            },
            Operation::LSRReg { m, dn } => GAInstruction {
                instruction_size: 16,
                operations: vec![
                    GAOperation::And {
                        destination: Operand::Local("shift".to_owned()),
                        operand1: arm_register_to_ga_operand(m),
                        operand2: Operand::Immidiate(DataWord::Word32(0xff)),
                    },
                    GAOperation::Srl {
                        destination: arm_register_to_ga_operand(dn),
                        operand: arm_register_to_ga_operand(dn),
                        shift: Operand::Local("shift".to_owned()),
                    },
                    GAOperation::SetNFlag(arm_register_to_ga_operand(dn)),
                    GAOperation::SetZFlag(arm_register_to_ga_operand(dn)),
                ],
            },
            Operation::MOVImm { d, imm } => {
                let destination = arm_register_to_ga_operand(d);
                let source = Operand::Immidiate(DataWord::Word32(*imm));

                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Move {
                            destination: destination.clone(),
                            source,
                        },
                        GAOperation::SetNFlag(destination.clone()),
                        GAOperation::SetZFlag(destination),
                    ],
                }
            }
            Operation::MOVReg { m, d, set_flags } => {
                let destination = arm_register_to_ga_operand(d);
                let source = arm_register_to_ga_operand(m);
                let mut operations = vec![GAOperation::Move {
                    destination: destination.clone(),
                    source: source.clone(),
                }];
                if *set_flags {
                    operations.push(GAOperation::SetNFlag(destination.clone()));
                    operations.push(GAOperation::SetZFlag(destination));
                }
                GAInstruction {
                    instruction_size: 16,
                    operations,
                }
            }
            Operation::MRS { d, sysm } => GAInstruction {
                instruction_size: 32,
                operations: vec![GAOperation::Move {
                    destination: arm_register_to_ga_operand(d),
                    source: arm_special_register_to_operand(sysm),
                }],
            },
            Operation::MSRReg { n, sysm } => GAInstruction {
                instruction_size: 32,
                operations: vec![GAOperation::Move {
                    source: arm_register_to_ga_operand(n),
                    destination: arm_special_register_to_operand(sysm),
                }],
            },
            Operation::MUL { n, dm } => {
                let n = arm_register_to_ga_operand(n);
                let dm = arm_register_to_ga_operand(dm);

                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Mul { destination: dm.clone(), operand1: n, operand2: dm.clone() },
                        GAOperation::SetNFlag(dm.clone()),
                        GAOperation::SetZFlag(dm.clone()),
                    ]
                }
            },
            Operation::MVNReg { m, d } => {
                let m = arm_register_to_ga_operand(m);
                let d = arm_register_to_ga_operand(d);
                
                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Not { destination: d.clone(), operand: m },
                        GAOperation::SetNFlag(d.clone()),
                        GAOperation::SetZFlag(d),
                    ]
                }
            },
            Operation::NOP => {
                GAInstruction {
                    instruction_size: 16,
                    operations: vec![]
                }
            },
            Operation::ORRReg { m, dn } => {
                let m = arm_register_to_ga_operand(m);
                let dn = arm_register_to_ga_operand(dn);

                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        GAOperation::Or { destination: dn.clone(), operand1: dn.clone(), operand2: m },
                        GAOperation::SetNFlag(dn.clone()),
                        GAOperation::SetZFlag(dn.clone()),
                    ]
                }
            },
            Operation::POP { reg_list } => {
                let mut operations: Vec<GAOperation> = vec![];
                // set up base address
                operations.push(GAOperation::Move {
                    destination: Operand::Local("Address".to_owned()),
                    source: Operand::Register("SP".to_owned()),
                });
                for reg in reg_list {
                    // write register to memory
                    operations.push(GAOperation::Move {
                        source: Operand::AddressInLocal("Address".to_owned(), 32),
                        destination: arm_register_to_ga_operand(reg),
                    });
                    // update address
                    operations.push(GAOperation::Add {
                        destination: Operand::Local("Address".to_owned()),
                        operand1: Operand::Local("Address".to_owned()),
                        operand2: Operand::Immidiate(DataWord::Word32(4)),
                    })
                }
                // update SP
                operations.push(GAOperation::Add {
                    destination: Operand::Register("SP".to_owned()),
                    operand1: Operand::Register("SP".to_owned()),
                    operand2: Operand::Immidiate(DataWord::Word32((4 * reg_list.len()) as u32)),
                });

                GAInstruction {
                    instruction_size: 16,
                    operations,
                }
            }
            Operation::PUSH { reg_list } => {
                let mut operations: Vec<GAOperation> = vec![];
                // set up base address
                operations.push(GAOperation::Sub {
                    destination: Operand::Local("Address".to_owned()),
                    operand1: Operand::Register("SP".to_owned()),
                    operand2: Operand::Immidiate(DataWord::Word32((4 * reg_list.len()) as u32)),
                });
                for reg in reg_list {
                    // write register to memory
                    operations.push(GAOperation::Move {
                        destination: Operand::AddressInLocal("Address".to_owned(), 32),
                        source: arm_register_to_ga_operand(reg),
                    });
                    // update address
                    operations.push(GAOperation::Add {
                        destination: Operand::Local("Address".to_owned()),
                        operand1: Operand::Local("Address".to_owned()),
                        operand2: Operand::Immidiate(DataWord::Word32(4)),
                    })
                }
                // update SP
                operations.push(GAOperation::Sub {
                    destination: Operand::Register("SP".to_owned()),
                    operand1: Operand::Register("SP".to_owned()),
                    operand2: Operand::Immidiate(DataWord::Word32((4 * reg_list.len()) as u32)),
                });

                GAInstruction {
                    instruction_size: 16,
                    operations,
                }
            }
            Operation::REV { m, d } => {
                let m = arm_register_to_ga_operand(m);
                let d = arm_register_to_ga_operand(d);
                let b1 = Operand::Local("b1".to_owned());
                let b2 = Operand::Local("B2".to_owned());
                let b3 = Operand::Local("B3".to_owned());
                let b4 = Operand::Local("B4".to_owned());

                let b1_mask = Operand::Immidiate(DataWord::Word32(0x000000ff));
                let b2_mask = Operand::Immidiate(DataWord::Word32(0x0000ff00));
                let b3_mask = Operand::Immidiate(DataWord::Word32(0x00ff0000));
                let b4_mask = Operand::Immidiate(DataWord::Word32(0xff000000));

                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        // set destination to 0
                        GAOperation::Move { destination: d.clone(), source: Operand::Immidiate(DataWord::Word32(0))},
                        // extract all bytes
                        GAOperation::And { destination: b1.clone(), operand1: m.clone(), operand2: b1_mask },
                        GAOperation::And { destination: b2.clone(), operand1: m.clone(), operand2: b2_mask },
                        GAOperation::And { destination: b3.clone(), operand1: m.clone(), operand2: b3_mask },
                        GAOperation::And { destination: b4.clone(), operand1: m.clone(), operand2: b4_mask },
                        // shift all bytes
                        GAOperation::Sl { destination: b1.clone(), operand: b1.clone(), shift: Operand::Immidiate(DataWord::Word32(24)) },
                        GAOperation::Sl { destination: b2.clone(), operand: b2.clone(), shift: Operand::Immidiate(DataWord::Word32(8)) },
                        GAOperation::Srl { destination: b3.clone(), operand: b3.clone(), shift: Operand::Immidiate(DataWord::Word32(8)) },
                        GAOperation::Srl { destination: b4.clone(), operand: b4.clone(), shift: Operand::Immidiate(DataWord::Word32(24)) },
                        // or in to destination
                        GAOperation::Or { destination: d.clone(), operand1: d.clone(), operand2: b1.clone() },
                        GAOperation::Or { destination: d.clone(), operand1: d.clone(), operand2: b2.clone() },
                        GAOperation::Or { destination: d.clone(), operand1: d.clone(), operand2: b3.clone() },
                        GAOperation::Or { destination: d.clone(), operand1: d.clone(), operand2: b4.clone() },
                    ]
                }
            },
            Operation::REV16 { m, d } => {
                let m = arm_register_to_ga_operand(m);
                let d = arm_register_to_ga_operand(d);
                let b1 = Operand::Local("b1".to_owned());
                let b2 = Operand::Local("B2".to_owned());
                let b3 = Operand::Local("B3".to_owned());
                let b4 = Operand::Local("B4".to_owned());

                let b1_mask = Operand::Immidiate(DataWord::Word32(0x000000ff));
                let b2_mask = Operand::Immidiate(DataWord::Word32(0x0000ff00));
                let b3_mask = Operand::Immidiate(DataWord::Word32(0x00ff0000));
                let b4_mask = Operand::Immidiate(DataWord::Word32(0xff000000));

                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        // set destination to 0
                        GAOperation::Move { destination: d.clone(), source: Operand::Immidiate(DataWord::Word32(0))},
                        // extract all bytes
                        GAOperation::And { destination: b1.clone(), operand1: m.clone(), operand2: b1_mask },
                        GAOperation::And { destination: b2.clone(), operand1: m.clone(), operand2: b2_mask },
                        GAOperation::And { destination: b3.clone(), operand1: m.clone(), operand2: b3_mask },
                        GAOperation::And { destination: b4.clone(), operand1: m.clone(), operand2: b4_mask },
                        // shift all bytes
                        GAOperation::Sl { destination: b1.clone(), operand: b1.clone(), shift: Operand::Immidiate(DataWord::Word32(8)) },
                        GAOperation::Srl { destination: b2.clone(), operand: b2.clone(), shift: Operand::Immidiate(DataWord::Word32(8)) },
                        GAOperation::Sl { destination: b3.clone(), operand: b3.clone(), shift: Operand::Immidiate(DataWord::Word32(8)) },
                        GAOperation::Srl { destination: b4.clone(), operand: b4.clone(), shift: Operand::Immidiate(DataWord::Word32(8)) },
                        // or in to destination
                        GAOperation::Or { destination: d.clone(), operand1: d.clone(), operand2: b1.clone() },
                        GAOperation::Or { destination: d.clone(), operand1: d.clone(), operand2: b2.clone() },
                        GAOperation::Or { destination: d.clone(), operand1: d.clone(), operand2: b3.clone() },
                        GAOperation::Or { destination: d.clone(), operand1: d.clone(), operand2: b4.clone() },
                    ]
                }
            },
            Operation::REVSH { m, d } => {
                let m = arm_register_to_ga_operand(m);
                let d = arm_register_to_ga_operand(d);
                let b1 = Operand::Local("b1".to_owned());
                let b2 = Operand::Local("B2".to_owned());

                let b1_mask = Operand::Immidiate(DataWord::Word32(0x000000ff));
                let b2_mask = Operand::Immidiate(DataWord::Word32(0x0000ff00));

                GAInstruction {
                    instruction_size: 16,
                    operations: vec![
                        // set destination to 0
                        GAOperation::Move { destination: d.clone(), source: Operand::Immidiate(DataWord::Word32(0))},
                        // extract all bytes
                        GAOperation::And { destination: b1.clone(), operand1: m.clone(), operand2: b1_mask },
                        GAOperation::And { destination: b2.clone(), operand1: m.clone(), operand2: b2_mask },
                        // shift all bytes
                        GAOperation::Sl { destination: b1.clone(), operand: b1.clone(), shift: Operand::Immidiate(DataWord::Word32(8)) },
                        GAOperation::Srl { destination: b2.clone(), operand: b2.clone(), shift: Operand::Immidiate(DataWord::Word32(8)) },
                        // or in to destination
                        GAOperation::Or { destination: d.clone(), operand1: d.clone(), operand2: b1.clone() },
                        GAOperation::Or { destination: d.clone(), operand1: d.clone(), operand2: b2.clone() },
                        // sign extend
                        GAOperation::SignExtend { destination: d.clone(), operand: d.clone(), bits: 16 },
                    ]
                }
            },
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
            Operation::SUBImmSP { imm } => GAInstruction {
                instruction_size: 16,
                operations: vec![GAOperation::Sub {
                    destination: Operand::Register("SP".to_owned()),
                    operand1: Operand::Register("SP".to_owned()),
                    operand2: Operand::Immidiate(DataWord::Word32(*imm)),
                }],
            },
            Operation::SVC { imm } => todo!(),
            Operation::SXTB { m, d } => todo!(),
            Operation::SXTH { m, d } => todo!(),
            Operation::TSTReg { m, n } => todo!(),
            Operation::UDFT1 { imm } => todo!(),
            Operation::UDFT2 { imm } => todo!(),
            Operation::UXTB { m, d } => GAInstruction {
                instruction_size: 16,
                operations: vec![GAOperation::ZeroExtend {
                    destination: arm_register_to_ga_operand(d),
                    operand: arm_register_to_ga_operand(m),
                    bits: 8,
                }],
            },
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
        Register::SP => "SP".to_owned(),
        Register::LR => "LR".to_owned(),
        Register::PC => "PC".to_owned(),
    })
}

fn arm_special_register_to_operand(reg: &SpecialRegister) -> Operand {
    Operand::Register(match reg {
        SpecialRegister::APSR => "APSR".to_owned(),
        SpecialRegister::IAPSR => "IAPSR".to_owned(),
        SpecialRegister::EAPSR => "EAPSR".to_owned(),
        SpecialRegister::XPSR => "XPSR".to_owned(),
        SpecialRegister::IPSR => "IPSR".to_owned(),
        SpecialRegister::EPSR => "EPSR".to_owned(),
        SpecialRegister::IEPSR => "IEPSR".to_owned(),
        SpecialRegister::MSP => "MSP".to_owned(),
        SpecialRegister::PSP => "PSP".to_owned(),
        SpecialRegister::PRIMASK => "PRIMASK".to_owned(),
        SpecialRegister::CONTROL => "CONTROL".to_owned(),
    })
}

fn arm_cond_to_ga_cond(conditon: &ArmCodition) -> Condition {
    match conditon {
        armv6_m_instruction_parser::conditions::Condition::EQ => Condition::EQ,
        armv6_m_instruction_parser::conditions::Condition::NE => Condition::NE,
        armv6_m_instruction_parser::conditions::Condition::CS => Condition::CS,
        armv6_m_instruction_parser::conditions::Condition::CC => Condition::CC,
        armv6_m_instruction_parser::conditions::Condition::MI => Condition::MI,
        armv6_m_instruction_parser::conditions::Condition::PL => Condition::PL,
        armv6_m_instruction_parser::conditions::Condition::VS => Condition::VS,
        armv6_m_instruction_parser::conditions::Condition::VC => Condition::VC,
        armv6_m_instruction_parser::conditions::Condition::HI => Condition::HI,
        armv6_m_instruction_parser::conditions::Condition::LS => Condition::LS,
        armv6_m_instruction_parser::conditions::Condition::GE => Condition::GE,
        armv6_m_instruction_parser::conditions::Condition::LT => Condition::LT,
        armv6_m_instruction_parser::conditions::Condition::GT => Condition::GT,
        armv6_m_instruction_parser::conditions::Condition::LE => Condition::LE,
        armv6_m_instruction_parser::conditions::Condition::None => Condition::None,
    }
}

fn arm_reg_list_to_ga_op_list(reg_list: &Vec<Register>) -> Vec<Operand> {
    let mut ret = vec![];
    for reg in reg_list {
        ret.push(arm_register_to_ga_operand(reg));
    }
    ret
}
