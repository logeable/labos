mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    memory_set::init_memory_set();
}

pub use address::PhysPageNum;
pub use address::VirtAddr;
pub use memory_set::MapPermission;
pub use memory_set::MemorySet;
pub use memory_set::KERNEL_SPACE;
pub use page_table::translated_byte_buffer;
