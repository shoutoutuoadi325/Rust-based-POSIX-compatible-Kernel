//! Language items for #![no_std] kernel

use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("[KERNEL PANIC] at {}:{}", location.file(), location.line());
    } else {
        println!("[KERNEL PANIC]");
    }
    if let Some(message) = info.message() {
        println!("{}", message);
    }
    shutdown()
}

#[no_mangle]
#[link_section = ".text.abort"]
pub extern "C" fn abort() -> ! {
    panic!("abort!");
}
