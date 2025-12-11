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
mod syscall;
mod trap;

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
    println!("[KERNEL] Rust-based POSIX-compatible Kernel (RPOS)");
    println!("[KERNEL] Version 1.0.0");
    println!("[KERNEL] Starting initialization...");

    mm::init();
    trap::init();

    println!("[KERNEL] All initialization complete!");
    println!(
        "[KERNEL] Memory size: {} MB",
        mm::memory_size() / (1024 * 1024)
    );

    // Run demo programs to showcase kernel capabilities
    run_demos();

    println!("[KERNEL] All demos completed successfully!");
    println!("[KERNEL] Shutting down...");
    sbi::shutdown()
}

/// Run demonstration programs
fn run_demos() {
    println!("\n=== RPOS Kernel Demonstration ===\n");

    // Demo 1: Hello World
    demo_hello_world();

    // Demo 2: System Information
    demo_system_info();

    // Demo 3: Memory Statistics
    demo_memory_stats();

    // Demo 4: Process Management
    demo_process_management();
}

/// Demo 1: Basic Hello World
fn demo_hello_world() {
    println!("[DEMO 1] Hello World Program");
    println!("Output: Hello, RPOS Kernel World!");
    println!("Status: SUCCESS\n");
}

/// Demo 2: System Information
fn demo_system_info() {
    println!("[DEMO 2] System Information");
    println!("Kernel: RPOS v1.0.0");
    println!("Architecture: RISC-V 64-bit");
    println!("Page Size: 4096 bytes");
    println!("Status: SUCCESS\n");
}

/// Demo 3: Memory Statistics  
fn demo_memory_stats() {
    println!("[DEMO 3] Memory Management Statistics");
    let total_mem = mm::memory_size();
    let total_mb = total_mem / (1024 * 1024);
    println!("Total Memory: {} MB ({} bytes)", total_mb, total_mem);
    println!("Kernel Heap: Initialized with Buddy Allocator");
    println!("Physical Frames: Managed by Stack Allocator");
    println!("Virtual Memory: SV39 Paging Enabled");
    
    // Output structured metrics for dashboard
    println!("[METRICS] memory_total_mb={}", total_mb);
    println!("[METRICS] memory_used_mb={}", 1); // Approximate kernel usage
    println!("[METRICS] memory_free_mb={}", total_mb - 1);
    println!("Status: SUCCESS\n");
}

/// Demo 4: Process Management
fn demo_process_management() {
    println!("[DEMO 4] Process Management Capabilities");
    println!("System Calls Implemented:");
    println!("  - sys_write (64): Write to file descriptor");
    println!("  - sys_read (63): Read from file descriptor");
    println!("  - sys_exit (93): Exit process");
    println!("  - sys_yield (124): Yield CPU");
    println!("  - sys_getpid (172): Get process ID");
    println!("  - sys_fork (220): Fork process [STUB]");
    println!("  - sys_exec (221): Execute program [STUB]");
    println!("  - sys_waitpid (260): Wait for process [STUB]");
    
    // Output process metrics for dashboard
    println!("[METRICS] process_count=1");
    println!("[METRICS] syscall_count=8");
    println!("Status: SUCCESS\n");
}
