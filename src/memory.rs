use flat_device_tree::Fdt;

use self::paging::{align_down, align_up, GIGA_PAGE_SIZE};

pub mod memory_map;
pub mod paging;

extern "C" {
    static kernel_start: u8;
    static kernel_end: u8;
}

pub fn init(fdt: &Fdt) {
    let aligned_kernel_start = align_down(unsafe { &kernel_start as *const u8 }, GIGA_PAGE_SIZE);
    let aligned_kernel_end = align_up(unsafe { &kernel_end as *const u8 }, GIGA_PAGE_SIZE);

    let memory_map = memory_map::MemoryMap::init(fdt, aligned_kernel_start..aligned_kernel_end);

    // Safety: kernel_end is calculated on the linking step
    unsafe { paging::init(&memory_map) };
}
