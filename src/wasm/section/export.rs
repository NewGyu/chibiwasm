use crate::decoder::WasmModuleBinaryRead;
use anyhow::*;

pub type Content = Vec<Export>;
pub struct Export {
    pub name: Name,
    pub desc: ExportDesc,
}
pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}
pub type FuncIdx = u32;
pub type TableIdx = u32;
pub type MemIdx = u32;
pub type GlobalIdx = u32;
pub type Name = String;

pub fn decode(reader: &mut impl WasmModuleBinaryRead) -> Result<Content> {
    todo!()
}
