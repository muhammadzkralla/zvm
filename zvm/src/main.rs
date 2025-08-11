use crate::parser::reader::Reader;
mod parser;

fn main() {
    let mut reader = Reader::new();
    reader.read();
    reader.print();
}
