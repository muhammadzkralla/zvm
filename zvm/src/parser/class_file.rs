use crate::parser::{
    attribute_info::AttributeInfo, constant_pool_info::CpInfo, field_info::FieldInfo,
    method_info::MethodInfo,
};

/// Hold the parsed contents of a class file bytes in memory
#[derive(Debug, Clone)]
pub struct ClassFile {
    pub magic: u32,
    pub minor: u16,
    pub major: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<CpInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    /// Creates a new `ClassFile` instance with all fields initialized to default values
    pub fn new() -> Self {
        ClassFile {
            magic: 0,
            minor: 0,
            major: 0,
            constant_pool_count: 0,
            constant_pool: Vec::new(),
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interfaces_count: 0,
            interfaces: Vec::new(),
            fields_count: 0,
            fields: Vec::new(),
            methods_count: 0,
            methods: Vec::new(),
            attributes_count: 0,
            attributes: Vec::new(),
        }
    }
}
