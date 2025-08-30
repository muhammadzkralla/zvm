// Bytecode opcodes (just to run the current class file, not all the opcodes are supported for now)
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    Iconstm1 = 0x02,      // 2
    Iconst0 = 0x03,       // 3
    Iconst1 = 0x04,       // 4
    Iconst2 = 0x05,       // 5
    Iconst3 = 0x06,       // 6
    Iconst4 = 0x07,       // 7
    Iconst5 = 0x08,       // 8
    Aaload = 0x32,        // 50
    Aload = 0x19,         // 25
    Aload_0 = 0x2A,       // 42
    Aload_1 = 0x2B,       // 43
    Aload_2 = 0x2C,       // 44
    Aload_3 = 0x2D,       // 45
    Invokespecial = 0xB7, // 183
    Return = 0xB1,        // 177
    Getstatic = 0xB2,     // 178
    Ldc = 0x12,           // 18
    Invokevirtual = 0xB6, // 182
    Invokestatic = 0xB8,  // 184
    Bipush = 0x10,        // 16
    Putstatic = 0xB3,     // 179
    Sipush = 0x11,        // 17
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0x32 => Opcode::Aaload,
            0x19 => Opcode::Aload,
            0x2A => Opcode::Aload_0,
            0x2B => Opcode::Aload_1,
            0x2C => Opcode::Aload_2,
            0x2D => Opcode::Aload_3,
            0x02 => Opcode::Iconstm1,
            0x03 => Opcode::Iconst0,
            0x04 => Opcode::Iconst1,
            0x05 => Opcode::Iconst2,
            0x06 => Opcode::Iconst3,
            0x07 => Opcode::Iconst4,
            0x08 => Opcode::Iconst5,
            0xB7 => Opcode::Invokespecial,
            0xB1 => Opcode::Return,
            0xB2 => Opcode::Getstatic,
            0x12 => Opcode::Ldc,
            0xB6 => Opcode::Invokevirtual,
            0xB8 => Opcode::Invokestatic,
            0x10 => Opcode::Bipush,
            0xB3 => Opcode::Putstatic,
            0x11 => Opcode::Sipush,
            _ => panic!("Unknown opcode: 0x{:02X}", byte),
        }
    }
}
