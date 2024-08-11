#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(assert_matches)]
#![test_runner(crate::test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;

use devices::{sbi_debug_console::GLOBAL_DEBUG_CONSOLE, syscon::GLOBAL_SYSCON};
use sbi::base::{sbi_get_impl_id, sbi_get_spec_version};

mod devices;
mod io;
mod sbi;
mod user;

#[cfg(test)]
mod test_framework;

#[no_mangle]
extern "C" fn kmain() -> ! {
    devices::init_devices();

    let sbi_version = sbi_get_spec_version();
    let sbi_implementation_id = sbi_get_impl_id();
    println!(
        "Running on {}, with SBI v{}",
        sbi_implementation_id, sbi_version
    );

    #[cfg(test)]
    test_main();

    println!("System initialization done, jumping to usermode...");

    unsafe { user_mode_jump(user::umain as usize) };
}

// Safety: Caller must ensure the function_address is the start of a C-Style
// function in user code, and that the function diverges.
unsafe fn user_mode_jump(function_address: usize) -> ! {
    asm!("csrw sepc, {ptr}", "sret", ptr = in(reg) function_address, options(noreturn));
}

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    // Safety: The thread panicked, therefore the handle won't be used anymore.
    unsafe {
        GLOBAL_DEBUG_CONSOLE.force_unlock();
        GLOBAL_SYSCON.force_unlock();
    }

    let mut debug_console_handle = GLOBAL_DEBUG_CONSOLE.lock();
    let mut syscon_handle = GLOBAL_SYSCON.lock();

    // Panic could happen before UART initialization
    if let Some(debug_console) = debug_console_handle.get_mut() {
        #[cfg(test)]
        test_framework::panic_hook(panic_info, debug_console);

        #[cfg(not(test))]
        let _ = writeln!(debug_console, "\n--- KERNEL PANIC! ---");

        let _ = writeln!(debug_console, "{}", panic_info);
    }

    // Panic could happen before SYSCON initialization
    if let Some(syscon) = syscon_handle.get_mut() {
        syscon.shutdown();
    }

    loop {}
}

#[cfg(test)]
mod test {
    #[test_case]
    fn trivial() {
        assert_eq!(1, 1)
    }
}
