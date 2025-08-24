use crate::vm::{local::LocalVariables, operand_stack::OperandStack};

/// Method execution stack call frame
pub struct Frame {
    pub operand_stack: OperandStack,
    pub local_variables: LocalVariables,
    pub pc: usize,
}

impl Frame {
    pub fn new(max_locals: usize) -> Self {
        Self {
            operand_stack: OperandStack::new(),
            local_variables: LocalVariables::new(max_locals),
            pc: 0,
        }
    }
}
