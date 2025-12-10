//! File system related syscalls

/// Read from file descriptor
pub fn sys_read(_fd: usize, _buf: *const u8, _len: usize) -> isize {
    // TODO: Implement file reading
    0
}

/// Write to file descriptor
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        1 | 2 => {
            // stdout/stderr
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            println!("[KERNEL] Unsupported fd in sys_write!");
            -1
        }
    }
}
