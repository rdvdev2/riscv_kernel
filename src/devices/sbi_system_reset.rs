use core::cell::OnceCell;

use spin::Mutex;

use crate::sbi::{
    base::sbi_probe_extension,
    system_reset::{self, sbi_system_reset, SystemResetReason, SystemResetType},
};

pub static GLOBAL_SBI_SYSTEM_RESET: Mutex<OnceCell<SbiSystemReset>> = Mutex::new(OnceCell::new());

#[derive(Clone, Copy)]
pub struct SbiSystemReset {
    _private: (),
}

impl SbiSystemReset {
    pub fn get() -> Option<Self> {
        if sbi_probe_extension(system_reset::EID) == 1 {
            Some(Self { _private: () })
        } else {
            None
        }
    }

    pub fn init() {
        let _ = GLOBAL_SBI_SYSTEM_RESET
            .lock()
            .set(Self::get().expect("System reset should be available"));
    }

    fn try_sbi_and_spin(reset_type: SystemResetType, reset_reason: SystemResetReason) -> ! {
        match sbi_system_reset(reset_type, reset_reason) {
            Err(_) => loop {},
        }
    }

    pub fn shutdown(&self) -> ! {
        Self::try_sbi_and_spin(SystemResetType::Shutdown, SystemResetReason::NoReason)
    }

    pub fn failure_shutdown(&self) -> ! {
        Self::try_sbi_and_spin(SystemResetType::Shutdown, SystemResetReason::SystemFailure)
    }
}
