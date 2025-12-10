# Rust OS Development Debugging Guide

## Common Issues and Solutions

This guide documents typical bugs encountered during RPOS kernel development and their solutions, helping other developers learn from our experiences.

## 1. Borrow Checker Errors

### Problem: Multiple Mutable References

**Error Message:**
```
error[E0499]: cannot borrow `allocator` as mutable more than once at a time
```

**Problematic Code:**
```rust
let frame1 = allocator.alloc();
let frame2 = allocator.alloc();  // Error!
```

**Root Cause:**
- Rust's borrow checker prevents data races
- Can't have multiple mutable borrows simultaneously
- Frame allocator uses `RefCell` internally

**Solution:**
Use interior mutability pattern with `UPSafeCell`:

```rust
pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

impl<T> UPSafeCell<T> {
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}

// Usage
static ALLOCATOR: UPSafeCell<FrameAllocator> = ...;

let frame1 = ALLOCATOR.exclusive_access().alloc();
drop(frame1);  // Release borrow
let frame2 = ALLOCATOR.exclusive_access().alloc();
```

**Key Lesson:** Use `RefCell` for runtime borrow checking when compile-time checks are too restrictive.

## 2. Stack Overflow in Kernel

### Problem: Kernel Panic on Boot

**Symptom:**
```
[KERNEL PANIC] at src/main.rs:45
Store/AMO page fault
```

**Root Cause:**
- Default stack size too small (4KB)
- Large local variables on stack
- Deep recursion

**Investigation:**
```rust
// Check stack usage
extern "C" {
    fn boot_stack();
    fn boot_stack_top();
}

println!("Stack size: {} bytes", 
    boot_stack_top as usize - boot_stack as usize);
```

**Solution:**
Increase stack size in `entry.asm`:

```asm
.section .bss.stack
.globl boot_stack
boot_stack:
    .space 4096 * 16   # Changed from 4096 * 4 to 4096 * 16
.globl boot_stack_top
boot_stack_top:
```

**Prevention:**
- Avoid large stack allocations
- Use heap (`Box`, `Vec`) for large data
- Be careful with recursion depth

## 3. Page Fault Debugging

### Problem: Unexpected Page Faults

**Symptom:**
```
[KERNEL] Page fault at 0x80201234, bad addr = 0x10000008
```

**Common Causes:**
1. Unmapped pages
2. Wrong permission bits
3. Null pointer dereference
4. Stack overflow

**Debugging Steps:**

```rust
// 1. Check if page is mapped
pub fn debug_page_fault(va: VirtAddr, cause: &str) {
    println!("[DEBUG] Page fault: {}", cause);
    println!("  Virtual Address: {:?}", va);
    println!("  Page offset: {:#x}", va.page_offset());
    
    let vpn = va.floor();
    println!("  VPN: {:?}", vpn);
    
    // Try to translate
    if let Some(pte) = page_table.translate(vpn) {
        println!("  PTE found: flags={:?}", pte.flags());
        println!("  PPN: {:?}", pte.ppn());
    } else {
        println!("  No PTE found - page not mapped!");
    }
}
```

**Solution Patterns:**

**Missing Mapping:**
```rust
// Ensure page is mapped before access
page_table.map(vpn, ppn, PTEFlags::R | PTEFlags::W | PTEFlags::V);
```

**Wrong Permissions:**
```rust
// Check flags match access type
// For code: X | R
// For data: R | W  
// For user: U
let flags = PTEFlags::R | PTEFlags::W | PTEFlags::U | PTEFlags::V;
```

## 4. Linker Script Issues

### Problem: Sections Not Aligned

**Error:**
```
Misaligned section at 0x80200123
```

**Root Cause:**
- Page tables require 4KB alignment
- Linker script doesn't enforce alignment

**Solution:**
Add alignment in `linker.ld`:

```ld
.text : {
    *(.text.entry)
    *(.text .text.*)
}
. = ALIGN(4K);  /* Add this! */

.rodata : {
    *(.rodata .rodata.*)
}
. = ALIGN(4K);  /* And this! */
```

**Verification:**
```bash
rust-objdump -h target/riscv64gc-unknown-none-elf/release/rpos-kernel

# Check section addresses are 0x...000
```

## 5. Unsafe Code Bugs

### Problem: Dereferencing Raw Pointers

**Error:**
```
error[E0133]: dereference of raw pointer is unsafe
```

**Issue:**
```rust
let ptr = 0x80200000 as *mut u8;
*ptr = 42;  // Error: unsafe operation
```

**Solution:**
Always wrap unsafe operations and document why they're safe:

```rust
// SAFETY: This address is guaranteed to be valid kernel memory
// within the .data section, as verified by the linker script
unsafe {
    let ptr = 0x80200000 as *mut u8;
    *ptr = 42;
}
```

**Best Practices:**
1. Minimize unsafe blocks
2. Document safety invariants
3. Use safe abstractions when possible
4. Review all unsafe code carefully

## 6. Global State Initialization

### Problem: Static Initialization Error

**Error:**
```
error[E0015]: cannot call non-const fn in statics
```

**Problematic Code:**
```rust
static ALLOCATOR: Mutex<FrameAllocator> = 
    Mutex::new(FrameAllocator::new());  // Error!
```

**Root Cause:**
- Statics must be const-initialized
- `new()` isn't a const function

**Solution 1: lazy_static**
```rust
use lazy_static::lazy_static;

lazy_static! {
    static ref ALLOCATOR: Mutex<FrameAllocator> = {
        Mutex::new(FrameAllocator::new())
    };
}
```

**Solution 2: Manual Initialization**
```rust
static mut ALLOCATOR: Option<Mutex<FrameAllocator>> = None;

pub fn init() {
    unsafe {
        ALLOCATOR = Some(Mutex::new(FrameAllocator::new()));
    }
}
```

## 7. Interrupt Handling Race Conditions

### Problem: Corrupted State During Interrupts

**Symptom:**
- Random crashes
- Inconsistent data
- Lost updates

**Root Cause:**
- Interrupt modifies shared state
- No synchronization

**Solution:**
Disable interrupts during critical sections:

```rust
pub fn critical_section<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // Disable interrupts
    let sie = sie::read();
    unsafe { sie::clear_sie(); }
    
    // Execute critical section
    let result = f();
    
    // Restore interrupts
    unsafe { 
        if sie.bits() != 0 {
            sie::set_sie();
        }
    }
    
    result
}

// Usage
critical_section(|| {
    // Safe from interrupts here
    SHARED_STATE.modify();
});
```

## 8. Memory Leaks

### Problem: Running Out of Memory

**Detection:**
```rust
pub fn check_leaks() {
    println!("Allocated frames: {}", allocator.allocated_count());
    println!("Free frames: {}", allocator.free_count());
    
    // After operations, counts should return to baseline
}
```

**Common Causes:**

**1. Forgetting to Drop:**
```rust
// Bad: Never dropped
let frame = frame_alloc().unwrap();
forget(frame);  // Memory leak!

// Good: Automatic drop
{
    let frame = frame_alloc().unwrap();
}  // Dropped here
```

**2. Circular References:**
```rust
// Bad: Reference cycle
struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

// Good: Use weak references
struct Node {
    next: Option<Weak<RefCell<Node>>>,
}
```

**Solution:**
Use RAII pattern everywhere - rely on Drop trait.

## 9. Assembly/Rust Interface Issues

### Problem: Wrong Calling Convention

**Symptom:**
- Corrupted registers
- Segfaults after assembly calls
- Wrong return values

**Example:**
```asm
# Wrong: Doesn't preserve callee-saved registers
my_function:
    # Modifies s0-s11 without saving
    ret

# Correct:
my_function:
    # Save callee-saved registers
    addi sp, sp, -16
    sd s0, 0(sp)
    sd ra, 8(sp)
    
    # Function body
    
    # Restore registers
    ld s0, 0(sp)
    ld ra, 8(sp)
    addi sp, sp, 16
    ret
```

**Rust Side:**
```rust
extern "C" {
    fn my_function();  // Must use C ABI
}
```

## 10. Build System Issues

### Problem: Linker Errors

**Error:**
```
undefined reference to `__rust_alloc`
```

**Solution:**
Ensure proper features in `Cargo.toml`:

```toml
[dependencies]
buddy_system_allocator = "0.9"

[profile.release]
panic = "abort"  # Required for #![no_std]
```

**Problem: Wrong Target**

**Error:**
```
unknown target triple 'riscv64gc'
```

**Solution:**
```bash
rustup target add riscv64gc-unknown-none-elf
```

## Debugging Tools and Techniques

### 1. GDB with QEMU

```bash
# Terminal 1: Start QEMU with GDB server
qemu-system-riscv64 \
    -machine virt \
    -kernel kernel.elf \
    -s -S  # Wait for GDB

# Terminal 2: Connect GDB
riscv64-unknown-elf-gdb kernel.elf
(gdb) target remote :1234
(gdb) break rust_main
(gdb) continue
```

### 2. Print Debugging

```rust
macro_rules! trace {
    ($($arg:tt)*) => {
        println!("[TRACE {}:{}] {}", 
            file!(), line!(), format_args!($($arg)*))
    };
}

// Usage
trace!("Allocating frame");
let frame = frame_alloc().unwrap();
trace!("Got frame: {:?}", frame.ppn);
```

### 3. Assertions

```rust
// Runtime checks
assert!(frame.is_valid());
assert_eq!(frame.ppn.0 & 0xFFF, 0, "Frame not aligned!");

// Debug-only checks
debug_assert!(expensive_check());
```

### 4. Memory Dumps

```rust
pub fn dump_memory(addr: usize, len: usize) {
    println!("Memory dump at {:#x}:", addr);
    let slice = unsafe {
        core::slice::from_raw_parts(addr as *const u8, len)
    };
    
    for (i, byte) in slice.iter().enumerate() {
        if i % 16 == 0 {
            print!("\n{:#x}: ", addr + i);
        }
        print!("{:02x} ", byte);
    }
    println!();
}
```

## Prevention Checklist

Before committing code:

- [ ] All unsafe blocks documented
- [ ] No warnings from `cargo clippy`
- [ ] Code formatted with `cargo fmt`
- [ ] Tests pass
- [ ] No obvious memory leaks
- [ ] Error paths handled
- [ ] Assertions for invariants
- [ ] Comments for complex logic

## Useful Resources

- Rust Embedded Book: https://rust-embedded.github.io/book/
- RISC-V ISA Manual: https://riscv.org/technical/specifications/
- rCore Tutorial: https://rcore-os.github.io/rCore-Tutorial-Book-v3/
- Rust Nomicon (unsafe): https://doc.rust-lang.org/nomicon/

## Contributing Debugging Experiences

If you encounter new issues, please document:
1. Error message
2. Root cause analysis
3. Solution that worked
4. Prevention tips

This helps future developers avoid the same pitfalls!
