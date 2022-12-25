use anyhow::Result;
use std::io::{BufRead, Cursor, Read};

///
pub trait Decoder
where
    Self: Sized,
{
    fn decode<R: Read + WasmModuleBinaryRead>(reader: &mut R) -> Result<Self>;
}

/// Extensions for Read to help to parse wasm binary
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

    fn read_u64(&mut self) -> Result<u64> {
        Ok(leb128::read::unsigned(self)?)
    }

    fn read_u32(&mut self) -> Result<u32> {
        self.read_u64().map(|x| x as u32)
    }

    fn read_i64(&mut self) -> Result<i64> {
        Ok(leb128::read::signed(self)?)
    }

    fn read_i32(&mut self) -> Result<i32> {
        self.read_i64().map(|x| x as i32)
    }
}

pub trait ReadableBytes {
    fn to_wasm_read(&self) -> Box<dyn WasmModuleBinaryRead + '_>;
}

impl<R: Read + BufRead> WasmModuleBinaryRead for R {}

impl ReadableBytes for Vec<u8> {
    fn to_wasm_read(&self) -> Box<dyn WasmModuleBinaryRead + '_> {
        Box::new(Cursor::new(self))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::decoder::ReadableBytes;

    use super::WasmModuleBinaryRead;

    #[test]
    fn test_read() {
        //Given
        let test_bytes = b"\0asm".to_vec();
        let mut read = test_bytes.to_wasm_read();
        //When Then
        let b = read.read_byte().unwrap();
        assert_eq!(b, 0x00);

        //When Then
        let b = read.read_bytes(2).unwrap();
        assert_eq!(b, vec![0x61, 0x73]);
        assert!(read.has_next().unwrap());

        let b = read.read_byte().unwrap();
        assert_eq!(b, 0x6d);
        assert!(!read.has_next().unwrap());
    }

    #[test]
    fn test_u32_le() {
        //Given
        let test_bytes = vec![0x01_u8, 0x02, 0x03, 0x04];
        let mut cur = Cursor::new(test_bytes);
        //When
        let x = cur.read_u32_le().unwrap();
        //Then
        assert_eq!(x, 0x04_03_02_01_u32);
    }

    // 344865 => 0x54_32_1 => 0b101_0100_0011_0010_0001
    // 7bit split =>    10101  0000110  0100001
    // Add MSB =>    00010101 10000110 10100001
    // HEX     =>    0x15     0x86     0xA1
    // LE      =>    0xA1     0x86     0x15
    #[test]
    fn test_unsigned_leb() {
        //Given
        let test_bytes = vec![0xA1_u8, 0x86, 0x15];
        let mut cur = Cursor::new(&test_bytes);
        //When
        let x = cur.read_u64().unwrap();
        //Then
        assert_eq!(x, 344865);

        //When
        let mut cur = Cursor::new(&test_bytes);
        let x = cur.read_u32().unwrap();
        //Then
        assert_eq!(x, 344865);
    }

    #[test]
    fn test_signed_leb() {
        //Given
        let mut buf = [0u8; 64];
        {
            let mut buf = &mut buf[..];
            let _ = leb128::write::signed(&mut buf, -512);
        }
        //When
        let mut cur = Cursor::new(&buf);
        let x = cur.read_i64().unwrap();
        //Then
        assert_eq!(x, -512);

        //When
        let mut cur = Cursor::new(&buf);
        let x = cur.read_i32().unwrap();
        //Then
        assert_eq!(x, -512);
    }
}
