mod dtb;
pub mod sbi_debug_console;
pub mod syscon;

pub fn init_devices() {
    let _dtb = dtb::get_dtb();

    sbi_debug_console::SbiDebugConsole::init();
    syscon::Syscon::init();
}
