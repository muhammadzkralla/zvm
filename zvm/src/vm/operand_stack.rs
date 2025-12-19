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

    pub fn push_at(&mut self, item: Value, index: usize) {
        self.stack.insert(index, item);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&Value> {
        self.stack.last()
    }

    pub fn peek_at(&self, index: usize) -> Option<&Value> {
        self.stack.get(index)
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.stack.swap(a, b);
    }
}
