use crate::{parser::class_file::ClassFile, vm::runtime::RuntimeDataArea};

/// The virtual machine
pub struct Vm {
    runtime_data: RuntimeDataArea,
    class_file: ClassFile,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            runtime_data: RuntimeDataArea::new(),
            class_file: Default::default(),
        }
    }

    pub fn init_class_file(&mut self, class_file: ClassFile) {
        self.class_file = class_file;
    }
}
