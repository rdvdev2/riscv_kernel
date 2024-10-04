use core::ops::{Range, RangeBounds};

use flat_device_tree::Fdt;

use crate::memory::paging::GIGA_PAGE_SIZE;

pub struct MemoryMap {
    physical_space: Range<*const u8>,
    // NOTE: Platform is assumed to comprehend all the space below the kernel
    platform: Range<*const u8>,
    // NOTE: We will consider the bootloader to be part of kernel_static
    kernel_static: Range<*const u8>,
    kernel_dynamic: Option<Range<*const u8>>,
}

impl MemoryMap {
    pub fn init(fdt: &Fdt, kernel_static: Range<*const u8>) -> Self {
        let physhical_space = fdt.memory().unwrap().regions().next().unwrap();
        let physical_space = physhical_space.starting_address
            ..(physhical_space
                .starting_address
                .wrapping_add(physhical_space.size.unwrap()));

        assert!(kernel_static.start.is_aligned_to(GIGA_PAGE_SIZE));
        assert!(kernel_static.end.is_aligned_to(GIGA_PAGE_SIZE));

        assert!(physical_space.contains(&kernel_static.start));
        assert!(physical_space.contains(&kernel_static.end));

        Self {
            physical_space,
            platform: (0 as *const u8)..kernel_static.end,
            kernel_static,
            kernel_dynamic: None,
        }
    }

    pub fn get_platform(&self) -> &Range<*const u8> {
        &self.platform
    }

    pub fn get_kernel_static(&self) -> &Range<*const u8> {
        &self.kernel_static
    }

    pub fn get_kernel_dynamic(&self) -> Option<&Range<*const u8>> {
        self.kernel_dynamic.as_ref()
    }
}
