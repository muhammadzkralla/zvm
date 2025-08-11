pub struct Buffer {
    bytes: Vec<u8>,
    offset: usize,
}

impl Buffer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Buffer {
            bytes: bytes,
            offset: 0,
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let bytes = &self.bytes;
        let result = bytes[self.offset];

        self.offset += 1;

        result
    }

    pub fn read_u16(&mut self) -> u16 {
        let bytes = &self.bytes;
        let result = u16::from_be_bytes([bytes[self.offset], bytes[self.offset + 1]]);

        self.offset += 2;

        result
    }

    pub fn read_u32(&mut self) -> u32 {
        let bytes = &self.bytes;
        let result = u32::from_be_bytes([
            bytes[self.offset],
            bytes[self.offset + 1],
            bytes[self.offset + 2],
            bytes[self.offset + 3],
        ]);

        self.offset += 4;

        result
    }
}
