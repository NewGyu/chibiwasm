mod bin_read;
mod decoder;

use anyhow::*;
use std::io::Read;

use super::{
    section::{Export, FunctionBody, Section},
    types::FuncType,
};
pub use decoder::Decoder;

#[derive(Debug, Default)]
pub struct Module {
    pub version: u32,
    pub type_section: Option<Vec<FuncType>>,
    pub function_section: Option<Vec<u32>>,
    pub code_section: Option<Vec<FunctionBody>>,
    pub export_section: Option<Vec<Export>>,
}

impl Module {
    pub fn decode<R: Read>(reader: R) -> Result<Module> {
        let mut decoder = Decoder::new(reader);
        let mut module = decoder.decode_header()?;
        while let Some(section) = decoder.decode_section()? {
            module.add_section(section);
        }
        Ok(module)
    }

    fn add_section(&mut self, section: Section) {
        match section {
            Section::Type(section) => self.type_section = Some(section),
            Section::Function(section) => self.function_section = Some(section),
            Section::Code(section) => self.code_section = Some(section),
            Section::Export(section) => self.export_section = Some(section),
        };
    }
}
