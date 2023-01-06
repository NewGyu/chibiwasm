use std::io::Read;

use crate::{
    binary::{decode::*, instructions::choose_inst_factory},
    structure::{
        instructions::{Expr, Instruction},
        types::ValType,
    },
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
        content.push(decode_func(func_bytes)?);
    }
    Ok(content)
}

fn decode_func(bytes: Vec<u8>) -> Result<Func> {
    let (locals, remainings) = decode_locals(bytes)?;
    let expr = decode_expr(remainings)?;
    Ok(Func { locals, expr })
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

fn decode_expr(bytes: Vec<u8>) -> Result<Expr> {
    let mut reader = to_wasmread(bytes);
    let mut expr = Vec::<Instruction>::new();
    while reader.has_next()? {
        let b = reader.read_byte()?;
        let factory_method = choose_inst_factory(b)?;
        let inst = factory_method(&mut reader)?;
        if inst != Instruction::End {
            expr.push(inst);
        }
    }
    Ok(expr)
}

//ベタにWamsModuleBinaryReadにキャストする方法がわからない・・・
fn to_wasmread(bytes: Vec<u8>) -> Box<dyn WasmModuleBinaryRead> {
    Box::new(std::io::Cursor::new(bytes))
}

#[cfg(test)]
mod tests {
    use crate::binary::module::section::code::Func;
    use crate::structure::instructions::Instruction::*;

    #[test]
    fn decode_func() {
        let bytes = vec![0x00u8, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b];
        let f = super::decode_func(bytes).unwrap();
        assert_eq!(
            f,
            Func {
                locals: vec![],
                expr: vec![LocalGet(0), LocalGet(1), I32Add]
            }
        );
    }
}
