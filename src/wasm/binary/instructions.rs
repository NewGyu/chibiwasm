use std::io::Cursor;

use crate::structure::{
    instructions::{
        BlockType,
        Instruction::{self, *},
    },
    types::ValType,
};
use anyhow::*;

use super::decode::WasmModuleBinaryRead;

pub struct Instructions(Vec<Instruction>);
impl TryFrom<Vec<u8>> for Instructions {
    type Error = anyhow::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self> {
        let mut r: Box<dyn WasmModuleBinaryRead> = Box::new(Cursor::new(bytes));
        let mut insts = Instructions(Vec::<Instruction>::new());
        while r.has_next()? {
            let b = r.read_byte()?;
            let factory_method = choose_inst_factory(b)?;
            let inst = factory_method(&mut r)?;
            if inst == Instruction::End {
                break;
            }
            insts.0.push(inst);
        }
        Ok(insts)
    }
}

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

mod block {
    use crate::{
        binary::decode::WasmModuleBinaryRead,
        structure::{
            instructions::{BlockType, Instruction},
            types::ValType,
        },
    };
    use anyhow::*;

    struct Block {
        block_type: BlockType,
        first: Vec<Instruction>,
        second: Option<Vec<Instruction>>,
    }

    impl TryFrom<&mut Box<dyn WasmModuleBinaryRead>> for Block {
        type Error = anyhow::Error;

        fn try_from(reader: &mut Box<dyn WasmModuleBinaryRead>) -> Result<Self> {
            let (first, second) = reader.read_and_split_else()?;
            let block_type = BlockType::try_from(&first[..])?;
            let first = (&first[block_type.len()..]).to_vec();
            let first = super::Instructions::try_from(first)?;
            let second = second.map(|b| super::Instructions::try_from(b)?);
            /*
            first.iter().map(|b| {

            })
            */
            todo!()
        }
    }

    trait BlockInstRead: WasmModuleBinaryRead {
        /// read "^(...)0x0B"
        fn read_until_end_marker(&mut self) -> Result<Vec<u8>> {
            let mut buf = Vec::<u8>::new();
            let _ = self.read_until(0x0B, &mut buf)?;
            let _ = buf.remove(buf.len() - 1);
            Ok(buf)
        }

        /// read "^(...)0x05(...)0x0B"
        fn read_and_split_else(&mut self) -> Result<(Vec<u8>, Option<Vec<u8>>)> {
            let bytes = self.read_until_end_marker()?;
            let mut iter = bytes.split(|b| b == &0x05);
            Ok((
                iter.next()
                    .context("First block must not be empty.")?
                    .to_vec(),
                iter.next().map(|val| val.to_vec()),
            ))
        }
    }
    impl<R: WasmModuleBinaryRead> BlockInstRead for R {}

    impl TryFrom<&[u8]> for BlockType {
        type Error = anyhow::Error;

        fn try_from(mut bytes: &[u8]) -> Result<Self> {
            let b = bytes[0];
            if b == 0x40 {
                Ok(BlockType::Empty)
            } else if let Result::Ok(v) = ValType::try_from(b) {
                Ok(BlockType::ValType(v))
            } else {
                Ok(BlockType::TypeIdx(bytes.read_u32()?))
            }
        }
    }

    impl BlockType {
        pub fn len(&self) -> usize {
            match self {
                BlockType::Empty => 1,
                BlockType::ValType(_) => 1,
                BlockType::TypeIdx(n) => num_of_leb128_bytes(*n),
            }
        }
    }

    fn num_of_leb128_bytes(mut n: u32) -> usize {
        if n == 0 {
            1
        } else {
            let mut i = 0;
            while n > 0 {
                n /= 128; //means 2^7
                i += 1;
            }
            i
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::structure::{
            instructions::BlockType,
            types::{NumType, ValType},
        };

        use super::{num_of_leb128_bytes, BlockInstRead};

        #[test]
        fn block_type_try_from() {
            let bytes = vec![0x40u8, 0x01, 0x02];
            assert_eq!(BlockType::try_from(&bytes[..]).unwrap(), BlockType::Empty);

            let bytes = vec![0x7Du8, 0x01, 0x02];
            assert_eq!(
                BlockType::try_from(&bytes[..]).unwrap(),
                BlockType::ValType(ValType::Number(NumType::F32))
            );

            let bytes = vec![0xA1_u8, 0x86, 0x15];
            assert_eq!(
                BlockType::try_from(&bytes[..]).unwrap(),
                BlockType::TypeIdx(344865)
            );
        }

        #[test]
        fn block_inst_read() {
            //Given
            let bytes = vec![0xA1_u8, 0x86, 0x15, 0x05, 0x01, 0x02, 0x0b];
            //When
            let (first, second) = (&bytes[..]).read_and_split_else().unwrap();
            //Then
            assert_eq!(first, vec![0xA1, 0x86, 0x15]);
            assert_eq!(second, Some(vec![0x01, 0x02]));

            //Given
            let bytes = vec![0xA1_u8, 0x86, 0x15, 0x0b];
            //When
            let (first, second) = (&bytes[..]).read_and_split_else().unwrap();
            //Then
            assert_eq!(first, vec![0xA1, 0x86, 0x15]);
            assert_eq!(second, None);
        }

        #[test]
        fn calc_num_of_leb128_byte() {
            assert_eq!(num_of_leb128_bytes(0), 1);
            assert_eq!(num_of_leb128_bytes(1), 1);
            assert_eq!(num_of_leb128_bytes(127), 1);
            assert_eq!(num_of_leb128_bytes(128), 2);
            assert_eq!(num_of_leb128_bytes(512), 2);
            assert_eq!(num_of_leb128_bytes(16384), 3);
        }

        #[test]
        fn test() {
            let v = vec![1, 2, 3, 4];
            assert_eq!(&v[2..], &[2, 3, 4]);
        }
    }
}
