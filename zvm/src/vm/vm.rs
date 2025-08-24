use crate::{
    parser::{attribute_info, class_file::ClassFile, method_info::MethodInfo, opcode::Opcode},
    vm::{runtime::RuntimeDataArea, stack_frame::Frame, value::Value},
};

/// The virtual machine
pub struct Vm {
    /// Stores runtime data such as static fields and heap
    runtime_data: RuntimeDataArea,
    /// Parsed class file currently loaded in the VM
    class_file: ClassFile,
}

impl Vm {
    /// Creates a new instance of the virtual machine
    ///
    /// Initializes `runtime_data` and sets `class_file` to default
    pub fn new() -> Self {
        Self {
            runtime_data: RuntimeDataArea::new(),
            class_file: Default::default(),
        }
    }

    /// Sets the class file to be executed by the VM
    pub fn init_class_file(&mut self, class_file: ClassFile) {
        self.class_file = class_file;
    }

    /// Executes the `<clinit>` (class initializer) method of the loaded class file
    pub fn execute_clinit(&mut self) {
        println!("Executing <clinit> method...");

        // I'm pretty sure it won't panic, as I've already seen the actual bytes and
        // I'm sure the method_info is there
        // TODO: Handle this gracefully
        let method_info = self
            .find_clinit()
            .expect("Failed to get the <clinit> method info");

        // I assume that there will be always one attribute and it's the code attribute
        let attribute_info = method_info.attributes[0].clone();
        let info_bytes = attribute_info.info;

        // <clinit> bytecode: [16, 69, 179, 0, 21, 17, 1, 164, 179, 0, 30, 177]
        // TODO: Introduce a way to extract the bytecode from the info bytes
        // dynamically, as they are not always be those specified indices here
        // for any other program
        let bytecode = info_bytes[8..8 + 12].to_vec();

        // TODO: Introduce a frame call stack to handle all the stack frames
        let mut frame = Frame::new(1);

        // TODO: Delegate this logic to the frame call stack when implemented
        while frame.pc < bytecode.len() {
            let opcode = Opcode::from(bytecode[frame.pc]);
            println!("Executing opcode: {:?} at pc: {}", opcode, frame.pc);

            // TODO: Delegate this logic to the opcode file
            match opcode {
                Opcode::Bipush => {
                    // Push byte value onto stack
                    frame.pc += 1;
                    let value = bytecode[frame.pc] as i32;
                    frame.operand_stack.push(Value::Int(value));

                    println!("  bipush {}", value);
                }
                Opcode::Sipush => {
                    // Push short value onto stack
                    frame.pc += 1;
                    let high = bytecode[frame.pc] as u16;
                    frame.pc += 1;
                    let low = bytecode[frame.pc] as u16;

                    // AS SPECIFIED BY THE SPECS:
                    // (byte1 << 8) | byte2
                    let value = ((high << 8) | low) as i32;
                    frame.operand_stack.push(Value::Int(value));

                    println!("  sipush {}", value);
                }
                Opcode::Putstatic => {
                    // Store static field
                    frame.pc += 1;
                    let index_high = bytecode[frame.pc] as u16;
                    frame.pc += 1;
                    let index_low = bytecode[frame.pc] as u16;

                    // AS SPECIFIED BY THE SPECS:
                    // (indexbyte1 << 8) | indexbyte2
                    let field_ref = (index_high << 8) | index_low;

                    if let Some(value) = frame.operand_stack.pop() {
                        if let Some((class_name, field_name, _)) =
                            self.class_file.get_field_info(field_ref)
                        {
                            self.runtime_data
                                .static_fields
                                .insert(class_name.clone(), value.clone());
                            println!("  putstatic {}.{} = {:?}", class_name, field_name, value);
                        }
                    }
                }
                Opcode::Return => {
                    println!("  return");
                    break;
                }
                _ => {
                    println!("  Unhandled opcode in <clinit>: {:?}", opcode);
                }
            }

            frame.pc += 1;
        }
    }

    /// Runs the virtual machine with the given class file
    pub fn run(&mut self, class_file: ClassFile) {
        println!("Starting JVM execution...\n");

        // Initialize class file
        self.init_class_file(class_file);

        // Execute class static initializer
        self.execute_clinit();

        println!("\nJVM execution completed.");
    }

    /// Finds the `<clinit>` method from the loaded class file if it exists
    fn find_clinit(&self) -> Option<MethodInfo> {
        for method_info in self.class_file.methods.iter() {
            let name_index = method_info.name_index;
            if let Some(method_name) = self.class_file.get_utf8(name_index) {
                if method_name == "<clinit>" {
                    return Some(method_info.clone());
                }
            }
        }

        return None;
    }
}
