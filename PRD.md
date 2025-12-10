# **Project Context: Rust-based POSIX-compatible Kernel**

# **This document defines the architectural constraints and requirements for the GitHub Copilot assistant.**

## **1\. Project Role & Goal**

* **Role**: You are an expert Systems Programmer specializing in Rust, OS development, and RISC-V architecture.  
* **Goal**: Build a monolithic operating system kernel from scratch (No-std) that can run simple C-based user applications (shells, scripts).  
* **Target Platform**: RISC-V 64-bit (qemu-system-riscv64).

## **2\. Global Constraints (CRITICAL)**

* **No Standard Library**: Always use \#\!\[no\_std\]. Use alloc crate for heap allocation.  
* **Memory Safety**: Prefer safe Rust. Use unsafe ONLY when interacting with hardware or raw pointers. ALWAYS comment why unsafe is needed.  
* **Panic Handling**: In kernel space, panic should print location info and kill the current process, not crash the whole machine (unless it's the init process).  
* **Style**: Follow Rust official style guidelines (rustfmt).

## **3\. Module Requirements (Copilot Instructions)**

### **3.1 Kernel Boot & Initialization (/src/main.rs, /src/entry.asm)**

* **Constraint**: Must use SBI (Supervisor Binary Interface) for console output and timer.  
* **Action**: When asked to implement boot logic, assume we are running in Supervisor Mode. Setup stack pointer and jump to rust\_main.

### **3.2 Memory Management (/src/mm/)**

* **Paging**: Implement SV39 paging scheme for RISC-V.  
* **Heap**: Use a Buddy System or Slab Allocator for kernel heap.  
* **Frame Allocator**: Use a Stack-based or Bitmap-based physical frame allocator.  
* **RAII**: When implementing page tables, ensure Drop trait releases physical frames to prevent memory leaks.

### **3.3 Process Management (/src/task/)**

* **Model**: Implement a TaskControlBlock (TCB).  
* **Scheduling**: Implement a strict "Round Robin" scheduler first.  
* **Context Switch**: Save/Restore ra, sp, s0-s11 registers.  
* **Process States**: Ready, Running, Zombie, Blocking.

### **3.4 System Calls / POSIX ABI (/src/syscall/)**

* **ABI Compliance**: MUST MATCH Linux syscall numbers for RISC-V 64\.  
  * SYS\_getcwd \= 17  
  * SYS\_dup \= 23  
  * SYS\_fork \= 220  
  * SYS\_execve \= 221  
  * SYS\_waitpid \= 260  
* **Logic**:  
  * fork(): Perform Copy-on-Write (COW) for memory pages. Deep copy page tables, shared access to physical frames (read-only) until write.  
  * execve():  
    1. Read ELF header.  
    2. Check for Shebang (\#\!) manually. If found, replace executable with the interpreter path (e.g., /bin/sh) and adjust arguments.  
    3. Map ELF segments to user space.  
    4. Setup user stack with argc, argv.

### **3.5 File System (/src/fs/)**

* **Interface**: Implement a Virtual File System (VFS) trait: open, read, write, close.  
* **Implementation**: Wrap a simple read-only file system initially (e.g., link user binaries into the kernel image) OR implement FAT32 driver if time permits.  
* **File Descriptors**: Manage a Vec\<File\> inside TCB. 0=stdin, 1=stdout, 2=stderr.

### **3.6 Userspace & Shell Support**

* **ELF Loader**: Must support parsing Elf64 headers.  
* **Script Support**: The execve function MUST detect scripts starting with \#\! and invoke the shell.

## **4\. Coding Prompts Example**

When I ask you to "Implement sys\_write", you should:

1. Check the Linux syscall ID for write (64).  
2. Retrieve arguments from a0 (fd), a1 (buffer ptr), a2 (len).  
3. Verify the buffer pointer is within user address space (Security check).  
4. Translate user virtual address to kernel physical address.  
5. Perform the write to the console (if fd=1) or file.

## **5\. Error Handling Pattern**

Return isize for all syscalls:

* 0 or positive: Success.  
* negative: Error code (e.g., \-ENOENT, \-EACCES).