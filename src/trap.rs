use core::arch::asm;

mod cause;

extern "C" {
    fn trap_entry();
}

pub fn init() {
    unsafe { register_trap_entry(trap_entry as usize) };
}

#[no_mangle]
extern "C" fn ktrap_entry() {
    panic!("Trapped: {:?}", cause::TrapCause::get());
}

// Safety: Caller must ensure the function_address is a valid trap handler, which
// saves and restores all state, and that is 4-byte aligned.
unsafe fn register_trap_entry(function_address: usize) {
    asm!("csrw stvec, {ptr}", ptr = in(reg) function_address);
}
