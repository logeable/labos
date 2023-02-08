use buddy_system_allocator::LockedHeap;

const USER_HEAP_SIZE: usize = 1 << 14;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

pub fn init_heap() {
    let mut ha = HEAP_ALLOCATOR.lock();
    unsafe {
        ha.init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
}
