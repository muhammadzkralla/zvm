use crate::{
    parser::{class_file::ClassFile, opcode::Opcode},
    vm::{
        local::LocalVariables, operand_stack::OperandStack, runtime::RuntimeDataArea, value::Value,
    },
};

/// Method execution stack call frame
pub struct Frame {
    pub method_name: Option<String>,
    pub operand_stack: OperandStack,
    pub local_variables: LocalVariables,
    pub pc: usize,
    pub bytecode: Vec<u8>,
}

impl Frame {
    pub fn new(method_name: Option<String>, max_locals: usize, bytecode: Vec<u8>) -> Self {
        Self {
            method_name: Some(method_name.unwrap_or("".to_string())),
            operand_stack: OperandStack::new(),
            local_variables: LocalVariables::new(max_locals),
            pc: 0,
            bytecode: bytecode,
        }
    }

    pub fn execute_frame(
        &mut self,
        class_file: &ClassFile,
        runtime_data_area: &mut RuntimeDataArea,
    ) {
        let name = self.method_name.clone().expect("Failed to get method name");

        println!("\nEXECUTING FRAME: {}", name);

        let mut current_pc = self.pc;
        let bytecode = &self.bytecode;

        while current_pc < bytecode.len() {
            let opcode = Opcode::from(bytecode[current_pc]);
            println!("Executing opcode: {:?} at pc: {}", opcode, current_pc);

            match opcode {
                Opcode::Bipush => {
                    // Push byte value onto stack
                    current_pc += 1;
                    let value = bytecode[current_pc] as i32;
                    self.operand_stack.push(Value::Int(value));

                    println!("  bipush {}", value);
                }
                Opcode::Sipush => {
                    // Push short value onto stack
                    current_pc += 1;
                    let high = bytecode[current_pc] as u16;
                    current_pc += 1;
                    let low = bytecode[current_pc] as u16;

                    // AS SPECIFIED BY THE SPECS:
                    // (byte1 << 8) | byte2
                    let value = ((high << 8) | low) as i32;
                    self.operand_stack.push(Value::Int(value));

                    println!("  sipush {}", value);
                }
                Opcode::Putstatic => {
                    // Store static field
                    current_pc += 1;
                    let index_high = bytecode[current_pc] as u16;
                    current_pc += 1;
                    let index_low = bytecode[current_pc] as u16;

                    // AS SPECIFIED BY THE SPECS:
                    // (indexbyte1 << 8) | indexbyte2
                    let field_ref = (index_high << 8) | index_low;

                    if let Some(value) = self.operand_stack.pop() {
                        if let Some((class_name, field_name, _)) =
                            class_file.get_field_info(field_ref)
                        {
                            runtime_data_area
                                .static_fields
                                .insert(class_name.clone(), value.clone());
                            println!("  putstatic {}.{} = {:?}", class_name, field_name, value);
                        }
                    }
                }
                Opcode::Getstatic => {
                    // Get static field
                    current_pc += 1;
                    let index_high = bytecode[current_pc] as u16;
                    current_pc += 1;
                    let index_low = bytecode[current_pc] as u16;
                    let field_ref = (index_high << 8) | index_low;

                    if field_ref == 7 {
                        // System.out
                        self.operand_stack
                            .push(Value::Reference("System.out".to_string()));
                        println!("  getstatic System.out");
                    } else if field_ref == 21 || field_ref == 30 {
                        // Main.num1 or Main.num2
                        let field_name = if field_ref == 21 {
                            "Main.num1"
                        } else {
                            "Main.num2"
                        };
                        if let Some(value) = runtime_data_area.static_fields.get(field_name) {
                            self.operand_stack.push(value.clone());
                            println!("  getstatic {} = {:?}", field_name, value);
                        }
                    }
                }
                Opcode::Ldc => {
                    // Load constant
                    current_pc += 1;
                    let index = bytecode[current_pc] as u16;
                    if let Some(string_val) = class_file.get_string(index) {
                        self.operand_stack.push(Value::Object(string_val.clone()));
                        println!("  ldc \"{}\"", string_val);
                    }
                }
                Opcode::Invokevirtual => {
                    // Invoke virtual method
                    current_pc += 1;
                    let index_high = bytecode[current_pc] as u16;
                    current_pc += 1;
                    let index_low = bytecode[current_pc] as u16;

                    let method_ref = (index_high << 8) | index_low;

                    if method_ref == 15 || method_ref == 27 {
                        // PrintStream.println
                        if let Some(arg) = self.operand_stack.pop() {
                            if let Some(_print_stream) = self.operand_stack.pop() {
                                match arg {
                                    Value::Object(s) => println!("{}", s),
                                    Value::Int(i) => println!("{}", i),
                                    _ => println!("{:?}", arg),
                                }
                            }
                        }
                    }
                }
                Opcode::Invokestatic => {
                    current_pc += 1;
                    let index_high = bytecode[current_pc] as u16;
                    current_pc += 1;
                    let index_low = bytecode[current_pc] as u16;

                    let method_ref = (index_high << 8) | index_low;

                    if let Some((class_name, method_name, descriptor)) =
                        class_file.get_method_info(method_ref)
                    {
                        println!(
                            "  invokestatic {}.{}:{}",
                            class_name, method_name, descriptor
                        );

                        //TODO: Complete implementation
                    }
                }
                Opcode::Return => {
                    println!("  return");
                    break;
                }
                _ => {
                    println!("  Unhandled opcode in main: {:?}", opcode);
                }
            }
            current_pc += 1;
        }
    }
}
