# RPOS Kernel Architecture Design

## Overview

RPOS (Rust-based POSIX-compatible Operating System) is a monolithic kernel designed from scratch using Rust for the RISC-V 64-bit architecture. The kernel provides POSIX-compatible system calls to support running simple C-based user applications.

## Design Philosophy

### Why Rust?
- **Memory Safety**: Rust's ownership system prevents memory leaks, buffer overflows, and data races at compile time
- **Zero-cost Abstractions**: High-level constructs without runtime overhead
- **No Garbage Collection**: Predictable performance suitable for kernel development
- **Strong Type System**: Catches bugs at compile time

### Why POSIX Compatibility?
- **Rich Ecosystem**: Leverage existing POSIX-compliant software
- **Standard Interface**: Well-defined system call interface
- **Portability**: Easy to port applications from other POSIX systems
- **Educational Value**: Learn from established OS design patterns

## System Architecture

```
┌─────────────────────────────────────────────────────┐
│                 User Space                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐         │
│  │  Shell   │  │   App1   │  │   App2   │         │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘         │
└───────┼─────────────┼─────────────┼───────────────┘
        │             │             │
        │    System Call Interface (POSIX)
        │             │             │
┌───────┼─────────────┼─────────────┼───────────────┐
│       │             │             │               │
│  ┌────┴─────────────┴─────────────┴────┐         │
│  │      Syscall Dispatcher              │         │
│  └─────┬─────────┬─────────┬────────────┘         │
│        │         │         │                      │
│  ┌─────┴──┐ ┌────┴───┐ ┌──┴─────┐  ┌──────────┐ │
│  │Process │ │  File  │ │ Memory │  │  Trap    │ │
│  │  Mgmt  │ │ System │ │  Mgmt  │  │ Handler  │ │
│  └────────┘ └────────┘ └────────┘  └──────────┘ │
│                                                   │
│  ┌────────────────────────────────────────────┐  │
│  │          Hardware Abstraction             │  │
│  │  ┌──────┐  ┌──────┐  ┌────────┐          │  │
│  │  │ SBI  │  │ MMU  │  │ Timer  │          │  │
│  │  └──────┘  └──────┘  └────────┘          │  │
│  └────────────────────────────────────────────┘  │
│                Kernel Space                      │
└───────────────────────────────────────────────────┘
        │             │             │
┌───────┼─────────────┼─────────────┼───────────────┐
│       │             │             │               │
│    RISC-V 64-bit Hardware (Physical/QEMU)        │
└───────────────────────────────────────────────────┘
```

## Core Modules

### 1. Boot & Initialization (`src/main.rs`, `src/entry.asm`)

**Responsibilities:**
- Setup initial stack pointer in supervisor mode
- Clear BSS segment
- Initialize all kernel subsystems
- Transfer control to Rust entry point

**Implementation Details:**
- Assembly entry point `_start` sets up stack
- Jumps to `rust_main()` in Rust
- Uses SBI (Supervisor Binary Interface) for M-mode services

### 2. Memory Management (`src/mm/`)

**Components:**

#### Address Types (`address.rs`)
- `PhysAddr` / `VirtAddr`: Physical and virtual addresses
- `PhysPageNum` / `VirtPageNum`: Page numbers
- Type-safe conversions and calculations

#### Frame Allocator (`frame_allocator.rs`)
- **Algorithm**: Stack-based allocation
- **RAII Pattern**: `FrameTracker` automatically deallocates on drop
- **Thread Safety**: Protected by `UPSafeCell`

#### Heap Allocator (`heap_allocator.rs`)
- **Algorithm**: Buddy system (32 levels)
- **Size**: 3MB kernel heap
- **Global Allocator**: Implements Rust's global allocator trait

#### Page Tables (`page_table.rs`)
- **Scheme**: SV39 (3-level page table)
- **Features**:
  - Virtual address translation
  - Page table entry flags (V, R, W, X, U, G, A, D)
  - Automatic frame allocation for page tables
  - RAII for page table cleanup

**Memory Layout:**
```
┌─────────────────────────────────────┐ 0xFFFFFFFF_FFFFFFFF
│      Not Accessible                 │
├─────────────────────────────────────┤ TRAMPOLINE
│      Trampoline Page                │
├─────────────────────────────────────┤ TRAP_CONTEXT
│      Trap Context                   │
├─────────────────────────────────────┤
│      User Stack                     │
├─────────────────────────────────────┤
│      User Heap                      │
├─────────────────────────────────────┤
│      User Data/BSS                  │
├─────────────────────────────────────┤
│      User Text                      │
├─────────────────────────────────────┤ 0x10000
│      (Reserved)                     │
├─────────────────────────────────────┤ 0x0
└─────────────────────────────────────┘
```

### 3. Trap Handling (`src/trap/`)

**Components:**

#### Trap Context (`context.rs`)
- Saves all general-purpose registers (x0-x31)
- Saves `sstatus` and `sepc`
- Used for context switching

#### Trap Handler (`mod.rs`)
- **Exceptions**: Syscalls, page faults, illegal instructions
- **Interrupts**: Timer interrupts
- **Entry/Exit**: Assembly code in `trap.S`

**Trap Flow:**
1. Hardware saves `sepc`, `scause`, `stval`
2. Jump to `__alltraps` (Assembly)
3. Save all registers to kernel stack
4. Call `trap_handler()` in Rust
5. Restore registers from trap context
6. `sret` to return

### 4. System Calls (`src/syscall/`)

**POSIX System Calls Implemented:**

| Syscall | ID | Description |
|---------|-----|-------------|
| read | 63 | Read from file descriptor |
| write | 64 | Write to file descriptor |
| exit | 93 | Exit process |
| yield | 124 | Yield CPU |
| getpid | 172 | Get process ID |
| fork | 220 | Fork process |
| execve | 221 | Execute program |
| wait4 | 260 | Wait for process |

**Syscall Flow:**
1. User app executes `ecall` instruction
2. Trap to kernel mode
3. Syscall dispatcher reads syscall number from `x17`
4. Arguments in `x10`, `x11`, `x12`
5. Call appropriate syscall handler
6. Return value in `x10`

### 5. Process Management (TODO)

**Task Control Block (TCB):**
- Process ID
- Parent process ID
- Process state (Ready, Running, Blocked, Zombie)
- Page table
- Trap context
- Kernel stack
- File descriptor table

**Scheduler:**
- **Algorithm**: Round Robin
- **Time Slice**: Configurable quantum
- **Priority**: Future enhancement

### 6. File System (TODO)

**Virtual File System (VFS):**
- Abstract file operations (open, read, write, close)
- File descriptor management (0=stdin, 1=stdout, 2=stderr)

**Implementation Options:**
1. Embedded filesystem (link user binaries into kernel)
2. FAT32 driver for block device
3. Simple in-memory filesystem

## Hardware Interface

### SBI (Supervisor Binary Interface)
- **Console I/O**: putchar/getchar
- **Timer**: Set timer interrupts
- **Shutdown**: Shutdown machine

### RISC-V Privileges
- **M-Mode (Machine)**: OpenSBI firmware
- **S-Mode (Supervisor)**: RPOS kernel
- **U-Mode (User)**: User applications

### CSR Registers Used
- `sstatus`: Supervisor status
- `stvec`: Trap vector
- `sepc`: Exception PC
- `scause`: Trap cause
- `stval`: Trap value
- `satp`: Address translation

## Security Considerations

### Memory Safety
- Rust's ownership prevents common vulnerabilities
- No use-after-free, no double-free
- Bounds checking on user buffers

### Privilege Separation
- User mode vs Supervisor mode
- System calls validate user pointers
- No direct hardware access from user space

### Page Protection
- User pages marked with U flag
- Kernel pages not accessible from user mode
- Copy-on-Write for fork()

## Performance Optimizations

### Compile-Time Optimization
- LTO (Link-Time Optimization) enabled
- Opt-level 3 for release builds
- Zero-cost abstractions

### Runtime Optimization
- Buddy allocator for fast allocation
- RAII for automatic cleanup
- Minimal copying in syscalls

## Future Enhancements

1. **Multi-processor Support**: SMP scheduling
2. **Advanced Scheduler**: Priority-based, CFS
3. **Network Stack**: TCP/IP support
4. **Device Drivers**: Block devices, character devices
5. **IPC**: Pipes, shared memory, message queues
6. **Signals**: POSIX signal handling
7. **Dynamic Loading**: Load user programs from disk

## References

- RISC-V Privileged Architecture Specification
- POSIX.1-2017 Standard
- Rust Embedded Book
- rCore Tutorial
