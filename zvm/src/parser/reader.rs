use std::{fs::File, io::Read};

use crate::parser::{buffer::Buffer, class_file::ClassFile};

/// A `Reader` is responsible for reading the bytes of the class file
/// into a `Buffer` and parsing its contents into a `ClassFile` object
#[derive(Debug, Clone)]
pub struct Reader {
    buffer: Buffer,
    class_file: ClassFile,
}

impl Reader {
    /// Creates a new `Reader` instance by loading the `Main.class` file from disk
    /// and initializing a `Buffer` and an empty `ClassFile`
    pub fn new() -> Self {
        let mut buf = Vec::new();

        let mut file = File::open("Main.class").unwrap();
        let _ = file.read_to_end(&mut buf).unwrap();

        let buffer = Buffer::new(buf);

        Reader {
            buffer: buffer,
            class_file: ClassFile::new(),
        }
    }

    /// Read the bytes from the class file sequentially and parse them
    /// into the class file
    pub fn read(&mut self) {
        let magic = self.buffer.read_u32().unwrap();
        let minor = self.buffer.read_u16().unwrap();
        let major = self.buffer.read_u16().unwrap();

        self.class_file.magic = magic;
        self.class_file.minor = minor;
        self.class_file.major = major;
    }

    /// Prints the parsed contents of the class file in console
    pub fn print(self) {
        let magic = self.class_file.magic;
        let minor = self.class_file.minor;
        let major = self.class_file.major;

        println!("Magic: 0x{:04X}", magic);
        println!("Minor: 0x{:02X}", minor);
        println!("Major: 0x{:02X}", major);
    }
}
