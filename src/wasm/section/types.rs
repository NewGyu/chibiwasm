use crate::{decoder::WasmModuleBinaryRead, grammer::types::FuncType};
use anyhow::*;

pub type Content = Vec<FuncType>;
pub fn decode(reader: &mut impl WasmModuleBinaryRead) -> Result<Content> {
    todo!()
}
