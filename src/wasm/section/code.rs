use crate::{
    decoder::WasmModuleBinaryRead,
    grammer::{instruction::Instruction, types::ValueType},
};
use anyhow::*;

pub type Content = Vec<Code>;
pub type Code = Func;
pub struct Func {
    locals: Vec<ValueType>,
    expr: Expr,
}
pub type Expr = Vec<Instruction>;
pub fn decode(reader: &mut impl WasmModuleBinaryRead) -> Result<Content> {
    todo!()
}
