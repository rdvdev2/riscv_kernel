use flat_device_tree::{node::FdtNode, Fdt};

mod dtb;
pub mod syscon;
pub mod uart;

pub fn init_devices() {
    let dtb = dtb::get_dtb();

    syscon::Syscon::init();

    if let Some(node) = uart::Uart::find_compatible(&dtb) {
        if let Some(uart_device) = unsafe { uart::Uart::get(&node) } {
            uart::GLOBAL_UART
                .lock()
                .set(uart_device)
                .expect("Global UART already set");
        }
    }
}

pub trait Device {
    fn find_compatible<'a, 'b>(fdt: &'b Fdt<'a>) -> Option<FdtNode<'b, 'a>>;
    unsafe fn get(fdt_node: &FdtNode) -> Option<&'static mut Self>;
}

pub trait OutDevice<Message>: Device {
    fn send(&mut self, message: Message);
}

pub trait InDevice<Message>: Device {
    fn receive(&mut self) -> Message;
}
