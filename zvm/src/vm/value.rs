/// JVM Value types
#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Object(String),    // Simplified, just store string representation
    Reference(String), // For object references
    Array(Vec<Value>), // Support arrays
}
