mod dtb;
pub mod sbi_debug_console;
pub mod sbi_system_reset;

// Safety: dtb_addr must be valid
pub unsafe fn init_devices(dtb_addr: *const u8) {
    sbi_debug_console::SbiDebugConsole::init();
    sbi_system_reset::SbiSystemReset::init();

    let _dtb = unsafe { dtb::get_dtb(dtb_addr) };
}
