use std::io::Read;

use crate::{
    binary::{decode::*, instructions::decode_instructions},
    structure::{instructions::Expr, types::ValType},
};
use anyhow::*;

pub type Content = Vec<Code>;
pub type Code = Func;
#[derive(PartialEq, Eq, Debug)]
pub struct Func {
    pub locals: Vec<ValType>,
    pub expr: Expr,
}

pub fn decode(bytes: Vec<u8>) -> Result<Content> {
    let mut reader = &bytes[..];
    let num_of_funcs = reader.read_u32()? as usize;
    let mut content = Vec::<Func>::with_capacity(num_of_funcs);
    while reader.has_next()? {
        let size = reader.read_u32()? as usize;
        let func_bytes = reader.read_bytes(size)?;
        content.push(Func::try_from(func_bytes)?);
    }
    Ok(content)
}

impl TryFrom<Vec<u8>> for Func {
    type Error = anyhow::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let (locals, remainings) = decode_locals(bytes)?;
        let expr = decode_instructions(remainings)?;
        Ok(Func { locals, expr })
    }
}

fn decode_locals(bytes: Vec<u8>) -> Result<(Vec<ValType>, Vec<u8>)> {
    let mut reader = &bytes[..];
    let num_of_locals = reader.read_u32()?;
    let mut locals = Vec::<ValType>::new();
    for _ in 0..num_of_locals {
        let num_of_valtypes = reader.read_u32()?;
        for _ in 0..num_of_valtypes {
            locals.push(ValType::try_from(reader.read_byte()?)?);
        }
    }
    let mut remainings = Vec::<u8>::new();
    let _ = reader.read_to_end(&mut remainings);
    Ok((locals, remainings))
}

#[cfg(test)]
mod tests {
    use crate::binary::module::section::code::Func;
    use crate::structure::instructions::Instruction::*;

    #[test]
    fn decode_func() {
        let bytes = vec![0x00u8, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b];
        let f = super::Func::try_from(bytes).unwrap();
        assert_eq!(
            f,
            Func {
                locals: vec![],
                expr: vec![LocalGet(0), LocalGet(1), I32Add]
            }
        );
    }
}
