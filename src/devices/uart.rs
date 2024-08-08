use core::{cell::OnceCell, fmt::Write};

use flat_device_tree::{node::FdtNode, Fdt};
use spin::Mutex;

pub static GLOBAL_UART: Mutex<OnceCell<&mut Uart>> = Mutex::new(OnceCell::new());

const LS_DR: u8 = 1 << 0;

#[repr(C)]
#[derive(Debug)]
pub struct Uart {
    buffer: u8,
    _f1: u8,
    _f2: u8,
    _f3: u8,
    _f4: u8,
    line_status: u8,
    _f6: u8,
}

impl super::Device for Uart {
    fn find_compatible<'a, 'b>(fdt: &'b Fdt<'a>) -> Option<FdtNode<'b, 'a>> {
        fdt.find_compatible(&["ns16550a"])
    }

    unsafe fn get(fdt_node: &flat_device_tree::node::FdtNode) -> Option<&'static mut Self> {
        let addr = fdt_node.reg().next()?.starting_address;
        unsafe { Some(&mut *(addr as *mut Uart)) }
    }
}

impl super::OutDevice<u8> for Uart {
    fn send(&mut self, message: u8) {
        self.buffer = message;
    }
}

impl super::OutDevice<&str> for Uart {
    fn send(&mut self, message: &str) {
        for b in message.bytes() {
            super::OutDevice::send(self, b);
        }
    }
}

impl super::InDevice<Option<u8>> for Uart {
    fn receive(&mut self) -> Option<u8> {
        if self.line_status & LS_DR != 0 {
            Some(self.buffer)
        } else {
            None
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(super::OutDevice::send(self, s))
    }
}
