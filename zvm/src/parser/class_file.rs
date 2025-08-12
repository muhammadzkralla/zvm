use crate::parser::constant_pool_info::CpInfo;

/// Hold the parsed contents of a class file bytes in memory
#[derive(Debug, Clone)]
pub struct ClassFile {
    pub magic: u32,
    pub minor: u16,
    pub major: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<CpInfo>,
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
        }
    }
}
