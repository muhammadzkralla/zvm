use std::env;

use crate::{parser::reader::Reader, vm::vm::Vm};
mod parser;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    let class_file_path = &args[1];

    let mut reader = Reader::new(class_file_path.clone());
    reader.read();
    reader.print();

    let class_file = reader.get_class_file();

    let env_args: Vec<String> = args[2..].to_vec();

    let mut jvm = Vm::new();
    jvm.run(class_file, env_args);
}
