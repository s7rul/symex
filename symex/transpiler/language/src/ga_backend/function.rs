//! Defines transpiling rules for the ast
//! [`Functions`](crate::ast::function::Function).

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    ast::{function::*, operand::Operand, operations::BinOp},
    Compile,
    Error,
    TranspilerState,
};

impl Compile for Function {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        Ok(match self {
            // This should not be managed by us
            Self::Ident(i, args) => {
                quote! {#i(#(#args),*)}
            }
            Self::Intrinsic(i) => i.compile(state)?,
        })
    }
}

impl Compile for Intrinsic {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        match self {
            Self::ZeroExtend(z) => z.compile(state),
            Self::SignExtend(s) => s.compile(state),
            Self::Resize(r) => r.compile(state),
            Self::SetNFlag(n) => n.compile(state),
            Self::SetZFlag(z) => z.compile(state),
            Self::LocalAddress(a) => a.compile(state),
            Self::SetVFlag(f) => f.compile(state),
            Self::SetCFlag(f) => f.compile(state),
            Self::SetCFlagRot(f) => f.compile(state),
            Self::Flag(f) => f.compile(state),
            Self::Register(r) => r.compile(state),
            Self::Ror(r) => r.compile(state),
            Self::Sra(s) => s.compile(state),
            Self::Signed(s) => s.compile(state),
        }
    }
}

impl Compile for FunctionCall {
    type Output = TokenStream;

    fn compile(
        &self,
        state: &mut crate::TranspilerState<Self::Output>,
    ) -> Result<Self::Output, Error> {
        let f: TokenStream = self.ident.clone().compile(state)?;
        let args = self.args.clone();

        Ok(quote! {
            #f(#(#args),*)
        })
    }
}

impl Compile for Signed {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let lhs = self.op1.clone();
        let rhs = self.op2.clone();
        let mut op = self.operation.clone();
        op.signed();
        let dst = state.intermediate();
        let operation = BinOp {
            lhs,
            rhs,
            dest: Operand::Ident(dst.clone()),
            op,
        }
        .compile(state)?;
        state.to_insert_above.push(operation);
        let dst = dst.compile(state)?;
        Ok(quote!(
        #dst
        ))
    }
}

impl Compile for LocalAddress {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let name = self.name.clone();
        state.access_str(
            name.clone()
                .into_token_stream()
                .to_string()
                .strip_prefix('\"')
                .unwrap()
                .strip_suffix('\"')
                .unwrap()
                .to_string(),
        );
        let bits = self.bits.clone();

        Ok(quote!(Operand::AddressInLocal(#name.to_owned(),#bits)))
    }
}

impl Compile for Register {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let name = self.name.clone();
        state.access_str(
            name.clone()
                .into_token_stream()
                .to_string()
                .strip_prefix('\"')
                .unwrap()
                .strip_suffix('\"')
                .unwrap()
                .to_string(),
        );
        Ok(quote!(Operand::Register(#name.to_owned())))
    }
}

impl Compile for Flag {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let name = self.name.clone();
        state.access_str(
            name.clone()
                .into_token_stream()
                .to_string()
                .strip_prefix('\"')
                .unwrap()
                .strip_suffix('\"')
                .unwrap()
                .to_string(),
        );
        Ok(quote!(Operand::Flag(#name.to_owned())))
    }
}

impl Compile for Jump {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let operand = self.target.clone().compile(state)?;
        Ok(match self.condtion.clone() {
            Some(condition) => {
                quote!(Operation::ConditionalJump { destination: #operand,condition:#condition.clone() })
            }
            None => {
                quote!(Operation::ConditionalJump { destination: #operand,condition:Condition::None })
            }
        })
    }
}

impl Compile for SetNFlag {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let operand = self.operand.compile(state)?;
        Ok(quote!(Operation::SetNFlag( #operand )))
    }
}

impl Compile for SetZFlag {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let operand = self.operand.compile(state)?;
        Ok(quote!(Operation::SetZFlag (#operand)))
    }
}

impl Compile for SetVFlag {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let operand1 = self.operand1.compile(state)?;
        let operand2 = self.operand2.compile(state)?;
        let carry = self.carry.clone();
        let sub = self.sub.clone();

        Ok(quote!(
        Operation::SetVFlag {
            operand1: #operand1,
            operand2: #operand2,
            carry: #carry,
            sub: #sub
        }))
    }
}

impl Compile for SetCFlagRot {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let operand1 = self.operand1.compile(state)?;
        if self.rotation == Rotation::Ror {
            return Ok(quote!(
            Operation::SetCFlagRor(#operand1)
            ));
        }
        let operand2 = self
            .operand2
            .clone()
            .expect("Parser is broken")
            .compile(state)?;

        Ok(match self.rotation {
            Rotation::Lsl => quote!(
                Operation::SetCFlagShiftLeft{
                    operand:#operand1,
                    shift:#operand2
                }
            ),
            Rotation::Rsl => quote!(
                Operation::SetCFlagSrl{
                    operand:#operand1,
                    shift:#operand2
                }
            ),
            Rotation::Rsa => quote!(
                Operation::SetCFlagSra{
                    operand:#operand1,
                    shift:#operand2
                }
            ),
            Rotation::Ror => quote!(
                Operation::SetCFlagRor(#operand1)
            ),
        })
    }
}

impl Compile for SetCFlag {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let operand1 = self.operand1.compile(state)?;
        let operand2 = self.operand2.compile(state)?;
        let carry = self.carry.clone();
        let sub = self.sub.clone();

        Ok(quote!(
        Operation::SetCFlag {
            operand1: #operand1,
            operand2: #operand2,
            carry: #carry,
            sub: #sub
        }))
    }
}

impl Compile for Resize {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let intermediate = state.intermediate().compile(state)?;
        let operand = self.operand.compile(state)?;
        let bits = self.bits.clone();
        state.to_insert_above.push(quote!(Operation::Resize {
                destination: #intermediate.clone(),
                operand: #operand, bits: #bits.clone()
        }));
        Ok(quote!(#intermediate))
    }
}

impl Compile for SignExtend {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let intermediate = state.intermediate().compile(state)?;
        let operand = self.operand.compile(state)?;
        let bits = self.bits.clone();
        state.to_insert_above.push(quote!(Operation::SignExtend {
                destination: #intermediate.clone(),
                operand: #operand, bits: #bits.clone()
        }));
        Ok(quote!(#intermediate))
    }
}

impl Compile for ZeroExtend {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let intermediate = state.intermediate().compile(state)?;
        let operand = self.operand.compile(state)?;
        let bits = self.bits.clone();
        state.to_insert_above.push(quote!(Operation::ZeroExtend {
                destination: #intermediate.clone(),
                operand: #operand, bits: #bits.clone()
        }));
        Ok(quote!(#intermediate))
    }
}

impl Compile for Sra {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let intermediate = state.intermediate().compile(state)?;
        let operand = self.operand.compile(state)?;
        let shift = self.n.clone();
        state.to_insert_above.push(quote!(Operation::Sra {
                destination: #intermediate.clone(),
                operand: #operand, shift: #shift.clone()
        }));
        Ok(quote!(#intermediate))
    }
}
impl Compile for Ror {
    type Output = TokenStream;

    fn compile(&self, state: &mut TranspilerState<Self::Output>) -> Result<Self::Output, Error> {
        let intermediate = state.intermediate().compile(state)?;
        let operand = self.operand.compile(state)?;
        let shift = self.n.clone();
        state.to_insert_above.push(quote!(Operation::Sror {
                destination: #intermediate.clone(),
                operand: #operand, shift: #shift.clone()
        }));
        Ok(quote!(#intermediate))
    }
}
