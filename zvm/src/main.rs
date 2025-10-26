use std::env;

use crate::{parser::reader::Reader, vm::vm::Vm};
mod parser;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    let class_file_path = &args[1];

    let mut reader = Reader::new(class_file_path.clone());
    reader.read();

    let class_file = reader.get_class_file();

    let env_args: Vec<String> = args[2..].to_vec();

    let mut jvm = Vm::new();
    jvm.run(class_file, env_args);
}
//
// use std::{env, mem};
//
// use crate::{
//     parser::reader::Reader,
//     vm::{
//         heap::{self, Block, Heap},
//         vm::Vm,
//     },
// };
// mod parser;
// mod vm;
//
// fn main() {
//     let block_size = mem::size_of::<Block>();
//     let usize_size = mem::size_of::<usize>();
//
//     println!("{}", block_size);
//     println!("{}", usize_size);
//
//     let mut heap = Heap::new(4096);
//     let free_list_size = heap.free_list.len();
//     println!("{}", free_list_size);
//
//     println!("Initial heap stats: {:?}", heap.stats());
//
//     // Allocate some memory
//     let ptr1 = heap.zmalloc(256).expect("Allocation 1 failed");
//     let ptr2 = heap.zmalloc(512).expect("Allocation 2 failed");
//     let ptr3 = heap.zmalloc(128).expect("Allocation 3 failed");
//
//     println!("\nAfter allocations: {:?}", heap.stats());
//
//     // Free some memory
//     heap.zfree(ptr2).expect("Deallocation failed");
//     println!("\nAfter freeing ptr2: {:?}", heap.stats());
//
//     // Allocate again (should reuse freed space)
//     let ptr4 = heap.zmalloc(512).expect("Allocation 4 failed");
//     println!("\nAfter reallocation: {:?}", heap.stats());
//
//     // Cleanup
//     heap.zfree(ptr1).expect("Deallocation failed");
//     heap.zfree(ptr3).expect("Deallocation failed");
//     heap.zfree(ptr4).expect("Deallocation failed");
//
//     println!("\nFinal heap stats: {:?}", heap.stats());
//
//     let ptr5 = heap.zmalloc(1).expect("Allocation 5 failed");
//     println!("\nFinal heap stats: {:?}", heap.stats());
// }
