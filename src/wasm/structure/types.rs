/// https://webassembly.github.io/spec/core/syntax/types.html#number-types
#[derive(PartialEq, Eq, Debug)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(PartialEq, Eq, Debug)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ValType {
    Number(NumType),
    Ref(RefType),
    Vec,
}

/// https://webassembly.github.io/spec/core/syntax/types.html#result-types
pub struct ResultType(pub Vec<ValType>);

/// https://webassembly.github.io/spec/core/syntax/types.html#function-types
pub struct FuncType(pub ResultType, pub ResultType);
