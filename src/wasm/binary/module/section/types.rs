use crate::{
    binary::decode::{ReadableBytes, WasmModuleBinaryRead},
    structure::types::{FuncType, ResultType},
};
use anyhow::*;

pub type Content = Vec<FuncType>;
pub fn decode(bytes: Vec<u8>) -> Result<Content> {
    let mut reader = bytes.to_wasm_read();
    let num_of_functype = reader.read_u32()? as usize;
    let mut functypes = Vec::<FuncType>::with_capacity(num_of_functype);
    for _ in 0..num_of_functype {
        let func_type = decode_func_type(&mut reader)?;
        functypes.push(func_type);
    }
    Ok(functypes)
}

fn decode_func_type(reader: &mut impl WasmModuleBinaryRead) -> Result<FuncType> {
    if reader.read_byte()? != 0x60 {
        bail!("functype have to start with 0x60");
    }
    Ok(FuncType(
        decode_result_type(reader)?,
        decode_result_type(reader)?,
    ))
}

fn decode_result_type(reader: &mut impl WasmModuleBinaryRead) -> Result<ResultType> {
    let len = reader.read_u32()? as usize;
    let bytes = reader.read_bytes(len)?;
    ResultType::try_from(bytes)
}
