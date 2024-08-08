use core::cell::OnceCell;

use spin::Mutex;

pub static GLOBAL_SYSCON: Mutex<OnceCell<&mut Syscon>> = Mutex::new(OnceCell::new());

#[repr(transparent)]
pub struct Syscon(u16);

impl Syscon {
    // Safety: Caller must ensure this is called only once
    unsafe fn get() -> &'static mut Self {
        unsafe { &mut *(0x100000 as *mut Self) }
    }

    pub fn init() {
        let _ = GLOBAL_SYSCON.lock().set(unsafe { Self::get() });
    }

    pub fn shutdown(&mut self) -> ! {
        self.0 = 0x5555;
        loop {}
    }

    pub fn reboot(&mut self) -> ! {
        self.0 = 0x7777;
        loop {}
    }
}
