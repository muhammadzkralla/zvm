use std::fs::File;
use std::io::Read;

use crate::parser::{buffer::Buffer, class_file::ClassFile, constant_pool_info::CpInfo};

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
        // Will be used to store the bytes read from the class file in memory
        let mut buf = Vec::new();

        // Trying to open the class file and read it, safely of course, because Rust
        let mut file = match File::open("Main.class") {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                std::process::exit(1);
            }
        };

        let _ = match file.read_to_end(&mut buf) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                std::process::exit(1);
            }
        };

        // Create a new `Buffer` object with the stored bytes in memory
        let buffer = Buffer::new(buf);

        Reader {
            buffer: buffer,
            class_file: ClassFile::new(),
        }
    }

    /// Read the bytes from the buffer sequentially and parse them
    /// into the class file instance in memory
    pub fn read(&mut self) {
        self.read_header();
        self.read_cp();
        self.read_flags_and_classes();
    }

    /// Read the header bytes from the buffer (first 8 bytes) and store them in memory
    fn read_header(&mut self) {
        let magic = self.buffer.read_u32().expect("Failed to read magic bytes");
        let minor = self.buffer.read_u16().expect("Failed to read minor bytes");
        let major = self.buffer.read_u16().expect("Failed to read major bytes");

        self.class_file.magic = magic;
        self.class_file.minor = minor;
        self.class_file.major = major;
    }

    /// Read the constant pool bytes from the buffer and store them in memory
    fn read_cp(&mut self) {
        let constant_pool_count = self
            .buffer
            .read_u16()
            .expect("Failed to read the constant pool count bytes");
        self.class_file.constant_pool_count = constant_pool_count;

        self.read_cp_entries();
    }

    /// Read all the constant pool entries from the buffer and store them in memory
    fn read_cp_entries(&mut self) {
        let pool_count = self.class_file.constant_pool_count as usize;

        // Initialize with empty entries
        self.class_file.constant_pool = vec![CpInfo::Empty; pool_count];

        // Constant pool is 1-indexed
        let mut i = 1 as usize;
        while i < pool_count {
            let entry = self.read_single_cp_entry();

            // Store the entry in the constant pool
            // Why we need deep copying here??
            // Because we still want to use the entry variable further in this function
            self.class_file.constant_pool[i] = entry.clone();

            // Long and Double entries take up two slots, so we need to assign the next
            // entry to them as empty and jump to the next entry
            if self.is_double_width_entry(&entry) {
                // Set the next entry as empty and skip one entry
                self.class_file.constant_pool[i + 1] = CpInfo::Empty;
                i += 1;
            }

            i += 1;
        }
    }

    /// Read a single constant pool table entry from the buffer and return it
    fn read_single_cp_entry(&mut self) -> CpInfo {
        let tag = self
            .buffer
            .read_u8()
            .expect("Failed to read constant pool tag");

        match tag {
            1 => self.read_utf8_entry(),
            3 => self.read_integer_entry(),
            4 => self.read_float_entry(),
            5 => self.read_long_entry(),
            6 => self.read_double_entry(),
            7 => self.read_class_entry(),
            8 => self.read_string_entry(),
            9 => self.read_fieldref_entry(),
            10 => self.read_methodref_entry(),
            11 => self.read_interface_methodref_entry(),
            12 => self.read_name_and_type_entry(),
            15 => self.read_method_handle_entry(),
            16 => self.read_method_type_entry(),
            18 => self.read_invoke_dynamic_entry(),
            _ => panic!("Unknown constant pool tag: {}", tag),
        }
    }

    fn read_utf8_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `length` field
        let length = self.buffer.read_u16().expect("Failed to read UTF8 length");

        // Take the `length` bytes
        let bytes = self
            .buffer
            .read_un(length as usize)
            .expect("Failed to read UTF8 bytes");

        // Return entry
        CpInfo::Utf8 { length, bytes }
    }

    fn read_integer_entry(&mut self) -> CpInfo {
        // Take the four bytes of the `bytes` field
        let bytes = self
            .buffer
            .read_u32()
            .expect("Failed to read integer bytes");

        CpInfo::Integer { bytes }
    }

    fn read_float_entry(&mut self) -> CpInfo {
        // Take the four bytes of the `bytes` field
        let bytes = self.buffer.read_u32().expect("Failed to read float bytes");

        CpInfo::Float { bytes }
    }

    fn read_long_entry(&mut self) -> CpInfo {
        // Take the four bytes of the `high_bytes` field
        let high_bytes = self
            .buffer
            .read_u32()
            .expect("Failed to read long high bytes");
        // Take the four bytes of the `low_bytes` field
        let low_bytes = self
            .buffer
            .read_u32()
            .expect("Failed to read long low bytes");

        CpInfo::Long {
            high_bytes,
            low_bytes,
        }
    }

    fn read_double_entry(&mut self) -> CpInfo {
        // Take the four bytes of the `high_bytes` field
        let high_bytes = self
            .buffer
            .read_u32()
            .expect("Failed to read double high bytes");
        // Take the four bytes of the `low_bytes` field
        let low_bytes = self
            .buffer
            .read_u32()
            .expect("Failed to read double low bytes");

        CpInfo::Double {
            high_bytes,
            low_bytes,
        }
    }

    fn read_class_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `name_index` field
        let name_index = self
            .buffer
            .read_u16()
            .expect("Failed to read class name index");

        CpInfo::Class { name_index }
    }

    fn read_string_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `string_index` field
        let string_index = self.buffer.read_u16().expect("Failed to read string index");

        CpInfo::String { string_index }
    }

    fn read_fieldref_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `class_index` field
        let class_index = self
            .buffer
            .read_u16()
            .expect("Failed to read fieldref class index");

        // Take the two bytes of the `name_and_type_index` field
        let name_and_type_index = self
            .buffer
            .read_u16()
            .expect("Failed to read fieldref name_and_type index");

        CpInfo::Fieldref {
            class_index,
            name_and_type_index,
        }
    }

    fn read_methodref_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `class_index` field
        let class_index = self
            .buffer
            .read_u16()
            .expect("Failed to read methodref class index");

        // Take the two bytes of the `name_and_type_index` field
        let name_and_type_index = self
            .buffer
            .read_u16()
            .expect("Failed to read methodref name_and_type index");

        CpInfo::Methodref {
            class_index,
            name_and_type_index,
        }
    }

    fn read_interface_methodref_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `class_index` field
        let class_index = self
            .buffer
            .read_u16()
            .expect("Failed to read interface methodref class index");

        // Take the two bytes of the `name_and_type_index` field
        let name_and_type_index = self
            .buffer
            .read_u16()
            .expect("Failed to read interface methodref name_and_type index");

        CpInfo::InterfaceMethodref {
            class_index,
            name_and_type_index,
        }
    }

    fn read_name_and_type_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `name_index` field
        let name_index = self
            .buffer
            .read_u16()
            .expect("Failed to read name_and_type name index");

        // Take the two bytes of the `descriptor_index` field
        let descriptor_index = self
            .buffer
            .read_u16()
            .expect("Failed to read name_and_type descriptor index");

        CpInfo::NameAndType {
            name_index,
            descriptor_index,
        }
    }

    fn read_method_handle_entry(&mut self) -> CpInfo {
        // Take the byte of the `reference_kind` field
        let reference_kind = self
            .buffer
            .read_u8()
            .expect("Failed to read method handle reference kind");

        // Take the two bytes of the `reference_index` field
        let reference_index = self
            .buffer
            .read_u16()
            .expect("Failed to read method handle reference index");

        CpInfo::MethodHandle {
            reference_kind,
            reference_index,
        }
    }

    fn read_method_type_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `descriptor_index`
        let descriptor_index = self
            .buffer
            .read_u16()
            .expect("Failed to read method type descriptor index");

        CpInfo::MethodType { descriptor_index }
    }

    fn read_invoke_dynamic_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `bootstrap_method_attr_index`
        let bootstrap_method_attr_index = self
            .buffer
            .read_u16()
            .expect("Failed to read invoke dynamic bootstrap method attr index");

        // Take the two bytes of the `name_and_type_index`
        let name_and_type_index = self
            .buffer
            .read_u16()
            .expect("Failed to read invoke dynamic name_and_type index");

        CpInfo::InvokeDynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        }
    }

    fn is_double_width_entry(&self, entry: &CpInfo) -> bool {
        matches!(entry, CpInfo::Long { .. } | CpInfo::Double { .. })
    }

    fn read_flags_and_classes(&mut self) {
        let access_flags = self
            .buffer
            .read_u16()
            .expect("Failed to read access_flags bytes");
        let this_class = self
            .buffer
            .read_u16()
            .expect("Failed to read this_class bytes");
        let super_class = self
            .buffer
            .read_u16()
            .expect("Failed to read super_class bytes");

        self.class_file.access_flags = access_flags;
        self.class_file.this_class = this_class;
        self.class_file.super_class = super_class;
    }

    /// Prints the parsed contents of the class file in console
    pub fn print(&self) {
        let magic = self.class_file.magic;
        let minor = self.class_file.minor;
        let major = self.class_file.major;
        let constant_pool_count = self.class_file.constant_pool_count;

        println!("Magic: 0x{:08X}", magic);
        println!("Minor: 0x{:04X}", minor);
        println!("Major: 0x{:04X}", major);

        println!("Constant Pool Count: {}", constant_pool_count);
        self.print_constant_pool();

        self.print_access_flags();
        println!("This Class: #{}", self.class_file.this_class);
        println!("Super Class: #{}", self.class_file.super_class);
    }

    fn print_constant_pool(&self) {
        println!("\nConstant Pool:");

        for (i, entry) in self.class_file.constant_pool.iter().enumerate() {
            // Constant pool is 1-indexed
            if i == 0 {
                continue;
            }

            match entry {
                CpInfo::Utf8 { length, bytes } => {
                    let string = String::from_utf8_lossy(bytes);
                    println!("  #{}: Utf8 [{}]", i, string);
                }
                CpInfo::Integer { bytes } => {
                    println!("  #{}: Integer [{}]", i, *bytes as i32);
                }
                CpInfo::Float { bytes } => {
                    let float_val = f32::from_bits(*bytes);
                    println!("  #{}: Float [{}]", i, float_val);
                }
                CpInfo::Long {
                    high_bytes,
                    low_bytes,
                } => {
                    // AS SPECIFIED BY THE SPECS:
                    // ((long) high_bytes << 32) + low_bytes
                    let long = ((*high_bytes as u64) << 32) + (*low_bytes as u64);
                    println!("  #{}: Long [{}]", i, long as i64);
                }
                CpInfo::Double {
                    high_bytes,
                    low_bytes,
                } => {
                    // AS SPECIFIED BY THE SPECS:
                    // ((long) high_bytes << 32) + low_bytes
                    let bits = ((*high_bytes as u64) << 32) + (*low_bytes as u64);
                    let double = f64::from_bits(bits);
                    println!("  #{}: Double [{}]", i, double);
                }
                CpInfo::Class { name_index } => {
                    println!("  #{}: Class [name_index=#{}]", i, name_index);
                }
                CpInfo::String { string_index } => {
                    println!("  #{}: String [string_index=#{}]", i, string_index);
                }
                CpInfo::Fieldref {
                    class_index,
                    name_and_type_index,
                } => {
                    println!(
                        "  #{}: Fieldref [class_index=#{}, name_and_type_index=#{}]",
                        i, class_index, name_and_type_index
                    );
                }
                CpInfo::Methodref {
                    class_index,
                    name_and_type_index,
                } => {
                    println!(
                        "  #{}: Methodref [class_index=#{}, name_and_type_index=#{}]",
                        i, class_index, name_and_type_index
                    );
                }
                CpInfo::InterfaceMethodref {
                    class_index,
                    name_and_type_index,
                } => {
                    println!(
                        "  #{}: InterfaceMethodref [class_index=#{}, name_and_type_index=#{}]",
                        i, class_index, name_and_type_index
                    );
                }
                CpInfo::NameAndType {
                    name_index,
                    descriptor_index,
                } => {
                    println!(
                        "  #{}: NameAndType [name_index=#{}, descriptor_index=#{}]",
                        i, name_index, descriptor_index
                    );
                }
                CpInfo::MethodHandle {
                    reference_kind,
                    reference_index,
                } => {
                    println!(
                        "  #{}: MethodHandle [reference_kind={}, reference_index=#{}]",
                        i, reference_kind, reference_index
                    );
                }
                CpInfo::MethodType { descriptor_index } => {
                    println!(
                        "  #{}: MethodType [descriptor_index=#{}]",
                        i, descriptor_index
                    );
                }
                CpInfo::InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                } => {
                    println!(
                        "  #{}: InvokeDynamic [bootstrap_method_attr_index={}, name_and_type_index=#{}]",
                        i, bootstrap_method_attr_index, name_and_type_index
                    );
                }
                CpInfo::Empty => {
                    println!("EMPTY ENTRY!")
                }
            }
        }
    }

    fn print_access_flags(&self) {
        println!("\nAccess Flags: 0x{:04X}", self.class_file.access_flags);

        let flags = self.class_file.access_flags;
        let mut flag_names = Vec::new();

        // Check each access flag bit according to JVM spec
        if flags & 0x0001 != 0 {
            flag_names.push("ACC_PUBLIC");
        }
        if flags & 0x0010 != 0 {
            flag_names.push("ACC_FINAL");
        }
        if flags & 0x0020 != 0 {
            flag_names.push("ACC_SUPER");
        }
        if flags & 0x0200 != 0 {
            flag_names.push("ACC_INTERFACE");
        }
        if flags & 0x0400 != 0 {
            flag_names.push("ACC_ABSTRACT");
        }
        if flags & 0x1000 != 0 {
            flag_names.push("ACC_SYNTHETIC");
        }
        if flags & 0x2000 != 0 {
            flag_names.push("ACC_ANNOTATION");
        }
        if flags & 0x4000 != 0 {
            flag_names.push("ACC_ENUM");
        }
        if flags & 0x8000 != 0 {
            flag_names.push("ACC_MODULE");
        }

        if flag_names.is_empty() {
            println!("  No access flags set");
        } else {
            println!("  Flags: {}", flag_names.join(", "));
        }

        println!()
    }
}
