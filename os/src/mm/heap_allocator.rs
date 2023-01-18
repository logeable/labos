use buddy_system_allocator::LockedHeap;

use crate::config::KERNEL_HEAP_SIZE;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init_heap() {
    let mut ha = HEAP_ALLOCATOR.lock();
    unsafe {
        ha.init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
    println!(
        "KERNEL HEAP total = {}, alloc = {}",
        ha.stats_total_bytes(),
        ha.stats_alloc_actual()
    )
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
