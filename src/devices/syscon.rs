use core::cell::OnceCell;

use crate::spin::SpinLock;

pub static GLOBAL_SYSCON: SpinLock<OnceCell<&mut Syscon>> = SpinLock::new(OnceCell::new());

#[repr(transparent)]
pub struct Syscon(u16);

impl Syscon {
    // Safety: Caller must ensure this is called only once
    unsafe fn get() -> &'static mut Self {
        unsafe { &mut *(0x100000 as *mut Self) }
    }

    pub fn init() {
        let _ = GLOBAL_SYSCON.acquire().set(unsafe { Self::get() });
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
