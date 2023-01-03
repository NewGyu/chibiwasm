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
            funcs: try_merge_to_funcs(sections.function_section, sections.code_section)?,
            exports: sections.export_section,
        };
        Ok(module)
    }
}

fn try_merge_to_funcs(
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
    use crate::binary::decode::test_util;

    use anyhow::*;

    #[allow(dead_code)]
    fn init() {
        //        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn decode_header_test() {
        use super::ModuleHeaderRead;
        //Given
        let mut reader = test_util::wasm_reader(br#"(module)"#);
        //When & Then
        let version = reader.decode_header().unwrap();
        assert_eq!(version, 1);
    }

    #[test]
    fn decode() -> Result<()> {
        use crate::structure::{
            instructions::Instruction,
            module::{Export, Func},
            types::{FuncType, NumType, ResultType, ValType},
        };
        //Given
        let wat = br#"(module
            (func $i32.add (param $lhs i32) (param $rhs i32) (result i32)
                local.get $lhs
                local.get $rhs
                i32.add
            )
        )"#;
        let mut reader = test_util::wasm_reader(wat);
        //when
        let module = super::decode(&mut reader)?;
        //then
        assert_eq!(module.exports, Vec::<Export>::new());
        assert_eq!(
            module.types,
            vec![FuncType(
                ResultType(vec![
                    ValType::Number(NumType::I32),
                    ValType::Number(NumType::I32)
                ]),
                ResultType(vec![ValType::Number(NumType::I32)])
            )]
        );
        assert_eq!(
            module.funcs,
            vec![Func {
                type_: 0,
                locals: vec![],
                body: vec![
                    Instruction::LocalGet(0),
                    Instruction::LocalGet(1),
                    Instruction::I32Add
                ]
            }]
        );

        Ok(())
    }
}
