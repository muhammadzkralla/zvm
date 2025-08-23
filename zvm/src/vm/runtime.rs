use std::collections::HashMap;

use crate::vm::value::Value;

/// Runtime data area
pub struct RuntimeDataArea {
    pub static_fields: HashMap<String, Value>,
}

impl RuntimeDataArea {
    pub fn new() -> Self {
        Self {
            static_fields: HashMap::new(),
        }
    }
}
