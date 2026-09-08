use std::{cell::RefCell, rc::Rc};

/// JVM Value types
#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),                       // Represent integer values
    Long(i64),                      // Represent long values
    Float(f32),                     // Represent float values
    Double(f64),                    // Represent double values
    Reference(String),              // For object references
    Array(Rc<RefCell<Vec<Value>>>), // Support arrays
    Null,
}
