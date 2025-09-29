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
    Bipush = 0x10,        // 16
    Sipush = 0x11,        // 17
    Ldc = 0x12,           // 18
    Ldc2_w = 0x14,        // 20
    Iload = 0x15,         // 21
    Lload = 0x16,         // 22
    Iload0 = 0x1A,        // 26
    Iload1 = 0x1B,        // 27
    Iload2 = 0x1C,        // 28
    Iload3 = 0x1D,        // 29
    Lload0 = 0x1E,        // 30
    Lload1 = 0x1F,        // 31
    Lload2 = 0x20,        // 32
    Lload3 = 0x21,        // 33
    Aload = 0x19,         // 25
    Aload_0 = 0x2A,       // 42
    Aload_1 = 0x2B,       // 43
    Aload_2 = 0x2C,       // 44
    Aload_3 = 0x2D,       // 45
    Aaload = 0x32,        // 50
    Istore = 0x36,        // 54
    Istore_0 = 0x3B,      // 59
    Istore_1 = 0x3C,      // 60
    Istore_2 = 0x3D,      // 61
    Istore_3 = 0x3E,      // 62
    Iadd = 0x60,          // 96
    Isub = 0x64,          // 100
    Imul = 0x68,          // 104
    Idiv = 0x6C,          // 108
    Irem = 0x70,          // 112
    Ineg = 0x74,          // 116
    Ifeq = 0x99,          // 153
    Ifne = 0x9A,          // 154
    Iflt = 0x9B,          // 155
    Ifge = 0x9C,          // 156
    Ifgt = 0x9D,          // 157
    Ifle = 0x9E,          // 158
    If_icmpeq = 0x9F,     // 159
    If_icmpne = 0xA0,     // 160
    If_icmplt = 0xA1,     // 161
    If_icmpge = 0xA2,     // 162
    If_icmpgt = 0xA3,     // 163
    If_icmple = 0xA4,     // 164
    Return = 0xB1,        // 177
    Getstatic = 0xB2,     // 178
    Putstatic = 0xB3,     // 179
    Invokevirtual = 0xB6, // 182
    Invokespecial = 0xB7, // 183
    Invokestatic = 0xB8,  // 184
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0x02 => Opcode::Iconstm1,
            0x03 => Opcode::Iconst0,
            0x04 => Opcode::Iconst1,
            0x05 => Opcode::Iconst2,
            0x06 => Opcode::Iconst3,
            0x07 => Opcode::Iconst4,
            0x08 => Opcode::Iconst5,
            0x10 => Opcode::Bipush,
            0x11 => Opcode::Sipush,
            0x12 => Opcode::Ldc,
            0x14 => Opcode::Ldc2_w,
            0x15 => Opcode::Iload,
            0x16 => Opcode::Lload,
            0x1A => Opcode::Iload0,
            0x1B => Opcode::Iload1,
            0x1C => Opcode::Iload2,
            0x1D => Opcode::Iload3,
            0x1E => Opcode::Lload0,
            0x1F => Opcode::Lload1,
            0x20 => Opcode::Lload2,
            0x21 => Opcode::Lload3,
            0x19 => Opcode::Aload,
            0x2A => Opcode::Aload_0,
            0x2B => Opcode::Aload_1,
            0x2C => Opcode::Aload_2,
            0x2D => Opcode::Aload_3,
            0x32 => Opcode::Aaload,
            0x36 => Opcode::Istore,
            0x3B => Opcode::Istore_0,
            0x3C => Opcode::Istore_1,
            0x3D => Opcode::Istore_2,
            0x3E => Opcode::Istore_3,
            0x60 => Opcode::Iadd,
            0x64 => Opcode::Isub,
            0x68 => Opcode::Imul,
            0x6C => Opcode::Idiv,
            0x70 => Opcode::Irem,
            0x74 => Opcode::Ineg,
            0x99 => Opcode::Ifeq,
            0x9A => Opcode::Ifne,
            0x9B => Opcode::Iflt,
            0x9C => Opcode::Ifge,
            0x9D => Opcode::Ifgt,
            0x9E => Opcode::Ifle,
            0x9F => Opcode::If_icmpeq,
            0xA0 => Opcode::If_icmpne,
            0xA1 => Opcode::If_icmplt,
            0xA2 => Opcode::If_icmpge,
            0xA3 => Opcode::If_icmpgt,
            0xA4 => Opcode::If_icmple,
            0xB1 => Opcode::Return,
            0xB2 => Opcode::Getstatic,
            0xB3 => Opcode::Putstatic,
            0xB6 => Opcode::Invokevirtual,
            0xB7 => Opcode::Invokespecial,
            0xB8 => Opcode::Invokestatic,
            _ => panic!("Unknown opcode: 0x{:02X}", byte),
        }
    }
}
