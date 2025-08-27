use crate::vm::value::Value;

/// Operand Stack for a call stack method execution frame
#[derive(Clone)]
pub struct OperandStack {
    stack: Vec<Value>,
}

impl OperandStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&Value> {
        self.stack.last()
    }
}
