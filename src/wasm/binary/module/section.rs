use anyhow::*;
use num::FromPrimitive;
use num_derive::FromPrimitive;

use super::super::decode::WasmModuleBinaryRead;
pub use code::Content as CodeContent;
pub use function::Content as FunctionContent;

#[derive(Default)]
pub struct Sections {
    pub type_section: types::Content,
    pub function_section: function::Content,
    pub code_section: code::Content,
    pub export_section: export::Content,
}

#[derive(Debug, FromPrimitive, PartialEq)]
// Refer to : https://webassembly.github.io/spec/core/binary/modules.html#sections
enum SectionID {
    Custom = 0x00,
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

pub trait ModuleSectionRead {
    fn decode_sections(&mut self) -> Result<Sections>;
}
impl<R: WasmModuleBinaryRead> ModuleSectionRead for R {
    fn decode_sections(&mut self) -> Result<Sections> {
        let mut sections: Sections = Default::default();
        while self.has_next()? {
            let (section_id, content) = decode_section_type(self)?;

            match section_id {
                SectionID::Custom => (),
                SectionID::Type => sections.type_section = types::decode(content)?,
                SectionID::Function => sections.function_section = function::decode(content)?,
                SectionID::Export => sections.export_section = export::decode(content)?,
                SectionID::Code => sections.code_section = code::decode(content)?,
                _ => bail!("uninplemented section_id {:?}", section_id),
            };
        }
        Ok(sections)
    }
}

fn decode_section_type(reader: &mut impl WasmModuleBinaryRead) -> Result<(SectionID, Vec<u8>)> {
    let section_id = reader.read_byte()?;
    let length = reader.read_u64()? as usize;
    let section_id = SectionID::from_u8(section_id)
        .ok_or_else(|| anyhow!("unknown section_id {}", section_id))?;
    let content = reader.read_bytes(length)?;
    Ok((section_id, content))
}

mod code;
mod export;
mod function;
mod types;

#[cfg(test)]
mod tests {
    use crate::binary::decode::test_util;

    use super::*;
    use std::io::Cursor;
    use wasmer::wat2wasm;

    #[test]
    fn decode_section_type_test() {
        //Given
        let mut reader = test_util::wasm_reader(
            br#"(module
            (func $i32.add (param $lhs i32) (param $rhs i32) (result i32)
                local.get $lhs
                local.get $rhs
                i32.add
            )
        )"#,
        );
        let _ = reader.read_bytes(8);
        //When
        let (sec, content) = super::decode_section_type(&mut reader).unwrap();
        //Then
        assert_eq!((sec, content.len()), (SectionID::Type, 7));

        //When
        let (sec, content) = decode_section_type(&mut reader).unwrap();
        //Then
        assert_eq!((sec, content.len()), (SectionID::Function, 2));

        //When
        let (sec, content) = decode_section_type(&mut reader).unwrap();
        //Then
        assert_eq!((sec, content.len()), (SectionID::Code, 9));
    }

    #[test]
    fn decode_section() -> Result<()> {
        //Given
        let wat = br#"(module
            (func $i32.add (param $lhs i32) (param $rhs i32) (result i32)
                local.get $lhs
                local.get $rhs
                i32.add
            )
        )"#;
        let wasm = wat2wasm(wat)?;
        let mut reader = Cursor::new(wasm);
        let _ = reader.read_bytes(8);

        //When
        let sections = reader.decode_sections()?;

        //Then
        assert_eq!(sections.type_section.len(), 1);
        assert_eq!(sections.function_section.len(), 1);
        assert_eq!(sections.export_section.len(), 0);
        assert_eq!(sections.code_section.len(), 1);
        Ok(())
    }
}
