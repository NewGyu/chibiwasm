use crate::structure::{
    instructions::{
        BlockType,
        Instruction::{self, *},
    },
    types::ValType,
};
use anyhow::*;

use super::decode::WasmModuleBinaryRead;

type FactoryMethod = fn(reader: &mut Box<dyn WasmModuleBinaryRead>) -> Result<Instruction>;

pub fn choose_inst_factory(b: u8) -> Result<FactoryMethod> {
    Ok(match b {
        //Control Instructions
        0x00 => |_| Ok(Unreachable),
        0x01 => |_| Ok(Nop),
        0x04 => |r| {
            /*
            let mut buf:Vec<u8> = vec![];
            r.read_until(0x0B, &mut buf);

            let x = &buf[..];
            x.re
            let blockType = BlockType::try_from(r)?;
            while
            If((blockType, ))
            */
            todo!()
        },
        0x0F => |_| Ok(Return),
        0x10 => |r| Ok(Call(r.read_u32()?)),

        0x20 => |r| Ok(LocalGet(r.read_u32()?)),
        0x6A => |_| Ok(I32Add),
        0x6B => |_| Ok(I32Sub),
        0x6C => |_| Ok(I32Mul),
        0x6D => |_| Ok(I32DivS),
        0x6F => |_| Ok(I32DivU),
        0x41 => |r| Ok(I32Const(r.read_i32()?)),
        0x0B => |_| Ok(End),
        _ => bail!("{:#X} is undefined instruction.", b),
    })
}

impl TryFrom<&mut Box<dyn WasmModuleBinaryRead>> for BlockType {
    type Error = anyhow::Error;

    fn try_from(reader: &mut Box<dyn WasmModuleBinaryRead>) -> Result<Self, Self::Error> {
        let b = reader.read_byte()?;
        if b == 0x40 {
            Ok(BlockType::Empty)
        } else if let Result::Ok(v) = ValType::try_from(b) {
            Ok(BlockType::ValType(v))
        } else {
            //ここ実装難しくね？ 1byte戻してからsigned leb128としてreadする必要があるけど、1byte戻る方法がわからない
            todo!("The case of typeidx is unimplemented")
        }
    }
}
