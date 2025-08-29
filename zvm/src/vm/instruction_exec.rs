use crate::{
    parser::{class_file::ClassFile, opcode::Opcode},
    vm::{call_stack::CallStack, runtime::RuntimeDataArea, stack_frame::Frame, value::Value},
};

pub struct InstructionExecutor;

impl InstructionExecutor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute_instruction(
        &self,
        opcode: Opcode,
        frame: &mut Frame,
        class_file: &ClassFile,
        runtime_data_area: &mut RuntimeDataArea,
        call_stack: &mut CallStack,
        pc: &mut usize,
    ) -> Result<bool, String> {
        match opcode {
            Opcode::Bipush => self.execute_bipush(frame, pc),
            Opcode::Sipush => self.execute_sipush(frame, pc),
            Opcode::Putstatic => self.execute_putstatic(frame, class_file, runtime_data_area, pc),
            Opcode::Getstatic => self.execute_getstatic(frame, class_file, runtime_data_area, pc),
            Opcode::Ldc => self.execute_ldc(frame, class_file, pc),
            Opcode::Invokevirtual => self.execute_invokevirtual(frame, class_file, pc),
            Opcode::Invokestatic => {
                self.execute_invokestatic(frame, class_file, runtime_data_area, call_stack, pc)
            }
            Opcode::Return => self.execute_return(),

            _ => {
                println!("  Unhandled opcode: {:?}", opcode);
                Ok(true)
            }
        }
    }

    fn execute_bipush(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        *pc += 1;
        let value = frame.bytecode[*pc] as i32;
        frame.operand_stack.push(Value::Int(value));
        println!("  bipush {}", value);

        Ok(true)
    }

    fn execute_sipush(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        *pc += 1;
        let high = frame.bytecode[*pc] as u16;
        *pc += 1;
        let low = frame.bytecode[*pc] as u16;

        // AS SPECIFIED BY THE SPECS: (byte1 << 8) | byte2
        let value = ((high << 8) | low) as i32;
        frame.operand_stack.push(Value::Int(value));
        println!("  sipush {}", value);

        Ok(true)
    }

    fn execute_putstatic(
        &self,
        frame: &mut Frame,
        class_file: &ClassFile,
        runtime_data_area: &mut RuntimeDataArea,
        pc: &mut usize,
    ) -> Result<bool, String> {
        *pc += 1;
        let index_high = frame.bytecode[*pc] as u16;
        *pc += 1;
        let index_low = frame.bytecode[*pc] as u16;

        // AS SPECIFIED BY THE SPECS: (indexbyte1 << 8) | indexbyte2
        let field_ref = (index_high << 8) | index_low;

        if let Some(value) = frame.operand_stack.pop() {
            if let Some((class_name, field_name, _)) = class_file.get_field_info(field_ref) {
                runtime_data_area
                    .static_fields
                    .insert(format!("{}.{}", class_name, field_name), value.clone());
                println!("  putstatic {}.{} = {:?}", class_name, field_name, value);
            }
        }

        Ok(true)
    }

    fn execute_getstatic(
        &self,
        frame: &mut Frame,
        class_file: &ClassFile,
        runtime_data_area: &mut RuntimeDataArea,
        pc: &mut usize,
    ) -> Result<bool, String> {
        *pc += 1;
        let index_high = frame.bytecode[*pc] as u16;
        *pc += 1;
        let index_low = frame.bytecode[*pc] as u16;

        // AS SPECIFIED BY THE SPECS: (byte1 << 8) | byte2
        let field_ref = (index_high << 8) | index_low;

        //TODO: Handle all java standard classes
        if let Some((class_name, field_name, descriptor)) = class_file.get_field_info(field_ref) {
            println!("GETSTATIC: {}.{}:{}", class_name, field_name, descriptor);

            //TODO: Handle all java standard classes
            if class_name == "java/lang/System" {
                frame
                    .operand_stack
                    .push(Value::Reference("System.out".to_string()));
                println!("  getstatic System.out");
            } else {
                let static_field = format!("{}.{}", class_name, field_name);
                if let Some(value) = runtime_data_area.static_fields.get(&static_field) {
                    frame.operand_stack.push(value.clone());
                    println!("  getstatic {} = {:?}", field_name, value);
                }
            }
        }

        Ok(true)
    }

    fn execute_ldc(
        &self,
        frame: &mut Frame,
        class_file: &ClassFile,
        pc: &mut usize,
    ) -> Result<bool, String> {
        *pc += 1;
        let index = frame.bytecode[*pc] as u16;
        if let Some(string_val) = class_file.get_string(index) {
            frame.operand_stack.push(Value::Object(string_val.clone()));
            println!("  ldc \"{}\"", string_val);
        }

        Ok(true)
    }

    fn execute_invokevirtual(
        &self,
        frame: &mut Frame,
        class_file: &ClassFile,
        pc: &mut usize,
    ) -> Result<bool, String> {
        *pc += 1;
        let index_high = frame.bytecode[*pc] as u16;
        *pc += 1;
        let index_low = frame.bytecode[*pc] as u16;

        // AS SPECIFIED BY THE SPECS: (indexbyte1 << 8) | indexbyte2
        let method_ref = (index_high << 8) | index_low;

        //TODO: Handle all java standard classes
        if let Some((class_name, method_name, descriptor)) = class_file.get_method_info(method_ref)
        {
            println!(
                "INVOKEVIRTUAL: {}.{}:{}",
                class_name, method_name, descriptor
            );

            if class_name == "java/io/PrintStream" {
                if let Some(arg) = frame.operand_stack.pop() {
                    if let Some(_print_stream) = frame.operand_stack.pop() {
                        match arg {
                            Value::Object(s) => println!("{}", s),
                            Value::Int(i) => println!("{}", i),
                            _ => println!("{:?}", arg),
                        }
                    }
                }
            } else {
                println!("Unsupported Class yet");
            }
        }

        Ok(true)
    }

    fn execute_invokestatic(
        &self,
        frame: &mut Frame,
        class_file: &ClassFile,
        runtime_data_area: &mut RuntimeDataArea,
        call_stack: &mut CallStack,
        pc: &mut usize,
    ) -> Result<bool, String> {
        *pc += 1;
        let index_high = frame.bytecode[*pc] as u16;
        *pc += 1;
        let index_low = frame.bytecode[*pc] as u16;

        // AS SPECIFIED BY THE SPECS: (indexbyte1 << 8) | indexbyte2
        let method_ref = (index_high << 8) | index_low;

        if let Some((class_name, method_name, descriptor)) = class_file.get_method_info(method_ref)
        {
            println!(
                "  invokestatic {}.{}:{}",
                class_name, method_name, descriptor
            );

            //TODO: Complete implementation
            let params_count = self.count_method_params(&descriptor);
            let mut params = Vec::new();

            for _ in 0..params_count {
                if let Some(arg) = frame.operand_stack.pop() {
                    params.push(arg);
                }
            }

            params.reverse();

            // Handle local class method
            let method_info = match class_file.find_method(&method_name) {
                Some(method) => method,
                None => {
                    println!("No {} method found", method_name);
                    return Ok(true);
                }
            };

            // I assume that there will be always one attribute and it's the code attribute
            let attribute_info = &method_info.attributes[0];
            let info_bytes = &attribute_info.info;

            // Extract max_locals (two big-endian bytes) from info_bytes[2..3]
            let max_locals = u16::from_be_bytes([info_bytes[2], info_bytes[3]]);

            // Extract code_length (four big-endian bytes) from info_bytes[4..8]
            let code_length =
                u32::from_be_bytes([info_bytes[4], info_bytes[5], info_bytes[6], info_bytes[7]])
                    as usize;

            // Extract bytecode from info_bytes[8..8+code_length]
            let bytecode = info_bytes[8..8 + code_length].to_vec();

            let method_name = class_file
                .get_utf8(method_info.name_index)
                .ok_or("Failed to get method name")?;

            // TODO: Change the hardcoded max_locals value and handle env args array
            call_stack.push_frame(method_name, bytecode, max_locals as usize, params);

            //TODO: Solve clone duplication issue
            let mut top_frame = call_stack
                .current_frame()
                .ok_or("Could not acquire top frame")?
                .clone();

            top_frame.execute_frame(class_file, runtime_data_area, call_stack)?;

            if let Some(popped_frame) = call_stack.pop_frame() {
                println!(
                    "\n\nFINISHED EXECUTING FRAME: {}\n\n",
                    popped_frame.method_name.unwrap_or_default()
                );
            }

            //TODO: Handle external class methods
        }

        Ok(true)
    }

    fn execute_return(&self) -> Result<bool, String> {
        println!("  return");
        // Signal to break the execution loop
        Ok(false)
    }

    fn count_method_params(&self, descriptor: &str) -> usize {
        //TODO: Implement later
        0
    }
}
