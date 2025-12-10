# Memory Management Design

## Overview

RPOS implements a comprehensive memory management system for RISC-V 64-bit architecture using the SV39 paging scheme. The system provides both physical and virtual memory management with strong safety guarantees through Rust's type system.

## Physical Memory Management

### Memory Layout

The physical memory is organized as follows:

```
0x80000000 ─────────┐
           OpenSBI  │
0x80200000 ─────────┤  ← Kernel Entry Point
           .text    │
           .rodata  │
           .data    │
           .bss     │
0x???????? ─────────┤  ← End of Kernel (ekernel)
           Heap     │
           (3 MB)   │
           ─────────┤
           Free     │
           Frames   │
           ─────────│
0x88000000 ─────────┘  ← MEMORY_END (128 MB)
```

### Physical Frame Allocator

**Design**: Stack-based allocator with recycling

**Implementation** (`src/mm/frame_allocator.rs`):

```rust
pub struct StackFrameAllocator {
    current: usize,      // Next frame to allocate
    end: usize,          // End of allocatable frames
    recycled: Vec<usize>, // Recycled frames (LIFO)
}
```

**Algorithm**:
1. **Allocation**: 
   - First check recycled frames (pop from stack)
   - If empty, allocate from current pointer
   - Increment current until reaching end
2. **Deallocation**:
   - Push frame back to recycled stack
   - Validate frame was previously allocated

**RAII Pattern**:
```rust
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);  // Auto-deallocate
    }
}
```

**Benefits**:
- No memory leaks: Frames automatically returned
- Type-safe: Can't forget to deallocate
- Clear ownership semantics

### Kernel Heap Allocator

**Design**: Buddy System Allocator (32 levels)

**Size**: 3 MB static array

**Implementation** (`src/mm/heap_allocator.rs`):
```rust
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
```

**Algorithm**:
- Splits memory into blocks of size 2^n
- Merges adjacent free blocks (buddies)
- Fast allocation/deallocation: O(log n)

**Usage**:
```rust
let v = Vec::new();           // Uses heap
let b = Box::new(data);       // Uses heap
```

## Virtual Memory Management

### SV39 Paging Scheme

RISC-V SV39 provides 39-bit virtual addresses with 3-level page tables:

```
Virtual Address (39 bits):
┌────────┬────────┬────────┬────────────┐
│ VPN[2] │ VPN[1] │ VPN[0] │   Offset   │
│ 9 bits │ 9 bits │ 9 bits │  12 bits   │
└────────┴────────┴────────┴────────────┘
```

**Address Space**:
- Total: 512 GB (2^39 bytes)
- Page Size: 4 KB (2^12 bytes)
- Pages per table: 512 (2^9 entries)

### Page Table Entry

```rust
pub struct PageTableEntry {
    pub bits: usize,
}
```

**Format** (64 bits):
```
┌──────────────────────┬────────┬───┬───┬───┬───┬───┬───┬───┬───┐
│     PPN (44 bits)    │Reserved│ D │ A │ G │ U │ X │ W │ R │ V │
└──────────────────────┴────────┴───┴───┴───┴───┴───┴───┴───┴───┘
```

**Flags**:
- **V** (Valid): Entry is valid
- **R** (Readable): Page can be read
- **W** (Writable): Page can be written
- **X** (Executable): Page can be executed
- **U** (User): User mode accessible
- **G** (Global): Global mapping
- **A** (Accessed): Page has been accessed
- **D** (Dirty): Page has been modified

### Page Table Implementation

```rust
pub struct PageTable {
    root_ppn: PhysPageNum,           // Root page table
    frames: Vec<FrameTracker>,       // Owned frames (RAII)
}
```

**Operations**:

1. **Create**: Allocate root page table frame
```rust
pub fn new() -> Self {
    let frame = frame_alloc().unwrap();
    PageTable {
        root_ppn: frame.ppn,
        frames: vec![frame],
    }
}
```

2. **Map**: Create mapping from VPN to PPN
```rust
pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags)
```
- Traverses 3-level page table
- Allocates intermediate tables if needed
- Sets leaf PTE with flags

3. **Unmap**: Remove mapping
```rust
pub fn unmap(&mut self, vpn: VirtPageNum)
```

4. **Translate**: VPN → PPN
```rust
pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry>
```

### Address Translation

**Process**:
1. Split virtual address into VPN[2:0] and offset
2. Read root PTE at index VPN[2]
3. If not valid, raise page fault
4. Get next level PPN from PTE
5. Repeat for VPN[1] and VPN[0]
6. Combine leaf PPN with offset for physical address

**Hardware Translation**:
- satp register points to root page table
- MMU walks page table automatically
- TLB caches translations

## Address Types

### Type Safety

All addresses are strongly typed to prevent errors:

```rust
pub struct PhysAddr(pub usize);
pub struct VirtAddr(pub usize);
pub struct PhysPageNum(pub usize);
pub struct VirtPageNum(pub usize);
```

**Conversions**:
```rust
// Address ↔ Page Number
let ppn = pa.floor();           // Round down
let ppn = pa.ceil();            // Round up
let pa: PhysAddr = ppn.into();  // Convert back

// Check alignment
assert!(pa.aligned());
```

### Page Ranges

**Iteration over pages**:
```rust
pub type VPNRange = SimpleRange<VirtPageNum>;

for vpn in VPNRange::new(start_vpn, end_vpn) {
    page_table.map(vpn, ppn, flags);
    ppn = ppn.step();
}
```

## Memory Safety Features

### 1. Ownership

- Each frame has exactly one owner (FrameTracker)
- When owner drops, frame is automatically freed
- Prevents double-free and memory leaks

### 2. Borrowing

- Page tables can borrow frames temporarily
- Compiler enforces no use-after-free
- No dangling pointers possible

### 3. Type Safety

- Cannot mix physical and virtual addresses
- Cannot create invalid page numbers
- Compile-time checks for alignment

### 4. Bounds Checking

- Array accesses are checked
- Page table walks validate each level
- User buffer access validated in syscalls

## User Space Memory

### User Address Space Layout

```
0xFFFFFFFF_FFFFFFFF ─┐
                      │ Kernel (inaccessible)
TRAMPOLINE           ─┤ 0xFFFFFFFF_FFFFF000
                      │ Trampoline page
TRAP_CONTEXT         ─┤ 0xFFFFFFFF_FFFFE000
                      │ Trap context
                     ─┤
                      │ User stack (grows down)
                     ─┤
                      │ User heap (grows up)
                     ─┤
                      │ User .bss
                     ─┤
                      │ User .data
                     ─┤
                      │ User .text
0x10000              ─┤
                      │ (Reserved)
0x0                  ─┘
```

### Copy-on-Write (Future)

For `fork()` system call:
1. Parent and child share physical pages
2. Mark all pages read-only
3. On write, trap to kernel
4. Allocate new frame and copy
5. Update page table with writable mapping

## Performance Considerations

### TLB (Translation Lookaside Buffer)

- Hardware cache for address translations
- Hit: Fast translation (1 cycle)
- Miss: Page table walk (multiple memory accesses)

**Optimization**:
- Minimize TLB flushes
- Use large pages where possible (future)

### Page Table Walks

- 3 memory accesses per translation (SV39)
- Cached by TLB
- Keep working set small

### Memory Allocation

**Buddy System**:
- Fast: O(log n) allocation
- Low fragmentation for power-of-2 sizes
- Efficient for kernel data structures

**Frame Allocation**:
- O(1) allocation from recycled frames
- O(1) deallocation
- Simple and efficient

## Testing

### Frame Allocator Test

```rust
pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    
    // Allocate 5 frames
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        v.push(frame);
    }
    
    // Drop all frames (automatic deallocation)
    v.clear();
    
    // Allocate again (should reuse recycled frames)
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        v.push(frame);
    }
}
```

### Heap Test

```rust
pub fn heap_test() {
    let a = Box::new(5);
    assert_eq!(*a, 5);
    
    let mut v: Vec<usize> = Vec::new();
    for i in 0..500 {
        v.push(i);
    }
    
    for (i, val) in v.iter().enumerate() {
        assert_eq!(*val, i);
    }
}
```

## Future Enhancements

1. **Huge Pages**: Support for 2MB/1GB pages
2. **NUMA**: Non-uniform memory access
3. **Memory Compaction**: Reduce fragmentation
4. **Swap**: Disk-backed virtual memory
5. **mmap**: Memory-mapped files
6. **Shared Memory**: Inter-process communication

## References

- RISC-V Privileged ISA Specification v1.10
- "The RISC-V Reader" by Patterson & Waterman
- Linux Memory Management Documentation
- Buddy System Allocator paper
