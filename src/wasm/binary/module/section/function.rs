use anyhow::*;

use crate::{binary::decode::WasmModuleBinaryRead, structure::module::indices::TypeIdx};

pub type Content = Vec<TypeIdx>;
pub fn decode(bytes: Vec<u8>) -> Result<Content> {
    let mut reader = &bytes[..];
    let mut func_indicies: Vec<TypeIdx> = vec![];
    let count = reader.read_u32()?;
    for _ in 0..count {
        func_indicies.push(reader.read_u32()?);
    }
    Ok(func_indicies)
}

#[cfg(test)]
mod tests {
    use anyhow::*;

    #[test]
    fn test_decode() -> Result<()> {
        //given
        let bytes = vec![0x02u8, 0x00, 0x02];
        //when
        let x = super::decode(bytes)?;
        //then
        assert_eq!(x.len(), 2);
        assert_eq!(x, vec![0x00u32, 0x02]);
        Ok(())
    }
}
