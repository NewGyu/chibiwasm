use crate::structure::types::{NumType, RefType, ResultType, ValType};
use anyhow::*;

impl TryFrom<u8> for ValType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x7F => ValType::Number(NumType::I32),
            0x7E => ValType::Number(NumType::I64),
            0x7D => ValType::Number(NumType::F32),
            0x7C => ValType::Number(NumType::F64),
            0x7B => ValType::Vec,
            0x70 => ValType::Ref(RefType::FuncRef),
            0x6F => ValType::Ref(RefType::ExternRef),
            _ => bail!("unknown ValType {}", value),
        })
    }
}

impl TryFrom<Vec<u8>> for ResultType {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let val_types: Result<Vec<ValType>> = value.into_iter().map(ValType::try_from).collect();
        Ok(ResultType(val_types?))
    }
}

#[cfg(test)]
mod test {
    use anyhow::*;

    use crate::structure::types::{NumType, RefType, ResultType, ValType};

    #[test]
    fn decode_result_type() -> Result<()> {
        //given
        let bytes = vec![0x7Fu8, 0x7B, 0x6F];
        //when
        let ResultType(x) = ResultType::try_from(bytes)?;
        //then
        assert_eq!(x.len(), 3);
        assert_eq!(x[0], ValType::Number(NumType::I32));
        assert_eq!(x[1], ValType::Vec);
        assert_eq!(x[2], ValType::Ref(RefType::ExternRef));
        Ok(())
    }
}
