pub mod paging;

extern "C" {
    static kernel_end: u8;
}

pub fn init() {
    // Safety: kernel_end is calculated on the linking step
    unsafe { paging::init(&kernel_end as *const u8) };
}
