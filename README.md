# zvm
The Virtual Machine. In Rust... Because, Well, I Can

## Introduction

**ZVM** (pronounced "The Virtual Machine") is an educational, single-threaded, garbage-collected, zero-dependency implementation of the [official JVM virtual machine specifications by Oracle](https://docs.oracle.com/javase/specs/jvms/se8/html/index.html).

Built from the ground up in Rust, ZVM aims to demystify how the Java Virtual Machine works under the hood by implementing core JVM components without relying on external dependencies. From parsing class files and managing the constant pool to executing bytecode instructions and handling the operand stack, every aspect is implemented manually to provide maximum learning value.

This project follows the official JVM specification closely, with careful attention to detail on instruction semantics, type conversions, arithmetic overflow behavior, and standards like the IEEE 754 floating-point operations for example, and more. Whether you're curious about how virtual machines work or exploring systems programming in Rust, ZVM offers a hands-on, readable implementation that's small enough to understand yet complete enough to run real Java programs' bytecode.

## Why ZVM?

ZVM was created as a deep dive into virtual machine architecture and bytecode execution. The goal was to understand exactly how the JVM interprets compiled Java programs, manages memory, and handles the various data types and operations defined in the specification. By building it in Rust, the project also explores systems-level programming concepts like memory safety, type conversions, and low-level data manipulation.

I decided to implement everything from scratch, including the class file parser, instruction executor, and runtime data structures, to gain a thorough understanding of each component. The implementation strictly follows the JVM specification, with particular focus on edge cases like integer overflow wrapping, floating-point special values, proper sign extension during type conversions, and others.

Whether you're learning about virtual machines, exploring Rust's systems programming capabilities, or just curious about what happens when you run a Java program, ZVM is designed to be approachable and educational.

## Features

### Class File Parsing

### Instruction Set
