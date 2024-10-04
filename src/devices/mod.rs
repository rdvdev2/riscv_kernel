use flat_device_tree::Fdt;

mod dtb;
pub mod sbi_debug_console;
pub mod sbi_system_reset;

// Safety: dtb_addr must be valid
pub unsafe fn init_devices<'a>(dtb_addr: *const u8) -> Fdt<'a> {
    sbi_debug_console::SbiDebugConsole::init();
    sbi_system_reset::SbiSystemReset::init();

    let dtb = unsafe { dtb::get_dtb(dtb_addr) };

    // TODO: More initializations

    dtb
}
