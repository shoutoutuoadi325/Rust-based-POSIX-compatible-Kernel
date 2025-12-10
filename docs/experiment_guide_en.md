# RPOS Kernel Development Lab Guide

## Introduction

This experimental guide helps students understand operating system concepts by implementing key features in the RPOS kernel. Each lab builds on previous work and includes fill-in-the-blank exercises to reinforce learning.

## Prerequisites

- Basic knowledge of Rust programming
- Understanding of computer architecture
- Familiarity with RISC-V assembly (recommended)
- Linux command line experience

## Lab Environment Setup

### Step 1: Install Required Tools

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install QEMU for RISC-V
sudo apt-get update
sudo apt-get install qemu-system-misc

# Add RISC-V target
rustup target add riscv64gc-unknown-none-elf

# Install additional tools
rustup component add rustfmt clippy
```

### Step 2: Clone and Build

```bash
git clone <repository-url>
cd Rust-based-POSIX-compatible-Kernel
make build
```

### Step 3: Test Your Setup

```bash
make run
# You should see: [KERNEL] Rust-based POSIX-compatible Kernel
```

---

## Lab 1: Understanding the Boot Process

### Objectives
- Understand RISC-V boot sequence
- Learn about SBI interface
- Implement console output

### Theory

When the RISC-V processor starts:
1. OpenSBI firmware runs in M-mode
2. Control transfers to kernel at 0x80200000 (S-mode)
3. Kernel sets up stack and jumps to Rust code

### Exercise 1.1: Implement Hello World via SBI

**File**: `src/sbi/mod.rs`

The SBI provides basic services. Implement the missing parts:

```rust
/// Print a character to console
pub fn console_putchar(c: usize) {
    sbi_call(________, c, 0, 0);  // Fill in: Which SBI call number?
}

/// Get a character from console  
pub fn console_getchar() -> usize {
    sbi_call(________, 0, 0, 0)   // Fill in: SBI_CONSOLE_GETCHAR value
}
```

**Hints**:
- Check the constant definitions at the top of the file
- SBI_CONSOLE_PUTCHAR = 1
- SBI_CONSOLE_GETCHAR = 2

### Exercise 1.2: Format String Output

**File**: `src/console.rs`

Implement the `write_str` method to output strings:

```rust
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(________ as usize);  // What goes here?
        }
        Ok(())
    }
}
```

**Question**: What type is `c`? How do we convert it to `usize`?

### Checkpoint 1

Run your kernel:
```bash
make run
```

Expected output:
```
[KERNEL] Rust-based POSIX-compatible Kernel
[KERNEL] Starting initialization...
```

---

## Lab 2: Memory Management - Physical Frames

### Objectives
- Understand physical memory layout
- Implement frame allocator
- Learn RAII pattern in Rust

### Theory

Physical memory is divided into 4KB frames. We need:
- A way to allocate frames
- A way to track which frames are free
- Automatic deallocation (RAII)

### Exercise 2.1: Frame Allocator Initialization

**File**: `src/mm/frame_allocator.rs`

Complete the initialization function:

```rust
pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR.exclusive_access().init(
        PhysAddr::from(________ as usize).ceil(),  // Start: end of kernel
        PhysAddr::from(________).floor(),          // End: MEMORY_END constant
    );
}
```

**Questions**:
1. Why do we start after `ekernel`?
2. Why use `ceil()` for start and `floor()` for end?

### Exercise 2.2: Implement Frame Deallocation

**File**: `src/mm/frame_allocator.rs`

```rust
fn dealloc(&mut self, ppn: PhysPageNum) {
    let ppn = ppn.0;
    
    // Validation check - fill in the condition
    if ppn >= ________ || self.recycled.iter().any(|&v| v == ppn) {
        panic!("Frame ppn={:#x} has not been allocated!", ppn);
    }
    
    // Push to recycled stack
    self.recycled.________;  // What method adds to Vec?
}
```

### Exercise 2.3: RAII Pattern

Explain why this code prevents memory leaks:

```rust
{
    let frame = frame_alloc().unwrap();
    // Use frame...
}  // ← What happens here?
```

**Answer**: _______________________________________

### Checkpoint 2

Test your allocator:
```rust
pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("Allocated: {:?}", frame.ppn);
        v.push(frame);
    }
    v.clear();  // Frames should be recycled
    
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("Re-allocated: {:?}", frame.ppn);  // Should reuse frames
        v.push(frame);
    }
}
```

---

## Lab 3: Virtual Memory - Page Tables

### Objectives
- Understand SV39 paging
- Implement page table operations
- Learn address translation

### Theory

SV39 uses 3-level page tables:
- Virtual address = [VPN[2]][VPN[1]][VPN[0]][Offset]
- Each VPN is 9 bits (512 entries)
- Page size = 4KB

### Exercise 3.1: Page Table Entry Flags

**File**: `src/mm/page_table.rs`

```rust
impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ________ << 10 | flags.bits() as usize,
        }
    }
    
    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> ________ & ((1usize << 44) - 1)).into()
    }
    
    pub fn is_valid(&self) -> bool {
        (self.flags() & ________) != PTEFlags::empty()
    }
}
```

**Fill in**:
1. Line 4: What should be shifted left by 10?
2. Line 9: How many bits to shift right to get PPN?
3. Line 13: Which flag indicates validity?

### Exercise 3.2: Page Table Mapping

**File**: `src/mm/page_table.rs`

Implement the mapping function:

```rust
pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
    let pte = self.find_pte_create(vpn).unwrap();
    
    // Check if already mapped
    assert!(!pte.________(), "vpn {:?} is mapped before mapping", vpn);
    
    // Create new PTE with flags
    *pte = PageTableEntry::new(ppn, flags | ________);
}
```

**Questions**:
1. What method checks if PTE is valid?
2. What flag must always be set for a valid entry?

### Exercise 3.3: Address Translation

Given a virtual address `0x12345678`, fill in the table:

| Component | Bits | Value (hex) | Calculation |
|-----------|------|-------------|-------------|
| VPN[2] | 30-38 | _____ | (0x12345678 >> 30) & 0x1FF |
| VPN[1] | 21-29 | _____ | (0x12345678 >> 21) & 0x1FF |
| VPN[0] | 12-20 | _____ | (0x12345678 >> 12) & 0x1FF |
| Offset | 0-11 | _____ | 0x12345678 & 0xFFF |

### Checkpoint 3

```rust
pub fn page_table_test() {
    let mut pt = PageTable::new();
    let vpn = VirtPageNum::from(0x1000);
    let ppn = frame_alloc().unwrap().ppn;
    
    pt.map(vpn, ppn, PTEFlags::R | PTEFlags::W | PTEFlags::V);
    
    let pte = pt.translate(vpn).unwrap();
    assert_eq!(pte.ppn(), ppn);
    
    println!("[TEST] Page table test passed!");
}
```

---

## Lab 4: Trap Handling and System Calls

### Objectives
- Understand RISC-V trap mechanism
- Implement trap handler
- Create system call interface

### Theory

When a trap occurs:
1. Hardware saves PC to `sepc`
2. Sets `scause` to trap cause
3. Jumps to `stvec` (trap vector)
4. Kernel handles trap
5. Returns via `sret`

### Exercise 4.1: Trap Context

**File**: `src/trap/context.rs`

```rust
#[repr(C)]
pub struct TrapContext {
    pub x: [usize; ________],  // How many general registers?
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[________] = sp;  // Which register is SP?
    }
}
```

### Exercise 4.2: System Call Dispatcher

**File**: `src/syscall/mod.rs`

```rust
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(
            args[0],              // fd
            args[1] as *const u8, // buf
            args[________]        // What's the third argument?
        ),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        _ => {
            println!("[KERNEL] Unsupported syscall_id: {}", syscall_id);
            ________  // What should we return for unsupported syscalls?
        }
    }
}
```

### Exercise 4.3: Implement sys_write

**File**: `src/syscall/fs.rs`

```rust
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        1 | 2 => {  // stdout/stderr
            let slice = unsafe { 
                core::slice::from_raw_parts(________, ________)
            };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            ________ as isize  // Return value = bytes written
        }
        _ => -1  // Unsupported fd
    }
}
```

### Checkpoint 4

If implemented correctly, user programs can print to console:
```rust
// User code
syscall(SYSCALL_WRITE, [1, buffer_ptr, length]);
// Should print buffer contents
```

---

## Lab 5: Process Management (Advanced)

### Objectives
- Implement Task Control Block
- Create process scheduler
- Understand context switching

### Exercise 5.1: Task Control Block

**File**: `src/task/task.rs` (to be created)

```rust
pub struct TaskControlBlock {
    pub pid: usize,
    pub parent: Option<usize>,
    pub status: TaskStatus,
    pub cx: TaskContext,
    // Fill in missing fields:
    // - Page table?
    // - Kernel stack?
    // - File descriptors?
}

pub enum TaskStatus {
    Ready,
    ________,  // Task is currently executing
    ________,  // Task is waiting
    Zombie,
}
```

### Exercise 5.2: Context Switching

Fill in the context switch logic:

```rust
pub fn switch_to(&mut self, next: &mut TaskControlBlock) {
    // Save current context
    // Load next context
    // Update current task pointer
    
    // What registers must be saved/restored?
    // Hint: ra, sp, s0-s11
}
```

### Checkpoint 5

Implement and test round-robin scheduling with 2 tasks.

---

## Lab 6: Challenge Exercises

### Challenge 1: Implement Copy-on-Write

Modify `fork()` to use COW instead of deep copy:
1. Share physical pages between parent and child
2. Mark all pages read-only
3. On write fault, copy page

### Challenge 2: ELF Loader

Implement an ELF64 loader:
1. Parse ELF header
2. Load program segments
3. Set entry point
4. Create user stack

### Challenge 3: Simple Shell

Create a minimal shell that:
1. Reads commands from stdin
2. Forks and executes programs
3. Waits for completion

---

## Grading Rubric

| Lab | Points | Criteria |
|-----|--------|----------|
| Lab 1 | 10 | Boot and console output |
| Lab 2 | 15 | Frame allocator working |
| Lab 3 | 20 | Page tables and translation |
| Lab 4 | 20 | Trap handling and syscalls |
| Lab 5 | 20 | Process management |
| Lab 6 | 15 | Challenge exercises |
| **Total** | **100** | |

---

## Submission Guidelines

1. Code must compile without warnings
2. All tests must pass
3. Code should be formatted (`cargo fmt`)
4. Include a lab report documenting:
   - What you learned
   - Challenges faced
   - Solutions implemented

---

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [RISC-V Specification](https://riscv.org/technical/specifications/)
- [RPOS Documentation](../README.md)

---

## Instructor Notes

### Common Student Mistakes

1. **Forgetting to add PTEFlags::V**: Pages won't be valid
2. **Not handling unsafe correctly**: Document why unsafe is safe
3. **Stack overflow**: Increase stack size if needed
4. **Alignment errors**: Use `.aligned()` checks

### Extension Ideas

- Implement priority scheduling
- Add memory-mapped I/O
- Create a simple filesystem
- Add network stack basics

---

## Conclusion

By completing these labs, students gain hands-on experience with:
- Low-level systems programming in Rust
- Memory management techniques
- Operating system concepts
- RISC-V architecture

This knowledge is transferable to other OS development projects and embedded systems programming.
