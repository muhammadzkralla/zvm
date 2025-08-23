use crate::parser::reader::Reader;
mod parser;
mod vm;

fn main() {
    let mut reader = Reader::new();
    reader.read();
    reader.print();
}
