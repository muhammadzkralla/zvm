/// A byte buffer that supports sequential reading of a byte array
#[derive(Debug, Clone)]
pub struct Buffer {
    bytes: Vec<u8>,
    offset: usize,
}

impl Buffer {
    /// Creates a new `Buffer` from a vector of bytes, starting at offset 0.
    pub fn new(bytes: Vec<u8>) -> Self {
        Buffer {
            bytes: bytes,
            offset: 0,
        }
    }

    /// Reads the next byte (`u8`) from the buffer.
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

    /// Reads the next byte (`u16`) from the buffer.
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

    /// Reads the next byte (`u32`) from the buffer.
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
        let bytes = &self.bytes;

        if self.offset + n < bytes.len() {
            let mut result = Vec::with_capacity(n);

            for _ in 0..n {
                if let Some(byte) = self.read_u8() {
                    result.push(byte);
                }
            }
            Some(result)
        } else {
            None
        }
    }
}
