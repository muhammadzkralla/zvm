use crate::{
    debug_log,
    parser::{class_file::ClassFile, opcode::Opcode},
    vm::{
        call_stack::CallStack, instruction_exec::InstructionExecutor, local::LocalVariables,
        operand_stack::OperandStack, runtime::RuntimeDataArea,
    },
};

/// Method execution stack call frame
#[derive(Clone)]
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
        call_stack: &mut CallStack,
    ) -> Result<(), String> {
        let name = self.method_name.clone().expect("Failed to get method name");

        debug_log!("\n\nEXECUTING FRAME: {}\n\n", name);

        let mut current_pc = self.pc;
        let bytecode = self.bytecode.clone();

        let instruction_executor = InstructionExecutor::new();

        while current_pc < bytecode.len() {
            let opcode = Opcode::from(bytecode[current_pc]);
            debug_log!("Executing opcode: {:?} at pc: {}", opcode, current_pc);

            match instruction_executor.execute_instruction(
                opcode,
                self,
                class_file,
                runtime_data_area,
                call_stack,
                &mut current_pc,
            ) {
                Ok(should_continue_flag) => {
                    if !should_continue_flag {
                        // Since we outsourced control logic to another function, we can't
                        // break from there, so this is a workaround for now
                        //TODO: Clean later
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error executing instruction: {}", e);
                    return Err(e);
                }
            }

            current_pc += 1;
        }

        Ok(())
    }
}
