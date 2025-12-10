//! SBI (Supervisor Binary Interface) for RISC-V
//! Provides basic services from M-mode to S-mode

#![allow(unused)]

// Legacy SBI extensions
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;

// SBI extension IDs (for new SBI v0.2+ interface)
const SBI_EXT_SRST: usize = 0x53525354; // System Reset Extension

// SRST reset types
const SBI_SRST_RESET_TYPE_SHUTDOWN: usize = 0;
const SBI_SRST_RESET_TYPE_COLD_REBOOT: usize = 1;
const SBI_SRST_RESET_TYPE_WARM_REBOOT: usize = 2;

// SRST reset reasons
const SBI_SRST_RESET_REASON_NONE: usize = 0;
const SBI_SRST_RESET_REASON_SYSTEM_FAILURE: usize = 1;

/// Legacy SBI call (for extensions 0-8)
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}

/// New SBI v0.2+ call with extension ID and function ID
#[inline(always)]
fn sbi_call_ext(ext: usize, fid: usize, arg0: usize, arg1: usize) -> (usize, usize) {
    let error;
    let value;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") arg0 => error,
            inlateout("x11") arg1 => value,
            in("x16") fid,
            in("x17") ext,
        );
    }
    (error, value)
}

/// Print a character to console
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

/// Get a character from console
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

/// Set timer for next interrupt
pub fn set_timer(timer: usize) {
    sbi_call(SBI_SET_TIMER, timer, 0, 0);
}

/// Shutdown the system using SRST extension (SBI v0.2+)
pub fn shutdown() -> ! {
    // Use SRST extension for system reset/shutdown
    sbi_call_ext(
        SBI_EXT_SRST,
        0, // function ID 0 = sbi_system_reset
        SBI_SRST_RESET_TYPE_SHUTDOWN,
        SBI_SRST_RESET_REASON_NONE,
    );
    // If SRST fails, loop forever
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
