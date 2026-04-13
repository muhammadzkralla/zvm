# zvm

The Virtual Machine. In Rust. Because, Well, I Can

## Table of Contents

- [Overview](#overview)
- [Motivation](#motivation)
- [Architecture](#architecture)
- [Parser Components](#parser-components)
  - [Class File Reader](#class-file-reader)
  - [Class File Structure](#class-file-structure)
  - [Constant Pool](#constant-pool)
  - [Fields and Methods](#fields-and-methods)
  - [Attributes](#attributes)
  - [Opcode Enumeration](#opcode-enumeration)
- [Virtual Machine Components](#virtual-machine-components)
  - [Virtual Machine Core](#virtual-machine-core)
  - [Call Stack](#call-stack)
  - [Stack Frame](#stack-frame)
  - [Operand Stack](#operand-stack)
  - [Local Variables](#local-variables)
  - [Runtime Data Area](#runtime-data-area)
  - [Value Types](#value-types)
  - [Instruction Executor](#instruction-executor)
- [Instruction Set](#instruction-set)
- [Execution Model](#execution-model)
- [Installation and Usage](#installation-and-usage)

## Overview

`ZVM` is an educational, single-threaded, garbage-collected implementation of the [official Oracle Java Virtual Machine specifications](https://docs.oracle.com/javase/specs/jvms/se8/html/index.html).

Built from scratch in `Rust` without external dependencies, it provides a hands-on understanding of how a `JVM` works internally.

Every component from class file parsing to bytecode execution is implemented manually to maximize learning value.

## Motivation

`ZVM` exists as a deep dive into virtual machine architecture and bytecode execution. The primary goal is to understand how the `JVM` interprets compiled Java programs, manages memory, and handles the various data types defined in the specification.

Building in `Rust` also provides an opportunity to explore systems-level programming concepts such as memory safety, type conversions, and low-level data manipulation.

Everything from the class file parser to the instruction executor is implemented from scratch.

The implementation follows the `JVM` specification with focus on edge cases such as integer overflow wrapping, floating-point special values, and proper sign extension during type conversions.

## Architecture

The project is organized into two main modules: the parser and the virtual machine.

The parser handles reading Java class files according to the `JVM` class file format specification.

The virtual machine handles bytecode execution, managing the call stack, operand stack, local variables, runtime data area, and more.

## Parser Components

The parser reads binary Java class files and converts them into in-memory data structures that the virtual machine can execute.

### Class File Reader

The reader module provides low-level byte reading utilities that handle big-endian byte order as required by the `JVM` specification. It reads raw bytes from class files and converts them into the appropriate integer and numeric types used throughout the parser.

### Class File Structure

The class file structure holds the parsed contents. The `JVM` class file format follows a specific layout that the parser must interpret:

```
ClassFile {
    u4 magic;                    // 0xCAFEBABE
    u2 minor_version;
    u2 major_version;
    u2 constant_pool_count;
    cp_info constant_pool[constant_pool_count - 1];
    u2 access_flags;
    u2 this_class;
    u2 super_class;
    u2 interfaces_count;
    u2 interfaces[interfaces_count];
    u2 fields_count;
    field_info fields[fields_count];
    u2 methods_count;
    method_info methods[methods_count];
    u2 attributes_count;
    attribute_info attributes[attributes_count];
}
```

In code, this translates to a struct holding each field:

```rust
pub struct ClassFile {
    pub magic: u32,
    pub minor: u16,
    pub major: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<CpInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}
```

### Constant Pool

The constant pool info module represents the various constant pool entry types defined in the `JVM` specification. It includes integers, longs, floats, doubles, UTF-8 strings, class references, field and method references, name and type descriptors, and others.

The constant pool serves as a repository of all symbolic information needed by the class.

### Fields and Methods

The field info and method info modules represent the fields and methods declared in a class. Each contains access flags describing visibility and properties, name and descriptor indices pointing into the constant pool, and attribute information. Methods carry the `Code` attribute which contains the executable bytecode, maximum stack depth, and local variable count.

### Attributes

The attribute info module handles the various attribute types that can appear in a class file. The `Code` attribute is the most significant, containing the executable bytecode for methods, exception table for try-catch handling, and line number information for debugging.

### Opcode Enumeration

The opcode module enumerates all `JVM` bytecode opcodes with their numeric values. It provides conversion from raw bytes to the enumerated type, mapping each instruction code to its symbolic name:

```rust
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    Nop = 0x00,
    Iconst0 = 0x03,
    Iload = 0x15,
    Iadd = 0x60,
    Ireturn = 0xAC,
    // ... many more
}
```

## Virtual Machine Components

The virtual machine executes bytecode instructions parsed from class files. It manages memory, executes instructions, and coordinates method calls.

### Virtual Machine Core

The `vm` module serves as the main entry point, coordinating class file loading, method execution initialization, and the overall execution lifecycle. It handles locating the `main` method, executing the class static initializer, and managing the transition to bytecode execution.

### Call Stack

The call stack module manages the stack of method invocation frames. When a method is called, a new frame is pushed onto the stack. When a method returns, its frame is popped.

The call stack maintains a configurable maximum depth to prevent unbounded stack growth.

### Stack Frame

The stack frame module represents a single method invocation frame. Each frame contains the method's bytecode, an operand stack for storing intermediate values, a local variables array for holding method arguments and temporary values, and a program counter tracking the position in the bytecode:

```rust
pub struct Frame {
    pub method_name: Option<String>,
    pub operand_stack: OperandStack,
    pub local_variables: LocalVariables,
    pub pc: usize,
    pub bytecode: Vec<u8>,
}
```

### Operand Stack

The operand stack module implements a stack data structure used for pushing and popping values during bytecode execution. It supports all `JVM` value types including `int`, `long`, `float`, `double`, and object references. The operand stack is the primary workspace for instruction execution:

```rust
pub struct OperandStack {
    stack: Vec<Value>,
}

impl OperandStack {
    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }
}
```

### Local Variables

The local variables module represents the array of local variables available within a method frame. Variables are accessed by slot number, with the first few slots typically holding method arguments.

The specification requires that `long` and `double` values occupy two consecutive slots, while all other types occupy a single slot.

### Runtime Data Area

The runtime data area module manages the runtime state of the virtual machine. It maintains static fields for each loaded class, storing class-level data that persists across method invocations.

The runtime data area also serves as the heap for allocating object and array instances.

### Value Types

The value module defines the discriminated union of all `JVM` value types. It represents integers, longs, floats, doubles, object references, array references, and the null value:

```rust
#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Reference(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Null,
}
```

### Instruction Executor

The instruction executor module contains the core bytecode dispatch and execution logic. It interprets each opcode and performs the corresponding operation on the operand stack, local variables, or runtime data area.

The executor maintains the instruction fetch-decode-execute cycle:

```rust
while current_pc < bytecode.len() {
    let opcode = Opcode::from(bytecode[current_pc]);
    
    match executor.execute_instruction(
        opcode,
        self,
        class_file,
        runtime_data_area,
        call_stack,
        &mut current_pc,
    ) {
        Ok(InstructionCompleted::ReturnFromMethod(v)) => return Ok(v),
        Ok(InstructionCompleted::ContinueMethodExecution) => continue,
        Err(e) => return Err(e),
    }
    
    current_pc += 1;
}
```

## Instruction Set

The implementation supports a comprehensive subset of `JVM` bytecode instructions covering all major categories defined in the specification.

- **Constant instructions**: `iconst`, `lconst`, `fconst`, `dconst`, `bipush`, `sipush`, `ldc` variants for loading constant values onto the operand stack.
- **Load and store instructions**: `iload`, `lload`, `fload`, `dload`, `aload` and their indexed variants for accessing local variables.
- **Arithmetic instructions**: `add`, `sub`, `mul`, `div`, `rem`, `neg` for each numeric type, plus `shl`, `shr`, `ushr`, `and`, `or`, `xor` for integer and long types.
- **Type conversion instructions**: `i2l`, `i2f`, `i2d`, `l2i`, `l2f`, `l2d`, `f2i`, `f2l`, `f2d`, `d2i`, `d2l`, `d2f`, plus narrowing conversions `i2b`, `i2c`, `i2s`.
- **Comparison instructions**: `lcmp`, `fcmpl`, `fcmpg`, `dcmpl`, `dcmpg` for comparing long, float, and double values.
- **Branch instructions**: `ifeq`, `ifne`, `iflt`, `ifge`, `ifgt`, `ifle`, `if_icmpeq`, `if_icmpne`, `if_icmplt`, `if_icmpge`, `if_icmpgt`, `if_icmple`, `ifnull`, `ifnonnull`, `goto`.
- **Control flow**: `invokevirtual`, `invokespecial`, `invokestatic` for method invocation, and `ireturn`, `lreturn`, `freturn`, `dreturn`, `areturn`, `return` for method return.
- **Field access**: `getstatic`, `putstatic` for accessing static fields on classes.
- **Array instructions**: `newarray` for creation, `arraylength` for length, and various `aload`/`astore` variants for element access.
- **Stack manipulation**: `pop`, `pop2`, `dup`, `dup_x1`, `dup_x2`, `dup2`, `swap` for controlling the operand stack structure.

The implementation handles all JVM data types including the primitive types `int`, `long`, `float`, `double`, `byte`, `char`, `short`, `boolean`, as well as reference types for object and array references.

## Execution Model

The virtual machine executes instructions sequentially within each stack frame, maintaining a program counter that tracks the position in the bytecode. When a method is invoked, a new frame is pushed onto the call stack with its own operand stack and local variables array. Method invocation instructions push new frames, and return instructions pop the current frame and optionally push a return value onto the caller's operand stack.

The garbage collection and memory management are not yet implemented. This is the next major area of work for the project. The runtime data area provides the heap structure for allocating object and array instances, but the actual reclamation of unused memory through garbage collection remains to be implemented. All execution occurs in a single thread as defined by the specification for the base execution model.

The virtual machine loads a class file, locates the `main` method, initializes the class by executing the static initializer if present, and then begins executing bytecode from the main method entry point.

## Installation and Usage

To build and run `ZVM`, ensure you have `Rust` installed. Clone the repository and navigate to the project directory.

```bash
cd zvm
cargo build --release
```

This produces an executable at `./target/release/zvm`. You can run a Java class file by executing the binary directly:

```bash
./target/release/zvm Main
```

You can also pass arguments to the Java program:

```bash
./target/release/zvm Main arg1 arg2
```

Alternatively, you can run using `cargo run`:

```bash
cargo run --release -- <class_file> [args]
```

For example:

```bash
cargo run --release -- HelloWorld
```

Enable debug logging during development to see detailed execution information:

```bash
cargo run --features debug-logging -- <class_file> [args]
```

Or with the compiled binary:

```bash
./target/release/zvm --features debug-logging Main
```
