use std::char;

use crate::{
    debug_log,
    parser::{class_file::ClassFile, constant_pool_info::CpInfo, opcode::Opcode},
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
            Opcode::Iconstm1 => self.execute_iconst_m1(frame),
            Opcode::Iconst0 => self.execute_iconst_0(frame),
            Opcode::Iconst1 => self.execute_iconst_1(frame),
            Opcode::Iconst2 => self.execute_iconst_2(frame),
            Opcode::Iconst3 => self.execute_iconst_3(frame),
            Opcode::Iconst4 => self.execute_iconst_4(frame),
            Opcode::Iconst5 => self.execute_iconst_5(frame),
            Opcode::Bipush => self.execute_bipush(frame, pc),
            Opcode::Sipush => self.execute_sipush(frame, pc),
            Opcode::Ldc => self.execute_ldc(frame, class_file, pc),
            Opcode::Ldc2_w => self.execute_ldc2_w(frame, class_file, pc),
            //TODO: For now, Iload<n>, Lload<n>, Fload<n>, and Dload<n>
            // instructions can be handled by the same function
            // as I don't do type validation yet, but in the future,
            // this should be separated
            Opcode::Iload => self.execute_iload(frame, pc),
            Opcode::Lload => self.execute_iload(frame, pc),
            Opcode::Fload => self.execute_iload(frame, pc),
            Opcode::Iload0 => self.execute_iload_0(frame),
            Opcode::Iload1 => self.execute_iload_1(frame),
            Opcode::Iload2 => self.execute_iload_2(frame),
            Opcode::Iload3 => self.execute_iload_3(frame),
            Opcode::Lload0 => self.execute_iload_0(frame),
            Opcode::Lload1 => self.execute_iload_1(frame),
            Opcode::Lload2 => self.execute_iload_2(frame),
            Opcode::Lload3 => self.execute_iload_3(frame),
            Opcode::Fload0 => self.execute_iload_0(frame),
            Opcode::Fload1 => self.execute_iload_1(frame),
            Opcode::Fload2 => self.execute_iload_2(frame),
            Opcode::Fload3 => self.execute_iload_3(frame),
            Opcode::Aload => self.execute_aload(frame, pc),
            Opcode::Aload_0 => self.execute_aload_0(frame),
            Opcode::Aload_1 => self.execute_aload_1(frame),
            Opcode::Aload_2 => self.execute_aload_2(frame),
            Opcode::Aload_3 => self.execute_aload_3(frame),
            Opcode::Aaload => self.execute_aaload(frame),
            //TODO: For now, Istore_<n>, Lstore_<n>, Fstore_<n>, and Dstore_<n>
            // instructions can be handled by the same function
            // as I don't do type validation yet, but in the future,
            // this should be separated
            Opcode::Istore => self.execute_istore(frame, pc),
            Opcode::Lstore => self.execute_istore(frame, pc),
            Opcode::Fstore => self.execute_istore(frame, pc),
            Opcode::Istore_0 => self.execute_istore_0(frame),
            Opcode::Istore_1 => self.execute_istore_1(frame),
            Opcode::Istore_2 => self.execute_istore_2(frame),
            Opcode::Istore_3 => self.execute_istore_3(frame),
            Opcode::Lstore_0 => self.execute_istore_0(frame),
            Opcode::Lstore_1 => self.execute_istore_1(frame),
            Opcode::Lstore_2 => self.execute_istore_2(frame),
            Opcode::Lstore_3 => self.execute_istore_3(frame),
            Opcode::Fstore_0 => self.execute_istore_0(frame),
            Opcode::Fstore_1 => self.execute_istore_1(frame),
            Opcode::Fstore_2 => self.execute_istore_2(frame),
            Opcode::Fstore_3 => self.execute_istore_3(frame),
            Opcode::Iadd => self.execute_iadd(frame),
            Opcode::Ladd => self.execute_ladd(frame),
            Opcode::Fadd => self.execute_fadd(frame),
            Opcode::Isub => self.execute_isub(frame),
            Opcode::Lsub => self.execute_lsub(frame),
            Opcode::Fsub => self.execute_fsub(frame),
            Opcode::Imul => self.execute_imul(frame),
            Opcode::Lmul => self.execute_lmul(frame),
            Opcode::Fmul => self.execute_fmul(frame),
            Opcode::Idiv => self.execute_idiv(frame),
            Opcode::Ldiv => self.execute_ldiv(frame),
            Opcode::Fdiv => self.execute_fdiv(frame),
            Opcode::Irem => self.execute_irem(frame),
            Opcode::Lrem => self.execute_lrem(frame),
            Opcode::Frem => self.execute_frem(frame),
            Opcode::Ineg => self.execute_ineg(frame),
            Opcode::Lneg => self.execute_lneg(frame),
            Opcode::Fneg => self.execute_fneg(frame),
            Opcode::Ifeq => self.execute_ifeq(frame, pc),
            Opcode::Ifne => self.execute_ifne(frame, pc),
            Opcode::Iflt => self.execute_iflt(frame, pc),
            Opcode::Ifge => self.execute_ifge(frame, pc),
            Opcode::Ifgt => self.execute_ifgt(frame, pc),
            Opcode::Ifle => self.execute_ifle(frame, pc),
            Opcode::If_icmpeq => self.execute_if_icmpeq(frame, pc),
            Opcode::If_icmpne => self.execute_if_icmpne(frame, pc),
            Opcode::If_icmplt => self.execute_if_icmplt(frame, pc),
            Opcode::If_icmpge => self.execute_if_icmpge(frame, pc),
            Opcode::If_icmpgt => self.execute_if_icmpgt(frame, pc),
            Opcode::If_icmple => self.execute_if_icmple(frame, pc),
            Opcode::Return => self.execute_return(),
            Opcode::Getstatic => self.execute_getstatic(frame, class_file, runtime_data_area, pc),
            Opcode::Putstatic => self.execute_putstatic(frame, class_file, runtime_data_area, pc),
            Opcode::Invokevirtual => self.execute_invokevirtual(frame, class_file, pc),
            Opcode::Invokespecial => {
                // TODO: implement invokespecial
                debug_log!("  Unhandled opcode: {:?}", opcode);
                Ok(true)
            }
            Opcode::Invokestatic => {
                self.execute_invokestatic(frame, class_file, runtime_data_area, call_stack, pc)
            }

            _ => {
                debug_log!("  Unhandled opcode: {:?}", opcode);
                Ok(true)
            }
        }
    }

    /// Push integer constant -1 onto the operand stack
    fn execute_iconst_m1(&self, frame: &mut Frame) -> Result<bool, String> {
        frame.operand_stack.push(Value::Int(-1));
        debug_log!("  iconst_m1");
        Ok(true)
    }

    /// Push integer constant 0 onto the operand stack
    fn execute_iconst_0(&self, frame: &mut Frame) -> Result<bool, String> {
        frame.operand_stack.push(Value::Int(0));
        debug_log!("  iconst_0");
        Ok(true)
    }

    /// Push integer constant 1 onto the operand stack
    fn execute_iconst_1(&self, frame: &mut Frame) -> Result<bool, String> {
        frame.operand_stack.push(Value::Int(1));
        debug_log!("  iconst_1");
        Ok(true)
    }

    /// Push integer constant 2 onto the operand stack
    fn execute_iconst_2(&self, frame: &mut Frame) -> Result<bool, String> {
        frame.operand_stack.push(Value::Int(2));
        debug_log!("  iconst_2");
        Ok(true)
    }

    /// Push integer constant 3 onto the operand stack
    fn execute_iconst_3(&self, frame: &mut Frame) -> Result<bool, String> {
        frame.operand_stack.push(Value::Int(3));
        debug_log!("  iconst_3");
        Ok(true)
    }

    /// Push integer constant 4 onto the operand stack
    fn execute_iconst_4(&self, frame: &mut Frame) -> Result<bool, String> {
        frame.operand_stack.push(Value::Int(4));
        debug_log!("  iconst_4");
        Ok(true)
    }

    /// Push integer constant 5 onto the operand stack
    fn execute_iconst_5(&self, frame: &mut Frame) -> Result<bool, String> {
        frame.operand_stack.push(Value::Int(5));
        debug_log!("  iconst_5");
        Ok(true)
    }

    /// Push the next byte's value from the bytecode to the operand stack
    fn execute_bipush(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        *pc += 1;
        let value = frame.bytecode[*pc] as i32;
        frame.operand_stack.push(Value::Int(value));
        debug_log!("  bipush {}", value);

        Ok(true)
    }

    /// Push the next two bytes' value from the bytecode to the operand stack
    /// after applying the indexing equation specified by the specs
    fn execute_sipush(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        *pc += 1;
        let high = frame.bytecode[*pc] as u16;
        *pc += 1;
        let low = frame.bytecode[*pc] as u16;

        // AS SPECIFIED BY THE SPECS: (byte1 << 8) | byte2
        let value = ((high << 8) | low) as i32;
        frame.operand_stack.push(Value::Int(value));
        debug_log!("  sipush {}", value);

        Ok(true)
    }

    /// Load a String value from the constant pool and push it to the operand stack
    fn execute_ldc(
        &self,
        frame: &mut Frame,
        class_file: &ClassFile,
        pc: &mut usize,
    ) -> Result<bool, String> {
        *pc += 1;
        let index = frame.bytecode[*pc] as u16;

        if let Some(cp_entry) = class_file.constant_pool.get(index as usize) {
            match cp_entry {
                CpInfo::String { .. } => {
                    if let Some(string_val) = class_file.get_string(index) {
                        frame.operand_stack.push(Value::Object(string_val.clone()));
                        debug_log!("  ldc \"{}\"", string_val);
                    }
                }
                CpInfo::Integer { .. } => {
                    if let Some(int_val) = class_file.get_integer(index) {
                        frame.operand_stack.push(Value::Int(int_val));
                        debug_log!("  ldc {}", int_val);
                    }
                }
                CpInfo::Float { .. } => {
                    if let Some(float_val) = class_file.get_float(index) {
                        frame.operand_stack.push(Value::Float(float_val));
                        debug_log!("  ldc {}f", float_val);
                    }
                }
                _ => {
                    return Err(format!(
                        "Invalid constant pool entry type for ldc at index {}",
                        index
                    ));
                }
            }
        }

        Ok(true)
    }

    /// Load a long or a double value from the constant pool and push it to the operand stack
    fn execute_ldc2_w(
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
        let index = ((index_high << 8) | index_low) as usize;

        if let Some(CpInfo::Long {
            high_bytes,
            low_bytes,
        }) = class_file.constant_pool.get(index)
        {
            // AS SPECIFIED BY THE SPECS:
            // ((long) high_bytes << 32) + low_bytes
            let long = ((*high_bytes as i64) << 32) + (*low_bytes as i64);
            let value = Value::Long(long);
            frame.operand_stack.push(value.clone());
            debug_log!("  ldc2_w {:?}", value);
        }

        Ok(true)
    }

    /// Load an integer value at the index of the next byte's value from the bytecode
    /// from the frame's local variables and push it to the operand stack
    fn execute_iload(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        //TODO: I assume the variable will always be an integer type as specified by the specs
        // I think we should do a check here, but I'll choose to keep the logic simple
        // Same applies to the other iload_<n> instruction implementations
        *pc += 1;
        let index = frame.bytecode[*pc] as usize;
        if let Some(variable) = frame.local_variables.get(index) {
            frame.operand_stack.push(variable.clone());
            debug_log!("  iload \"{:?}\"", variable);
        }

        Ok(true)
    }

    /// Load an integer value at the index of 0
    /// from the frame's local variables and push it to the operand stack
    fn execute_iload_0(&self, frame: &mut Frame) -> Result<bool, String> {
        let index = 0 as usize;
        if let Some(variable) = frame.local_variables.get(index) {
            frame.operand_stack.push(variable.clone());
            debug_log!("  iload_0 \"{:?}\"", variable);
        }

        Ok(true)
    }

    /// Load an integer value at the index of 1
    /// from the frame's local variables and push it to the operand stack
    fn execute_iload_1(&self, frame: &mut Frame) -> Result<bool, String> {
        let index = 1 as usize;
        if let Some(variable) = frame.local_variables.get(index) {
            frame.operand_stack.push(variable.clone());
            debug_log!("  iload_1 \"{:?}\"", variable);
        }

        Ok(true)
    }

    /// Load an integer value at the index of 2
    /// from the frame's local variables and push it to the operand stack
    fn execute_iload_2(&self, frame: &mut Frame) -> Result<bool, String> {
        let index = 2 as usize;
        if let Some(variable) = frame.local_variables.get(index) {
            frame.operand_stack.push(variable.clone());
            debug_log!("  iload_2 \"{:?}\"", variable);
        }

        Ok(true)
    }

    /// Load an integer value at the index of 3
    /// from the frame's local variables and push it to the operand stack
    fn execute_iload_3(&self, frame: &mut Frame) -> Result<bool, String> {
        let index = 3 as usize;
        if let Some(variable) = frame.local_variables.get(index) {
            frame.operand_stack.push(variable.clone());
            debug_log!("  iload_3 \"{:?}\"", variable);
        }

        Ok(true)
    }

    /// Load the reference located at the index of the next byte's value in the bytecode
    /// from the frame's local variables and push it to the operand stack
    fn execute_aload(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        //TODO: I assume the variable will always be a reference type as specified by the specs
        // I think we should do a check here, but I'll choose to keep the logic simple
        // Same applies to the other aload_<n> instruction implementations
        *pc += 1;
        let index = frame.bytecode[*pc] as usize;

        if let Some(value) = frame.local_variables.get(index) {
            frame.operand_stack.push(value.clone());
            debug_log!("  aload = {:?}", value);
        } else {
            return Err("Local variable is not initialized".to_string());
        }

        Ok(true)
    }

    /// Load the reference located at the index of 0
    /// from the frame's local variables and push it to the operand stack
    fn execute_aload_0(&self, frame: &mut Frame) -> Result<bool, String> {
        if let Some(value) = frame.local_variables.get(0) {
            frame.operand_stack.push(value.clone());
            debug_log!("  aload_0 = {:?}", value);
        } else {
            return Err("Local variable 0 is not initialized".to_string());
        }

        Ok(true)
    }

    /// Load the reference located at the index of 1
    /// from the frame's local variables and push it to the operand stack
    fn execute_aload_1(&self, frame: &mut Frame) -> Result<bool, String> {
        if let Some(value) = frame.local_variables.get(1) {
            frame.operand_stack.push(value.clone());
            debug_log!("  aload_1 = {:?}", value);
        } else {
            return Err("Local variable 1 is not initialized".to_string());
        }

        Ok(true)
    }

    /// Load the reference located at the index of 2
    /// from the frame's local variables and push it to the operand stack
    fn execute_aload_2(&self, frame: &mut Frame) -> Result<bool, String> {
        if let Some(value) = frame.local_variables.get(2) {
            frame.operand_stack.push(value.clone());
            debug_log!("  aload_2 = {:?}", value);
        } else {
            return Err("Local variable 2 is not initialized".to_string());
        }

        Ok(true)
    }

    /// Load the reference located at the index of 3
    /// from the frame's local variables and push it to the operand stack
    fn execute_aload_3(&self, frame: &mut Frame) -> Result<bool, String> {
        if let Some(value) = frame.local_variables.get(3) {
            frame.operand_stack.push(value.clone());
            debug_log!("  aload_3 = {:?}", value);
        } else {
            return Err("Local variable 3 is not initialized".to_string());
        }

        Ok(true)
    }

    /// Load a reference value from an array and push it to the operand stack
    fn execute_aaload(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle missing index and array ref and StackOverFlowException
        if let Some(Value::Int(index)) = frame.operand_stack.pop() {
            if let Some(arrayref) = frame.operand_stack.pop() {
                match arrayref {
                    Value::Array(ref arr) => {
                        if index >= 0 && (index as usize) < arr.len() {
                            let item = arr[index as usize].clone();
                            frame.operand_stack.push(item.clone());
                            debug_log!("  aaload [{}] = {:?}", index, item);
                        } else {
                            return Err(format!("Array index out of bounds: {}", index));
                        }
                    }
                    _ => {
                        return Err(format!("Expected array reference, got {:?}", arrayref));
                    }
                }
            }
        }

        Ok(true)
    }

    /// Store an integer value popped from the operand stack
    /// at the index of the next byte's value from the bytecode in the frame's local variables
    fn execute_istore(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        *pc += 1;
        let index = frame.bytecode[*pc] as usize;

        if let Some(value) = frame.operand_stack.pop() {
            frame.local_variables.set(index, value.clone());
            debug_log!("  istore[{}] = {:?}", index, value);
        }

        Ok(true)
    }

    /// Store an integer value popped from the operand stack
    /// at the index of the 0 in the frame's local variables
    fn execute_istore_0(&self, frame: &mut Frame) -> Result<bool, String> {
        let index = 0 as usize;

        if let Some(value) = frame.operand_stack.pop() {
            frame.local_variables.set(index, value.clone());
            debug_log!("  istore_0[{}] = {:?}", index, value);
        }

        Ok(true)
    }

    /// Store an integer value popped from the operand stack
    /// at the index of the 1 in the frame's local variables
    fn execute_istore_1(&self, frame: &mut Frame) -> Result<bool, String> {
        let index = 1 as usize;

        if let Some(value) = frame.operand_stack.pop() {
            frame.local_variables.set(index, value.clone());
            debug_log!("  istore_1[{}] = {:?}", index, value);
        }

        Ok(true)
    }

    /// Store an integer value popped from the operand stack
    /// at the index of the 2 in the frame's local variables
    fn execute_istore_2(&self, frame: &mut Frame) -> Result<bool, String> {
        let index = 2 as usize;

        if let Some(value) = frame.operand_stack.pop() {
            frame.local_variables.set(index, value.clone());
            debug_log!("  istore_2[{}] = {:?}", index, value);
        }

        Ok(true)
    }

    /// Store an integer value popped from the operand stack
    /// at the index of the 3 in the frame's local variables
    fn execute_istore_3(&self, frame: &mut Frame) -> Result<bool, String> {
        let index = 3 as usize;

        if let Some(value) = frame.operand_stack.pop() {
            frame.local_variables.set(index, value.clone());
            debug_log!("  istore_3[{}] = {:?}", index, value);
        }

        Ok(true)
    }

    /// Pop two integer values from the operand stack, adds them, and then
    /// push the result back onto the operand stack
    fn execute_iadd(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Int(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Int(value1)) = frame.operand_stack.pop() {
                let value = value1.wrapping_add(value2);

                frame.operand_stack.push(Value::Int(value));
            }
        }

        Ok(true)
    }

    /// Pop two long values from the operand stack, adds them, and then
    /// push the result back onto the operand stack
    fn execute_ladd(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Long(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Long(value1)) = frame.operand_stack.pop() {
                let value = value1.wrapping_add(value2);

                frame.operand_stack.push(Value::Long(value));
            }
        }

        Ok(true)
    }

    /// Pop two float values from the operand stack, adds them, and then
    /// push the result back onto the operand stack
    fn execute_fadd(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Float(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Float(value1)) = frame.operand_stack.pop() {
                let value = value1 + value2;

                frame.operand_stack.push(Value::Float(value));
            }
        }

        Ok(true)
    }

    /// Pop two integer values from the operand stack, subtracts them, and then
    /// push the result back onto the operand stack
    fn execute_isub(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Int(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Int(value1)) = frame.operand_stack.pop() {
                let value = value1.wrapping_sub(value2);

                frame.operand_stack.push(Value::Int(value));
            }
        }

        Ok(true)
    }

    /// Pop two long values from the operand stack, subtracts them, and then
    /// push the result back onto the operand stack
    fn execute_lsub(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Long(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Long(value1)) = frame.operand_stack.pop() {
                let value = value1.wrapping_sub(value2);

                frame.operand_stack.push(Value::Long(value));
            }
        }

        Ok(true)
    }

    /// Pop two float values from the operand stack, subtracts them, and then
    /// push the result back onto the operand stack
    fn execute_fsub(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Float(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Float(value1)) = frame.operand_stack.pop() {
                let value = value1 - value2;

                frame.operand_stack.push(Value::Float(value));
            }
        }

        Ok(true)
    }

    /// Pop two integer values from the operand stack, multiplies them, and then
    /// push the result back onto the operand stack
    fn execute_imul(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Int(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Int(value1)) = frame.operand_stack.pop() {
                let value = value1.wrapping_mul(value2);

                frame.operand_stack.push(Value::Int(value));
            }
        }

        Ok(true)
    }

    /// Pop two long values from the operand stack, multiplies them, and then
    /// push the result back onto the operand stack
    fn execute_lmul(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Long(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Long(value1)) = frame.operand_stack.pop() {
                let value = value1.wrapping_mul(value2);

                frame.operand_stack.push(Value::Long(value));
            }
        }

        Ok(true)
    }

    /// Pop two float values from the operand stack, multiplies them, and then
    /// push the result back onto the operand stack
    fn execute_fmul(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Float(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Float(value1)) = frame.operand_stack.pop() {
                let value = value1 * value2;

                frame.operand_stack.push(Value::Float(value));
            }
        }

        Ok(true)
    }

    /// Pop two integer values from the operand stack, divides them, and then
    /// push the result back onto the operand stack
    fn execute_idiv(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        //TODO: Handle division by zero
        if let Some(Value::Int(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Int(value1)) = frame.operand_stack.pop() {
                let value = value1.wrapping_div(value2);

                frame.operand_stack.push(Value::Int(value));
            }
        }

        Ok(true)
    }

    /// Pop two long values from the operand stack, divides them, and then
    /// push the result back onto the operand stack
    fn execute_ldiv(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        //TODO: Handle division by zero
        if let Some(Value::Long(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Long(value1)) = frame.operand_stack.pop() {
                let value = value1.wrapping_div(value2);

                frame.operand_stack.push(Value::Long(value));
            }
        }

        Ok(true)
    }

    /// Pop two float values from the operand stack, divides them, and then
    /// push the result back onto the operand stack
    fn execute_fdiv(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        //TODO: Handle division by zero
        if let Some(Value::Float(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Float(value1)) = frame.operand_stack.pop() {
                let value = value1 / value2;

                frame.operand_stack.push(Value::Float(value));
            }
        }

        Ok(true)
    }

    /// Pop two integer values from the operand stack, calculates their remainder,
    /// and then push the result back onto the operand stack
    fn execute_irem(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        //TODO: Handle division by zero
        if let Some(Value::Int(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Int(value1)) = frame.operand_stack.pop() {
                debug_log!("value1: {}, value2: {}", value1, value2);

                let value = value1.wrapping_rem(value2);

                debug_log!("value: {}", value);

                frame.operand_stack.push(Value::Int(value));
            }
        }

        Ok(true)
    }

    /// Pop two long values from the operand stack, calculates their remainder,
    /// and then push the result back onto the operand stack
    fn execute_lrem(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        //TODO: Handle division by zero
        if let Some(Value::Long(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Long(value1)) = frame.operand_stack.pop() {
                debug_log!("value1: {}, value2: {}", value1, value2);

                let value = value1.wrapping_rem(value2);

                debug_log!("value: {}", value);

                frame.operand_stack.push(Value::Long(value));
            }
        }

        Ok(true)
    }

    /// Pop two float values from the operand stack, calculates their remainder,
    /// and then push the result back onto the operand stack
    fn execute_frem(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        //TODO: Handle division by zero
        if let Some(Value::Float(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Float(value1)) = frame.operand_stack.pop() {
                debug_log!("value1: {}, value2: {}", value1, value2);

                let value = value1 % value2;

                debug_log!("value: {}", value);

                frame.operand_stack.push(Value::Float(value));
            }
        }

        Ok(true)
    }

    /// Pop an integer value from the operand stack, negates it, and then
    /// push the result back onto the operand stack
    fn execute_ineg(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Int(value)) = frame.operand_stack.pop() {
            debug_log!("value: {}", value);

            let negated_value = value.wrapping_neg();

            debug_log!("negated_value: {}", negated_value);

            frame.operand_stack.push(Value::Int(negated_value));
        }

        Ok(true)
    }

    /// Pop a long value from the operand stack, negates it, and then
    /// push the result back onto the operand stack
    fn execute_lneg(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Long(value)) = frame.operand_stack.pop() {
            debug_log!("value: {}", value);

            let negated_value = value.wrapping_neg();

            debug_log!("negated_value: {}", negated_value);

            frame.operand_stack.push(Value::Long(negated_value));
        }

        Ok(true)
    }

    /// Pop a float value from the operand stack, negates it, and then
    /// push the result back onto the operand stack
    fn execute_fneg(&self, frame: &mut Frame) -> Result<bool, String> {
        //TODO: Handle insufficient number of values in the operand stack
        //TODO: Handle overflows
        if let Some(Value::Float(value)) = frame.operand_stack.pop() {
            debug_log!("value: {}", value);

            let negated_value = -value;

            debug_log!("negated_value: {}", negated_value);

            frame.operand_stack.push(Value::Float(negated_value));
        }

        Ok(true)
    }

    /// Pop some value from the operand stack and check if it equals zero
    fn execute_ifeq(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value)) = frame.operand_stack.pop() {
            if value == 0 {
                *pc += 1;
                let index_high = frame.bytecode[*pc] as u16;
                *pc += 1;
                let index_low = frame.bytecode[*pc] as u16;

                // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                let offset = ((index_high << 8) | index_low) as usize;

                // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                // not the current PC
                *pc -= 3;
                *pc += offset;
            } else {
                *pc += 2;
            }
        }

        Ok(true)
    }

    /// Pop some value from the operand stack and check if it doesn't equals zero
    fn execute_ifne(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value)) = frame.operand_stack.pop() {
            if value != 0 {
                *pc += 1;
                let index_high = frame.bytecode[*pc] as u16;
                *pc += 1;
                let index_low = frame.bytecode[*pc] as u16;

                // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                let offset = ((index_high << 8) | index_low) as usize;

                // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                // not the current PC
                *pc -= 3;
                *pc += offset;
            } else {
                *pc += 2;
            }
        }
        Ok(true)
    }

    /// Pop some value from the operand stack and check if it is less than zero
    fn execute_iflt(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value)) = frame.operand_stack.pop() {
            if value < 0 {
                *pc += 1;
                let index_high = frame.bytecode[*pc] as u16;
                *pc += 1;
                let index_low = frame.bytecode[*pc] as u16;

                // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                let offset = ((index_high << 8) | index_low) as usize;

                // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                // not the current PC
                *pc -= 3;
                *pc += offset;
            } else {
                *pc += 2;
            }
        }

        Ok(true)
    }

    /// Pop some value from the operand stack and check if it is greater than or equal zero
    fn execute_ifge(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value)) = frame.operand_stack.pop() {
            if value >= 0 {
                *pc += 1;
                let index_high = frame.bytecode[*pc] as u16;
                *pc += 1;
                let index_low = frame.bytecode[*pc] as u16;

                // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                let offset = ((index_high << 8) | index_low) as usize;

                // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                // not the current PC
                *pc -= 3;
                *pc += offset;
            } else {
                *pc += 2;
            }
        }

        Ok(true)
    }

    /// Pop some value from the operand stack and check if it is greater than zero
    fn execute_ifgt(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value)) = frame.operand_stack.pop() {
            if value > 0 {
                *pc += 1;
                let index_high = frame.bytecode[*pc] as u16;
                *pc += 1;
                let index_low = frame.bytecode[*pc] as u16;

                // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                let offset = ((index_high << 8) | index_low) as usize;

                // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                // not the current PC
                *pc -= 3;
                *pc += offset;
            } else {
                *pc += 2;
            }
        }

        Ok(true)
    }

    /// Pop some value from the operand stack and check if it is less than or equal zero
    fn execute_ifle(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value)) = frame.operand_stack.pop() {
            if value <= 0 {
                *pc += 1;
                let index_high = frame.bytecode[*pc] as u16;
                *pc += 1;
                let index_low = frame.bytecode[*pc] as u16;

                // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                let offset = ((index_high << 8) | index_low) as usize;

                // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                // not the current PC
                *pc -= 3;
                *pc += offset;
            } else {
                *pc += 2;
            }
        }

        Ok(true)
    }

    /// Pop two integer values from the operand stack and check if they are equal
    fn execute_if_icmpeq(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Int(value1)) = frame.operand_stack.pop() {
                if value1 == value2 {
                    *pc += 1;
                    let index_high = frame.bytecode[*pc] as u16;
                    *pc += 1;
                    let index_low = frame.bytecode[*pc] as u16;

                    // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                    let offset = ((index_high << 8) | index_low) as usize;

                    // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                    // not the current PC
                    *pc -= 3;
                    *pc += offset;
                } else {
                    *pc += 2;
                }
            }
        }

        Ok(true)
    }

    /// Pop two integer values from the operand stack and check if they are not equal
    fn execute_if_icmpne(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Int(value1)) = frame.operand_stack.pop() {
                if value1 != value2 {
                    *pc += 1;
                    let index_high = frame.bytecode[*pc] as u16;
                    *pc += 1;
                    let index_low = frame.bytecode[*pc] as u16;

                    // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                    let offset = ((index_high << 8) | index_low) as usize;

                    // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                    // not the current PC
                    *pc -= 3;
                    *pc += offset;
                } else {
                    *pc += 2;
                }
            }
        }

        Ok(true)
    }

    /// Pop two integer values from the operand stack and check if the first is less than the
    /// second
    fn execute_if_icmplt(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Int(value1)) = frame.operand_stack.pop() {
                if value1 < value2 {
                    *pc += 1;
                    let index_high = frame.bytecode[*pc] as u16;
                    *pc += 1;
                    let index_low = frame.bytecode[*pc] as u16;

                    // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                    let offset = ((index_high << 8) | index_low) as usize;

                    // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                    // not the current PC
                    *pc -= 3;
                    *pc += offset;
                } else {
                    *pc += 2;
                }
            }
        }

        Ok(true)
    }

    /// Pop two integer values from the operand stack and check if the first is greater than or equal
    /// the second
    fn execute_if_icmpge(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Int(value1)) = frame.operand_stack.pop() {
                if value1 >= value2 {
                    *pc += 1;
                    let index_high = frame.bytecode[*pc] as u16;
                    *pc += 1;
                    let index_low = frame.bytecode[*pc] as u16;

                    // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                    let offset = ((index_high << 8) | index_low) as usize;

                    // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                    // not the current PC
                    *pc -= 3;
                    *pc += offset;
                } else {
                    *pc += 2;
                }
            }
        }

        Ok(true)
    }

    /// Pop two integer values from the operand stack and check if the first is greater than the second
    fn execute_if_icmpgt(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Int(value1)) = frame.operand_stack.pop() {
                if value1 > value2 {
                    *pc += 1;
                    let index_high = frame.bytecode[*pc] as u16;
                    *pc += 1;
                    let index_low = frame.bytecode[*pc] as u16;

                    // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                    let offset = ((index_high << 8) | index_low) as usize;

                    // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                    // not the current PC
                    *pc -= 3;
                    *pc += offset;
                } else {
                    *pc += 2;
                }
            }
        }

        Ok(true)
    }

    /// Pop two integer values from the operand stack and check if the first is less than or equal
    /// the second
    fn execute_if_icmple(&self, frame: &mut Frame, pc: &mut usize) -> Result<bool, String> {
        if let Some(Value::Int(value2)) = frame.operand_stack.pop() {
            if let Some(Value::Int(value1)) = frame.operand_stack.pop() {
                if value1 <= value2 {
                    *pc += 1;
                    let index_high = frame.bytecode[*pc] as u16;
                    *pc += 1;
                    let index_low = frame.bytecode[*pc] as u16;

                    // AS SPECIFIED BY THE SPECS: (branchbyte1 << 8) | branchbyte2
                    let offset = ((index_high << 8) | index_low) as usize;

                    // NOTE: The offset is relative to the address of the if<cond> opcode itself,
                    // not the current PC
                    *pc -= 3;
                    *pc += offset;
                } else {
                    *pc += 2;
                }
            }
        }

        Ok(true)
    }

    /// Breaks the current frame's execution loop
    fn execute_return(&self) -> Result<bool, String> {
        debug_log!("  return");
        // Signal to break the execution loop
        Ok(false)
    }

    /// Load a static field reference located at the index of the next two bytes' value in the bytecode
    /// after applying the indexing equation specified by the specs from the constant pool
    /// inside the runtime static fields and push it to the operand stack
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
            debug_log!("GETSTATIC: {}.{}:{}", class_name, field_name, descriptor);

            //TODO: Handle all java standard classes
            if class_name == "java/lang/System" {
                frame
                    .operand_stack
                    .push(Value::Reference("System.out".to_string()));
                debug_log!("  getstatic System.out");
            } else {
                let static_field = format!("{}.{}", class_name, field_name);
                if let Some(value) = runtime_data_area.static_fields.get(&static_field) {
                    frame.operand_stack.push(value.clone());
                    debug_log!("  getstatic {} = {:?}", field_name, value);
                }
            }
        }

        Ok(true)
    }

    /// Put a static field reference located at the index of the next two bytes' value in the bytecode
    /// after applying the indexing equation specified by the specs from the constant pool and
    /// insert it in the runtime static fields
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
                debug_log!("  putstatic {}.{} = {:?}", class_name, field_name, value);
            }
        }

        Ok(true)
    }

    /// Load a non-static method reference located at the index of the next two bytes' value in the bytecode
    /// after applying the indexing equation specified by the specs from the constant pool
    /// and invoke it
    /// (Needs an object reference, resolved at runtime with dynamic dispatch)
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
            debug_log!(
                "INVOKEVIRTUAL: {}.{}:{}",
                class_name,
                method_name,
                descriptor
            );

            if class_name == "java/io/PrintStream" {
                if let Some(arg) = frame.operand_stack.pop() {
                    if let Some(_print_stream) = frame.operand_stack.pop() {
                        match arg {
                            Value::Object(s) => println!("{}", s),
                            Value::Int(i) => println!("{}", i),
                            Value::Long(l) => println!("{}", l),
                            Value::Float(f) => println!("{}", f),
                            _ => println!("{:?}", arg),
                        }
                    }
                }
            } else {
                debug_log!("Unsupported Class yet");
            }
        }

        Ok(true)
    }

    /// Load a static method reference located at the index of the next two bytes' value in the bytecode
    /// after applying the indexing equation specified by the specs from the constant pool
    /// and invoke it
    /// (No object needed, resolved at compile time)
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
            debug_log!(
                "  invokestatic {}.{}:{}",
                class_name,
                method_name,
                descriptor
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
                    debug_log!("No {} method found", method_name);
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

            call_stack.push_frame(method_name, bytecode, max_locals as usize, params);

            let mut top_frame = call_stack
                .current_frame()
                .ok_or("Could not acquire top frame")?
                .clone();

            top_frame.execute_frame(class_file, runtime_data_area, call_stack)?;

            if let Some(popped_frame) = call_stack.pop_frame() {
                debug_log!(
                    "\n\nFINISHED EXECUTING FRAME: {}\n\n",
                    popped_frame.method_name.unwrap_or_default()
                );
            }

            //TODO: Handle external class methods
        }

        Ok(true)
    }

    /// Count the number of params passed to some function call
    fn count_method_params(&self, descriptor: &str) -> usize {
        // Extract the characters in the descriptor string
        // and initialize a final count variable and a loop pointer
        let chars: Vec<char> = descriptor.chars().collect();
        let mut count = 0;
        let mut i = 0;

        // Find the opening parenthesis
        while i < chars.len() && chars[i] != '(' {
            i += 1;
        }

        if i >= chars.len() {
            debug_log!("Warning: Invalid method descriptor format: {}", descriptor);
            return 0;
        }

        // Skip the opening '('
        i += 1;

        // Keep parsing until hitting the closing parenthesis
        while i < chars.len() && chars[i] != ')' {
            match chars[i] {
                // Primitive types take only one count
                'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' => {
                    count += 1;
                    i += 1;
                }
                // Object types take only one count and they start with 'L' and end with ';'
                'L' => {
                    count += 1;
                    i += 1;
                    while i < chars.len() && chars[i] != ';' {
                        i += 1;
                    }

                    // Skip the ';'
                    if i < chars.len() {
                        i += 1;
                    }
                }
                // Array types take only one count and they start with '['
                '[' => {
                    count += 1;
                    i += 1;

                    // Skip all the array dimensions
                    while i < chars.len() && chars[i] == '[' {
                        i += 1;
                    }

                    // Skip the component type
                    if i < chars.len() {
                        match chars[i] {
                            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' => {
                                i += 1;
                            }
                            'L' => {
                                i += 1;
                                while i < chars.len() && chars[i] != ';' {
                                    i += 1;
                                }

                                // Skip the ';'
                                if i < chars.len() {
                                    i += 1;
                                }
                            }
                            _ => {
                                debug_log!("Warning: Unknown array component type: {}", chars[i]);
                                i += 1;
                            }
                        }
                    }
                }
                _ => {
                    debug_log!("Warning: Unknown parameter type: {}", chars[i]);
                    i += 1;
                }
            }
        }
        debug_log!(
            "Method descriptor '{}' has {} parameters",
            descriptor,
            count
        );

        count
    }
}
