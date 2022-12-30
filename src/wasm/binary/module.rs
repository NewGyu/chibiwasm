mod section;

use anyhow::*;

use crate::structure::module::{Func, Module};

use self::section::Sections;

use super::decode::WasmModuleBinaryRead;
use section::ModuleSectionRead;

/// decode binary read to Module
pub fn decode(reader: &mut impl WasmModuleBinaryRead) -> Result<Module> {
    let (version, sections) = (reader.decode_header()?, reader.decode_sections()?);
    Module::try_from((version, sections))
}

// decode header
const MAGIC_NUMBER: &[u8] = b"\0asm";
type Version = u32;
trait ModuleHeaderRead {
    fn decode_header(&mut self) -> Result<Version>;
}

impl<R: WasmModuleBinaryRead> ModuleHeaderRead for R {
    fn decode_header(&mut self) -> Result<Version> {
        let magic = self.read_bytes(4)?;
        if magic.as_slice() != MAGIC_NUMBER {
            bail!("invalid binary magic")
        }

        let version = self.read_u32_le()?;
        if version != 1 {
            bail!("invalid binary version")
        }
        Ok(version)
    }
}

impl TryFrom<(Version, Sections)> for Module {
    type Error = anyhow::Error;

    fn try_from(value: (Version, Sections)) -> Result<Self, Self::Error> {
        let (version, sections) = value;
        let module = Module {
            version,
            types: sections.type_section,
            funcs: try_funcs(sections.function_section, sections.code_section)?,
            exports: sections.export_section,
        };
        Ok(module)
    }
}

fn try_funcs(
    function_section: section::FunctionContent,
    code_section: section::CodeContent,
) -> Result<Vec<Func>> {
    if code_section.len() != function_section.len() {
        bail!("code_section.len() should equal to function_section.len()")
    }
    let funcs: Vec<Func> = code_section
        .into_iter()
        .zip(function_section.into_iter())
        .map(|(code, idx)| Func {
            type_: idx,
            locals: code.locals,
            body: code.expr,
        })
        .collect();
    Ok(funcs)
}

#[cfg(test)]
mod tests {

    use super::ModuleHeaderRead;
    use std::io::Cursor;

    #[allow(dead_code)]
    fn init() {
        //        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn decode_header_test() {
        //Given
        let wat = br#"(module)"#;
        let wasm = wasmer::wat2wasm(wat).unwrap();
        let mut reader = Cursor::new(wasm);
        //When & Then
        let version = reader.decode_header().unwrap();
        assert_eq!(version, 1);
    }
}
