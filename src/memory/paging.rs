use core::arch::asm;

use bitfield::bitfield;

use super::memory_map::MemoryMap;

pub const PAGE_SIZE: usize = 1 << 12;
pub const MEGA_PAGE_SIZE: usize = PAGE_SIZE << 9;
pub const GIGA_PAGE_SIZE: usize = MEGA_PAGE_SIZE << 9;

// TODO: Modification safety
static mut ROOT_PAGE_TABLE: PageTable = PageTable::new();

// Safety: This should be called only once
pub unsafe fn init(memory_map: &MemoryMap) {
    // For now we will limit ourselves to enable paging with identity mapping.
    // No permissions for U-mode
    PageTable::create_from_memory_map(memory_map);

    // Safety: The value is never moved
    unsafe { PageTable::install_as_root(&raw const ROOT_PAGE_TABLE, 0) };
}

pub fn align_down(address: *const u8, alignment: usize) -> *const u8 {
    let address = address as usize;

    let aligned = address - (address % alignment);
    aligned as *const u8
}

pub fn align_up(address: *const u8, alignment: usize) -> *const u8 {
    let address = address as usize;

    let aligned = address - (address % alignment) + alignment;
    aligned as *const u8
}

#[repr(align(4096))]
#[derive(Debug, Clone)]
pub struct PageTable([PageTableEntry; 512]);

impl PageTable {
    pub const fn new() -> Self {
        PageTable([PageTableEntry::new(); 512])
    }

    // Safety: This should be called only once
    pub unsafe fn create_from_memory_map(memory_map: &MemoryMap) {
        #[allow(static_mut_refs)]
        let pt = unsafe { &mut ROOT_PAGE_TABLE };

        let platform = memory_map.get_platform();
        let kernel_static = memory_map.get_kernel_static();
        let kernel_dynamic = memory_map.get_kernel_dynamic();

        for (idx, pte) in pt.0.iter_mut().enumerate() {
            let page_start = (idx * GIGA_PAGE_SIZE) as *const u8;
            pte.set_permisions(PTEPermissionCombo::ReadWriteExecute);
            pte.set_ppn_2(idx as u32);
            pte.set_valid(
                platform.contains(&page_start)
                    || kernel_static.contains(&page_start)
                    || kernel_dynamic.is_some_and(|x| x.contains(&page_start)),
            );
        }

        let valid_count = pt.0.iter().filter(|pte| pte.is_valid()).count();
        crate::println!(
            "Initialized identity paging with {} valid pages.",
            valid_count
        );
    }

    // Safety: Value can't be moved or destroyed while installed
    pub unsafe fn install_as_root(pt: *const Self, asid: u16) {
        let ppn = (pt as usize) >> 12;
        let mode = 8; // MODE=8 enable Sv39 paging

        let ppn = ppn as u64;
        let asid = asid as u64;
        let mode = mode as u64;

        let satp = (mode << 60) | (asid << 44) | ppn;
        asm!("csrw satp, {satp}", satp = in(reg) satp);
    }
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct PageTableEntry(u64);
    impl Debug;
    bool;
    pub is_valid, set_valid: 0;
    pub is_readable, set_readable: 1;
    pub is_writtable, set_writtable: 2;
    pub is_executable, set_executable: 3;
    pub u8, from into PTEPermissionCombo, _, set_permisions: 3, 1;
    pub is_user, set_user: 4;
    pub is_global, set_global: 5;
    pub is_accessed, set_accessed: 6;
    pub is_dirty, set_dirty: 7;
    pub u8, get_rsw, set_rsw: 9, 8;
    pub u16, get_ppn_0, set_ppn_0: 18, 10;
    pub u16, get_ppn_1, set_ppn_1: 27, 19;
    pub u32, get_ppn_2, set_ppn_2: 53, 28;
    pub u64, get_physical_page_number, set_physical_page_number: 53, 10;
}

impl PageTableEntry {
    pub const fn new() -> Self {
        Self(0)
    }
}

#[derive(Clone, Copy)]
pub enum PTEPermissionCombo {
    NextLevelPointer,
    ReadOnly,
    ReadWrite,
    ExecuteOnly,
    ReadExecute,
    ReadWriteExecute,
}

impl From<PTEPermissionCombo> for u8 {
    fn from(value: PTEPermissionCombo) -> Self {
        match value {
            PTEPermissionCombo::NextLevelPointer => 0b000,
            PTEPermissionCombo::ReadOnly => 0b001,
            PTEPermissionCombo::ReadWrite => 0b011,
            PTEPermissionCombo::ExecuteOnly => 0b100,
            PTEPermissionCombo::ReadExecute => 0b101,
            PTEPermissionCombo::ReadWriteExecute => 0b111,
        }
    }
}
