use crate::{
    parser::{attribute_info, class_file::ClassFile, method_info::MethodInfo, opcode::Opcode},
    vm::{call_stack::CallStack, runtime::RuntimeDataArea, stack_frame::Frame, value::Value},
};

/// The virtual machine
pub struct Vm {
    /// Stores runtime data such as static fields and heap
    runtime_data: RuntimeDataArea,
    /// Parsed class file currently loaded in the VM
    class_file: ClassFile,
    /// The call stack to handle stack method frames execution
    call_stack: CallStack,
}

impl Vm {
    /// Creates a new instance of the virtual machine
    ///
    /// Initializes `runtime_data` and sets `class_file` to default
    pub fn new() -> Self {
        Self {
            runtime_data: RuntimeDataArea::new(),
            class_file: Default::default(),
            call_stack: CallStack::new(Some(1000)),
        }
    }

    /// Sets the class file to be executed by the VM
    pub fn init_class_file(&mut self, class_file: ClassFile) {
        self.class_file = class_file;
    }

    /// Executes the `<clinit>` (class initializer) method of the loaded class file
    pub fn execute_clinit(&mut self) {
        let clinit_method = match self.find_method("<clinit>".to_string()) {
            Some(method) => method,
            None => {
                println!("No <clinit> method found");
                return;
            }
        };

        // I assume that there will be always one attribute and it's the code attribute
        let attribute_info = &clinit_method.attributes[0];
        let info_bytes = &attribute_info.info;

        // Extract code_length (four big-endian bytes) from info_bytes[4..8]
        let code_length =
            u32::from_be_bytes([info_bytes[4], info_bytes[5], info_bytes[6], info_bytes[7]])
                as usize;

        // Extract bytecode from info_bytes[8..8+code_length]
        let bytecode = info_bytes[8..8 + code_length].to_vec();

        // TODO: Change the hardcoded max_locals value and handle env args array
        self.call_stack
            .push_frame("<clinit>".to_string(), bytecode, 10, vec![]);
    }

    pub fn execute_main(&mut self) {
        let main_method = match self.find_method("main".to_string()) {
            Some(method) => method,
            None => {
                println!("No main method found");
                return;
            }
        };

        // I assume that there will be always one attribute and it's the code attribute
        let attribute_info = &main_method.attributes[0];
        let info_bytes = &attribute_info.info;

        // Extract code_length (four big-endian bytes) from info_bytes[4..8]
        let code_length =
            u32::from_be_bytes([info_bytes[4], info_bytes[5], info_bytes[6], info_bytes[7]])
                as usize;

        // Extract bytecode from info_bytes[8..8+code_length]
        let bytecode = info_bytes[8..8 + code_length].to_vec();

        // TODO: Change the hardcoded max_locals value and handle env args array
        self.call_stack
            .push_frame("main".to_string(), bytecode, 10, vec![]);
    }

    /// Runs the virtual machine with the given class file
    pub fn run(&mut self, class_file: ClassFile) {
        println!("Starting JVM execution...\n");

        // Initialize class file
        self.init_class_file(class_file);

        // Execute the main method
        self.execute_main();

        // Execute class static initializer
        self.execute_clinit();

        let size = self.call_stack.size();

        println!("\nCURRENT CALL STACK SIZE? {}", size);

        self.call_stack
            .execute_frames(&self.class_file, &mut self.runtime_data);

        let flag = self.call_stack.is_empty();

        println!("\nIS THE CALL STACK EMPTY NOW? {}", flag);

        println!("\nJVM execution completed.");
    }

    /// Finds the `<clinit>` method from the loaded class file if it exists
    fn find_method(&self, name: String) -> Option<MethodInfo> {
        for method_info in self.class_file.methods.iter() {
            let name_index = method_info.name_index;
            if let Some(method_name) = self.class_file.get_utf8(name_index) {
                if method_name == name {
                    return Some(method_info.clone());
                }
            }
        }

        return None;
    }
}
