# Rust OS 开发调试指南

## 常见问题和解决方案

本指南记录了 RPOS 内核开发过程中遇到的典型错误及其解决方案，帮助其他开发者从我们的经验中学习。

## 1. 借用检查器错误

### 问题：多个可变引用

**错误信息：**
```
error[E0499]: 不能同时多次将 `allocator` 作为可变借用
```

**根本原因：**
- Rust 的借用检查器防止数据竞争
- 不能同时有多个可变借用
- 帧分配器内部使用 `RefCell`

**解决方案：**
使用 `UPSafeCell` 的内部可变性模式：

```rust
pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

impl<T> UPSafeCell<T> {
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}

// 使用方法
static ALLOCATOR: UPSafeCell<FrameAllocator> = ...;

let frame1 = ALLOCATOR.exclusive_access().alloc();
drop(frame1);  // 释放借用
let frame2 = ALLOCATOR.exclusive_access().alloc();
```

**关键教训：** 当编译时检查过于严格时，使用 `RefCell` 进行运行时借用检查。

## 2. 内核栈溢出

### 问题：启动时内核 Panic

**症状：**
```
[KERNEL PANIC] at src/main.rs:45
Store/AMO page fault
```

**根本原因：**
- 默认栈大小太小（4KB）
- 栈上的大型局部变量
- 深度递归

**解决方案：**
在 `entry.asm` 中增加栈大小：

```asm
.section .bss.stack
.globl boot_stack
boot_stack:
    .space 4096 * 16   # 从 4096 * 4 改为 4096 * 16
.globl boot_stack_top
boot_stack_top:
```

**预防措施：**
- 避免大的栈分配
- 为大数据使用堆（`Box`、`Vec`）
- 小心递归深度

## 3. 页错误调试

### 问题：意外的页错误

**症状：**
```
[KERNEL] Page fault at 0x80201234, bad addr = 0x10000008
```

**常见原因：**
1. 未映射的页面
2. 错误的权限位
3. 空指针解引用
4. 栈溢出

**调试步骤：**

```rust
// 1. 检查页面是否已映射
pub fn debug_page_fault(va: VirtAddr, cause: &str) {
    println!("[DEBUG] 页错误：{}", cause);
    println!("  虚拟地址：{:?}", va);
    println!("  页偏移：{:#x}", va.page_offset());
    
    let vpn = va.floor();
    println!("  VPN：{:?}", vpn);
    
    // 尝试转换
    if let Some(pte) = page_table.translate(vpn) {
        println!("  找到 PTE：标志={:?}", pte.flags());
        println!("  PPN：{:?}", pte.ppn());
    } else {
        println!("  未找到 PTE - 页面未映射！");
    }
}
```

## 4. 链接器脚本问题

### 问题：段未对齐

**错误：**
```
Misaligned section at 0x80200123
```

**解决方案：**
在 `linker.ld` 中添加对齐：

```ld
.text : {
    *(.text.entry)
    *(.text .text.*)
}
. = ALIGN(4K);  /* 添加这个！*/

.rodata : {
    *(.rodata .rodata.*)
}
. = ALIGN(4K);  /* 和这个！*/
```

## 5. 不安全代码错误

### 问题：解引用原始指针

**错误：**
```
error[E0133]: 解引用原始指针是不安全的
```

**解决方案：**
始终包装不安全操作并记录它们为什么是安全的：

```rust
// SAFETY：此地址保证是有效的内核内存
// 在 .data 段内，由链接器脚本验证
unsafe {
    let ptr = 0x80200000 as *mut u8;
    *ptr = 42;
}
```

**最佳实践：**
1. 最小化不安全块
2. 记录安全不变量
3. 可能时使用安全抽象
4. 仔细审查所有不安全代码

## 6. 全局状态初始化

### 问题：静态初始化错误

**错误：**
```
error[E0015]: 不能在静态中调用非 const 函数
```

**解决方案 1：lazy_static**
```rust
use lazy_static::lazy_static;

lazy_static! {
    static ref ALLOCATOR: Mutex<FrameAllocator> = {
        Mutex::new(FrameAllocator::new())
    };
}
```

## 7. 中断处理竞态条件

### 问题：中断期间状态损坏

**解决方案：**
在关键部分禁用中断：

```rust
pub fn critical_section<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // 禁用中断
    let sie = sie::read();
    unsafe { sie::clear_sie(); }
    
    // 执行关键部分
    let result = f();
    
    // 恢复中断
    unsafe { 
        if sie.bits() != 0 {
            sie::set_sie();
        }
    }
    
    result
}
```

## 8. 内存泄漏

### 问题：内存耗尽

**常见原因：**

**1. 忘记 Drop：**
```rust
// 错误：从不 drop
let frame = frame_alloc().unwrap();
forget(frame);  // 内存泄漏！

// 正确：自动 drop
{
    let frame = frame_alloc().unwrap();
}  // 在这里 drop
```

**解决方案：**
到处使用 RAII 模式 - 依赖 Drop trait。

## 9. 汇编/Rust 接口问题

### 问题：错误的调用约定

**示例：**
```asm
# 错误：不保存被调用者保存的寄存器
my_function:
    # 修改 s0-s11 而不保存
    ret

# 正确：
my_function:
    # 保存被调用者保存的寄存器
    addi sp, sp, -16
    sd s0, 0(sp)
    sd ra, 8(sp)
    
    # 函数体
    
    # 恢复寄存器
    ld s0, 0(sp)
    ld ra, 8(sp)
    addi sp, sp, 16
    ret
```

## 10. 构建系统问题

### 问题：链接器错误

**错误：**
```
undefined reference to `__rust_alloc`
```

**解决方案：**
确保 `Cargo.toml` 中有正确的特性：

```toml
[dependencies]
buddy_system_allocator = "0.9"

[profile.release]
panic = "abort"  # #![no_std] 需要
```

## 调试工具和技术

### 1. GDB 与 QEMU

```bash
# 终端 1：用 GDB 服务器启动 QEMU
qemu-system-riscv64 \
    -machine virt \
    -kernel kernel.elf \
    -s -S  # 等待 GDB

# 终端 2：连接 GDB
riscv64-unknown-elf-gdb kernel.elf
(gdb) target remote :1234
(gdb) break rust_main
(gdb) continue
```

### 2. 打印调试

```rust
macro_rules! trace {
    ($($arg:tt)*) => {
        println!("[TRACE {}:{}] {}", 
            file!(), line!(), format_args!($($arg)*))
    };
}

// 使用
trace!("分配帧");
let frame = frame_alloc().unwrap();
trace!("得到帧：{:?}", frame.ppn);
```

### 3. 断言

```rust
// 运行时检查
assert!(frame.is_valid());
assert_eq!(frame.ppn.0 & 0xFFF, 0, "帧未对齐！");

// 仅调试检查
debug_assert!(expensive_check());
```

## 预防检查表

提交代码前：

- [ ] 所有不安全块已记录
- [ ] `cargo clippy` 无警告
- [ ] 用 `cargo fmt` 格式化代码
- [ ] 测试通过
- [ ] 无明显内存泄漏
- [ ] 错误路径已处理
- [ ] 不变量的断言
- [ ] 复杂逻辑的注释

## 有用资源

- Rust 嵌入式书：https://rust-embedded.github.io/book/
- RISC-V ISA 手册：https://riscv.org/technical/specifications/
- rCore 教程：https://rcore-os.github.io/rCore-Tutorial-Book-v3/
- Rust Nomicon（不安全）：https://doc.rust-lang.org/nomicon/

## 贡献调试经验

如果您遇到新问题，请记录：
1. 错误消息
2. 根本原因分析
3. 有效的解决方案
4. 预防技巧

这有助于未来的开发者避免同样的陷阱！
