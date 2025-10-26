use std::{
    alloc::{Layout, alloc},
    mem,
    ptr::NonNull,
};

pub struct Heap {
    pub heap_size: usize,
    pub free_list: Vec<Block>,
    pub starting_pointer: NonNull<u8>,
    pub layout: Layout,
}

#[derive(Debug, Clone, Copy)]
pub struct Block {
    size: usize,
    free: bool,
    ptr: NonNull<u8>,
}

#[derive(Debug)]
pub struct HeapStats {
    pub total_size: usize,
    pub allocated: usize,
    pub free: usize,
    pub num_blocks: usize,
    pub num_allocated_blocks: usize,
    pub num_free_blocks: usize,
}

impl Heap {
    pub fn new(size: usize) -> Self {
        // Initialize memory layout aligner
        let layout = Layout::from_size_align(size, 8).expect("Failed to create layout!");

        // Initialize memory starting pointer
        let starting_pointer = unsafe {
            let ptr = alloc(layout);
            NonNull::new_unchecked(ptr)
        };

        // Initialize one large block in the free list
        let block = Block {
            size: size,
            free: true,
            ptr: starting_pointer,
        };

        // Now we can return the heap instance
        Self {
            heap_size: size,
            free_list: vec![block],
            starting_pointer: starting_pointer,
            layout: layout,
        }
    }

    pub fn zmalloc(&mut self, size: usize) -> Option<NonNull<u8>> {
        // Align memory. From ZHMM, alignment macro was:
        // #define ALIGN(size) (((size) + (sizeof(void*) - 1)) & ~(sizeof(void*) - 1))
        let sizeof_usize = mem::size_of::<usize>();
        let aligned_size = (size + (sizeof_usize - 1)) & !(sizeof_usize - 1);

        // First fit algorithm to find the first fitting block
        for i in 0..self.free_list.len() {
            if self.free_list[i].free && self.free_list[i].size >= aligned_size {
                // Split the block if size is over-fitting
                if self.free_list[i].size > aligned_size {
                    self.split_blocks(i, self.free_list[i].size, aligned_size);
                } else {
                    self.free_list[i].free = false;
                }

                return Some(self.free_list[i].ptr);
            }
        }

        None
    }

    pub fn zfree(&mut self, ptr: NonNull<u8>) -> Result<(), &'static str> {
        // Find the block with this pointer
        for i in 0..self.free_list.len() {
            if self.free_list[i].ptr == ptr && !self.free_list[i].free {
                // Mark block as free
                self.free_list[i].free = true;

                // Coalesce adjacent free blocks
                self.coalesce(i);
                break;
            }
        }

        Ok(())
    }

    /// Returns statistics about the heap
    pub fn stats(&self) -> HeapStats {
        HeapStats {
            total_size: self.total_size(),
            allocated: self.allocated_size(),
            free: self.free_size(),
            num_blocks: self.num_blocks(),
            num_allocated_blocks: self.num_allocated_blocks(),
            num_free_blocks: self.num_free_blocks(),
        }
    }

    /// Merges adjacent free blocks to reduce fragmentation
    fn coalesce(&mut self, index: usize) {
        // Try to merge with next block
        if index + 1 < self.num_blocks() {
            // Adds an unsigned offset to a pointer.
            //
            // This can only move the pointer forward (or not move it). If you need to move forward or
            // backward depending on the value, then you might want [`offset`](#method.offset) instead
            // which takes a signed offset.
            let current_end = unsafe {
                self.free_list[index]
                    .ptr
                    .as_ptr()
                    .add(self.free_list[index].size)
            };

            let next_start = self.free_list[index + 1].ptr.as_ptr();

            // If current pointer ends at the same address as the next pointer
            // starts, and the next pointer is also free, merge them in one block
            if current_end == next_start && self.free_list[index + 1].free {
                self.free_list[index].size += self.free_list[index + 1].size;
                self.free_list.remove(index + 1);
            }
        }

        // Try to merge with previous block
        if index > 0 {
            // Adds an unsigned offset to a pointer.
            //
            // This can only move the pointer forward (or not move it). If you need to move forward or
            // backward depending on the value, then you might want [`offset`](#method.offset) instead
            // which takes a signed offset.
            let prev_end = unsafe {
                self.free_list[index - 1]
                    .ptr
                    .as_ptr()
                    .add(self.free_list[index - 1].size)
            };

            let current_start = self.free_list[index].ptr.as_ptr();

            // If previous pointer ends at the same address as the current pointer
            // starts, and the previous pointer is also free, merge them in one block
            if prev_end == current_start && self.free_list[index - 1].free {
                self.free_list[index - 1].size += self.free_list[index].size;
                self.free_list.remove(index);
            }
        }
    }

    fn split_blocks(&mut self, i: usize, block_size: usize, aligned_size: usize) {
        let remaining_size = block_size - aligned_size;

        // Update current block
        self.free_list[i].size = aligned_size;
        self.free_list[i].free = false;

        // Create new free block for remaining space
        let new_block = Block {
            ptr: unsafe {
                NonNull::new_unchecked(self.free_list[i].ptr.as_ptr().add(aligned_size))
            },
            size: remaining_size,
            free: true,
        };

        self.free_list.insert(i + 1, new_block);
    }

    /// Returns the total heap size
    fn total_size(&self) -> usize {
        self.heap_size
    }

    /// Returns the amount of allocated memory
    fn allocated_size(&self) -> usize {
        self.free_list
            .iter()
            .filter(|b| !b.free)
            .map(|b| b.size)
            .sum()
    }

    /// Returns the amount of free memory
    fn free_size(&self) -> usize {
        self.free_list
            .iter()
            .filter(|b| b.free)
            .map(|b| b.size)
            .sum()
    }

    fn num_blocks(&self) -> usize {
        self.free_list.len()
    }

    fn num_allocated_blocks(&self) -> usize {
        self.free_list.iter().filter(|b| !b.free).count()
    }

    fn num_free_blocks(&self) -> usize {
        self.free_list.iter().filter(|b| b.free).count()
    }
}
