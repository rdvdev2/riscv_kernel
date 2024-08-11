use core::{fmt::Write, panic::PanicInfo};

use crate::{devices::sbi_system_reset::GLOBAL_SBI_SYSTEM_RESET, print, println};

pub trait TestCase {
    fn run(&self) -> ();
}

impl<T> TestCase for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("OKAY")
    }
}

pub fn test_runner(tests: &[&dyn TestCase]) {
    println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    println!("Done");

    GLOBAL_SBI_SYSTEM_RESET.lock().get_mut().unwrap().shutdown();
}

pub fn panic_hook<W: Write>(_panic_info: &PanicInfo, output_device: &mut W) {
    let _ = writeln!(output_device, "FAIL");
}
