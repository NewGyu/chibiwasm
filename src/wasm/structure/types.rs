/// https://webassembly.github.io/spec/core/syntax/types.html#number-types
#[derive(PartialEq, Eq, Debug)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

/// https://webassembly.github.io/spec/core/syntax/types.html#reference-types
#[derive(PartialEq, Eq, Debug)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

/// https://webassembly.github.io/spec/core/syntax/types.html#value-types
#[derive(PartialEq, Eq, Debug)]
pub enum ValType {
    Number(NumType),
    Ref(RefType),
    /// https://webassembly.github.io/spec/core/syntax/types.html#vector-types
    Vec,
}

/// https://webassembly.github.io/spec/core/syntax/types.html#result-types
#[derive(PartialEq, Eq, Debug)]
pub struct ResultType(pub Vec<ValType>);

/// https://webassembly.github.io/spec/core/syntax/types.html#function-types
#[derive(PartialEq, Eq, Debug)]
pub struct FuncType(pub ResultType, pub ResultType);

// https://webassembly.github.io/spec/core/syntax/types.html#limits
// https://webassembly.github.io/spec/core/syntax/types.html#memory-types
// https://webassembly.github.io/spec/core/syntax/types.html#table-types
// https://webassembly.github.io/spec/core/syntax/types.html#global-types
// https://webassembly.github.io/spec/core/syntax/types.html#external-types
