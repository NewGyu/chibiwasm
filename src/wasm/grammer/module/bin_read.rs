use anyhow::*;
use std::io::{BufRead, Read};

pub trait WasmModuleBinaryRead: Read + BufRead {
    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn has_next(&mut self) -> Result<bool> {
        Ok(self.fill_buf().map(|b| !b.is_empty())?)
    }

    fn read_u32_le(&mut self) -> Result<u32> {
        let b = self.read_bytes(4)?;
        Ok(u32::from_le_bytes(b.as_slice().try_into()?))
    }

    fn read_u64_leb(&mut self) -> Result<u64> {
        Ok(leb128::read::unsigned(self)?)
    }
}

impl<R: Read + BufRead> WasmModuleBinaryRead for R {}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::WasmModuleBinaryRead;

    #[test]
    fn test_read() {
        //Given
        let test_bytes = b"\0asm";
        let mut cur = Cursor::new(test_bytes);
        //When Then
        let b = cur.read_byte().unwrap();
        assert_eq!(b, 0x00);

        //When Then
        let b = cur.read_bytes(2).unwrap();
        assert_eq!(b, vec![0x61, 0x73]);
        assert!(cur.has_next().unwrap());

        let b = cur.read_byte().unwrap();
        assert_eq!(b, 0x6d);
        assert!(!cur.has_next().unwrap());
    }

    #[test]
    fn test_u32() {
        //Given
        let test_bytes = vec![0x01_u8, 0x02, 0x03, 0x04];
        let mut cur = Cursor::new(test_bytes);
        //When
        let x = cur.read_u32_le().unwrap();
        //Then
        assert_eq!(x, 0x04030201_u32);
    }
}
