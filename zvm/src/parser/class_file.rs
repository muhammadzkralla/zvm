use crate::parser::{
    attribute_info::AttributeInfo, constant_pool_info::CpInfo, field_info::FieldInfo,
    method_info::MethodInfo,
};

/// Hold the parsed contents of a class file bytes in memory
#[derive(Debug, Clone, Default)]
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

    /// Retrieves a `UTF-8` string from the constant pool at the given index.
    fn get_utf8(&self, index: u16) -> Option<String> {
        if let Some(CpInfo::Utf8 { bytes, .. }) = self.constant_pool.get(index as usize) {
            std::str::from_utf8(bytes).ok().map(|s| s.to_string())
        } else {
            None
        }
    }

    /// Retrieves the class name corresponding to the given constant pool index.
    fn get_class_name(&self, index: u16) -> Option<String> {
        if let Some(CpInfo::Class { name_index }) = self.constant_pool.get(index as usize) {
            self.get_utf8(*name_index)
        } else {
            None
        }
    }

    /// Retrieves the field name from a `NameAndType` entry in the constant pool.
    fn get_field_or_method_name(&self, index: u16) -> Option<String> {
        if let Some(CpInfo::NameAndType {
            name_index,
            descriptor_index,
        }) = self.constant_pool.get(index as usize)
        {
            self.get_utf8(*name_index)
        } else {
            None
        }
    }

    /// Retrieves the field descriptor (type signature) from a `NameAndType` entry
    /// in the constant pool.
    fn get_field_or_method_descriptor(&self, index: u16) -> Option<String> {
        if let Some(CpInfo::NameAndType {
            name_index,
            descriptor_index,
        }) = self.constant_pool.get(index as usize)
        {
            self.get_utf8(*descriptor_index)
        } else {
            None
        }
    }

    /// Retrieves detailed field information from a `Fieldref` entry in the constant pool.
    pub fn get_field_info(&self, index: u16) -> Option<(String, String, String)> {
        if let Some(CpInfo::Fieldref {
            class_index,
            name_and_type_index,
        }) = self.constant_pool.get(index as usize)
        {
            let class_name = self.get_class_name(*class_index)?;
            let field_name = self.get_field_or_method_name(*name_and_type_index)?;
            let field_descriptor = self.get_field_or_method_descriptor(*name_and_type_index)?;

            Some((class_name, field_name, field_descriptor))
        } else {
            None
        }
    }

    /// Retrieves detailed method information from a `Methodref` entry in the constant pool.
    pub fn get_method_info(&self, index: u16) -> Option<(String, String, String)> {
        if let Some(CpInfo::Methodref {
            class_index,
            name_and_type_index,
        }) = self.constant_pool.get(index as usize)
        {
            let class_name = self.get_class_name(*class_index)?;
            let field_name = self.get_field_or_method_name(*name_and_type_index)?;
            let field_descriptor = self.get_field_or_method_descriptor(*name_and_type_index)?;

            Some((class_name, field_name, field_descriptor))
        } else {
            None
        }
    }

    /// Retrieves a string constant from the constant pool.
    pub fn get_string(&self, index: u16) -> Option<String> {
        if let Some(CpInfo::String { string_index }) = self.constant_pool.get(index as usize) {
            self.get_utf8(*string_index)
        } else {
            None
        }
    }
}
