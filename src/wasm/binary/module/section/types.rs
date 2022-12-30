use crate::{binary::decode::ReadableBytes, structure::types::FuncType};
use anyhow::*;

pub type Content = Vec<FuncType>;
pub fn decode(bytes: Vec<u8>) -> Result<Content> {
    let mut reader = bytes.to_wasm_read();
    let num_of_functype = reader.read_u32()? as usize;
    let functypes = Vec::<FuncType>::with_capacity(num_of_functype);
    for _ in 0..num_of_functype {}
    todo!()
}
