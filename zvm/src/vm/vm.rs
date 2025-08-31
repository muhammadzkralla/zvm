use crate::{
    parser::class_file::ClassFile,
    vm::{call_stack::CallStack, runtime::RuntimeDataArea, value::Value},
};

// Debug logging macro - controlled by feature flag
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug-logging")]
        println!($($arg)*);
    };
}

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
        let clinit_method = match self.class_file.find_method("<clinit>") {
            Some(method) => method,
            None => {
                println!("No <clinit> method found");
                return;
            }
        };

        // I assume that there will be always one attribute and it's the code attribute
        let attribute_info = &clinit_method.attributes[0];
        let info_bytes = &attribute_info.info;

        // Extract max_locals (two big-endian bytes) from info_bytes[2..3]
        let max_locals = u16::from_be_bytes([info_bytes[2], info_bytes[3]]);

        // Extract code_length (four big-endian bytes) from info_bytes[4..8]
        let code_length =
            u32::from_be_bytes([info_bytes[4], info_bytes[5], info_bytes[6], info_bytes[7]])
                as usize;

        // Extract bytecode from info_bytes[8..8+code_length]
        let bytecode = info_bytes[8..8 + code_length].to_vec();

        // TODO: Change the hardcoded max_locals value and handle env args array
        self.call_stack.push_frame(
            "<clinit>".to_string(),
            bytecode,
            max_locals as usize,
            vec![],
        );
    }

    pub fn execute_main(&mut self, args: Vec<String>) {
        let main_method = match self.class_file.find_method("main") {
            Some(method) => method,
            None => {
                println!("No main method found");
                return;
            }
        };

        // I assume that there will be always one attribute and it's the code attribute
        let attribute_info = &main_method.attributes[0];
        let info_bytes = &attribute_info.info;

        // Extract max_locals (two big-endian bytes) from info_bytes[2..3]
        let max_locals = u16::from_be_bytes([info_bytes[2], info_bytes[3]]);

        // Extract code_length (four big-endian bytes) from info_bytes[4..8]
        let code_length =
            u32::from_be_bytes([info_bytes[4], info_bytes[5], info_bytes[6], info_bytes[7]])
                as usize;

        // Extract bytecode from info_bytes[8..8+code_length]
        let bytecode = info_bytes[8..8 + code_length].to_vec();

        let mut env_args = Vec::new();
        let mut array_values = Vec::new();

        for arg in args.iter() {
            let value = Value::Object(arg.clone());
            array_values.push(value);
        }

        let array = Value::Array(array_values);
        env_args.push(array);

        self.call_stack
            .push_frame("main".to_string(), bytecode, max_locals as usize, env_args);
    }

    /// Runs the virtual machine with the given class file
    pub fn run(&mut self, class_file: ClassFile, args: Vec<String>) {
        debug_log!("Starting JVM execution...\n");

        // Initialize class file
        self.init_class_file(class_file);

        // Execute the main method
        self.execute_main(args);

        // Execute class static initializer
        //TODO: <clinit> execution should not be pushed to the call stack and preprocessed
        self.execute_clinit();

        let size = self.call_stack.size();

        debug_log!("\nCURRENT CALL STACK SIZE? {}", size);

        self.call_stack
            .execute_frames(&self.class_file, &mut self.runtime_data);

        let flag = self.call_stack.is_empty();

        debug_log!("\nIS THE CALL STACK EMPTY NOW? {}", flag);

        debug_log!("\nJVM execution completed.");
    }
}
