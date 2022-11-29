use super::bin_read::WasmModuleBinaryRead;
use super::Module;
use anyhow::{bail, Result};
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
        let bytes = self.reader.read_bytes(size as usize)?;
        Ok(Some(Section::decode(id, bytes)?))
    }

    fn decode_section_type(&mut self) -> Result<(SectionID, u32)> {
        let id: SectionID = self.reader.read_byte()?.into();
        let size: u32 = self.reader.read_u64_leb()?.try_into()?;
        Ok((id, size))
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;
    use crate::wasm::grammer::section::SectionID;
    use wasmer::wat2wasm;

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
}
