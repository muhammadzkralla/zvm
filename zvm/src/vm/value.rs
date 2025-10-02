/// JVM Value types
#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),          // Represent integer values
    Long(i64),         // Represent long values
    Float(f32),        // Represent float values
    Object(String),    // Simplified, just store string representation
    Reference(String), // For object references
    Array(Vec<Value>), // Support arrays
}
