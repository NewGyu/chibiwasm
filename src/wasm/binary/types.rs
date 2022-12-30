use crate::structure::types::{NumType, RefType, ValType, VecType};
use anyhow::*;

impl ValType {
    pub fn from_u8(b: u8) -> Result<Self> {
        Ok(match b {
            0x7F => ValType::Number(NumType::I32),
            0x7E => ValType::Number(NumType::I64),
            0x7D => ValType::Number(NumType::F32),
            0x7C => ValType::Number(NumType::F64),
            0x7B => ValType::Vec(VecType()),
            0x70 => ValType::Ref(RefType::FuncRef),
            0x6F => ValType::Ref(RefType::ExternRef),
            _ => bail!("unknown ValType {}", b),
        })
    }
}
