use crate::decoder::WasmModuleBinaryRead;
use anyhow::*;

pub type TypeIdx = u32;
pub type Content = Vec<TypeIdx>;
pub fn decode(reader: &mut impl WasmModuleBinaryRead) -> Result<Content> {
    todo!()
}
