# 内存管理设计

## 概述

RPOS 为 RISC-V 64 位架构实现了一个综合的内存管理系统，使用 SV39 分页方案。该系统通过 Rust 的类型系统提供物理和虚拟内存管理，具有强大的安全保证。

## 物理内存管理

### 内存布局

物理内存组织如下：

```
0x80000000 ─────────┐
           OpenSBI  │
0x80200000 ─────────┤  ← 内核入口点
           .text    │
           .rodata  │
           .data    │
           .bss     │
0x???????? ─────────┤  ← 内核结束 (ekernel)
           堆       │
           (3 MB)   │
           ─────────┤
           空闲     │
           帧       │
           ─────────│
0x88000000 ─────────┘  ← MEMORY_END (128 MB)
```

### 物理帧分配器

**设计**：带回收的基于栈的分配器

**实现** (`src/mm/frame_allocator.rs`):

```rust
pub struct StackFrameAllocator {
    current: usize,      // 下一个要分配的帧
    end: usize,          // 可分配帧的结束
    recycled: Vec<usize>, // 回收的帧（LIFO）
}
```

**算法**:
1. **分配**：
   - 首先检查回收的帧（从栈中弹出）
   - 如果为空，从当前指针分配
   - 增加 current 直到达到 end
2. **释放**:
   - 将帧推回回收栈
   - 验证帧之前已分配

**RAII 模式**:
```rust
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);  // 自动释放
    }
}
```

**优点**:
- 无内存泄漏：帧自动返回
- 类型安全：不会忘记释放
- 清晰的所有权语义

### 内核堆分配器

**设计**：伙伴系统分配器（32 级）

**大小**：3 MB 静态数组

**实现** (`src/mm/heap_allocator.rs`):
```rust
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
```

**算法**:
- 将内存分割为大小为 2^n 的块
- 合并相邻的空闲块（伙伴）
- 快速分配/释放：O(log n)

## 虚拟内存管理

### SV39 分页方案

RISC-V SV39 提供 39 位虚拟地址和 3 级页表：

```
虚拟地址（39 位）：
┌────────┬────────┬────────┬────────────┐
│ VPN[2] │ VPN[1] │ VPN[0] │   偏移量   │
│ 9 位   │ 9 位   │ 9 位   │  12 位     │
└────────┴────────┴────────┴────────────┘
```

**地址空间**:
- 总计：512 GB (2^39 字节)
- 页大小：4 KB (2^12 字节)
- 每表页数：512 (2^9 项)

### 页表项

```rust
pub struct PageTableEntry {
    pub bits: usize,
}
```

**格式**（64 位）:
```
┌──────────────────────┬────────┬───┬───┬───┬───┬───┬───┬───┬───┐
│     PPN (44 位)      │保留    │ D │ A │ G │ U │ X │ W │ R │ V │
└──────────────────────┴────────┴───┴───┴───┴───┴───┴───┴───┴───┘
```

**标志**:
- **V** (Valid)：项有效
- **R** (Readable)：可读页面
- **W** (Writable)：可写页面
- **X** (Executable)：可执行页面
- **U** (User)：用户模式可访问
- **G** (Global)：全局映射
- **A** (Accessed)：已访问页面
- **D** (Dirty)：已修改页面

### 页表实现

```rust
pub struct PageTable {
    root_ppn: PhysPageNum,           // 根页表
    frames: Vec<FrameTracker>,       // 拥有的帧（RAII）
}
```

**操作**:

1. **创建**：分配根页表帧
```rust
pub fn new() -> Self {
    let frame = frame_alloc().unwrap();
    PageTable {
        root_ppn: frame.ppn,
        frames: vec![frame],
    }
}
```

2. **映射**：创建从 VPN 到 PPN 的映射
```rust
pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags)
```

3. **取消映射**：删除映射
```rust
pub fn unmap(&mut self, vpn: VirtPageNum)
```

4. **转换**：VPN → PPN
```rust
pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry>
```

## 地址类型

### 类型安全

所有地址都是强类型的以防止错误：

```rust
pub struct PhysAddr(pub usize);
pub struct VirtAddr(pub usize);
pub struct PhysPageNum(pub usize);
pub struct VirtPageNum(pub usize);
```

## 内存安全特性

### 1. 所有权

- 每个帧都有唯一所有者（FrameTracker）
- 所有者 drop 时，帧自动释放
- 防止双重释放和内存泄漏

### 2. 借用

- 页表可以临时借用帧
- 编译器强制无释放后使用
- 不可能有悬空指针

### 3. 类型安全

- 不能混合物理和虚拟地址
- 不能创建无效的页号
- 对齐的编译时检查

### 4. 边界检查

- 数组访问被检查
- 页表遍历验证每一级
- 系统调用中验证用户缓冲区访问

## 用户空间内存

### 用户地址空间布局

```
0xFFFFFFFF_FFFFFFFF ─┐
                      │ 内核（不可访问）
TRAMPOLINE           ─┤ 0xFFFFFFFF_FFFFF000
                      │ 跳板页
TRAP_CONTEXT         ─┤ 0xFFFFFFFF_FFFFE000
                      │ 陷阱上下文
                     ─┤
                      │ 用户栈（向下增长）
                     ─┤
                      │ 用户堆（向上增长）
                     ─┤
                      │ 用户 .bss
                     ─┤
                      │ 用户 .data
                     ─┤
                      │ 用户 .text
0x10000              ─┤
                      │ （保留）
0x0                  ─┘
```

### 写时复制（未来）

对于 `fork()` 系统调用：
1. 父进程和子进程共享物理页
2. 将所有页标记为只读
3. 写入时，陷入内核
4. 分配新帧并复制
5. 使用可写映射更新页表

## 性能考虑

### TLB（转换后备缓冲区）

- 地址转换的硬件缓存
- 命中：快速转换（1 个周期）
- 未命中：页表遍历（多次内存访问）

**优化**:
- 最小化 TLB 刷新
- 尽可能使用大页（未来）

### 页表遍历

- 每次转换 3 次内存访问（SV39）
- 由 TLB 缓存
- 保持工作集小

### 内存分配

**伙伴系统**:
- 快速：O(log n) 分配
- 2 的幂大小的低碎片
- 对内核数据结构高效

**帧分配**:
- 从回收帧 O(1) 分配
- O(1) 释放
- 简单高效

## 测试

### 帧分配器测试

```rust
pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    
    // 分配 5 个帧
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        v.push(frame);
    }
    
    // Drop 所有帧（自动释放）
    v.clear();
    
    // 再次分配（应重用回收的帧）
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        v.push(frame);
    }
}
```

## 未来增强

1. **大页**：支持 2MB/1GB 页
2. **NUMA**：非均匀内存访问
3. **内存压缩**：减少碎片
4. **交换**：磁盘支持的虚拟内存
5. **mmap**：内存映射文件
6. **共享内存**：进程间通信

## 参考文献

- RISC-V 特权 ISA 规范 v1.10
- "The RISC-V Reader" by Patterson & Waterman
- Linux 内存管理文档
- 伙伴系统分配器论文
