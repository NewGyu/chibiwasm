use super::{
    module::indices::*,
    types::{RefType, ValType},
};

// https://webassembly.github.io/spec/core/syntax/instructions.html
pub type Expr = Vec<Instruction>;
#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    // [Control Instructions](https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions)
    Unreachable,
    Nop,
    Block(BlockType, Vec<Instruction>),
    Loop(BlockType, Vec<Instruction>),
    If(BlockType, Vec<Instruction>, Option<Vec<Instruction>>),
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable(Vec<LabelIdx>, LabelIdx),
    Return,
    Call(FuncIdx),
    CallIndirect(TableIdx, TypeIdx),
    Else,
    End,
    //[Reference Instructions](https://webassembly.github.io/spec/core/binary/instructions.html#reference-instructions)
    RefNull(RefType),
    RefIsNull,
    RefFunc(FuncIdx),
    //Parametric Instructions
    Drop,
    Select(Option<Vec<ValType>>),
    //Variable Instructions
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),
    //Table Instructions
    TableGet(TableIdx),
    TableSet(TableIdx),
    TableInit(ElemIdx, TableIdx),
    TableDrop(ElemIdx),
    TableCopy(TableIdx, TableIdx),
    TableGrow(TableIdx),
    TableSize(TableIdx),
    TableFill(TableIdx),
    //Memory Instructions
    //Numeric Instructions
    I32Const(i32),
    I64Const(i64),
    //F32Const(f32),
    //F64Const(f64),
    I32Sub,
    I32Add,
    I32Mul,
    I32Clz,
    I32Ctz,
    I32DivS,
    I32DivU,
    I32Eq,
    I32Eqz,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeU,
    I32GeS,
    I32Popcnt,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32ShL,
    I32ShrS,
    I32ShrU,
    I32RtoL,
    I32RtoR,
    I32Extend8S,
    I32Extend16S,
    Void,
}

#[derive(PartialEq, Eq, Debug)]
pub enum BlockType {
    Empty,
    TypeIdx(TypeIdx),
    ValType(super::types::ValType),
}
