use super::{
    instructions::Expr,
    types::{FuncType, ValType},
};

pub struct Module {
    pub version: u32,
    pub types: Vec<FuncType>,
    pub funcs: Vec<Func>,
    pub exports: Vec<Export>,
}

pub struct Func {
    pub type_: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: Expr,
}

/// Indicies
/// https://webassembly.github.io/spec/core/syntax/modules.html#indices
pub type TypeIdx = u32;
pub type FuncIdx = u32;
pub type TableIdx = u32;
pub type MemIdx = u32;
pub type GlobalIdx = u32;

pub type Name = String;

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
