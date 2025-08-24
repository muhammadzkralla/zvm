use crate::vm::value::Value;

/// Local Variables for method execution
pub struct LocalVariables {
    vars: Vec<Option<Value>>,
}

impl LocalVariables {
    pub fn new(size: usize) -> Self {
        Self {
            vars: vec![None; size],
        }
    }

    pub fn set(&mut self, index: usize, value: Value) {
        if index < self.vars.len() {
            self.vars[index] = Some(value);
        }
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        // We want to return a reference instead of moving ownership
        self.vars.get(index).and_then(|v| v.as_ref())
    }
}
