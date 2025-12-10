//! Process related syscalls

use crate::sbi::shutdown;

/// Exit current process
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[KERNEL] Application exited with code {}", exit_code);
    shutdown()
}

/// Yield current process
pub fn sys_yield() -> isize {
    // TODO: Implement task scheduling
    0
}

/// Get process ID
pub fn sys_getpid() -> isize {
    // TODO: Return actual PID
    1
}

/// Fork current process
pub fn sys_fork() -> isize {
    // TODO: Implement fork
    -1
}

/// Execute program
pub fn sys_exec(_path: *const u8) -> isize {
    // TODO: Implement exec
    -1
}

/// Wait for process
pub fn sys_waitpid(_pid: isize, _exit_code: *mut i32) -> isize {
    // TODO: Implement waitpid
    -1
}
