use anyhow::*;

use crate::{
    binary::decode::ReadableBytes,
    structure::module::{Export, ExportDesc},
};

pub type Content = Vec<Export>;

pub fn decode(bytes: Vec<u8>) -> Result<Content> {
    let mut reader = bytes.to_wasm_read();
    let num_of_export = reader.read_u32()? as usize;
    let mut exports = Vec::<Export>::with_capacity(num_of_export);
    for _ in 0..num_of_export {
        let n = reader.read_u32()? as usize;
        let name = String::from_utf8(reader.read_bytes(n)?)?;

        let export_type = reader.read_byte()?;
        let idx = reader.read_u32()?;
        let desc = match export_type {
            0x00 => ExportDesc::Func(idx),
            0x01 => ExportDesc::Table(idx),
            0x02 => ExportDesc::Mem(idx),
            0x03 => ExportDesc::Global(idx),
            _ => bail!("invalid export desc: {:x}", export_type),
        };

        exports.push(Export { name, desc });
    }
    Ok(exports)
}