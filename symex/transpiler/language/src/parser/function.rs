//! Defines parsing rules for the ast
//! [`Functions`](crate::ast::function::Function).
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream, Result},
    Expr,
    Ident,
    Lit,
    LitStr,
    Token,
};

use crate::ast::{function::*, operand::Operand, operations::BinaryOperation};
impl Parse for Function {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        if let Ok(intrinsic) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Intrinsic(intrinsic));
        }
        let ident: Ident = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let inner = content.parse_terminated(Expr::parse, Token![,])?;
        Ok(Self::Ident(ident, inner.into_iter().collect()))
    }
}
impl Parse for Intrinsic {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Signed(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::LocalAddress(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Register(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Flag(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::ZeroExtend(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::SignExtend(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Resize(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::SetNFlag(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::SetCFlag(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::SetCFlagRot(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::SetVFlag(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Ror(el));
        }

        let speculative = input.fork();
        if let Ok(el) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Sra(el));
        }

        Ok(Self::SetZFlag(input.parse()?))
    }
}
impl Parse for FunctionCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Function = input.parse()?;

        let content;
        syn::parenthesized!(content in input);
        let args = content.parse_terminated(Expr::parse, syn::token::Comma)?;
        Ok(Self {
            ident,
            args: args.into_iter().collect(),
        })
    }
}

// =========================================================================
//                          Intrinsics parsing
// =========================================================================

impl Parse for Signed {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        if ident.to_string().to_lowercase().as_str() != "signed" {
            return Err(input.error("Expected: signed"));
        }

        let content;
        syn::parenthesized!(content in input);

        let op1: Operand = content.parse()?;
        let operation: BinaryOperation = content.parse()?;
        let op2: Operand = content.parse()?;

        if !content.is_empty() {
            return Err(content.error("Expected Signed( a {{operation}} b)"));
        }

        Ok(Self {
            op1,
            op2,
            operation,
        })
    }
}
impl Parse for Jump {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "jump" {
            return Err(input.error("jump"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);

        let target: Operand = content.parse()?;
        if content.peek(Token![,]) {
            let _: Token![,] = content.parse()?;
            let conditions = content.parse()?;
            if !content.is_empty() {
                return Err(content.error("Too many arguments"));
            }
            Ok(Self {
                target,
                condtion: Some(conditions),
            })
        } else {
            // if !content.is_empty() {
            //     return Err(content.error("Too many arguments"));
            // }
            Ok(Self {
                target,
                condtion: None,
            })
        }
    }
}

impl Parse for LocalAddress {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "localaddress" {
            return Err(input.error("localaddress"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);

        let name: Lit = match content.peek(Ident) {
            // If name is an identifier, we conver the identifer to a string
            true => {
                let ident: Ident = content.parse()?;
                Lit::Str(LitStr::new(&ident.to_string(), ident.span()))
            }
            false => content.parse()?,
        };

        let _: Token![,] = content.parse()?;
        let bits: Lit = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { name, bits })
    }
}

impl Parse for Register {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        let ident = ident.to_string().to_lowercase();
        if ident.as_str() != "register" && ident.as_str() != "reg" {
            return Err(input.error("expected Reg or Register"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);

        let name: Lit = match content.peek(Ident) {
            // If name is an identifier, we conver the identifer to a string
            true => {
                let ident: Ident = content.parse()?;
                Lit::Str(LitStr::new(&ident.to_string(), ident.span()))
            }
            false => content.parse()?,
        };
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { name })
    }
}

impl Parse for Flag {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "flag" {
            return Err(input.error("flag"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);

        let name: Lit = match content.peek(Ident) {
            // If name is an identifier, we conver the identifer to a string
            true => {
                let ident: Ident = content.parse()?;
                Lit::Str(LitStr::new(&ident.to_string(), ident.span()))
            }
            false => content.parse()?,
        };
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { name })
    }
}

impl Parse for ZeroExtend {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "zeroextend" {
            return Err(input.error("Expected zeroextend"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let n = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self {
            operand: op,
            bits: n,
        })
    }
}

impl Parse for Resize {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "resize" {
            return Err(input.error("Expected resize"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let n = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self {
            operand: op,
            bits: n,
        })
    }
}

impl Parse for SignExtend {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "signextend" {
            return Err(input.error("Expected signextend"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let n = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self {
            operand: op,
            bits: n,
        })
    }
}
impl Parse for ConditionalJump {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "conditionaljump" {
            return Err(input.error("Expected ConditionalJump"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let condition: Ident = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self {
            operand: op,
            condition,
        })
    }
}
impl Parse for SetNFlag {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "setnflag" {
            return Err(input.error("Expected setnflag"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { operand: op })
    }
}
impl Parse for SetZFlag {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "setzflag" {
            return Err(input.error("Expected setzflag"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { operand: op })
    }
}
impl Parse for SetCFlagRot {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "setcflag" {
            return Err(input.error("Expected setcflag"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op1: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;

        if content.peek2(Token![,]) {
            let op2: Operand = content.parse()?;

            let _: Token![,] = content.parse()?;

            let shift = {
                let ident: Ident = content.parse()?;
                match ident.to_string().to_lowercase().as_str() {
                    "lsl" => Rotation::Lsl,
                    "rsl" => Rotation::Rsl,
                    "rsa" => Rotation::Rsa,
                    _ => return Err(content.error("Expected, \"lsl, rsl, rsa\"")),
                }
            };
            return Ok(Self {
                operand1: op1,
                operand2: Some(op2),
                rotation: shift,
            });
        }
        let shift = {
            let ident: Ident = content.parse()?;
            match ident.to_string().to_lowercase().as_str() {
                "ror" => Rotation::Ror,
                _ => return Err(content.error("Expected, \"ror\"")),
            }
        };
        Ok(Self {
            operand1: op1,
            operand2: None,
            rotation: shift,
        })
    }
}
impl Parse for SetCFlag {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "setcflag" {
            return Err(input.error("Expected setcflag"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op1: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let op2: Operand = content.parse()?;

        let _: Token![,] = content.parse()?;

        let (sub, carry) = if content.peek(Ident) {
            // Now we can support ADC, SUB, SBC, ADD
            let ident: Ident = content.parse()?;
            let f = Lit::Bool(syn::LitBool {
                value: false,
                span: ident.span(),
            });
            let t = Lit::Bool(syn::LitBool {
                value: true,
                span: ident.span(),
            });
            let ident = ident.to_string();
            match ident.to_lowercase().as_str() {
                "adc" => (f, t),
                "sub" => (t, f),
                "sbc" => (t.clone(), t),
                "add" => (f.clone(), f),
                _ => return Err(input.error("Previous value must be adc, sub, sbc or add")),
            }
        } else {
            let sub: Lit = content.parse()?;

            let _: Token![,] = content.parse()?;
            let carry: Lit = content.parse()?;
            (sub, carry)
        };

        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self {
            operand1: op1,
            operand2: op2,
            sub,
            carry,
        })
    }
}
impl Parse for SetVFlag {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "setvflag" {
            return Err(input.error("Expected setvflag"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op1: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let op2: Operand = content.parse()?;

        let _: Token![,] = content.parse()?;

        let (sub, carry) = if content.peek(Ident) {
            // Now we can support ADC, SUB, SBC, ADD
            let ident: Ident = content.parse()?;
            let f = Lit::Bool(syn::LitBool {
                value: false,
                span: ident.span(),
            });
            let t = Lit::Bool(syn::LitBool {
                value: true,
                span: ident.span(),
            });
            let ident = ident.to_string();
            match ident.to_lowercase().as_str() {
                "adc" => (f, t),
                "sub" => (t, f),
                "sbc" => (t.clone(), t),
                "add" => (f.clone(), f),
                _ => return Err(input.error("Previous value must be adc, sub, sbc or add")),
            }
        } else {
            let sub: Lit = content.parse()?;

            let _: Token![,] = content.parse()?;
            let carry: Lit = content.parse()?;
            (sub, carry)
        };

        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self {
            operand1: op1,
            operand2: op2,
            sub,
            carry,
        })
    }
}

impl Parse for Ror {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "ror" {
            return Err(input.error("ror"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let n = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { operand: op, n })
    }
}

impl Parse for Sra {
    fn parse(input: ParseStream) -> Result<Self> {
        let speculative = input.fork();
        let ident: Ident = speculative.parse()?;
        if ident.to_string().to_lowercase().as_str() != "sra" {
            return Err(input.error("sra"));
        }
        input.advance_to(&speculative);
        let content;
        syn::parenthesized!(content in input);
        let op: Operand = content.parse()?;
        let _: Token![,] = content.parse()?;
        let n = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("Too many arguments"));
        }
        Ok(Self { operand: op, n })
    }
}
