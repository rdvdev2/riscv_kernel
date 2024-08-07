mod dtb;
pub mod syscon;
pub mod uart;

pub fn init_devices() {
    let dtb = dtb::get_dtb();

    syscon::Syscon::init();

    if let Some(node) = dtb.find_compatible(&["ns16550a"]) {
        if let Some(uart_addr) = node.reg().next() {
            uart::Uart::init(uart_addr.starting_address);
        }
    }
}
