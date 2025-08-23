use std::fs::File;
use std::io::Read;

use crate::parser::{
    attribute_info::AttributeInfo, buffer::Buffer, class_file::ClassFile,
    constant_pool_info::CpInfo, field_info::FieldInfo, method_info::MethodInfo,
};

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

    /// Reads the bytes from the buffer sequentially and parse them
    /// into the class file instance in memory
    pub fn read(&mut self) {
        self.read_header();
        self.read_cp();
        self.read_flags_and_classes();
        self.read_interfaces();
        self.read_fields();
        self.read_methods();
        self.read_attributes();
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

        println!("Interfaces Count: {}", self.class_file.interfaces_count);
        self.print_interfaces();

        println!("Fields Count: {}", self.class_file.fields_count);
        self.print_fields();

        println!("Methods Count: {}", self.class_file.methods_count);
        self.print_methods();

        println!("Attributes Count: {}", self.class_file.attributes_count);
        self.print_attributes();

        println!("------------------------------------");
        println!("PARSING THE CLASS FILE IS OVER");
        println!("Current Offset Value: 0x{:04X}", self.buffer.offset);
        println!("Bytes Processed: {}", self.buffer.offset);

        self.test_parsing_fields_and_methods();
    }

    /// Included just for testing, will be removed later
    fn test_parsing_fields_and_methods(&self) {
        let str = self
            .class_file
            .get_string(28u16)
            .expect("Failed to read String");
        println!("Parsed String on index #28 from the class file: {}", str);

        let field1 = self
            .class_file
            .get_field_info(7u16)
            .expect("Failed to read field #7");
        println!(
            "Parsed Field on index #7 from the class file: {:#?}",
            field1
        );

        let field2 = self
            .class_file
            .get_field_info(13u16)
            .expect("Failed to read field #13");
        println!(
            "Parsed Field on index #13 from the class file: {:#?}",
            field2
        );

        let field3 = self
            .class_file
            .get_field_info(25u16)
            .expect("Failed to read field #25");
        println!(
            "Parsed Field on index #25 from the class file: {:#?}",
            field3
        );

        let method1 = self
            .class_file
            .get_method_info(1u16)
            .expect("Failed to read method #1");
        println!(
            "Parsed Method on index #1 from the class file: {:#?}",
            method1
        );

        let method2 = self
            .class_file
            .get_method_info(19u16)
            .expect("Failed to read method #19");
        println!(
            "Parsed Method on index #19 from the class file: {:#?}",
            method2
        );

        let method3 = self
            .class_file
            .get_method_info(30u16)
            .expect("Failed to read method #30");
        println!(
            "Parsed Method on index #30 from the class file: {:#?}",
            method3
        );

        let method4 = self
            .class_file
            .get_method_info(33u16)
            .expect("Failed to read method #33");
        println!(
            "Parsed Method on index #33 from the class file: {:#?}",
            method4
        );

        let method5 = self
            .class_file
            .get_method_info(36u16)
            .expect("Failed to read method #36");
        println!(
            "Parsed Method on index #36 from the class file: {:#?}",
            method5
        );
    }

    /// Reads the header bytes from the buffer (first 8 bytes) and store them in memory
    fn read_header(&mut self) {
        let magic = self.buffer.read_u32().expect("Failed to read magic bytes");
        let minor = self.buffer.read_u16().expect("Failed to read minor bytes");
        let major = self.buffer.read_u16().expect("Failed to read major bytes");

        self.class_file.magic = magic;
        self.class_file.minor = minor;
        self.class_file.major = major;
    }

    /// Reads the constant pool bytes from the buffer and store them in memory
    fn read_cp(&mut self) {
        let constant_pool_count = self
            .buffer
            .read_u16()
            .expect("Failed to read the constant pool count bytes");
        self.class_file.constant_pool_count = constant_pool_count;

        self.read_cp_entries();
    }

    /// Reads all the constant pool entries from the buffer and store them in memory
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

    /// Reads a single constant pool table entry from the buffer and return it
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

    /// Reads the CONSTANT_UTF8 entry
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

    /// Reads the CONSTANT_INTEGER entry
    fn read_integer_entry(&mut self) -> CpInfo {
        // Take the four bytes of the `bytes` field
        let bytes = self
            .buffer
            .read_u32()
            .expect("Failed to read integer bytes");

        CpInfo::Integer { bytes }
    }

    /// Reads the CONSTANT_FLOAT entry
    fn read_float_entry(&mut self) -> CpInfo {
        // Take the four bytes of the `bytes` field
        let bytes = self.buffer.read_u32().expect("Failed to read float bytes");

        CpInfo::Float { bytes }
    }

    /// Reads the CONSTANT_LONG entry
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

    /// Reads the CONSTANT_DOUBLE entry
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

    /// Reads the CONSTANT_CLASS entry
    fn read_class_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `name_index` field
        let name_index = self
            .buffer
            .read_u16()
            .expect("Failed to read class name index");

        CpInfo::Class { name_index }
    }

    /// Reads the CONSTANT_STRING entry
    fn read_string_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `string_index` field
        let string_index = self.buffer.read_u16().expect("Failed to read string index");

        CpInfo::String { string_index }
    }

    /// Reads the CONSTANT_FIELDREF entry
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

    /// Reads the CONSTANT_METHODREF entry
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

    /// Reads the CONSTANT_INTERFACEMETHODREF entry
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

    /// Reads the CONSTANT_NAMEANDTYPE entry
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

    /// Reads the CONSTANT_METHODHANDLE entry
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

    /// Reads the CONSTANT_METHODTYPE entry
    fn read_method_type_entry(&mut self) -> CpInfo {
        // Take the two bytes of the `descriptor_index`
        let descriptor_index = self
            .buffer
            .read_u16()
            .expect("Failed to read method type descriptor index");

        CpInfo::MethodType { descriptor_index }
    }

    /// Reads the CONSTANT_INVOKEDYNAMIC entry
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

    /// Checks if the current entry is `Long` or `Double` as they take two entries in the
    /// constant pool table
    fn is_double_width_entry(&self, entry: &CpInfo) -> bool {
        matches!(entry, CpInfo::Long { .. } | CpInfo::Double { .. })
    }

    /// Reads the `access_flags`, `this_class`, and `super_class` bytes from the buffer
    /// and store them in memory
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

    /// Reads the interfaces bytes from the buffer and store them in memory
    fn read_interfaces(&mut self) {
        let interfaces_count = self
            .buffer
            .read_u16()
            .expect("Failed to read interfaces_count bytes");

        self.class_file.interfaces_count = interfaces_count;

        if interfaces_count == 0 {
            return;
        }

        for _ in 0..interfaces_count {
            let current_interface_ref = self
                .buffer
                .read_u16()
                .expect("Failed to read interface reference");
            self.class_file.interfaces.push(current_interface_ref);
        }
    }

    /// Reads the fields bytes from the buffer and store them in memory
    fn read_fields(&mut self) {
        let fields_count = self
            .buffer
            .read_u16()
            .expect("Failed to read fields_count bytes");
        self.class_file.fields_count = fields_count;

        let fields_count = self.class_file.fields_count as usize;

        if fields_count == 0 {
            return;
        }

        for _ in 0..fields_count {
            let field = self.parse_field_info();
            self.class_file.fields.push(field);
        }
    }

    /// parses the `field_info` bytes and return an instance of it to store in memory
    fn parse_field_info(&mut self) -> FieldInfo {
        let access_flags = self
            .buffer
            .read_u16()
            .expect("Failed to read access_flags bytes");
        let name_index = self
            .buffer
            .read_u16()
            .expect("Failed to read name_index bytes");
        let descriptor_index = self
            .buffer
            .read_u16()
            .expect("Failed to read descriptor_index bytes");
        let attributes_count = self
            .buffer
            .read_u16()
            .expect("Failed to read attributes_count bytes");

        let mut attributes = Vec::new();

        for _ in 0..attributes_count {
            let attr = self.parse_attr_info();
            attributes.push(attr);
        }

        FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }

    /// Reads the methods bytes from the buffer and store them in memory
    fn read_methods(&mut self) {
        let methods_count = self
            .buffer
            .read_u16()
            .expect("Failed to read methods_count bytes");
        self.class_file.methods_count = methods_count;

        let methods_count = self.class_file.methods_count as usize;

        for _ in 0..methods_count {
            let field = self.parse_method_info();
            self.class_file.methods.push(field);
        }
    }

    /// parses the `method_info` bytes and return an instance of it to store in memory
    fn parse_method_info(&mut self) -> MethodInfo {
        let access_flags = self
            .buffer
            .read_u16()
            .expect("Failed to read access_flags bytes");
        let name_index = self
            .buffer
            .read_u16()
            .expect("Failed to read name_index bytes");
        let descriptor_index = self
            .buffer
            .read_u16()
            .expect("Failed to read descriptor_index bytes");
        let attributes_count = self
            .buffer
            .read_u16()
            .expect("Failed to read attributes_count bytes");

        let mut attributes = Vec::new();

        for _ in 0..attributes_count {
            let attr = self.parse_attr_info();
            attributes.push(attr);
        }

        MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }

    /// Reads the attributes bytes from the buffer and store them in memory
    fn read_attributes(&mut self) {
        let attributes_count = self
            .buffer
            .read_u16()
            .expect("Failed to read attributes_count bytes");
        self.class_file.attributes_count = attributes_count;

        let attributes_count = self.class_file.attributes_count as usize;

        for _ in 0..attributes_count {
            let attr = self.parse_attr_info();
            self.class_file.attributes.push(attr);
        }
    }

    /// parses the `attribute_info` bytes and return an instance of it to store in memory
    fn parse_attr_info(&mut self) -> AttributeInfo {
        let attribute_name_index = self
            .buffer
            .read_u16()
            .expect("Failed to read attribute_name_index bytes");

        let attribute_length = self
            .buffer
            .read_u32()
            .expect("Failed to read attribute_length bytes");

        let mut info = Vec::new();

        for _ in 0..attribute_length {
            let b = self.buffer.read_u8().expect("Failed to read current bytes");
            info.push(b);
        }

        AttributeInfo {
            attribute_name_index,
            attribute_length,
            info,
        }
    }

    /// Prints the parsed `constant_pool` field of the class file
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

    /// Prints the parsed `access_flags` field of the class file
    fn print_access_flags(&self) {
        let flags = self.class_file.access_flags;
        println!("\nAccess Flags: 0x{:04X}", flags);

        let flag_list = [
            (0x0001, "ACC_PUBLIC"),
            (0x0010, "ACC_FINAL"),
            (0x0020, "ACC_SUPER"),
            (0x0200, "ACC_INTERFACE"),
            (0x0400, "ACC_ABSTRACT"),
            (0x1000, "ACC_SYNTHETIC"),
            (0x2000, "ACC_ANNOTATION"),
            (0x4000, "ACC_ENUM"),
            (0x8000, "ACC_MODULE"),
        ];

        let mut flag_names = Vec::new();

        // Check each access flag bit according to JVM spec
        for (mask, name) in flag_list {
            if flags & mask != 0 {
                flag_names.push(name);
            }
        }

        if flag_names.is_empty() {
            println!("  No access flags set");
        } else {
            println!("  Flags: {}", flag_names.join(", "));
        }

        println!()
    }

    /// Prints the parsed `interfaces` fields of the class file
    fn print_interfaces(&self) {
        if self.class_file.interfaces.is_empty() {
            println!("Interfaces: None");
            return;
        }

        println!("Interfaces:");

        for (i, interface_ref) in self.class_file.interfaces.iter().enumerate() {
            println!("  [{}]: #{}", i, interface_ref);
        }
    }

    /// Prints the parsed `fields` fields of the class file
    fn print_fields(&self) {
        if self.class_file.fields.is_empty() {
            println!("Fields: None");
            return;
        }

        println!("Fields:");
        for (i, field) in self.class_file.fields.iter().enumerate() {
            println!("  [{}]: Access Flags: 0x{:04X}", i, field.access_flags);
            println!("  [{}]: Name: {}", i, field.name_index);
            println!("  [{}]: Descriptor: {}", i, field.descriptor_index);
            println!("  [{}]: Attributes Count: {}", i, field.attributes_count);

            for (j, attr) in field.attributes.iter().enumerate() {
                println!("  Attributes:");
                println!("      [{}]: Name: {}", j, attr.attribute_name_index);
                println!("      [{}]: Length: {}", j, attr.attribute_length);
                println!("      [{}]: Info: {}", j, attr.attribute_length);

                for (k, b) in attr.info.iter().enumerate() {
                    println!("      Info:");
                    println!("          [{}]: Byte: {}", k, b);
                }
            }
        }
    }

    /// Prints the parsed `methods` fields of the class file
    fn print_methods(&self) {
        if self.class_file.methods.is_empty() {
            println!("Methods: None");
            return;
        }

        println!("Methods:");
        for (i, method) in self.class_file.methods.iter().enumerate() {
            println!("  [{}]: Access Flags: 0x{:04X}", i, method.access_flags);
            println!("  [{}]: Name: {}", i, method.name_index);
            println!("  [{}]: Descriptor: {}", i, method.descriptor_index);
            println!("  [{}]: Attributes Count: {}", i, method.attributes_count);

            for (j, attr) in method.attributes.iter().enumerate() {
                println!("  Attributes:");
                println!("      [{}]: Name: {}", j, attr.attribute_name_index);
                println!("      [{}]: Length: {}", j, attr.attribute_length);

                print!("      Info Bytes: ");
                for (k, b) in attr.info.iter().enumerate() {
                    print!("{}, ", b);
                }
                println!();
            }
        }
    }

    /// Prints the parsed `attributes` fields of the class file
    fn print_attributes(&self) {
        if self.class_file.attributes.is_empty() {
            println!("Attributes: None");
            return;
        }

        println!("Attributes:");
        for (i, attr) in self.class_file.attributes.iter().enumerate() {
            println!("      [{}]: Name: {}", i, attr.attribute_name_index);
            println!("      [{}]: Length: {}", i, attr.attribute_length);

            print!("      Info Bytes: ");
            for (j, b) in attr.info.iter().enumerate() {
                print!("{}, ", b);
            }
            println!();
        }
    }
}
