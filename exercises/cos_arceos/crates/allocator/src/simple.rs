//! Simple memory allocation.
//!
//! TODO: more efficient





use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator{
    start: usize,
    size: usize,
    allocations: usize,
    next: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            size: 0,
            allocations: 0,
            next: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
       self.start=_start;
       self.size=_size;
       self.next=_start;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(crate::AllocError::NoMemory)
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        let align = _layout.align();
        let size = _layout.size();

        // Calculate the number of bytes needed to satisfy the allocation
      //  let required_bytes = size + (align - 1);
        let mask = !(align - 1);
        let start = (self.next + (align - 1)) & mask;

        // Check if the allocation would exceed the available memory
        if start+size > self.start + self.size {
            return  Err(crate::AllocError::NoMemory);
        }
        self.next = start+size;

        // Increment the allocations count
        self.allocations += 1;

        // Return the allocated address
        Ok(NonZeroUsize::new(start).unwrap())
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        if self.allocations>0
        {
            self.allocations -= 1;
        }
        

    }

    fn total_bytes(&self) -> usize {
        self.size
    }

    fn used_bytes(&self) -> usize {
        self.next-self.start
    }

    fn available_bytes(&self) -> usize {
        self.size - self.used_bytes()
    }
}
