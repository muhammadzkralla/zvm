/// A byte buffer that supports sequential reading of a byte array
#[derive(Debug, Clone)]
pub struct Buffer {
    bytes: Vec<u8>,
    pub offset: usize,
}

impl Buffer {
    /// Creates a new `Buffer` from a vector of bytes, starting at offset 0
    pub fn new(bytes: Vec<u8>) -> Self {
        Buffer {
            bytes: bytes,
            offset: 0,
        }
    }

    /// Reads the next one byte (`u8`) from the buffer
    pub fn read_u8(&mut self) -> Option<u8> {
        let bytes = &self.bytes;

        if self.offset < bytes.len() {
            let result = bytes[self.offset];

            self.offset += 1;
            Some(result)
        } else {
            None
        }
    }

    /// Reads the next two bytes (`u16`) from the buffer
    pub fn read_u16(&mut self) -> Option<u16> {
        let bytes = &self.bytes;

        if self.offset + 1 < bytes.len() {
            let result = u16::from_be_bytes([bytes[self.offset], bytes[self.offset + 1]]);

            self.offset += 2;
            Some(result)
        } else {
            None
        }
    }

    /// Reads the next four bytes (`u32`) from the buffer
    pub fn read_u32(&mut self) -> Option<u32> {
        let bytes = &self.bytes;

        if self.offset + 3 < bytes.len() {
            let result = u32::from_be_bytes([
                bytes[self.offset],
                bytes[self.offset + 1],
                bytes[self.offset + 2],
                bytes[self.offset + 3],
            ]);

            self.offset += 4;
            Some(result)
        } else {
            None
        }
    }

    /// Reads the next `n` bytes from the buffer
    pub fn read_un(&mut self, n: usize) -> Option<Vec<u8>> {
        if self.offset + n <= self.bytes.len() {
            let result = self.bytes[self.offset..self.offset + n].to_vec();
            self.offset += n;
            Some(result)
        } else {
            None
        }
    }
}
