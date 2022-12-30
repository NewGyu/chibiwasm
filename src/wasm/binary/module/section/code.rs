use crate::{
    binary::decode::*,
    structure::{
        instructions::{Expr, Instruction},
        types::ValType,
    },
};
use anyhow::*;

pub type Content = Vec<Code>;
pub type Code = Func;
pub struct Func {
    pub locals: Vec<ValType>,
    pub expr: Expr,
}

pub fn decode(bytes: Vec<u8>) -> Result<Content> {
    let mut reader = bytes.to_wasm_read();
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
    let mut reader = bytes.to_wasm_read();
    let num_of_locals = reader.read_u32()?;
    let mut locals = Vec::<ValType>::new();
    for _ in 0..num_of_locals {
        let num_of_valtypes = reader.read_u32()?;
        for _ in 0..num_of_valtypes {
            locals.push(ValType::from_u8(reader.read_byte()?)?);
        }
    }
    let mut remainings = Vec::<u8>::new();
    let _ = reader.read_to_end(&mut remainings);
    Ok((locals, remainings))
}

fn decode_expr(bytes: Vec<u8>) -> Result<Expr> {
    let mut reader = bytes.to_wasm_read();
    let mut expr = Vec::<Instruction>::new();
    while reader.has_next()? {
        let b = reader.read_byte()?;
        let inst = match b {
            0x00 => Instruction::Unreachable,
            0x01 => Instruction::Nop,
            _ => bail!("undefined ope code {}", b),
        };
        expr.push(inst);
    }
    Ok(expr)
}
