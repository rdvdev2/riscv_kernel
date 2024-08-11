use crate::devices::sbi_debug_console::GLOBAL_DEBUG_CONSOLE;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;

    GLOBAL_DEBUG_CONSOLE
        .lock()
        .get_mut()
        .unwrap()
        .write_fmt(args)
        .unwrap();
}
