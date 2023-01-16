use super::{
    instructions::Expr,
    types::{FuncType, ValType},
};

/// https://webassembly.github.io/spec/core/syntax/modules.html#syntax-module
#[derive(PartialEq, Eq, Debug)]
pub struct Module {
    pub version: u32,
    pub types: Vec<FuncType>,
    pub funcs: Vec<Func>,
    pub exports: Vec<Export>,
}

/// https://webassembly.github.io/spec/core/syntax/modules.html#indices
pub mod indices {
    pub type TypeIdx = u32;
    pub type FuncIdx = u32;
    pub type TableIdx = u32;
    pub type MemIdx = u32;
    pub type GlobalIdx = u32;
    pub type LocalIdx = u32;
    pub type ElemIdx = u32;
    pub type DataIdx = u32;
    pub type LabelIdx = u32;
}

/// https://webassembly.github.io/spec/core/syntax/modules.html#functions
#[derive(PartialEq, Eq, Debug)]
pub struct Func {
    pub type_: indices::TypeIdx,
    pub locals: Vec<ValType>,
    pub body: Expr,
}

pub type Name = String;

#[derive(PartialEq, Eq, Debug)]
pub struct Export {
    pub name: Name,
    pub desc: ExportDesc,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ExportDesc {
    Func(indices::FuncIdx),
    Table(indices::TableIdx),
    Mem(indices::MemIdx),
    Global(indices::GlobalIdx),
}
