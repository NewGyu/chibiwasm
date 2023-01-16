use super::module::indices::*;

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
    LocalGet(u32),
    Else,
    End,
    //[Reference Instructions](https://webassembly.github.io/spec/core/binary/instructions.html#reference-instructions)
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
    I32Const(i32),
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
