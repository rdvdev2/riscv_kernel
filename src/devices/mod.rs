use flat_device_tree::{node::FdtNode, Fdt};

mod dtb;
pub mod sbi_debug_console;
pub mod syscon;

pub fn init_devices() {
    let _dtb = dtb::get_dtb();

    sbi_debug_console::SbiDebugConsole::init();
    syscon::Syscon::init();
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
