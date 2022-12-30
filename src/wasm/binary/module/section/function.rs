use anyhow::*;

use crate::binary::decode::ReadableBytes;

pub type TypeIdx = u32;
pub type Content = Vec<TypeIdx>;
pub fn decode(bytes: Vec<u8>) -> Result<Content> {
    let mut reader = bytes.to_wasm_read();
    let mut func_indicies: Vec<TypeIdx> = vec![];
    let count = reader.read_u32()?;
    for _ in 0..count {
        func_indicies.push(reader.read_u32()?);
    }
    Ok(func_indicies)
}
