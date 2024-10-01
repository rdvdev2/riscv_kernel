use core::arch::asm;

use bitfield::bitfield;

const PAGE_SIZE: usize = 1 << 12;
const MEGA_PAGE_SIZE: usize = PAGE_SIZE << 9;
const GIGA_PAGE_SIZE: usize = MEGA_PAGE_SIZE << 9;

// Safety: No address greater than kernel_end should be in use, all
// physical pages following this address will be elegible for allocation.
pub unsafe fn init(kernel_end: *const u8) {
    // For now we will limit ourselves to enable paging with identity mapping.
    // No permissions for U-mode
    let pt = PageTable::new_giga_identity_minimal(kernel_end);
    unsafe { pt.install_as_root(0) };
}

#[repr(align(4096))]
#[derive(Debug, Clone)]
pub struct PageTable([PageTableEntry; 512]);

impl PageTable {
    // Safety: addr must be page-aligned, and will be permanently allocated.
    pub unsafe fn new(addr: *const u8) -> &'static mut Self {
        let pt = unsafe { &mut *(addr as *mut Self) };
        for pte in pt.0.iter_mut() {
            *pte = PageTableEntry::new();
        }
        pt
    }

    // Safety: No address greater than kernel_end should be in use, all
    // physical pages following this address will be elegible for allocation.
    pub unsafe fn new_giga_identity_minimal(kernel_end: *const u8) -> &'static mut Self {
        let kernel_end = kernel_end as usize;
        let next_page = kernel_end - (kernel_end % PAGE_SIZE) + PAGE_SIZE;
        let next_page = next_page as *const u8;

        let pt = unsafe { Self::new(next_page) };

        for (idx, pte) in pt.0.iter_mut().enumerate() {
            let page_start = idx * GIGA_PAGE_SIZE;
            pte.set_permisions(PTEPermissionCombo::ReadWriteExecute);
            pte.set_ppn_2(idx as u32);
            pte.set_valid(page_start < (next_page as usize));
        }

        let valid_count = pt.0.iter().filter(|pte| pte.is_valid()).count();
        crate::println!(
            "Initialized identity paging with {} valid pages.",
            valid_count
        );

        pt
    }

    pub unsafe fn install_as_root(&'static self, asid: u16) {
        let ppn = ((self as *const _) as usize) >> 12;
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
    pub fn new() -> Self {
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
