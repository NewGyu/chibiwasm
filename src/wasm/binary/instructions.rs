use crate::structure::instructions::Instruction;
use anyhow::*;

use super::decode::WasmModuleBinaryRead;

type FactoryMethod = fn(reader: &mut Box<dyn WasmModuleBinaryRead>) -> Result<Instruction>;

pub fn choose_inst_factory(b: u8) -> Result<FactoryMethod> {
    Ok(match b {
        0x00 => |_| Ok(Instruction::Unreachable),
        0x01 => |_| Ok(Instruction::Nop),
        0x20 => |r| Ok(Instruction::LocalGet(r.read_u32()?)),
        0x6A => |_| Ok(Instruction::I32Add),
        0x0B => |_| Ok(Instruction::End),
        _ => bail!("undefined instruction {:#X}.", b),
    })
}
