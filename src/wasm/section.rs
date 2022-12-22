use std::io::{Cursor, Read};

use anyhow::*;
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::decoder::{Decoder, WasmModuleBinaryRead};

/// Refer to : https://webassembly.github.io/spec/core/binary/modules.html#sections
#[derive(Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum SectionID {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
    DataCount,
}

pub enum Section {
    Type(types::Content),
    Function(function::Content),
    Code(code::Content),
    Export(export::Content),
    Custom,
}

impl Decoder for Section {
    fn decode<R: Read + WasmModuleBinaryRead>(reader: &mut R) -> Result<Self> {
        let (id, content) = decode_section_type(reader)?;
        let mut reader = Cursor::new(content);
        let sec = match id {
            SectionID::Type => Section::Type(types::decode(&mut reader)?),
            SectionID::Function => Section::Function(function::decode(&mut reader)?),
            SectionID::Code => Section::Code(code::decode(&mut reader)?),
            SectionID::Export => Section::Export(export::decode(&mut reader)?),
            _ => bail!(format!("The section id {:?} is unimplemented.", id)),
        };
        Ok(sec)
    }
}

fn decode_section_type(reader: &mut impl WasmModuleBinaryRead) -> Result<(SectionID, Vec<u8>)> {
    let id = reader.read_byte()?;
    let id =
        SectionID::from_u8(reader.read_byte()?).context(format!("unknown section id {}", id))?;
    let size = reader.read_u64_leb()? as usize;
    let content = reader.read_bytes(size)?;
    Ok((id, content))
}

mod code;
mod export;
mod function;
mod types;
