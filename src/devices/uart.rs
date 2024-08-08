use core::{cell::OnceCell, fmt::Write};

use spin::Mutex;

pub static GLOBAL_UART: Mutex<OnceCell<&mut Uart>> = Mutex::new(OnceCell::new());

const LS_DR: u8 = 1 << 0;

#[repr(C)]
pub struct Uart {
    buffer: u8,
    _f1: u8,
    _f2: u8,
    _f3: u8,
    _f4: u8,
    line_status: u8,
    _f6: u8,
}

impl Uart {
    // Safety: Caller must ensure this is the address of a ns16550a device
    // Safety: Caller must ensure this is called only once
    unsafe fn get(addr: *const u8) -> &'static mut Uart {
        unsafe { &mut *(addr as *mut Uart) }
    }

    // Safety: Caller must ensure this is the address of a ns16550a device
    pub fn init(addr: *const u8) {
        let _ = GLOBAL_UART.lock().set(unsafe { Self::get(addr) });
    }

    pub fn put_byte(&mut self, b: u8) {
        self.buffer = b;
    }

    pub fn put_string(&mut self, s: &str) {
        for b in s.bytes() {
            self.put_byte(b);
        }
    }

    pub fn get_byte(&mut self) -> Option<u8> {
        if self.line_status & LS_DR != 0 {
            Some(self.buffer)
        } else {
            None
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(self.put_string(s))
    }
}
