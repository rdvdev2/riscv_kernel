mod dtb;
pub mod sbi_debug_console;
pub mod sbi_system_reset;

pub fn init_devices() {
    let _dtb = dtb::get_dtb();

    sbi_debug_console::SbiDebugConsole::init();
    sbi_system_reset::SbiSystemReset::init();
}
