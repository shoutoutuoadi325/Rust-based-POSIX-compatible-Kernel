//! Memory management module
//! Handles physical frame allocation, virtual memory, and heap allocation

mod address;
mod frame_allocator;
mod heap_allocator;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
pub use frame_allocator::{frame_alloc, frame_dealloc, FrameTracker};
pub use page_table::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, PTEFlags, PageTable,
    PageTableEntry,
};

use crate::config::MEMORY_END;

/// Initialize memory management system
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    println!("[KERNEL] Memory management initialized");
}

/// Get physical memory size
pub fn memory_size() -> usize {
    MEMORY_END
}
