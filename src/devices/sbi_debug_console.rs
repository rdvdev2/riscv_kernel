use core::{cell::OnceCell, fmt::Write};

use spin::Mutex;

use crate::sbi::{
    base::sbi_probe_extension,
    debug_console::{self, sbi_debug_console_write},
};

pub static GLOBAL_SBI_DEBUG_CONSOLE: Mutex<OnceCell<SbiDebugConsole>> = Mutex::new(OnceCell::new());

#[derive(Clone, Copy)]
pub struct SbiDebugConsole {
    _private: (),
}

impl SbiDebugConsole {
    pub fn get() -> Option<Self> {
        if sbi_probe_extension(debug_console::EID) == 1 {
            Some(Self { _private: () })
        } else {
            None
        }
    }

    pub fn init() {
        let _ = GLOBAL_SBI_DEBUG_CONSOLE
            .lock()
            .set(Self::get().expect("Debug console should be available"));
    }
}

impl Write for SbiDebugConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        sbi_debug_console_write(s).unwrap();
        Ok(())
    }
}
