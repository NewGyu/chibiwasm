use super::bin_read::WasmModuleBinaryRead;
use super::Module;
use anyhow::{bail, Context, Result};
use num::FromPrimitive;
use std::io::{BufReader, Read};

use crate::wasm::grammer::section::{Section, SectionID};

const MAGIC_NUMBER: &[u8] = b"\0asm";

pub struct Decoder<R> {
    reader: BufReader<R>,
}

impl<R: Read> Decoder<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }

    pub fn decode_header(&mut self) -> Result<Module> {
        let magic = self.reader.read_bytes(4)?;
        if magic.as_slice() != MAGIC_NUMBER {
            bail!("invalid binary magic")
        }

        let version = self.reader.read_u32_le()?;
        if version != 1 {
            bail!("invalid binary version")
        }
        Ok(Module {
            version,
            ..Module::default()
        })
    }

    pub fn decode_section(&mut self) -> Result<Option<Section>> {
        if !self.reader.has_next()? {
            return Ok(None);
        }

        let (id, size) = self.decode_section_type()?;
        log::debug!("{:?}", (&id, size));
        let bytes = self.reader.read_bytes(size as usize)?;
        log::debug!("{:?}", bytes);
        Ok(Some(Section::decode(id, bytes)?))
    }

    fn decode_section_type(&mut self) -> Result<(SectionID, u32)> {
        let id = SectionID::from_u8(self.reader.read_byte()?).context("unknown section id")?;
        let size: u32 = self.reader.read_u64_leb()? as u32;
        Ok((id, size))
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
        let reader = Cursor::new(wasm);
        //When
        let mut decoder = Decoder::new(reader);
        //Then
        let m = decoder.decode_header().unwrap();
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
        let reader = Cursor::new(wasm);
        //When
        let mut decoder = Decoder::new(reader);
        let _ = decoder.decode_header().unwrap();
        let (sec, size) = decoder.decode_section_type().unwrap();
        //Then
        assert_eq!((sec, size), (SectionID::Type, 7));

        //When
        let _ = decoder.reader.read_bytes(size as usize);
        let (sec, size) = decoder.decode_section_type().unwrap();
        //Then
        assert_eq!((sec, size), (SectionID::Function, 2));

        //When
        let _ = decoder.reader.read_bytes(size as usize);
        let (sec, size) = decoder.decode_section_type().unwrap();
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
        let reader = Cursor::new(wasm);
        let mut decoder = Decoder::new(reader);
        let _ = decoder.decode_header().unwrap();

        //When
        let mut sections: Vec<Section> = vec![];
        while let Some(s) = decoder.decode_section().unwrap() {
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
