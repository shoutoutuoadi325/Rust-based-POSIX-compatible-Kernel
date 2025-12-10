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
            // TODO: Add proper validation that buf is in user address space
            // and the memory region [buf, buf+len) is valid and readable
            // For now, this is only called from kernel space for testing
            if buf.is_null() || len == 0 {
                return 0;
            }
            // SAFETY: This is currently only safe when called from kernel space
            // with valid kernel buffers. Future implementation should use
            // page table translation to validate user space buffers.
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap_or("[Invalid UTF-8]");
            print!("{}", str);
            len as isize
        }
        _ => {
            println!("[KERNEL] Unsupported fd in sys_write!");
            -1
        }
    }
}
