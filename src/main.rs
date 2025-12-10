#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod mm;
mod sbi;
mod sync;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

/// Clear BSS segment
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

/// Main kernel entry point
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[KERNEL] Rust-based POSIX-compatible Kernel");
    println!("[KERNEL] Starting initialization...");

    mm::init();

    println!("[KERNEL] All initialization complete!");
    println!(
        "[KERNEL] Memory size: {} MB",
        mm::memory_size() / (1024 * 1024)
    );

    panic!("Shutdown machine!");
}
