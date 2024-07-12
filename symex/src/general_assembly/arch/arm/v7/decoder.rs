#![allow(missing_docs)]
use general_assembly::{
    condition::Condition,
    operand::{DataWord, Operand},
    operation::Operation,
    shift::Shift as GAShift,
};
use paste::paste;
use transpiler::pseudo;

use disarmv7::prelude::{
    Register,
    Shift,
    arch::set_flags::LocalUnwrap,
    Operation as V7Operation,
    Condition as ARMCondition
};

macro_rules! consume {
    (($($id:ident$($(.$e:expr)+)?),*) from $name:ident) => {
        #[allow(unused_parens)]
        let ($($id),*) = {
            paste!(
                let consumer = $name.consumer();
                $(
                    let ($id,consumer) = consumer.[<consume_ $id>]();
                    $(let $id = $id$(.$e)+;)?
                )*
                consumer.consume();
            );
            ($($id),*)
        };
    };
}
macro_rules! shift {
    ($ret:ident.$shift:ident $reg:ident -> $target:ident $(set c for $reg_flag:ident)?) => {
       if let Some(shift) = $shift {
            let (shift_t, shift_n) = (
                    shift.shift_t.local_into(),
                    (shift.shift_n as u32).local_into(),
            );
            $ret.push(
                Operation::Shift {
                    destination: $target.clone(),
                    operand: $reg.clone(),
                    shift_n: shift_n.clone(),
                    shift_t: shift_t.clone(),
            });
            $($ret.push( match shift_t{
                GAShift::Lsl => Operation::SetCFlagShiftLeft { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Asr => Operation::SetCFlagSra { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Lsr => Operation::SetCFlagSrl { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Rrx => todo!(),
                GAShift::Ror => todo!()
            });)?

       }
       else {

            $ret.push(
                Operation::Move{
                    destination:$target.clone(),
                    source:$reg.clone()
                });

       }
    };
}
macro_rules! shift_imm {
    ($ret:ident.($shift_t:ident,$($shift_n_const:literal)?$($shift_n:ident)?) $reg:ident -> $target:ident $(set c for $reg_flag:ident)?) => {
        {
            let (shift_t, shift_n) = (
                    $shift_t,
                    $($shift_n)?$($shift_n_const)?,
            );
            $($ret.push( match shift_t{
                GAShift::Lsl => Operation::SetCFlagShiftLeft { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Asr => Operation::SetCFlagSra { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Lsr => Operation::SetCFlagSrl { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Rrx => todo!(),
                GAShift::Ror => todo!()
            });)?
            $ret.push(
                Operation::Shift {
                    destination: $target.clone(),
                    operand: $reg.clone(),
                    shift_n,
                    shift_t,
            })
        }
    };
}

macro_rules! local {
    ($($id:ident),*) => {
        $(
            let $id = Operand::Local(stringify!($id).to_owned());
        )*
    };
}


pub trait Convert {
    fn convert(self,in_it_block:bool) -> Vec<Operation>;
}

impl Convert for (usize, V7Operation) {
    fn convert(self,in_it_block:bool) -> Vec<Operation> {
        'outer_block: {
            match self.1 {
                V7Operation::AdcImmediate(adc) => {
                    // Ensure that all fields are used
                    consume!((s.unwrap_or(false),rd,rn,imm) from adc);
                    let (rd, rn, imm): (
                        Option<Operand>,
                        Operand,
                        Operand
                    ) = (
                        rd.local_into(),
                        rn.local_into(),
                        imm.local_into()
                    );
                    let rd = rd.unwrap_or(rn.clone());
                    pseudo!([
                        let result = rn adc imm;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetCFlag(rn,imm,adc);
                            SetVFlag(rn,imm,adc);
                        }
                        rd = result;
                    ])
                }
                V7Operation::AdcRegister(adc) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd,
                            rn,
                            rm,
                            shift
                        ) from adc
                    );
                    let (rd, rn, rm) = (rd.local_into(), rn.local_into(), rm.local_into());
                    let rd = rd.unwrap_or(rn.clone());
                    local!(shifted);
                    let mut ret = vec![];
                    shift!(ret.shift rm -> shifted);
                    pseudo!(ret.extend[
                        let result = rn adc shifted;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetCFlag(rn,shifted,adc);
                            SetVFlag(rn,shifted,adc);
                        }
                        rd = result;
                    ]);
                    ret
                }
                V7Operation::AddImmediate(add) => {
                    consume!(
                        (
                          s.local_unwrap(in_it_block),
                          rd,
                          rn,
                          imm
                        ) from add
                    );

                    let (rd, rn, imm) = (rd.unwrap_or(rn).local_into(), rn.local_into(), imm.local_into());
                    pseudo!([
                        let result = imm + rn;
                        rd = result;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetCFlag(imm,rn,add);
                            SetVFlag(imm,rn,add);
                        }
                    ])
                }
                V7Operation::AddRegister(add) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd,
                            rn,
                            rm,
                            shift
                        ) from add
                    );
                    let should_jump = match rd {
                        Some(Register::PC) => true,
                        None => matches!(rn, Register::PC),
                        _ => false,
                    };

                    let (rd, rn, rm) = (
                        rd.local_into(),
                        rn.local_into(),
                        rm.local_into()
                    );
                    let rd = rd.unwrap_or(rn.clone());

                    let mut ret = vec![];
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);
                    pseudo!(ret.extend[
                        let result = shifted + rn;
                        if (should_jump) {
                            result = result<31:1> << 1.local_into();
                            Jump(result);
                        } else {
                            if (s) {
                                SetNFlag(result);
                                SetZFlag(result);
                                SetCFlag(shifted,rn,add);
                                SetVFlag(shifted,rn,add);
                            }
                            rd = result;
                        }
                    ]);
                    ret
                }
                V7Operation::AddSPImmediate(add) => {
                    consume!((
                            s.unwrap_or(false),
                            rd,
                            imm
                        ) from add
                    );
                    let (rd, imm) = (
                        rd.unwrap_or(Register::SP).local_into(),
                        imm.local_into()
                    );

                    pseudo!([
                        let result = Register("SP&") + imm;
                        
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetCFlag(Register("SP&"),imm,add);
                            SetVFlag(Register("SP&"),imm,add);
                        }
                        rd = result;
                    ])
                }
                V7Operation::AddSPRegister(add) => {
                    consume!(
                        (
                            s.unwrap_or(false),
                            rm,
                            rd,
                            shift
                        ) from add
                    );
                    let rd = rd.unwrap_or(rm);
                    let s = match rd {
                        Register::PC => false,
                        _ => s,
                    };
                    let (rd, rm) = (
                        rd.local_into(),
                        rm.local_into()
                    );
                    let mut ret = vec![];
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);
                    pseudo!(ret.extend[
                        let result = Register("SP&") + shifted;
                        
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetCFlag(Register("SP&"),shifted,add);
                            SetVFlag(Register("SP&"),shifted,add);
                        }
                        rd = result;
                    ]);
                    ret
                }
                V7Operation::Adr(adr) => {
                    consume!((rd,imm,add) from adr);
                    let (rd, imm) = (rd.local_into(), imm.local_into());
                    pseudo!([
                        let aligned = Register("PC+") & 0xFFFFFFFC.local_into();

                        let result = aligned - imm;
                        if (add) {
                            result = aligned + imm;
                        }
                        rd = result;
                    ])
                }
                V7Operation::AndImmediate(and) => {
                    consume!(
                        (
                            s.unwrap_or(false),
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            imm.local_into(),
                            carry
                        ) from and
                    );
                    pseudo!([

                            let result = rn & imm;
                            if (s) {
                                SetNFlag(result);
                                SetZFlag(result);
                            }
                            if (s && carry.is_some()){
                                Flag("C") = (carry.unwrap() as u32).local_into();
                            }
                            rd = result;
                    ])
                }
                V7Operation::AndRegister(and) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd,
                            rn,
                            rm,
                            shift
                        ) from and
                    );
                    let (rd, rn, rm) = (rd.unwrap_or(rn).local_into(), rn.local_into(), rm.local_into());
                    let mut ret = vec![];
                    local!(shifted);
                    if s {
                        shift!(ret.shift rm -> shifted set c for rm);
                    }
                    else {
                        shift!(ret.shift rm -> shifted);
                    }
                    pseudo!(ret.extend[
                        let result = rn & shifted;

                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                        }
                        rd = result;
                    ]);
                    ret
                }
                V7Operation::AsrImmediate(asr) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd,
                            rm,
                            imm
                        ) from asr
                    );
                    let (rd, rm, imm) = (rd.local_into(), rm.local_into(), imm.local_into());
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                        let result = rm asr imm;
                    
                        if (s) {
                            SetZFlag(result);
                            SetNFlag(result);
                            SetCFlag(rm,imm,rsa);
                            
                        }
                        rd = result;
                    ]);
                    ret
                }
                V7Operation::AsrRegister(asr) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd,
                            rm,
                            rn
                        ) from asr
                    );
                    let (rd, rm, rn) = (rd.local_into(), rm.local_into(), rn.local_into());
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                        let shift_n = rm<7:0>;
                        let result = rn asr shift_n;
                        if (s) {
                            SetZFlag(result);
                            SetNFlag(result);
                            SetCFlag(rn,shift_n,rsa);
                        }
                        rd = result;
                    ]);
                    
                    ret
                }
                V7Operation::B(b) => {
                    consume!((condition,imm) from b);
                    let (condition, imm) = (condition.local_into(), imm.local_into());
                    pseudo!([
                        let target = Register("PC+") + imm;
                        target = target<31:1> << 1.local_into();
                        Jump(target,condition);
                    ])
                }
                V7Operation::Bfc(bfc) => {
                    consume!((rd,lsb,msb) from bfc);
                    let rd = rd.local_into();
                    let mask = !mask_dyn(lsb, msb);
                    vec![
                        Operation::And { 
                            destination: rd.clone(), 
                            operand1: rd,
                            operand2: Operand::Immediate(DataWord::Word32(mask)) 
                        }
                    ]
                }
                V7Operation::Bfi(bfi) => {
                    consume!((rd,rn,lsb,msb) from bfi);
                    let (rd, rn) = (rd.local_into(), rn.local_into());
                    let diff = msb - lsb;
                    assert!(msb >= lsb);
                    pseudo!([
                        // Assume happy case here
                        let mask = ((diff - 1) << lsb).local_into();
                        mask = ! mask;
                        rd = rd & mask;
                        let intermediate = rn<diff:0> << lsb.local_into();
                        rd = rd | intermediate;
                    ])
                }
                V7Operation::BicImmediate(bic) => {
                    consume!((s.unwrap_or(false),rd,rn,imm,carry) from bic);
                    let (rd, rn, imm) = (rd.unwrap_or(rn).local_into(), rn.local_into(), imm.local_into());
                    let mut ret = vec![];
                    pseudo!(ret.extend[

                            let result = !imm;
                            result = rn & result;
                            rd = result;
                            if (s) {
                                SetNFlag(result);
                                SetZFlag(result);
                            }
                    ]);
                    if s {
                        if let Some(flag) = carry {
                            let flag: u32 = flag as u32;
                            pseudo!(ret.extend[
                                Flag("C") = flag.local_into();
                            ]);
                        }
                    }
                    ret
                }
                V7Operation::BicRegister(bic) => {
                    consume!((
                            s.local_unwrap(in_it_block),
                            rd,
                            rn,
                            rm,
                            shift
                        ) from bic
                    );

                    let (rd, rn, rm) = (rd.unwrap_or(rn).local_into(), rn.local_into(), rm.local_into());
                    let mut ret = vec![];
                    local!(shifted);

                    if s {
                        shift!(ret.shift rm -> shifted set c for rm);
                    }
                    else {
                        shift!(ret.shift rm -> shifted);
                    }

                    pseudo!(ret.extend[
                       let result = !shifted;
                       result &= rn;
                       if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                       }
                       rd = result;
                    ]);
                    ret
                }
                V7Operation::Bkpt(_) => vec![Operation::Nop],
                V7Operation::Bl(bl) => {
                    consume!((imm) from bl);
                    let imm = imm.local_into();
                    
                    pseudo!([
                            let next_instr_addr = Register("PC+");
                            Register("LR") = next_instr_addr<31:1> << 1.local_into();
                            Register("LR") |= 0b1.local_into();
                            next_instr_addr = Register("PC+") + imm;
                            next_instr_addr = next_instr_addr<31:1> << 1.local_into();
                            Register("PC+") = next_instr_addr;
                    ])
                }
                V7Operation::Blx(blx) => {
                    consume!((rm) from blx);
                    let rm = rm.local_into();
                    pseudo!([
                        let target = rm;
                        let next_instr_addr = Register("PC+") - 2.local_into();

                        Register("LR") = next_instr_addr<31:1> << 1.local_into();
                        Register("LR") |= 1.local_into();
                        Register("EPSR") = Register("EPSR") | (1 << 27).local_into();
                        target = target<31:1> << 1.local_into();
                        Register("PC+") = target;
                    ])
                }

                V7Operation::Bx(bx) => {
                    let rm = bx.rm.local_into();
                    pseudo!([
                        let next_addr = rm;
                        next_addr = next_addr<31:1> << 1.local_into();
                        Register("PC+") = next_addr;
                    ])
                }
                V7Operation::Cbz(cbz) => {
                    consume!((
                        non.unwrap_or(false), 
                        rn.local_into(),
                        imm
                        ) from cbz);
                    let imm = imm.local_into();

                    let cond = match non {
                        false => Condition::EQ,
                        true => Condition::NE,
                    };
                    pseudo!([
                        let old_z = Flag("Z");
                        SetZFlag(rn);
                        let dest = Register("PC+") + imm;
                        dest = dest<31:1> << 1.local_into();
                        Jump(dest,cond);
                        Flag("Z") = old_z;
                    ])
                }
                V7Operation::Clrex(_) => todo!("This should not be needed for now"),
                V7Operation::Clz(clz) => {
                    vec![Operation::CountLeadingZeroes{
                        destination: clz.rd.local_into(),
                        operand: clz.rm.local_into()
                    }]
                }
                V7Operation::CmnImmediate(cmn) => {
                    consume!((rn,imm) from cmn);
                    let (rn, imm) = (rn.local_into(), imm.local_into());
                    pseudo!([
                        let result = rn + imm;
                        SetNFlag(result);
                        SetZFlag(result);
                        SetCFlag(rn,imm,add);
                        SetVFlag(rn,imm,add);
                    ])
                }
                V7Operation::CmnRegister(cmn) => {
                    consume!((rn,rm,shift) from cmn);
                    let (rn, rm) = (rn.local_into(), rm.local_into());
                    let mut ret = vec![];
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);
                    pseudo!(ret.extend[
                        let result = rn + shifted;
                        SetNFlag(result);
                        SetZFlag(result);
                        SetCFlag(rn,shifted,add);
                        SetVFlag(rn,shifted,add);
                    ]);
                    ret
                }
                V7Operation::CmpImmediate(cmp) => {
                    consume!((rn,imm) from cmp);
                    let (rn, imm) = (rn.local_into(), imm.local_into());
                    pseudo!([
                        let result = rn - imm;
                        SetNFlag(result);
                        SetZFlag(result);
                        SetCFlag(rn,imm,sub);
                        SetVFlag(rn,imm,sub);
                    ])
                }
                V7Operation::CmpRegister(cmp) => {
                    consume!((rn,rm,shift) from cmp);
                    let (rn, rm) = (rn.local_into(), rm.local_into());
                    let mut ret = vec![];
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);
                    pseudo!(ret.extend[
                        let result = rn - shifted;
                        SetNFlag(result);
                        SetZFlag(result);
                        SetCFlag(rn,shifted,sub);
                        SetVFlag(rn,shifted,sub);
                    ]);
                    ret
                }
                V7Operation::Cps(cps) => {
                    consume!((enable,disable,affect_pri,affect_fault) from cps);
                    assert!(enable != disable);
                    let mut ret = Vec::with_capacity(1);
                    if enable {
                        if affect_pri {
                            // force lsb to 0
                            ret.push(
                                Operation::And { 
                                    destination: SpecialRegister::PRIMASK.local_into(),
                                    operand1: SpecialRegister::PRIMASK.local_into(),
                                    operand2: ((!(0b1u32)).local_into()) 
                                }
                            )
                        }
                        if affect_fault {
                            // force lsb to 0
                            ret.push(
                                Operation::And { 
                                    destination: SpecialRegister::FAULTMASK.local_into(),
                                    operand1: SpecialRegister::FAULTMASK.local_into(),
                                    operand2: ((!(0b1u32)).local_into()) 
                                }
                            )
                        }
                    } else {
                        if affect_pri {
                            // force lsb to 1
                            ret.push(
                                Operation::And { 
                                    destination: SpecialRegister::PRIMASK.local_into(), 
                                    operand1: SpecialRegister::PRIMASK.local_into(), 
                                    operand2: ((0b1u32).local_into()) 
                                }
                            )
                        }
                        if affect_fault {
                            // force lsb to 1
                            ret.push(
                                Operation::And {
                                    destination: SpecialRegister::FAULTMASK.local_into(),
                                    operand1: SpecialRegister::FAULTMASK.local_into(),
                                    operand2: ((0b1u32).local_into()) 
                                }
                            )
                        }
                    }
                    ret
                }
                // TODO! Decide wether or not to use this 
                V7Operation::Dbg(_) => vec![],
                V7Operation::Dmb(_) => {
                    todo!("This requires an exhaustive rewrite of the system to allow memory barriers")
                }
                V7Operation::Dsb(_) => {
                    todo!("This requires an exhaustive rewrite of the system to allow memory barriers")
                }
                V7Operation::EorImmediate(eor) => {
                    consume!(
                        (
                            s.unwrap_or(false),
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            imm.local_into(),
                            carry
                        ) from eor
                    );
                    pseudo!([
                        let result = rn ^ imm;
                        rd = result;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                        }
                        if (s && carry.is_some()){
                            Flag("C") = (carry.expect("Condition in EorImmediate broken") as u32).local_into();
                        }
                    ])
                }
                V7Operation::EorRegister(eor) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd,
                            rn,
                            rm,
                            shift
                        ) from eor
                    );
                    let (rd, rn, rm) = (rd.unwrap_or(rn).local_into(), rn.local_into(), rm.local_into());
                    let mut ret = Vec::with_capacity(10);
                    local!(shifted);
                    match s {
                        true => shift!(ret.shift rm -> shifted set c for rm),
                        false => shift!(ret.shift rm -> shifted)
                    };
                    pseudo!(ret.extend[
                        let result = rn ^ shifted;
                        
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                        }
                        rd = result;
                    ]);
                    ret

                }
                V7Operation::Isb(_) => todo!("This needs to be revisited when the executor can handle it"),
                V7Operation::It(it) => {
                    vec![
                    Operation::ConditionalExecution { 
                        conditions: it.conds.conditions.into_iter().map(|el| el.local_into()).collect() 
                    }
                ]
                },
                V7Operation::Ldm(ldm) => {
                    consume!((
                            rn,
                            w.unwrap_or(false),
                            registers
                        ) from ldm
                    );

                    let w = w && !registers.registers.contains(&rn);
                    let rn = rn.local_into();

                    let bc = registers.registers.len() as u32;
                    let mut contained = false;
                    let mut to_read: Vec<Operand> = vec![];
                    for reg in registers.registers.into_iter() {
                        if reg == Register::PC {
                            contained = true;
                        } else {
                            to_read.push(reg.local_into());
                        }
                    }
                    pseudo!([
                        let address = rn;

                        for reg in to_read.into_iter() {
                            reg = LocalAddress(address,32);
                            address += 4.local_into();
                        }

                        if (contained) {
                            let target = LocalAddress(address,4);
                            target = target<31:1> << 1.local_into();
                            Jump(target);
                        }
                        if (w) {
                            rn += (4*bc).local_into();
                        }
                    ])
                }
                V7Operation::Ldmdb(ldmdb) => {
                    consume!(
                        (
                            rn,
                            w.unwrap_or(false),
                            registers
                        ) from ldmdb
                    );

                    let w = w && !registers.registers.contains(&rn);
                    let rn = rn.local_into();

                    let bc = registers.registers.len() as u32;
                    let mut contained = false;
                    let mut to_read: Vec<Operand> = vec![];
                    for reg in registers.registers.into_iter() {
                        if reg == Register::PC {
                            contained = true;
                        } else {
                            to_read.push(reg.local_into());
                        }
                    }

                    pseudo!([
                        let address = rn - (4*bc).local_into();

                        for reg in to_read.into_iter() {
                            reg = LocalAddress(address,32);
                            address += 4.local_into();
                        }

                        if (contained) {
                            let target = LocalAddress(address,4);
                            target = target<31:1> << 1.local_into();
                            Jump(target);
                        }
                        if (w) {
                            rn -= (4*bc).local_into();
                        }
                    ])
                }
                V7Operation::LdrImmediate(ldr) => {
                    consume!((index,add,w.unwrap_or(false),rt,rn,imm) from ldr);
                    let old_rt = rt;
                    let is_pc = old_rt == Register::PC;
                    let (rt, rn, imm) = (rt.local_into(), rn.local_into(), imm.local_into());

 
                    let ret = pseudo!([
                        let offset_addr = rn-imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }

                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }

                        let data = LocalAddress(address,32);

                        if (w) {
                            rn = offset_addr;
                        }

                        if (is_pc) {
                            data = data<31:1> << 1.local_into();
                            Jump(data);
                        }
                        else {
                            rt = data;
                        }
                    ]);
                    ret
                }
                V7Operation::LdrLiteral(ldr) => {
                    consume!(
                        (
                            rt,
                            imm.local_into(),
                            add
                        ) from ldr
                    );
                    let new_t = rt.local_into();
                    pseudo!([
                        // Alling to 4
                        let base = Register("PC+")& 0xFFFFFFFC.local_into();

                        let address = base - imm;
                        if (add) {
                            address = base + imm;
                        }

                        let data = LocalAddress(address,32);
                        if (rt == Register::PC){
                            data = data<31:1> << 1.local_into();
                            Jump(data);
                        }
                        else {
                            new_t = data;
                        }
                    ])
                }
                V7Operation::LdrRegister(ldr) => {
                    consume!(
                        (
                            w.unwrap_or(false),
                            rt,
                            rn,
                            rm,
                            shift
                        ) from ldr
                    );
                    let _w = w;
                    let rt_old = rt;
                    let (rt, rn, rm) = (rt.local_into(), rn.local_into(), rm.local_into());
                    let should_shift = shift.is_some();
                    let shift = match shift {
                        Some(shift) => shift.shift_n as u32,
                        None => 0u32,
                    }
                    .local_into();
                    pseudo!([
                       let offset = rm;
                       if (should_shift) {
                            offset = rm << shift;
                       }

                       let address = rn + offset;
                       let data = LocalAddress(address,32) ;

                       if (w) {
                            rn = address;
                       }

                       if (rt_old == Register::PC){
                           data = data<31:1> << 1.local_into();
                           Jump(data);
                       }
                       else {
                           rt = data;
                       }
                    ])
                }
                V7Operation::LdrbImmediate(ldrb) => {
                    consume!(
                        (
                            index,
                            add.unwrap_or(false),
                            w.unwrap_or(false),
                            rt,
                            rn,
                            imm
                        ) from ldrb
                    );
                    let imm = imm.unwrap_or(0);
                    let (rt, rn, imm) = (rt.local_into(), rn.local_into(), imm.local_into());
                    pseudo!([
                        let offset_addr = rn - imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }

                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }

                        rt = ZeroExtend(LocalAddress(address,8),32);
                        if (w){
                            rn = offset_addr;
                        }
                    ])
                }
                V7Operation::LdrbLiteral(ldrb) => {
                    consume!((
                        add.unwrap_or(false),
                        rt.local_into(),
                        imm.local_into()
                        ) from ldrb);
                    pseudo!([
                        let base = Register("PC+") & 0xFFFFFFFC.local_into();

                        let address = base - imm;
                        if (add) {
                            address = base + imm;
                        }

                        rt = ZeroExtend(LocalAddress(address,8),32);
                    ])
                }
                V7Operation::LdrbRegister(ldrb) => {
                    consume!((rt,rn,rm,shift,add.unwrap_or(false)) from ldrb);
                    let (rt, rn, rm) = (rt.local_into(), rn.local_into(), rm.local_into());
                    let shift = match shift {
                        Some(shift) => shift.shift_n as u32,
                        _ => 0,
                    }
                    .local_into();
                    pseudo!([
                        let offset = rm << shift;
                        let offset_addr = rn - offset;
                        if (add) {
                            offset_addr = rn + offset;
                        }
                        let address = offset_addr;
                        rt = ZeroExtend(LocalAddress(address,8),32);
                    ])
                }
                V7Operation::Ldrbt(ldrbt) => {
                    consume!((rt,rn,imm) from ldrbt);
                    let (rt, rn, imm) = (rt.local_into(), rn.local_into(), imm.unwrap_or(0).local_into());
                    pseudo!([
                        let address = rn + imm;
                        rt = ZeroExtend(LocalAddress(address,8),32);
                    ])
                }
                V7Operation::LdrdImmediate(ldrd) => {
                    consume!((
                        rt.local_into(),
                        rt2.local_into(),
                        rn.local_into(),
                        imm.local_into(),
                        add.unwrap_or(false),
                        index.unwrap_or(false),
                        w.unwrap_or(false)
                        ) from ldrd);
                    pseudo!([
                        let offset_addr = rn - imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }

                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }

                        rt = LocalAddress(address,32);
                        address += 4.local_into();
                        rt2 = LocalAddress(address,32);

                        if (w) {
                            rn = offset_addr;
                        }
                    ])
                }
                V7Operation::LdrdLiteral(ldrd) => {
                    consume!((
                        rt.local_into(),
                        rt2.local_into(),
                        imm.local_into(),
                        add.unwrap_or(false),
                        w.unwrap_or(false),
                        index.unwrap_or(false)) from ldrd);
                    // These are not used in the pseudo code
                    let (_w, _index) = (w, index);
                    pseudo!([
                        let address = Register("PC+") - imm;
                        if (add) {
                            address = Register("PC+") + imm;
                        }
                        rt = LocalAddress(address,32);
                        address = address + 4.local_into();
                        rt2 = LocalAddress(address,32);
                    ])
                }
                V7Operation::Ldrex(_) => todo!("Hardware semaphores"),
                V7Operation::Ldrexb(_) => todo!("Hardware semaphores"),
                V7Operation::Ldrexh(_) => todo!("Hardware semaphores"),
                V7Operation::LdrhImmediate(ldrh) => {
                    consume!((
                            rt.local_into(),
                            rn.local_into(),
                            imm.local_into(),
                            add.unwrap_or(false),
                            w.unwrap_or(false),
                            index.unwrap_or(false)
                        ) from ldrh
                    );
                    pseudo!([
                        let offset_addr = rn - imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }

                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }

                        let data = LocalAddress(address,16);
                        if (w){
                            rn = offset_addr;
                        }
                        rt = ZeroExtend(data,32);
                    ])
                }
                V7Operation::LdrhLiteral(ldrh) => {
                    consume!(
                        (
                            rt.local_into(),
                            imm.local_into(),
                            add.unwrap_or(false)
                        ) from ldrh
                    );

                    pseudo!([
                        let aligned = Register("PC+") & 0xFFFFFFFC.local_into();

                        let address = aligned - imm;
                        if (add) {
                            address = aligned + imm;
                        }

                        let data = LocalAddress(address,16);
                        rt = ZeroExtend(data,32);
                    ])
                }
                V7Operation::LdrhRegister(ldrh) => {
                    consume!(
                        (
                            rt.local_into(),
                            rn.local_into(),
                            rm.local_into(),
                            shift
                        ) from ldrh
                    );

                    let mut ret = Vec::with_capacity(10);
                    let offset = Operand::Local("offset".to_owned());

                    shift!(ret.shift rm -> offset);
                    pseudo!(ret.extend[
                        let offset_addr = rn + offset;
                        let address = offset_addr;
                        let data = ZeroExtend(LocalAddress(address,16),32);
                        rt = data;
                    ]); 
                    ret
                }
                V7Operation::Ldrht(ldrht) => {
                    consume!(
                        (
                            rt.local_into(),
                            rn.local_into(),
                            imm.unwrap_or(0).local_into()
                        ) from ldrht
                    );
                    pseudo!([
                        let address = rn + imm;
                        let data = LocalAddress(address,16);
                        rt = ZeroExtend(data,32);
                    ])
                }
                V7Operation::LdrsbImmediate(ldrsb) => {
                    consume!((
                            rt.local_into(),
                            rn.local_into(),
                            imm.unwrap_or(0).local_into(),
                            add,
                            index,
                            wback
                        ) from ldrsb
                    );
                    pseudo!([
                        let offset_addr = rn - imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }

                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }

                        rt = SignExtend(LocalAddress(address,8),8);
                        if (wback) {
                            rn = offset_addr;
                        }
                    ])
                }
                V7Operation::LdrsbLiteral(ldrsb) => {
                    consume!((
                            rt.local_into(),
                            imm.local_into(),
                            add
                        ) from ldrsb
                    );
                    pseudo!([
                        let base = Register("PC+") & 0xFFFFFFFC.local_into();

                        let address = base - imm;
                        if (add) {
                            address = base + imm;
                        }

                        rt = SignExtend(LocalAddress(address,8),8);
                    ])
                }
                V7Operation::LdrsbRegister(ldrsb) => {
                    consume!(
                        (
                            rt.local_into(),
                            rn.local_into(),
                            rm.local_into(),
                            shift
                        ) from ldrsb
                    );
                    let mut ret = Vec::with_capacity(10);
                    let offset = Operand::Local("offset".to_owned());
                    
                    shift!(ret.shift rm -> offset);

                    
                    pseudo!(ret.extend[
                        let address = rn + offset;
                        rt = SignExtend(LocalAddress(address,8),8);
                    ]);

                    ret
                }
                V7Operation::Ldrsbt(ldrsbt) => {
                    consume!(
                        (
                            rt.local_into(),
                            rn.local_into(),
                            imm.local_into()
                        ) from ldrsbt
                    );

                    let address_setter = Operand::Local("address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 8);

                    vec![
                        Operation::Add {
                            destination: address_setter,
                            operand1: rn,
                            operand2: imm
                        },
                        Operation::SignExtend {
                            destination: rt,
                            operand: address,
                            bits: 8
                        }
                    ]
                }
                V7Operation::LdrshImmediate(ldrsh) => {
                    consume!((rt.local_into(), rn.local_into(), imm.unwrap_or(0).local_into(), add, index, wback ) from ldrsh);
                    let mut ret = Vec::with_capacity(10);
                    let address_setter = Operand::Local("address".to_owned());
                    let offset_address = Operand::Local("offset_address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 16);

                    ret.push(match add {
                        true => Operation::Add { destination: offset_address.clone(), operand1: rn.clone(), operand2: imm },
                        _ => Operation::Sub { destination: offset_address.clone(), operand1: rn.clone(), operand2: imm },
                    });

                    ret.push(match index {
                        true => Operation::Move { destination: address_setter.clone(), source: offset_address.clone() },
                        _ => Operation::Move { destination: address_setter.clone(), source: rn.clone() },
                    });

                    if wback {
                        ret.push(Operation::Move { destination: rn, source: offset_address })
                    }

                    ret.extend([
                        Operation::SignExtend { 
                            destination: rt,
                            operand: address,
                            bits: 16
                        }
                    ]);

                    ret
                }
                V7Operation::LdrshLiteral(ldrsh) => {
                    consume!(
                        (
                            rt.local_into(),
                            imm.local_into(),
                            add
                        ) from ldrsh
                    );
                    pseudo!([
                        let base = Register("PC+") & 0xFFFFFFFC.local_into();

                        let address = base - imm;
                        if (add) {
                            address = base + imm;
                        }

                        let data = LocalAddress(address,16);
                        rt = SignExtend(data,16);
                    ])
                }
                V7Operation::LdrshRegister(ldrsh) => {
                    consume!(
                        (
                            rt.local_into(),
                            rn.local_into(),
                            rm.local_into(),
                            shift
                        ) from ldrsh
                    );
                    let mut ret = Vec::with_capacity(10);
                    let offset = Operand::Local("offset".to_owned());
                    let address_setter = Operand::Local("address".to_owned());
                    let offset_address = Operand::Local("offset_address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 16);

                    shift!(ret.shift rm -> offset);

                    ret.extend([
                           Operation::Add {
                               destination: offset_address.clone(),
                               operand1: rn,
                               operand2: offset
                           },
                           Operation::Move {
                               destination: address_setter.clone(),
                               source: offset_address
                           },
                           Operation::Move {
                               destination: rt.clone(),
                               source: address
                           },
                           Operation::SignExtend {
                               destination: rt.clone(),
                               operand: rt,
                               bits: 16
                           }
                    ]);
                    ret
                }
                V7Operation::Ldrsht(ldrsht) => {
                    consume!(
                        (
                            rt.local_into(),
                            rn.local_into(),
                            imm.unwrap_or(0).local_into()
                        ) from ldrsht
                    );
                    let address_setter = Operand::Local("address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 16);
                    vec![
                        Operation::Add {
                            destination: address_setter,
                            operand1: rn,
                            operand2: imm
                        },
                        Operation::SignExtend {
                            destination: rt,
                            operand: address,
                            bits: 16
                        }
                    ]
                }
                V7Operation::Ldrt(ldrt) => {
                    consume!(
                        (
                            rt.local_into(),
                            rn.local_into(),
                            imm.unwrap_or(0).local_into()
                        ) from ldrt
                    );
                    let address_setter = Operand::Local("address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 32);
                    vec![
                        Operation::Add {
                            destination: address_setter,
                            operand1: rn,
                            operand2: imm
                        },
                        Operation::Move {
                            destination: rt,
                            source: address
                        }
                    ]
                }
                V7Operation::LslImmediate(lsl) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd.local_into(),
                            rm.local_into(),
                            imm
                        ) from lsl
                    );
                    let imm = (imm as u32).local_into();

                    pseudo!([
                        let result = rm << imm;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetCFlag(rm, imm, lsl);
                        }
                        rd = result;
                    ])
                }
                V7Operation::LslRegister(lsl) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd.local_into(),
                            rn.local_into(),
                            rm.local_into()
                        ) from lsl
                    );
                    local!(shift_n);

                    let mut ret = vec![
                        Operation::And {
                            destination: shift_n.clone(),
                            operand1: rm,
                            operand2: 0xff.local_into()
                        }
                    ];
                    let shift_t = Shift::Lsl.local_into();
                    match s {
                        true => shift_imm!(ret.(shift_t,shift_n) rn -> rd set c for rn),
                        false => shift_imm!(ret.(shift_t,shift_n) rn -> rd),
                    };

                    pseudo!(ret.extend[
                        if (s) {
                            SetNFlag(rd);
                            SetZFlag(rd);
                        }
                    ]);
                    ret
                }
                V7Operation::LsrImmediate(lsr) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd.local_into(),
                            rm.local_into(),
                            imm
                        ) from lsr
                    );
                    let imm = (imm as u32).local_into();

                    pseudo!([
                        let result = rm >> imm;

                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetCFlag(rm, imm, rsl);
                        }

                        rd = result;
                    ])
                }
                V7Operation::LsrRegister(lsr) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd.local_into(),
                            rn.local_into(),
                            rm.local_into()
                        ) from lsr
                    );
                    local!(shift_n);
                    let mut ret = vec![
                        Operation::And {
                            destination: shift_n.clone(),
                            operand1: rm,
                            operand2: 0xff.local_into()
                        }
                    ];
                    let shift_t = Shift::Lsr.local_into();
                    match s {
                        true => shift_imm!(ret.(shift_t,shift_n) rn -> rd set c for rn),
                        false => shift_imm!(ret.(shift_t,shift_n) rn -> rd),
                    };
                    pseudo!(ret.extend[
                        if (s) {
                            SetNFlag(rd);
                            SetZFlag(rd);
                        }
                    ]);
                    ret
                }
                V7Operation::Mla(mla) => {
                    consume!(
                        (
                            rn.local_into(),
                            ra.local_into(),
                            rd.local_into(),
                            rm.local_into()
                        ) from mla
                    );
                    let mut ret = Vec::with_capacity(3);
                    pseudo!(
                        ret.extend[
                           rd = rn*rm;
                           rd = rd+ra;
                        ]
                    );
                    ret
                }
                V7Operation::Mls(mls) => {
                    consume!(
                        (
                            rn.local_into(),
                            ra.local_into(),
                            rd.local_into(),
                            rm.local_into()
                        ) from mls
                    );
                    let mut ret = Vec::with_capacity(3);
                    pseudo!(
                        ret.extend[
                            rd = rn*rm;
                            rd = ra-rd;
                        ]
                    );
                    ret
                }
                // One single encoding, this needs to be revisited once it is needed
                V7Operation::MovImmediate(mov) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd.local_into(),
                            imm.local_into(),
                            carry
                        ) from mov
                    );
                    pseudo!([
                        let result = imm;
                        rd = result;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                        }
                        if (s && carry.is_some()) {
                            Flag("C") = (carry.expect("The if check is broken") as u32).local_into();
                        }
                    ])
                }
                V7Operation::MovRegister(mov) => {
                    consume!((s,rd, rm.local_into()) from mov);
                    if rd == Register::PC {
                        break 'outer_block pseudo!([
                           let dest = rm<31:1> << 1.local_into();
                           Jump(dest);
                        ]);
                    }
                    let rd = rd.local_into();
                    let mut ret = vec![
                        Operation::Move {
                            destination: rd.clone(),
                            source: rm
                        }
                    ];
                    if let Some(true) = s {
                        ret.extend([
                               Operation::SetNFlag(rd.clone()),
                               Operation::SetZFlag(rd)
                        ]);
                    }
                    ret
                }
                V7Operation::Movt(movt) => {
                    consume!((rd.local_into(),imm) from movt);
                    let imm = (imm as u32).local_into();
                    let mut ret = Vec::with_capacity(4);
                    let shift = 16.local_into();
                    local!(intermediate);
                    pseudo!(
                        ret.extend[
                        intermediate = imm << shift;
                        // Preserve the lower half word
                        rd = intermediate | rd<15:0>;
                        ]
                        );
                    ret
                }
                V7Operation::Mrs(mrs) => {
                    consume!(
                        (
                            rd.local_into(),
                            sysm
                        ) from mrs
                        );
                    pseudo!([
                            rd = 0.local_into();

                            if (((sysm>>3) & 0b11111) == 0 && (sysm&0b1 == 0)) {
                                rd = Register("IPSR");
                                rd = rd <8:0>;
                            }
                            // Ignoring the Epsr read as it evaluates to the same as RD already
                            // contains
                            if (((sysm>>3) & 0b11111) == 0 && (sysm & 0b10 == 0)) {
                                let intermediate = Register("APSR");
                                intermediate <<= 27.local_into();
                                rd |= intermediate;
                                // TODO! Add in DSP extension
                            }
                            if (((sysm>>3) & 0b11111) == 1 && (sysm & 0b100 == 0)) {
                                // TODO! Need to track wether or not the mode is priv
                            }

                            let primask = Register("PRIMASK");
                            let basepri = Register("BASEPRI");
                            let faultmask = Register("FAULTMASK");

                            if (((sysm>>3) & 0b11111) == 2 && (sysm & 0b111 == 0)) {
                                // TODO! Add in priv checks
                                rd &= (!1u32).local_into();
                                rd |= primask<0:0>;
                            }

                            if (((sysm>>3) & 0b11111) == 2 && (sysm & 0b111 == 1)) {
                                // TODO! Add in priv checks
                                rd &= (!0b1111111u32).local_into();
                                rd |= basepri<7:0>;
                            }

                            if (((sysm>>3) & 0b11111) == 2 && (sysm & 0b111 == 2)) {
                                // TODO! Add in priv checks
                                rd &= (!0b1111111u32).local_into();
                                rd |= basepri<7:0>;
                            }

                            if (((sysm>>3) & 0b11111) == 2 && (sysm & 0b111 == 3)) {
                                // TODO! Add in priv checks
                                rd &= (!1u32).local_into();
                                rd |= faultmask<0:0>;
                            }

                            if (((sysm>>3) & 0b11111) == 2 && (sysm & 0b111 == 4)) {
                                // TODO! Add in floating point support
                            }
                            ])
                }
                V7Operation::Msr(msr) => {
                    consume!(
                        (
                            rn.local_into(),
                            sysm,
                            mask
                        ) from msr
                        );
                    let mask: u32 = mask.into();
                    let apsr = SpecialRegister::APSR.local_into();
                    let primask = SpecialRegister::PRIMASK.local_into();
                    let basepri = SpecialRegister::BASEPRI.local_into();
                    let faultmask = SpecialRegister::FAULTMASK.local_into();
                    pseudo!([
                            if (((sysm>>3) & 0b11111) == 0 && (sysm&0b100 == 0)) {
                                if (mask & 0b10 == 2) {
                                    apsr = apsr<27:0>;
                                    let intermediate = rn<31:27><<27.local_into();
                                    apsr |= intermediate;
                                }
                            }
                            // Discarding the SP things for now
                            // TODO! add in SP things
                            if (((sysm>>3) & 0b11111) == 2 && (sysm&0b111 == 0)) {
                                // TODO! Add in priv checks
                                primask = primask<31:1> << 1.local_into();
                                let intermediate = rn<0:0>;
                                apsr |= intermediate;
                            }
                            if (((sysm>>3) & 0b11111) == 2 && (sysm&0b111 == 1)) {
                                // TODO! Add in priv checks
                                basepri = primask<31:8> << 8.local_into();
                                let intermediate = rn<7:0>;
                                basepri |= intermediate;
                            }
                            if (((sysm>>3) & 0b11111) == 2 && (sysm&0b111 == 2)) {
                                // TODO! Add in priv checks
                                basepri = primask<31:8> << 8.local_into();
                                let intermediate = rn<7:0>;
                                basepri |= intermediate;
                            }
                            if (((sysm>>3) & 0b11111) == 2 && (sysm&0b111 == 2)) {
                                // TODO! Add om priv and priority checks here
                                faultmask = faultmask<31:1> << 1.local_into();
                                let intermediate = rn<0:0>;
                                faultmask |= intermediate;
                            }
                            ])
                }
                V7Operation::Mul(mul) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rn,
                            rd.unwrap_or(rn).local_into(),
                            rm.local_into()
                        ) from mul
                    );
                    let rn = rn.local_into();
                    pseudo!([
                        rd = rn * rm;
                        
                        if (s) {
                            SetZFlag(rd);
                            SetNFlag(rd);
                        }
                    ])
                }
                V7Operation::MvnImmediate(mvn) => {
                    consume!(
                        (
                            s.unwrap_or(false),
                            rd.local_into(),
                            imm.local_into(),
                            carry
                        ) from mvn
                    );
                    pseudo!([
                        let result = !imm;
                        rd = result;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                        }
                        if (s && carry.is_some()){
                            let flag = (carry.unwrap() as u32).local_into();
                            Flag("C") = flag;
                        }
                    ])
                }
                V7Operation::MvnRegister(mvn) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd.local_into(),
                            rm.local_into(),
                            shift
                        ) from mvn
                    );
                    let mut ret = Vec::with_capacity(5);
                    local!(shifted);
                    match s {
                        true => shift!(ret.shift rm -> shifted set c for rm),
                        false => shift!(ret.shift rm -> shifted)
                    }
                    pseudo!(ret.extend[
                        rd = !shifted;
                        if (s) {
                            SetNFlag(rd);
                            SetZFlag(rd);
                        }
                    ]);
                    ret
                }
                V7Operation::Nop(_) => vec![Operation::Nop],
                V7Operation::OrnImmediate(orn) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        imm.local_into(),
                        carry,
                        s.unwrap_or(false)
                        ) from orn);
                    pseudo!([
                            let n_imm = !imm;
                            let result = rn | n_imm;
                            rd = result;

                            if (s) {
                                SetNFlag(result);
                                SetZFlag(result);
                            }
                            if (s && carry.is_some()){
                                let flag = (carry.unwrap() as u32).local_into();
                                Flag("C") = flag;
                            }
                    ])
                }
                V7Operation::OrnRegister(orn) => {
                    consume!(
                        (
                            s.unwrap_or(false),
                            rd,
                            rm.local_into(),
                            rn,
                            shift
                        ) from orn
                    );
                    let (rd, rn) = (rd.unwrap_or(rn).local_into(), rn.local_into());
                    let mut ret = Vec::with_capacity(5);
                    local!(shifted);
                    match s {
                        true => shift!(ret.shift rm -> shifted set c for rm),
                        false => shift!(ret.shift rm -> shifted)
                    }
                    pseudo!(ret.extend[
                        shifted = !shifted;
                        rd = rn | shifted;
                    
                        if (s) {
                            SetNFlag(rd);
                            SetZFlag(rd);
                        }
                    ]);
                    ret
                }
                V7Operation::OrrImmediate(orr) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        imm.local_into(),
                        carry,
                        s.unwrap_or(false)
                        ) from orr);
                    pseudo!([
                            let result = rn | imm;
                            rd = result;

                            if (s) {
                                SetNFlag(result);
                                SetZFlag(result);
                            }
                            if (s && carry.is_some()){
                                let flag = (carry.unwrap() as u32).local_into();
                                Flag("C") = flag;
                            }

                    ])
                }
                V7Operation::OrrRegister(orr) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd,
                            rm.local_into(),
                            rn,
                            shift
                        ) from orr
                    );
                    let (rd, rn) = (rd.unwrap_or(rn).local_into(), rn.local_into());
                    let mut ret = Vec::with_capacity(5);
                    local!(shifted);
                    match s {
                        true => shift!(ret.shift rm -> shifted set c for rm),
                        false => shift!(ret.shift rm -> shifted),
                    }
                    pseudo!(ret.extend[
                        rd = rn | shifted;
                        if (s) {
                            SetNFlag(rd);
                            SetZFlag(rd);
                        }
                    ]);
                    ret
                }
                V7Operation::Pkh(pkh) => {
                    consume!((rd,shift,rn,rm.local_into(),tb) from pkh);
                    let mut ret = Vec::with_capacity(5);
                    let (rd, rn) = (rd.unwrap_or(rn).local_into(), rn.local_into());
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);
                    let (msh, lsh) = match tb {
                        true => (rn, shifted),
                        _ => (shifted, rn),
                    };
                    pseudo!(
                        ret.extend[
                            lsh = lsh & (u16::MAX as u32).local_into();
                            msh = msh & (!(u16::MAX as u32)).local_into();
                            rd = msh | lsh;
                        ]
                    );
                    ret
                }
                V7Operation::PldImmediate(_pld) => {
                    todo!(" We need some speciality pre load instruction here")
                }
                V7Operation::PldLiteral(_) => todo!(" We need some speciality pre load instruction here"),
                V7Operation::PldRegister(_) => todo!(" We need some speciality pre load instruction here"),
                V7Operation::PliImmediate(_) => todo!(" We need some speciality pre load instruction here"),
                V7Operation::PliRegister(_) => todo!(" We need some speciality pre load instruction here"),
                V7Operation::Pop(pop) => {
                    consume!((registers) from pop);

                    let mut jump = false;
                    let mut to_pop = Vec::with_capacity(registers.registers.len());
                    let bc = registers.registers.len() as u32;
                    for reg in registers.registers {
                        if reg == Register::PC {
                            jump = true;
                        } else {
                            to_pop.push(reg.local_into());
                        }
                    }

                    pseudo!([
                        let address = Register("SP&");
                        Register("SP&") += (4*bc).local_into();

                        for reg in to_pop.into_iter(){
                            reg = LocalAddress(address,32);
                            address += 4.local_into();
                        }
                        if (jump) {
                            address = LocalAddress(address,32);
                            address = address<31:1> << 1.local_into();
                            Jump(address);
                        }
                    ])
                }
                V7Operation::Push(push) => {
                    consume!((registers) from push);
                    assert!(!registers.registers.contains(&Register::SP));
                    assert!(!registers.registers.contains(&Register::PC));

                    let n = registers.registers.len() as u32;
                    pseudo!([
                        let address = Register("SP&") - (4*n).local_into();

                        for reg in registers.registers {
                            LocalAddress(address,32) = reg.local_into();
                            address += 4.local_into();
                        }

                        Register("SP&") -= (4*n).local_into();
                    ])
                }
                V7Operation::Qadd(_) => todo!("Need to figure out how to do saturating operations"),
                V7Operation::Qadd16(_) => todo!("Need to figure out how to do saturating operations"),
                V7Operation::Qadd8(_) => todo!("Need to figure out how to do saturating operations"),
                V7Operation::Qasx(_) => todo!("Need to figure out how to do saturating operations"),
                V7Operation::Qdadd(_) => todo!("Need to figure out how to do saturating operations"),
                V7Operation::Qdsub(_) => todo!("Need to figure out how to do saturating operations"),
                V7Operation::Qsax(_) => todo!("Need to figure out how to do saturating operations"),
                V7Operation::Qsub(_) => {
                    todo!("Need to add in the flags APSR.Q");
                }
                V7Operation::Qsub16(_) => todo!("Need to figure out how to do saturating operations"),
                V7Operation::Qsub8(_) => todo!("Need to figure out how to do saturating operations"),
                V7Operation::Rbit(rbit) => {
                    consume!((rd.local_into(),rm.local_into()) from rbit);
                    let mut ret = vec![];
                    local!(intermediate);
                    let zero = 0.local_into();
                    for i in 0..31u32 {
                        let mask = (1 << i).local_into();
                        let shift = 31 - (i as i32) * 2i32;
                        match shift > 0 {
                            true => {
                                let shift = (shift as u32).local_into();
                                pseudo!(
                                    ret.extend[
                                    intermediate = zero;
                                    intermediate = rm & mask;
                                    intermediate =  intermediate << shift;
                                    rd = rd|intermediate;
                                    ]
                                    );
                            }
                            false => {
                                let shift = (-shift as u32).local_into();
                                pseudo!(
                                    ret.extend[
                                    intermediate = zero;
                                    intermediate = rm & mask;
                                    intermediate =  intermediate >> shift;
                                    rd = rd|intermediate;
                                    ]
                                    );
                            }
                        }
                    }
                    ret
                }
                V7Operation::Rev(rev) => {
                    consume!((rd.local_into(),rm.local_into()) from rev);
                    local!(int1, int2, int3, int4);
                    let mut ret = vec![];
                    let zero = 0.local_into();
                    pseudo!(
                        ret.extend[
                        int1 = rm<7:0>;
                        int2 = rm<15:8>;
                        int3 = rm<23:16>;
                        int4 = rm<31:24>;
                        int1 = int1 << (24).local_into();
                        int2 = int2 << (8).local_into();
                        int3 = int3 >> (8).local_into();
                        int4 = int4 >> (24).local_into();
                        rd = zero;
                        rd = rd | int1;
                        rd = rd | int2;
                        rd = rd | int3;
                        rd = rd | int4;
                        ]
                        );

                    ret
                }
                V7Operation::Rev16(rev) => {
                    consume!((rd.local_into(),rm.local_into()) from rev);
                    local!(int1, int2, int3, int4);
                    let mut ret = vec![];
                    let zero = 0.local_into();
                    pseudo!(
                        ret.extend[
                        int1 = rm<7:0>;
                        int2 = rm<15:8>;
                        int3 = rm<23:16>;
                        int4 = rm<31:24>;
                        int1 = int1 << 8.local_into();
                        int2 = int2 >> 8.local_into();
                        int3 = int3 << 8.local_into();
                        int4 = int4 >> 8.local_into();
                        rd = zero;
                        rd = rd | int1;
                        rd = rd | int2;
                        rd = rd | int3;
                        rd = rd | int4;
                        ]
                        );
                    ret
                }
                V7Operation::Revsh(revsh) => {
                    consume!((rd.local_into(),rm.local_into()) from revsh);
                    local!(int1, int2);
                    let mut ret = vec![];
                    let zero = 0.local_into();
                    pseudo!(
                        ret.extend[
                        int1 = rm<7:0>;
                        int2 = rm<15:8>;
                        int1 = int1 << 8.local_into();
                        int2 = int2 >> 8.local_into();
                        rd = zero;
                        ]
                        );
                    ret.push(
                        Operation::SignExtend { destination: rd.clone(), operand: int1, bits: 16 },
                    );
                    pseudo!(
                        ret.extend[
                        rd = rd | int2;
                        ]
                        );
                    ret
                }
                V7Operation::RorImmediate(ror) => {
                    consume!((s,rd.local_into(), rm.local_into(),imm) from ror);
                    let shift_n = imm.local_into();
                    let mut ret = vec![Operation::Sror { destination: rd.clone(), operand: rm.clone(), shift: shift_n.clone() }];
                    if let Some(true) = s {
                        ret.extend([
                            Operation::SetZFlag(rd.clone()),
                            Operation::SetNFlag(rd.clone()),
                            Operation::SetCFlagRor(rd.clone())
                        ]);
                    }
                    ret
                }
                V7Operation::RorRegister(ror) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rd.local_into(),
                            rm.local_into(),
                            rn.local_into()
                        ) from ror
                    );
                    local!(shift_n);
                    let mask = (u8::MAX as u32).local_into();

                    let mut ret = vec![
                        Operation::And {
                            destination: shift_n.clone(),
                            operand1: rm.clone(),
                            operand2: mask
                        },
                        Operation::Sror {
                            destination: rd.clone(),
                            operand: rn.clone(),
                            shift: shift_n.clone()
                        }
                    ];
                    if s {
                        ret.extend([
                            Operation::SetZFlag(rd.clone()),
                            Operation::SetNFlag(rd.clone()),
                            Operation::SetCFlagRor(rd.clone())
                        ]);
                    }
                    ret
                }
                V7Operation::Rrx(rrx) => {
                    consume!((s,rd.local_into(), rm.local_into()) from rrx);
                    // Let's fulhacka
                    let mask = (u32::MAX >> 1).local_into();
                    let lsb_mask = (1).local_into();
                    local!(lsb, result, msb);
                    let carry = Operand::Flag("C".to_owned());
                    let mut ret = Vec::with_capacity(10);
                    pseudo!(
                        ret.extend[
                        lsb = rm & lsb_mask;
                        result = rm >> 1.local_into();
                        msb = carry << 31.local_into();
                        // Clear the bit first
                        result = result & mask;
                        result = result | msb;
                        rd = result;
                        ]
                        );

                    if let Some(true) = s {
                        ret.extend([
                            Operation::SetNFlag(result.clone()),
                            Operation::SetZFlag(result.clone()),
                            Operation::Move {
                                destination: carry,
                                source: lsb
                            }
                        ]);
                    }
                    ret
                }
                V7Operation::RsbImmediate(rsb) => {
                    consume!((s,rd,rn,imm.local_into()) from rsb);
                    let (rd, rn) = (rd.unwrap_or(rn).local_into(), rn.local_into());
                    let s = s.local_unwrap(in_it_block);

                    pseudo!([
                        let result = imm - rn;

                        if (s) {
                            SetZFlag(result);
                            SetNFlag(result);
                            SetVFlag(imm,rn,sub);
                            SetCflag(imm,rn,sub);
                        }

                        rd = result;

                    ])
                }
                V7Operation::RsbRegister(rsb) => {
                    consume!((s,rd,rn,rm.local_into(), shift) from rsb);
                    let (rd, rn) = (rd.unwrap_or(rn).local_into(), rn.local_into());
                    let mut ret = Vec::with_capacity(10);
                    let carry = Operand::Flag("C".to_owned());
                    let one = 1.local_into();

                    local!(shifted, intermediate, old_carry);
                    shift!(ret.shift rm -> shifted);

                    pseudo!(
                        ret.extend[
                        // Backup carry bit
                        old_carry = carry;
                        // Set carry  bit to 1
                        carry = one;

                        intermediate = !rn;

                        // add with carry
                        rd = intermediate adc shifted;
                        ]
                        );
                    ret.extend(match s {
                        Some(true) => {
                            vec![
                                Operation::SetZFlag(rd.clone()),
                                Operation::SetNFlag(rd.clone()),
                                Operation::SetCFlag {
                                    operand1: intermediate,
                                    operand2: shifted,
                                    sub: false,
                                    carry: true
                                }
                            ]
                        }
                        _ => pseudo!([carry = old_carry;]),
                    });

                    ret
                }
                V7Operation::Sadd16(sadd) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from sadd);
                    pseudo!(
                        [
                            let sum1 = ZeroExtend(Signed(Resize(rn<15:0>,16) + Resize(rm<15:0>,16)),32);
                            let sum2 = ZeroExtend(Signed(Resize(rn<31:16>,16) + Resize(rm<31:16>,16)),32);
                            rd = ZeroExtend(sum1<15:0>,32);
                            let masked = ZeroExtend(sum2<15:0>,32) << 16.local_into();
                            rd = rd | masked;
                            // TODO! Add in ge flags here
                        ]
                    )
                }
                V7Operation::Sadd8(sadd) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from sadd);
                    pseudo!(
                        [
                            let sum1 = ZeroExtend(Signed(Resize(rn<7:0>,8) + Resize(rm<7:0>,8)),32);
                            let sum2 = ZeroExtend(Signed(Resize(rn<15:8>,8) + Resize(rm<15:8>,8)),32);
                            let sum3 = ZeroExtend(Signed(Resize(rn<23:16>,8) + Resize(rm<23:16>,8)),32);
                            let sum4 = ZeroExtend(Signed(Resize(rn<31:24>,8) + Resize(rm<31:24>,8)),32);
                            rd = ZeroExtend(sum1<7:0>,32);
                            let masked = ZeroExtend(sum2<7:0>,32) << 8.local_into();
                            rd = rd | masked;
                            masked = ZeroExtend(sum3<7:0>,32) << 16.local_into();
                            rd = rd | masked;
                            masked = ZeroExtend(sum4<7:0>,32) << 24.local_into();
                            rd = rd | masked;
                            // TODO! Add in ge flags here
                        ]
                    )
                }
                V7Operation::Sasx(sasx) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from sasx);
                    pseudo!(
                        [
                            let diff = ZeroExtend(Signed(Resize(rn<15:0>,16) - Resize(rm<31:16>,16)),32);
                            let sum  = ZeroExtend(Signed(Resize(rn<31:16>,16) + Resize(rm<15:0>,16)),32);
                            rd = ZeroExtend(diff<15:0>,32);
                            let masked = ZeroExtend(sum<15:0>,32) << 16.local_into();
                            rd = rd | masked;
                            // TODO! Add in ge flags here
                        ]
                    )
                }
                V7Operation::SbcImmediate(sbc) => {
                    consume!((
                            s.unwrap_or(false), 
                            rn.local_into(), 
                            rd.local_into().unwrap_or(rn.clone()),
                            imm.local_into()
                            ) from sbc);
                    let mut ret = Vec::with_capacity(7);
                    pseudo!(ret.extend[
                        let intermediate = !imm;
                        let result = rn adc intermediate;
                        if (s) {
                            SetZFlag(result);
                            SetNFlag(result);
                            SetCFlag(rn,imm,sbc);
                            SetVFlag(rn,imm,sbc);
                        }
                        rd = result;
                    ]);
                    ret
                }
                V7Operation::SbcRegister(sbc) => {
                    consume!((
                            s.local_unwrap(in_it_block),
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()), 
                            rm.local_into(),
                            shift
                            ) from sbc);
                    let mut ret = Vec::with_capacity(10);
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);
                    pseudo!(ret.extend[
                        let intermediate = !shifted;
                        let result = rn adc intermediate;
                        rd = result;
                    ]);
                    if s {
                        ret.extend([
                            Operation::SetZFlag(result.clone()),
                            Operation::SetNFlag(result.clone()),
                            Operation::SetCFlag {
                                operand1: rn.clone(),
                                operand2: intermediate.clone(),
                                sub: false,
                                carry: true
                            },
                            Operation::SetVFlag {
                                operand1: rn.clone(),
                                operand2: intermediate.clone(),
                                sub: false,
                                carry: true
                            }
                        ]);
                    }
                    ret
                }
                V7Operation::Sbfx(sbfx) => {
                    consume!((rd.local_into(), rn.local_into(), lsb, width) from sbfx);
                    let mut ret = vec![];

                    let msb = lsb + (width - 1);
                    let mask = ((1 << (msb - lsb)) - 1) << lsb;

                    pseudo!(
                        ret.extend[
                        let intermediate = rn & mask.local_into();
                        intermediate = intermediate >> lsb.local_into();
                        rd = SignExtend(intermediate,width);
                        ]
                        );
                    ret
                }
                V7Operation::Sdiv(sdiv) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from sdiv);
                    pseudo!([
                            let result = Signed(rn / rm);
                            rd = result;
                    ])
                }
                V7Operation::Sel(_) => todo!("SIMD"),
                V7Operation::Sev(_) => todo!("Modelling"),
                V7Operation::Shadd16(shadd) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from shadd);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                            let sum1 = ZeroExtend(Signed(Resize(rn<15:0>,16) + Resize(rm<15:0>,16)),32);
                            let sum2 = ZeroExtend(Signed(Resize(rn<31:16>,16) + Resize(rm<31:16>,16)),32);
                            rd = sum1<16:1>;
                            let intemediate_result = sum2<16:1> << 16.local_into();
                            rd = rd | intemediate_result;
                    ])
                }
                V7Operation::Shadd8(shadd) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from shadd);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                            let sum1 = ZeroExtend(Signed(Resize(rn<7:0>,8) + Resize(rm<7:0>,8)),32);
                            let sum2 = ZeroExtend(Signed(Resize(rn<15:8>,8) + Resize(rm<15:8>,8)),32);
                            let sum3 = ZeroExtend(Signed(Resize(rn<23:16>,8) + Resize(rm<23:16>,8)),32);
                            let sum4 = ZeroExtend(Signed(Resize(rn<31:24>,8) + Resize(rm<31:24>,8)),32);
                            rd = sum1<8:1>;
                            let intemediate_result = sum2<8:1> << 8.local_into();
                            rd = rd | intemediate_result;
                            intemediate_result = sum3<8:1> << 16.local_into();
                            rd = rd | intemediate_result;
                            intemediate_result = sum4<8:1> << 24.local_into();
                            rd = rd | intemediate_result;
                    ])
                }
                V7Operation::Shasx(shasx) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from shasx);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                            let diff = ZeroExtend(Signed(Resize(rn<15:0>,16) - Resize(rm<31:16>,16)),32);
                            let sum  = ZeroExtend(Signed(Resize(rn<31:16>,16) + Resize(rm<15:0>,16)),32);
                            rd = diff<16:1>;
                            let intemediate_result = sum<16:1> << 16.local_into();
                            rd = rd | intemediate_result;
                    ])
                }
                V7Operation::Shsax(shsax) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from shsax);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                            let sum = ZeroExtend(Signed(Resize(rn<15:0>,16) + Resize(rm<31:16>,16)),32);
                            let diff  = ZeroExtend(Signed(Resize(rn<31:16>,16) - Resize(rm<15:0>,16)),32);
                            rd = diff<16:1>;
                            let intemediate_result = sum<16:1> << 16.local_into();
                            rd = rd | intemediate_result;
                    ])
                }
                V7Operation::Shsub16(shsub) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from shsub);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                            let diff1 = ZeroExtend(Signed(Resize(rn<15:0>,16) - Resize(rm<15:0>,16)),32);
                            let diff2 = ZeroExtend(Signed(Resize(rn<31:16>,16) - Resize(rm<31:16>,16)),32);
                            rd = diff1<16:1>;
                            let intemediate_result = diff2<16:1> << 16.local_into();
                            rd = rd | intemediate_result;
                    ])
                }
                V7Operation::Shsub8(shsub) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from shsub);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                            let diff1 = ZeroExtend(Signed(Resize(rn<7:0>,8) - Resize(rm<7:0>,8)),32);
                            let diff2 = ZeroExtend(Signed(Resize(rn<15:8>,8) - Resize(rm<15:8>,8)),32);
                            let diff3 = ZeroExtend(Signed(Resize(rn<23:16>,8) - Resize(rm<23:16>,8)),32);
                            let diff4 = ZeroExtend(Signed(Resize(rn<31:24>,8) - Resize(rm<31:24>,8)),32);
                            rd = diff1<8:1>;
                            let intemediate_result = diff2<8:1> << 8.local_into();
                            rd = rd | intemediate_result;
                            intemediate_result = diff3<8:1> << 16.local_into();
                            rd = rd | intemediate_result;
                            intemediate_result = diff4<8:1> << 24.local_into();
                            rd = rd | intemediate_result;
                    ])
                }
                V7Operation::Smla(_) => todo!("Need to revisit SInt"),
                V7Operation::Smlad(_) => todo!("Need to revisit SInt"),
                V7Operation::Smlal(_) => todo!("Need to revisit SInt"),
                V7Operation::SmlalSelective(_) => todo!("Need to revisit SInt"),
                V7Operation::Smlald(_) => todo!("Need to revisit SInt"),
                V7Operation::Smlaw(_) => todo!("Need to revisit SInt"),
                V7Operation::Smlsd(_) => todo!("Need to revisit SInt"),
                V7Operation::Smlsld(_) => todo!("Need to revisit SInt"),
                V7Operation::Smmla(_) => todo!("Need to revisit SInt"),
                V7Operation::Smmls(_) => {
                    todo!()
                }
                V7Operation::Smmul(_) => todo!("Need to revisit SInt"),
                V7Operation::Smuad(_) => todo!("Need to revisit SInt"),
                V7Operation::Smul(_) => todo!("Need to revisit SInt"),
                V7Operation::Smull(_) => todo!("Need to revisit SInt"),
                V7Operation::Smulw(_) => todo!("Need to revisit SInt"),
                V7Operation::Smusd(_) => todo!("Need to revisit SInt"),
                V7Operation::Ssat(_) => todo!("Need to revisit SInt"),
                V7Operation::Ssat16(_) => todo!("Need to revisit SInt"),
                V7Operation::Ssax(_) => todo!("Need to revisit SInt"),
                V7Operation::Ssub16(_) => todo!("Need to revisit SInt"),
                V7Operation::Ssub8(_) => todo!("Need to revisit SInt"),
                V7Operation::Stm(stm) => {
                    consume!(
                        (
                            rn.local_into(),
                            registers,
                            w.unwrap_or(false)
                        ) from stm
                    );
                    let bc = registers.registers.len() as u32;

                    pseudo!([
                            let address = rn;

                            for reg in registers.registers {
                                LocalAddress(address,32) = reg.local_into();
                                address += 4.local_into();
                            }
                            if (w) {
                                rn += (4*bc).local_into();
                            }
                    ])
                }
                V7Operation::Stmdb(stmdb) => {
                    consume!(
                        (
                            w.unwrap_or(false), 
                            rn.local_into(), 
                            registers
                        ) from stmdb
                    );
                    let n = registers.registers.len() as u32;
                    pseudo!([
                        let address = rn - (4*n).local_into();
                        for reg in registers.registers{
                            LocalAddress(address,32) = reg.local_into();
                            address += 4.local_into();
                        }
                        if (w) {
                            rn = rn - (4u32* n).local_into();
                        }
                    ])
                }
                V7Operation::StrImmediate(str) => {
                    consume!(
                        (
                            w.unwrap_or(false),
                            add,
                            index.unwrap_or(false), 
                            rt.local_into(),
                            rn.local_into(), 
                            imm.local_into()
                        ) from str
                    );

                    pseudo!([

                        let offset_addr = 0.local_into();
                        if (add) {
                            offset_addr = rn + imm;
                        } else {
                            offset_addr = rn - imm;
                        }

                        let address = 0.local_into();
                        if (index) {
                            address = offset_addr;
                        } else {
                            address = rn;
                        }

                        LocalAddress(address,32) = rt;

                        if (w) {
                            rn = offset_addr;
                        }
                    ])
                }
                V7Operation::StrRegister(str) => {
                    consume!((
                            rt.local_into(),
                            rn.local_into(),
                            rm.local_into(),
                            shift) from str);
                    let shift_n = match shift {
                        Some(shift) => shift.shift_n as u32,
                        None => 0,
                    }
                    .local_into();
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                            // Shift will allways be LSL on the v7
                            let offset = rm << shift_n;
                            let address = rn + offset;
                            LocalAddress("address", 32) = rt;
                    ]);
                    ret
                }
                V7Operation::StrbImmediate(strb) => {
                    consume!(
                        (
                            w.unwrap_or(false),
                            add,
                            index.unwrap_or(false),
                            rt.local_into(),
                            rn.local_into(),
                            imm.local_into()
                        ) from strb
                    );
                    pseudo!([

                        let offset_addr = 0.local_into();
                        if (add) {
                            offset_addr = rn + imm;
                        } else {
                            offset_addr = rn - imm;
                        }

                        let address = 0.local_into();
                        if (index) {
                            address = offset_addr;
                        } else {
                            address = rn;
                        }

                        LocalAddress(address,8) = rt<7:0>;

                        if (w) {
                            rn = offset_addr;
                        }
                    ])
                }
                V7Operation::StrbRegister(strb) => {
                    consume!((
                            rt.local_into(),
                            rn.local_into(),
                            rm.local_into(),
                            shift
                            ) from strb);
                    let shift_n = match shift {
                        Some(shift) => shift.shift_n as u32,
                        None => 0,
                    }
                    .local_into();
                    pseudo!([
                            // Shift will allways be LSL on the v7
                            let offset = rm << shift_n;
                            let address = rn + offset;
                            LocalAddress("address", 8) = rt;
                    ])
                }
                V7Operation::Strbt(strbt) => {
                    consume!((
                            rt.local_into(),
                            rn.local_into(),
                            imm.unwrap_or(0).local_into()
                            ) from strbt);
                    let mut ret = vec![];
                    pseudo!(
                        ret.extend[
                        let address = rn + imm;
                        LocalAddress("address", 8) = rt;
                        ]);

                    ret
                }
                V7Operation::StrdImmediate(strd) => {
                    consume!((
                            rt.local_into(), 
                            rt2.local_into(), 
                            rn.local_into(),
                            add,
                            index.unwrap_or(true),
                            imm.unwrap_or(0).local_into(),
                            w.unwrap_or(false)
                            ) from strd);
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                            let offset_addr = rn - imm;
                            if (add) {
                                offset_addr = rn + imm;
                            }

                            let address = rn;
                            if (index) {
                                address = offset_addr;
                            }
                            LocalAddress("address",32) = rt;
                            address = address + 4.local_into();
                            LocalAddress("address",32) = rt2;

                            if (w) {
                                rn = offset_addr;
                            }
                    ]);
                    ret
                }
                V7Operation::Strex(strex) => {
                    consume!((
                            rd.local_into(),
                            rt.local_into(),
                            rn.local_into(),
                            imm.unwrap_or(0).local_into()
                            ) from strex
                            );
                    pseudo!([
                            let address = rn + imm;
                            // TODO! Add in exculisve addresses here
                            LocalAddress(address,32) = rt;
                            rd = 0.local_into();
                    ])
                }
                V7Operation::Strexb(strexb) => {
                    consume!((
                            rd.local_into(),
                            rt.local_into(),
                            rn.local_into()
                            ) from strexb
                            );
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                            let address = rn;
                            // TODO! Add in exculisve addresses here
                            LocalAddress(address,8) = rt;
                            rd = 0.local_into();
                    ]);
                    ret
                }
                V7Operation::Strexh(strexh) => {
                    consume!(
                        (
                            rd.local_into(),
                            rt.local_into(),
                            rn.local_into()
                        ) from strexh
                    );
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                            let address = rn;
                            // TODO! Add in exclusive address here
                            LocalAddress(address,16) = rt;
                            rd = 0.local_into();
                    ]);
                    ret
                }
                V7Operation::StrhImmediate(strh) => {
                    consume!(
                        (
                            rt.local_into(), 
                            rn.local_into(), 
                            imm.unwrap_or(0).local_into(),
                            w,
                            index,
                            add
                        ) from strh
                    );
                    pseudo!([
                        let offset_addr = rn - imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }
                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }
                        LocalAddress(address,16) = rt<15:0>;
                        if (w) {
                            rn = offset_addr;
                        }
                    ])
                }
                V7Operation::StrhRegister(strh) => {
                    consume!((
                            rt.local_into(),
                            rn.local_into(),
                            rm.local_into(),
                            shift
                            ) from strh);
                    let shift_n = match shift {
                        Some(shift) => {
                            assert!(shift.shift_t == Shift::Lsl);
                            shift.shift_n as u32
                        }
                        None => 0,
                    }
                    .local_into();
                    pseudo!([
                            let offset = rm << shift_n;
                            let address = rn + offset;
                            LocalAddress(address,16) = rt;
                    ])
                }
                V7Operation::Strht(strht) => {
                    consume!((
                            rt.local_into(),
                            rn.local_into(),
                            imm.unwrap_or(0).local_into()
                            ) from strht);
                    pseudo!([
                            let address = rn + imm;
                            LocalAddress(address,16) = rt;
                    ])
                }
                V7Operation::Strt(strt) => {
                    consume!((
                            rt.local_into(),
                            rn.local_into(),
                            imm.unwrap_or(0).local_into()
                            ) from strt);
                    pseudo!([
                            let address = rn + imm;
                            let data = rt;
                            LocalAddress(address,32) = data;
                    ])
                }
                V7Operation::SubImmediate(sub) => {
                    consume!(
                        (
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            imm.local_into(),
                            s.local_unwrap(in_it_block)
                        )from sub
                    );
                    pseudo!([
                        let result = rn - imm;

                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetCFlag(rn, imm, sub);
                            SetVFlag(rn, imm, sub);
                        }

                        rd = result;
                    ])
                }
                V7Operation::SubRegister(sub) => {
                    consume!(
                        (
                            s.local_unwrap(in_it_block),
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into(),
                            shift
                        ) from sub
                    );
                    let mut ret = vec![];
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);

                    pseudo!(ret.extend[
                            let result = rn - shifted;

                            if (s) {
                                SetNFlag(result);
                                SetZFlag(result);
                                SetCFlag(rn, shifted, sub);
                                SetVFlag(rn, shifted, sub);
                            }

                            rd = result;
                    ]);
                    ret
                }
                V7Operation::SubSpMinusImmediate(sub) => {
                    consume!(
                        (
                            s.unwrap_or(false),
                            rd.local_into().unwrap_or(Operand::Register("SP&".to_owned())),
                            imm.local_into()
                        ) from sub
                    );
                    let rn = Register::SP.local_into();

                    pseudo!([
                        let result = rn - imm;

                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetVFlag(rn, imm, sub);
                            SetCFlag(rn, imm, sub);
                        }

                        rd = result;
                    ])
                }
                V7Operation::SubSpMinusRegister(sub) => {
                    let rn = Register::SP.local_into();
                    consume!(
                        (
                            s.unwrap_or(false),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into(),
                            shift
                        ) from sub
                    );
                    let mut ret = vec![];
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);

                    pseudo!(ret.extend[
                        let result = rn - shifted;

                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetVFlag(rn,shifted,sub);
                            SetCFlag(rn,shifted,sub);
                        }

                        rd = result;
                    ]);
                    ret
                }
                V7Operation::Sxtab(sxtab) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into(),
                            rotation.unwrap_or(0)) from sxtab);
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                            let rotated = Ror(rm, rotation.local_into());
                            let masked = rotated & (u8::MAX as u32).local_into();
                            rd = rn + SignExtend(masked,8);
                    ]);
                    ret
                }
                V7Operation::Sxtab16(sxtab) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into(),
                            rotation.unwrap_or(0)) from sxtab);
                    pseudo!([
                            let rotated = Ror(rm, rotation.local_into());


                            // Clear the current rd
                            rd = 0.local_into();

                            let lsh_mask = (u16::MAX as u32).local_into();

                            let rotated_lsbyte = rotated & (u8::MAX as u32).local_into();
                            rd = rn & lsh_mask;
                            // TODO! Make note in the docs for GA that 8 is the msb in the number
                            // prior to sign extension
                            rd = rd + SignExtend(rotated_lsbyte,8);
                            rd = rd & lsh_mask;



                            //let msh_mask = ((u16::MAX as u32) << 16).local_into();
                            let msh_intermediate = rn >> 16.local_into();
                            rotated = rotated >> 16.local_into();
                            rotated = rotated & (u8::MAX as u32).local_into();
                            let intemediate_result = msh_intermediate + SignExtend(rotated,8);
                            intemediate_result = intemediate_result & lsh_mask;
                            intemediate_result = intemediate_result << 16.local_into();

                            rd =  rd | intemediate_result;
                            ])
                }
                V7Operation::Sxtah(sxtah) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into(),
                            rotation.unwrap_or(0).local_into()
                            ) from sxtah);
                    pseudo!([
                            let rotated = Ror(rm,rotation);
                            rotated = rotated & ( u16::MAX as u32).local_into();
                            rd = rn + SignExtend(rotated,16);
                    ])
                }
                V7Operation::Sxtb(sxtb) => {
                    consume!((
                            rd.local_into(),
                            rm.local_into(),
                            rotation.unwrap_or(0).local_into()
                            ) from sxtb);
                    pseudo!([
                            let rotated = Ror(rm,rotation);
                            rotated = rotated & ( u8::MAX as u32).local_into();
                            rd = SignExtend(rotated,8);
                    ])
                }
                V7Operation::Sxtb16(sxtb) => {
                    consume!((
                            rm.local_into(),
                            rd.local_into().unwrap_or(rm.clone()),
                            rotation.unwrap_or(0).local_into()
                            ) from sxtb);
                    pseudo!([
                            let rotated = Ror(rm,rotation);
                            let lsbyte = rotated & ( u8::MAX as u32).local_into();
                            rd = SignExtend(lsbyte,16) &  (u16::MAX as u32).local_into();

                            let msbyte = rotated >> 16.local_into();
                            msbyte = msbyte & (u8::MAX as u32).local_into();
                            msbyte = SignExtend(msbyte,16) & (u16::MAX as u32).local_into();
                            msbyte = msbyte << 16.local_into();

                            rd = rd | msbyte;
                    ])
                }
                V7Operation::Sxth(sxth) => {
                    consume!(
                        (
                            rd.local_into(),
                            rm.local_into(),
                            rotation.unwrap_or(0).local_into()
                        ) from sxth
                    );
                    pseudo!([
                            let rotated = Ror(rm,rotation) & (u16::MAX as u32).local_into();
                            rd = SignExtend(rotated, 16);
                    ])
                }
                V7Operation::Tb(tb) => {
                    consume!(
                        (
                            rn,
                            rm,
                            is_tbh.unwrap_or(false)
                        ) from tb
                    ); 

                    let (rn,rm) = (rn.local_into(),rm.local_into());

                    pseudo!([
                            let halfwords = 0.local_into();

                            if (is_tbh) {
                                let address = rm << 1.local_into();
                                address = address + rn;
                                halfwords = ZeroExtend(LocalAddress(address,16),32);
                            } else {
                                let address = rn + rm;
                                halfwords = ZeroExtend(LocalAddress(address,8),32);
                            }
                            let target = halfwords*2.local_into();
                            target = target + Register("PC+");
                            target = target<31:1> << 1.local_into();
                            Jump(target);
                    ])
                }
                V7Operation::TeqImmediate(teq) => {
                    consume!(
                        (
                            rn.local_into(),
                            imm.local_into(),
                            carry
                        ) from teq);
                    pseudo!([
                            let result = rn ^ imm;
                            SetNFlag(result);
                            SetZFlag(result);
                            if (carry.is_some()){
                                Flag("C") = (carry.unwrap() as u32).local_into();
                            }
                    ])
                }
                V7Operation::TeqRegister(teq) => {
                    consume!(
                        (
                            rn.local_into(),
                            rm.local_into(),
                            shift
                        ) from teq
                    );
                    let mut ret = vec![];
                    local!(intermediate);
                    shift!(ret.shift rm -> intermediate set c for rn);
                    pseudo!(ret.extend[
                            let result = rn ^ intermediate;
                            SetZFlag(result);
                            SetNFlag(result);
                    ]);
                    ret
                }
                V7Operation::TstImmediate(tst) => {
                    consume!((
                            rn.local_into(),
                            imm.local_into(),
                            carry
                            ) from tst);
                    pseudo!([
                        let result = rn & imm;
                        SetZFlag(result);
                        SetNFlag(result);
                        if (carry.is_some()){
                            Flag("C") = (carry.unwrap() as u32).local_into();
                        }
                    ])
                }
                V7Operation::TstRegister(tst) => {
                    let (rn, rm, shift) = (
                        tst.rn.local_into(),
                        tst.rm.local_into(),
                        tst.shift
                    );
                    let mut ret = vec![];
                    local!(shifted);
                    shift!(ret.shift rm -> shifted set c for rm);
                    pseudo!(ret.extend[
                            let result = rn & shifted;
                            SetNFlag(result);
                            SetZFlag(result);
                    ]);
                    ret
                }
                V7Operation::Uadd16(uadd) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from uadd);
                    pseudo!([
                        let lsh_mask = (u16::MAX as u32).local_into();

                        let rn_lsh = rn & lsh_mask;
                        let rm_lsh = rm & lsh_mask;

                        let sum1 = rn_lsh + rm_lsh;
                        sum1 = sum1 & lsh_mask;

                        let rn_msh = rn >> 16.local_into();
                        rn_msh = rn_msh & lsh_mask;

                        let rm_msh = rm >> 16.local_into();
                        rm_msh = rm & lsh_mask;

                        let sum2 = rn_msh + rm_msh;
                        sum2 = sum2 & lsh_mask;
                        sum2 = sum2 << 16.local_into();

                        rd = sum1 | sum2;

                        // TODO! Fix GE flags
                    ])
                }
                V7Operation::Uadd8(uadd) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from uadd);
                    pseudo!([
                            let sum1 = rn<7:0> + rm<7:0>;
                            let sum2 = rn<15:8> + rm<15:8>;
                            let sum3 = rn<23:16> + rm<23:16>;
                            let sum4 = rn<31:24> + rm<31:24>;
                            rd = sum1<7:0>;
                            let intermediate = sum2<7:0> << 8.local_into();
                            rd = rd | intermediate;
                            intermediate = sum3<7:0> << 16.local_into();
                            rd = rd | intermediate;
                            intermediate = sum4<7:0> << 24.local_into();
                            rd = rd | intermediate;
                            // TODO! Add in GE flags
                    ])
                }
                V7Operation::Uasx(uasx) => {
                    consume!(
                        (
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                        ) from uasx
                        );
                    pseudo!([
                            let diff = rn<15:0> - rm<31:16>;
                            let sum = rn<31:16> + rm<15:0>;
                            rd = diff<15:0>;
                            let shifted = sum<15:0> << 16.local_into();
                            rd = rd | shifted;
                            // TODO! Implement aspr.ge
                    ])
                }
                V7Operation::Ubfx(ubfx) => {
                    consume!(
                        (
                            rd.local_into(),
                            rn.local_into(),
                            lsb,
                            width
                        )
                        from ubfx
                        );
                    let msbit = lsb + (width - 1);
                    pseudo!([
                            rd = rn<msbit:lsb>;
                    ])
                }
                V7Operation::Udf(_) => vec![Operation::Nop],
                V7Operation::Udiv(udiv) => {
                    consume!(
                        (
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                        ) from udiv
                        );
                    pseudo!([
                            let result = rn/rm;
                            rd = result;
                    ])
                }
                V7Operation::Uhadd16(uhadd) => {
                    consume!(
                        (
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                        ) from uhadd
                        );
                    pseudo!([
                            let sum1 = rn<15:0> + rm<15:0>;
                            let sum2 = rn<31:16> + rm<31:16>;
                            rd = sum1<16:1>;
                            let sum2_half = sum2<16:1> << 16.local_into();
                            rd = rd | sum2_half;
                    ])
                }
                V7Operation::Uhadd8(uhadd) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from uhadd);
                    pseudo!([
                            let sum1 = rn<7:0> + rm<7:0>;
                            let sum2 = rn<15:8> + rm<15:8>;
                            let sum3 = rn<23:16> + rm<23:16>;
                            let sum4 = rn<31:24> + rm<31:24>;

                            rd = sum1<8:1>;

                            let sum2_shifted = sum2<8:1> << 8.local_into();
                            let sum3_shifted = sum3<8:1> << 16.local_into();
                            let sum4_shifted = sum4<8:1> << 24.local_into();

                            rd = rd | sum2_shifted;
                            rd = rd | sum3_shifted;
                            rd = rd | sum4_shifted;
                    ])
                }
                V7Operation::Uhasx(uhasx) => {
                    consume!(
                        (
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                        ) from uhasx
                        );
                    pseudo!([
                            let diff = rn<15:0> - rm<31:16>;
                            let sum = rn<31:16> + rm<15:0>;
                            rd = diff<16:1>;
                            let shifted = sum<16:1> << 16.local_into();
                            rd = rd | shifted;
                            // TODO! Implement aspr.ge
                    ])
                }
                V7Operation::Uhsax(uhsax) => {
                    consume!(
                        (
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                        ) from uhsax
                        );
                    pseudo!([
                            let diff = rn<15:0> + rm<31:16>;
                            let sum = rn<31:16> - rm<15:0>;
                            rd = diff<16:1>;
                            let shifted = sum<16:1> << 16.local_into();
                            rd = rd | shifted;
                            // TODO! Implement aspr.ge
                    ])
                }
                V7Operation::Uhsub16(uhsub) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from uhsub);
                    pseudo!([
                            let diff1 = rn<15:0> + rm<15:0>;
                            let diff2 = rn<31:16> + rm<31:16>;
                            rd = diff1<16:1>;
                            let diff2_shifted = diff2<16:1> << 16.local_into();
                            rd = rd | diff2_shifted;


                    ])
                }
                V7Operation::Uhsub8(uhsub) => {
                    consume!((
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                            ) from uhsub);
                    pseudo!([
                        let diff1 = rn<7:0> - rm<7:0>;
                        let diff2 = rn<15:8> - rm<15:8>;
                        let diff3 = rn<23:16> - rm<23:16>;
                        let diff4 = rn<31:24> - rm<31:24>;
                        rd = diff1<8:1>;
                        let intermediate = diff2<8:1> << 8.local_into();
                        rd = rd | intermediate;
                        intermediate = diff3<8:1> << 16.local_into();
                        rd = rd | intermediate;
                        intermediate = diff4<8:1> << 24.local_into();
                        rd = rd | intermediate;
                    ])
                }
                V7Operation::Umaal(umaal) => {
                    consume!(
                        (
                            rdlo.local_into(),
                            rdhi.local_into(),
                            rn.local_into(),
                            rm.local_into()
                        ) from umaal
                        );
                    pseudo!([
                        let result = rn*rm;

                        result = ZeroExtend(result,64) + ZeroExtend(rdlo,64);
                        result = result + ZeroExtend(rdhi,64);

                        rdhi = result<63:32:u64>;
                        rdlo = result<32:0:u64>;
                    ])
                }
                V7Operation::Umlal(umlal) => {
                    consume!(
                        (
                            rdlo.local_into(),
                            rdhi.local_into(),
                            rn.local_into(),
                            rm.local_into()
                        ) from umlal
                        );
                    pseudo!([
                        let result = rn*rm;

                        // Compose the rd
                        let rd_composite= ZeroExtend(0.local_into(), 64);
                        rd_composite = rdhi << 32.local_into();
                        rd_composite = rd_composite | rdlo;

                        result = ZeroExtend(result,64) + rd_composite;

                        rdhi = result<63:32:u64>;
                        rdlo = result<32:0:u64>;
                    ])
                }
                V7Operation::Umull(umull) => {
                    consume!(
                        (
                            rdlo.local_into(),
                            rdhi.local_into(),
                            rn.local_into(),
                            rm.local_into()
                        ) from umull
                        );
                    pseudo!([
                        let result = ZeroExtend(0.local_into(),64);
                        result = ZeroExtend(rn,64)*ZeroExtend(rm,64);
                        rdhi = result<63:32:u64>;
                        rdlo = result<31:0:u64>;
                    ])
                }
                V7Operation::Uqadd16(_) => todo!("TODO! Look in to saturating operators"),
                V7Operation::Uqadd8(_) => todo!("TODO! Look in to saturating operators"),
                V7Operation::Uqasx(_) => todo!("TODO! Look in to saturating"),
                V7Operation::Uqsax(_) => todo!("TODO! ^"),
                V7Operation::Uqsub16(_) => todo!("TODO! ^"),
                V7Operation::Uqsub8(_) => todo!("TODO! ^"),
                V7Operation::Uqsad8(_) => todo!("TODO! ^"),
                V7Operation::Usada8(_) => todo!("TODO! ^"),
                V7Operation::Usad8(_) => todo!("TODO! Look in to why ABS is needed here"),
                V7Operation::Usat(_) => todo!("TODO! Look in to why ABS is needed here"),
                V7Operation::Usat16(_) => todo!("TODO! Look in to SAT"),
                V7Operation::Usax(usax) => {
                    let (rn, rd, rm) = (usax.rn.local_into(), usax.rd.local_into(), usax.rm.local_into());
                    let rd = rd.unwrap_or(rn.clone());
                    pseudo!([
                        let sum = rn<15:0> + rm<31:16>;
                        let diff = rn<31:16> - rm<15:0>;
                        rd = sum<15:0>;
                        diff = diff<15:0> << 16.local_into();
                        rd = rd | diff;
                        // TODO! Look in to the GE register setting
                    ])
                }
                V7Operation::Usub16(usub) => {
                    let (rn, rd, rm) = (usub.rn.local_into(), usub.rd.local_into(), usub.rm.local_into());
                    let rd = rd.unwrap_or(rn.clone());

                    pseudo!([
                        let diff1 = rn<15:0> - rm<15:0>;
                        let diff2 = rn<31:16> - rm<31:16>;
                        rd = diff1<15:0>;
                        diff2 = diff2<15:0> << 16.local_into();
                        rd = rd | diff2;

                            // TODO! Look in to the GE register setting
                    ])
                }
                V7Operation::Usub8(_) => {
                    todo!("SIMD needs more work");
                }
                V7Operation::Uxtab(uxtab) => {
                    let (
                        rn,
                        rd,
                        rm,
                        rotation
                        ) = (
                            uxtab.rn.local_into(),
                            uxtab.rd.local_into(),
                            uxtab.rm.local_into(),
                            uxtab.rotation.unwrap_or(0)
                    );
                    let rd = rd.unwrap_or(rn.clone());
                    pseudo!([
                        let rotated = Ror(rm,rotation.local_into());
                        rd = rn + ZeroExtend(rotated<7:0>,32);
                    ])
                }
                V7Operation::Uxtab16(uxtab) => {
                    let (rn, rd, rm, rotation) = (uxtab.rn.local_into(), uxtab.rd.local_into(), uxtab.rm.local_into(), uxtab.rotation.unwrap_or(0));
                    let rd = rd.unwrap_or(rn.clone());
                    pseudo!([
                        let rotated = Ror(rm,rotation.local_into());
                        rd = rn<15:0> + ZeroExtend(rotated<7:0>,32);
                        let intermediate = rn<31:16> + ZeroExtend(rotated<23:16>,32);
                        intermediate = intermediate<15:0> << 16.local_into();
                        rd = rd<15:0> | intermediate;
                    ])
                }
                V7Operation::Uxtah(uxtah) => {
                    let (rn, rd, rm, rotation) = (uxtah.rn.local_into(), uxtah.rd.local_into(), uxtah.rm.local_into(), uxtah.rotation.unwrap_or(0));
                    let rd = rd.unwrap_or(rn.clone());
                    pseudo!([
                        let rotated = Ror(rm,rotation.local_into());
                        rd = rn + ZeroExtend(rotated<15:0>,32);
                    ])
                }
                V7Operation::Uxtb(uxtb) => {
                    let (rd, rm, rotation) = (uxtb.rd.local_into(), uxtb.rm.local_into(), uxtb.rotation.unwrap_or(0));
                    pseudo!([
                        let rotated = Ror(rm,rotation.local_into());
                        rd = ZeroExtend(rotated<7:0>,32);
                    ])
                }
                V7Operation::Uxtb16(uxtb) => {
                    let (rd, rm, rotation) = (uxtb.rd.local_into(), uxtb.rm.local_into(), uxtb.rotation.unwrap_or(0));
                    let rd = rd.unwrap_or(rm.clone());
                    pseudo!([
                        let rotated = Ror(rm,rotation.local_into());
                        rd = ZeroExtend(rotated<7:0>,32);
                        rotated = rotated<23:16> << 16.local_into();
                        rd = rd | rotated;
                    ])
                }
                V7Operation::Uxth(uxth) => {
                    let (rd, rm, rotation) = (uxth.rd.local_into(), uxth.rm.local_into(), uxth.rotation.unwrap_or(0));
                    pseudo!([
                        let rotated = rm;
                        if (rotation != 0) {
                            rotated = Ror(rm,rotation.local_into());
                        }
                        rd = ZeroExtend(rotated<15:0>,32);
                    ])
                }
                V7Operation::Wfe(_) => todo!("This requires extensive system modelling"),
                V7Operation::Wfi(_) => todo!("This requires extensive system modelling"),
                V7Operation::Yield(_) => todo!("This requires extensive system modelling"),
                V7Operation::Svc(_) => todo!(),
                V7Operation::Stc(_) => todo!(),
                V7Operation::Mcr(_) => todo!(),
                V7Operation::Mrc(_) => todo!(),
                V7Operation::Mrrc(_) => todo!(),
                V7Operation::Mcrr(_) => todo!(),
                V7Operation::Cdp(_) => todo!(),
                V7Operation::LdcLiteral(_) => todo!(),
                V7Operation::LdcImmediate(_) => todo!(),
            }
        }
    }
}

mod sealed {
    pub trait Into<T> {
        fn local_into(self) -> T;
    }
    pub trait ToString {
        fn to_string(self) -> String;
    }
}

use sealed::Into;

use self::sealed::ToString;


impl sealed::Into<Operand> for Register {
    fn local_into(self) -> Operand {
        Operand::Register(self.to_string())
    }
}

impl sealed::Into<Condition> for ARMCondition {
    fn local_into(self) -> Condition {
        match self {
            Self::Eq => Condition::EQ,
            Self::Ne => Condition::NE,
            Self::Mi => Condition::MI,
            Self::Pl => Condition::PL,
            Self::Vs => Condition::VS,
            Self::Vc => Condition::VC,
            Self::Hi => Condition::HI,
            Self::Ge => Condition::GE,
            Self::Lt => Condition::LT,
            Self::Gt => Condition::GT,
            Self::Ls => Condition::LS,
            Self::Le => Condition::LE,
            Self::Cs => Condition::CS,
            Self::Cc => Condition::CC,
            Self::None => Condition::None,
        }
    }
}

pub enum SpecialRegister {
    APSR,
    IAPSR,
    EAPSR,
    XPSR,
    IPSR,
    EPSR,
    IEPSR,
    MSP,
    PSP,
    PRIMASK,
    CONTROL,
    FAULTMASK,
    BASEPRI,
}

impl Into<Operand> for SpecialRegister {
    fn local_into(self) -> Operand {
        Operand::Register(match self {
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
            SpecialRegister::FAULTMASK => "FAULTMASK".to_owned(),
            SpecialRegister::BASEPRI => "BASEPRI".to_owned(),
        })
    }
}

impl sealed::ToString for Register {
    fn to_string(self) -> String {
        match self {
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
            Register::SP => "SP&".to_owned(),
            Register::LR => "LR".to_owned(),
            Register::PC => "PC+".to_owned(),
        }
    }
}
impl sealed::Into<Option<Operand>> for Option<Register> {
    fn local_into(self) -> Option<Operand> {
        Some(Operand::Register(self?.to_string()))
    }
}
impl sealed::Into<GAShift> for Shift {
    fn local_into(self) -> GAShift {
        match self {
            Self::Lsl => GAShift::Lsl,
            Self::Lsr => GAShift::Lsr,
            Self::Asr => GAShift::Asr,
            Self::Rrx => GAShift::Rrx,
            Self::Ror => GAShift::Ror,
        }
    }
}

impl Into<Operand> for u32 {
    fn local_into(self) -> Operand {
        Operand::Immediate(DataWord::Word32(self))
    }
}
fn mask_dyn(start: u32, end: u32) -> u32 {
    (1 << (end - start + 1)) - 1
}
