use flat_device_tree::Fdt;
use linked_list_allocator::LockedHeap;

use self::paging::{align_down, align_up, GIGA_PAGE_SIZE};

pub mod memory_map;
pub mod paging;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

extern "C" {
    static kernel_start: u8;
    static kernel_end: u8;
}

pub fn init(fdt: &Fdt) {
    let aligned_kernel_start = align_down(unsafe { &kernel_start as *const u8 }, GIGA_PAGE_SIZE);
    let aligned_kernel_end = align_up(unsafe { &kernel_end as *const u8 }, GIGA_PAGE_SIZE);

    let mut memory_map = memory_map::MemoryMap::init(fdt, aligned_kernel_start..aligned_kernel_end);
    memory_map.reserve_kernel_dynamic(GIGA_PAGE_SIZE);

    unsafe { paging::init(&memory_map) };

    let heap = memory_map.get_kernel_dynamic().unwrap();
    // Safety: This is reached only once and the memory is properly initialized
    unsafe {
        ALLOCATOR.lock().init(
            heap.start as *mut u8,
            (heap.end as usize) - (heap.start as usize),
        )
    };
}

#[cfg(test)]
mod test {
    use alloc::boxed::Box;

    #[test_case]
    fn heap() {
        let my_box = Box::new(42);
        assert_eq!(*my_box, 42);
    }
}
