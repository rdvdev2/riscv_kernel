#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;

use devices::syscon::GLOBAL_SYSCON;
use devices::uart::GLOBAL_UART;

mod devices;
mod io;
mod spin;
mod user;

#[no_mangle]
extern "C" fn kmain() -> ! {
    devices::init_devices();

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
    let mut uart_handle = unsafe { GLOBAL_UART.acquire_unchecked() };
    let mut syscon_handle = unsafe { GLOBAL_SYSCON.acquire_unchecked() };

    // Panic could happen before UART initialization
    if let Some(uart) = uart_handle.get_mut() {
        let _ = writeln!(uart, "--- KERNEL PANIC! ---");
        let _ = writeln!(uart, "{}", panic_info);
    }

    // Panic could happen before SYSCON initialization
    if let Some(syscon) = syscon_handle.get_mut() {
        syscon.shutdown();
    }

    loop {}
}
