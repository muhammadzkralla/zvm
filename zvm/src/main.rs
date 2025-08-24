use crate::{parser::reader::Reader, vm::vm::Vm};
mod parser;
mod vm;

fn main() {
    let mut reader = Reader::new();
    reader.read();
    reader.print();

    let class_file = reader.get_class_file();

    let mut jvm = Vm::new();
    jvm.run(class_file);
}
