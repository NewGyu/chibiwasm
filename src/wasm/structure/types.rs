/// https://webassembly.github.io/spec/core/syntax/types.html#number-types
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

pub enum RefType {
    FuncRef,
    ExternRef,
}
pub struct VecType();

pub enum ValType {
    Number(NumType),
    Ref(RefType),
    Vec(VecType),
}

/// https://webassembly.github.io/spec/core/syntax/types.html#result-types
pub type ResultType = Vec<ValType>;

/// https://webassembly.github.io/spec/core/syntax/types.html#function-types
pub struct FuncType(ResultType, ResultType);
