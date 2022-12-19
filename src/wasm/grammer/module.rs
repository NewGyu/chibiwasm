use anyhow::*;
use std::io::Read;

use super::super::decoder::{Decoder, WasmModuleBinaryRead};
use super::section::SectionID;
use super::{
    section::{Export, FunctionBody, Section},
    types::FuncType,
};
use num::FromPrimitive;

#[derive(Debug, Default)]
pub struct Module {
    pub version: u32,
    pub type_section: Option<Vec<FuncType>>,
    pub function_section: Option<Vec<u32>>,
    pub code_section: Option<Vec<FunctionBody>>,
    pub export_section: Option<Vec<Export>>,
}

const MAGIC_NUMBER: &[u8] = b"\0asm";

impl Module {
    fn decode_header<R: Read + WasmModuleBinaryRead>(reader: &mut R) -> Result<Module> {
        let magic = reader.read_bytes(4)?;
        if magic.as_slice() != MAGIC_NUMBER {
            bail!("invalid binary magic")
        }

        let version = reader.read_u32_le()?;
        if version != 1 {
            bail!("invalid binary version")
        }
        Ok(Module {
            version: 1,
            ..Default::default()
        })
    }

    fn decode_section(reader: &mut impl WasmModuleBinaryRead) -> Result<Option<Section>> {
        if !reader.has_next()? {
            return Ok(None);
        }

        let (id, size) = Module::decode_section_type(reader)?;
        log::debug!("{:?}", (&id, size));
        let bytes = reader.read_bytes(size)?;
        log::debug!("{:?}", bytes);
        Ok(Some(Section::decode(id, bytes)?))
    }

    fn decode_section_type(reader: &mut impl WasmModuleBinaryRead) -> Result<(SectionID, usize)> {
        let id = SectionID::from_u8(reader.read_byte()?).context("unknown section id")?;
        let size = reader.read_u64_leb()? as usize;
        Ok((id, size))
    }

    fn add_section(&mut self, section: Section) {
        match section {
            Section::Type(section) => self.type_section = Some(section),
            Section::Function(section) => self.function_section = Some(section),
            Section::Code(section) => self.code_section = Some(section),
            Section::Export(section) => self.export_section = Some(section),
            _ => (),
        };
    }
}

impl Decoder for Module {
    fn decode<R: Read + WasmModuleBinaryRead>(reader: &mut R) -> Result<Self> {
        let mut module = Module::decode_header(reader)?;
        while let Some(section) = Module::decode_section(reader)? {
            module.add_section(section);
        }
        Ok(module)
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;
    use crate::wasm::grammer::section::SectionID;
    use wasmer::wat2wasm;

    #[allow(dead_code)]
    fn init() {
        //        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn decode_header() {
        //Given
        let wat = br#"(module)"#;
        let wasm = wat2wasm(wat).unwrap();
        let mut reader = Cursor::new(wasm);
        //When & Then
        let m = Module::decode_header(&mut reader).unwrap();
        assert_eq!(m.version, 1);
    }

    #[test]
    fn decode_section_type() {
        //Given
        let wat = br#"(module
            (func $i32.add (param $lhs i32) (param $rhs i32) (result i32)
                local.get $lhs
                local.get $rhs
                i32.add
            )
        )"#;
        let wasm = wat2wasm(wat).unwrap();
        let mut reader = Cursor::new(wasm);
        let _ = Module::decode_header(&mut reader).unwrap();
        //When
        let (sec, size) = Module::decode_section_type(&mut reader).unwrap();
        //Then
        assert_eq!((sec, size), (SectionID::Type, 7));

        //When
        let _ = reader.read_bytes(size as usize);
        let (sec, size) = Module::decode_section_type(&mut reader).unwrap();
        //Then
        assert_eq!((sec, size), (SectionID::Function, 2));

        //When
        let _ = reader.read_bytes(size as usize);
        let (sec, size) = Module::decode_section_type(&mut reader).unwrap();
        //Then
        assert_eq!((sec, size), (SectionID::Code, 9));
    }

    #[test]
    fn decode_section() {
        init();
        //Given
        let wat = br#"(module
            (func $i32.add (param $lhs i32) (param $rhs i32) (result i32)
                local.get $lhs
                local.get $rhs
                i32.add
            )
        )"#;
        let wasm = wat2wasm(wat).unwrap();
        let mut reader = Cursor::new(wasm);
        let _ = Module::decode_header(&mut reader).unwrap();

        //When
        let mut sections: Vec<Section> = vec![];
        while let Some(s) = Module::decode_section(&mut reader).unwrap() {
            sections.push(s);
        }

        //Then
        assert_eq!(sections.len(), 4);
        assert!(matches!(sections[0], Section::Type(_)));
        assert!(matches!(sections[1], Section::Function(_)));
        assert!(matches!(sections[2], Section::Code(_)));
        assert!(matches!(sections[3], Section::Custom));
    }
}
