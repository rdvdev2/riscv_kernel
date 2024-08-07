use crate::println;

pub extern "C" fn umain() -> ! {
    println!("Hello from userland!\n");
    loop {}
}
