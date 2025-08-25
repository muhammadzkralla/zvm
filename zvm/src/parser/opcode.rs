// Bytecode opcodes (just to run the current class file, not all the opcodes are supported for now)
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    Aload_0 = 0x2A,       // 42
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
            0x2A => Opcode::Aload_0,
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
