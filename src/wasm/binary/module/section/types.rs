use crate::{
    binary::decode::{ReadableBytes, WasmModuleBinaryRead},
    structure::types::{FuncType, ResultType},
};
use anyhow::*;

pub type Content = Vec<FuncType>;
pub fn decode(bytes: Vec<u8>) -> Result<Content> {
    let mut reader = bytes.to_wasm_read();
    let num_of_functype = reader.read_u32()? as usize;
    let mut functypes = Vec::<FuncType>::with_capacity(num_of_functype);
    for _ in 0..num_of_functype {
        let func_type = decode_func_type(&mut reader)?;
        functypes.push(func_type);
    }
    Ok(functypes)
}

fn decode_func_type(reader: &mut impl WasmModuleBinaryRead) -> Result<FuncType> {
    if reader.read_byte()? != 0x60 {
        bail!("functype have to start with 0x60");
    }
    Ok(FuncType(
        decode_result_type(reader)?,
        decode_result_type(reader)?,
    ))
}

fn decode_result_type(reader: &mut impl WasmModuleBinaryRead) -> Result<ResultType> {
    let len = reader.read_u32()? as usize;
    let bytes = reader.read_bytes(len)?;
    ResultType::try_from(bytes)
}

#[cfg(test)]
mod tests {
    use anyhow::*;

    use crate::structure::types::{FuncType, NumType, ResultType, ValType};

    #[test]
    fn test() -> Result<()> {
        //given
        // See: https://qiita.com/kgtkr/items/f4b3e2d83c7067f3cfcb#%E3%83%90%E3%82%A4%E3%83%8A%E3%83%AA%E3%82%92%E8%AA%AD%E3%82%93%E3%81%A7%E3%81%BF%E3%82%88%E3%81%86
        let bytes = vec![0x01u8, 0x60, 0x02, 0x7f, 0x7C, 0x01, 0x7b];
        //when
        let content = super::decode(bytes)?;
        //then
        assert_eq!(content.len(), 1);
        assert_eq!(
            content[0],
            FuncType(
                ResultType(vec![
                    ValType::Number(NumType::I32),
                    ValType::Number(NumType::F64)
                ]),
                ResultType(vec![ValType::Vec])
            )
        );

        Ok(())
    }
}
