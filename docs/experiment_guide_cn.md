# RPOS 内核开发实验指导

## 简介

本实验指南通过在 RPOS 内核中实现关键功能，帮助学生理解操作系统概念。每个实验都建立在之前的工作基础上，包含填空练习以强化学习。

## 前置要求

- Rust 编程基础知识
- 计算机架构理解
- RISC-V 汇编熟悉度（推荐）
- Linux 命令行经验

## 实验环境设置

### 步骤 1：安装所需工具

```bash
# 安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 RISC-V 的 QEMU
sudo apt-get update
sudo apt-get install qemu-system-misc

# 添加 RISC-V 目标
rustup target add riscv64gc-unknown-none-elf

# 安装附加工具
rustup component add rustfmt clippy
```

### 步骤 2：克隆并构建

```bash
git clone <repository-url>
cd Rust-based-POSIX-compatible-Kernel
make build
```

### 步骤 3：测试您的设置

```bash
make run
# 您应该看到：[KERNEL] Rust-based POSIX-compatible Kernel
```

---

## 实验 1：理解启动过程

### 目标
- 理解 RISC-V 启动序列
- 学习 SBI 接口
- 实现控制台输出

### 练习 1.1：通过 SBI 实现 Hello World

**文件**：`src/sbi/mod.rs`

实现缺失的部分：

```rust
/// 向控制台打印一个字符
pub fn console_putchar(c: usize) {
    sbi_call(________, c, 0, 0);  // 填空：哪个 SBI 调用号？
}

/// 从控制台获取一个字符
pub fn console_getchar() -> usize {
    sbi_call(________, 0, 0, 0)   // 填空：SBI_CONSOLE_GETCHAR 值
}
```

**提示**：
- 检查文件顶部的常量定义
- SBI_CONSOLE_PUTCHAR = 1
- SBI_CONSOLE_GETCHAR = 2

---

## 实验 2：内存管理 - 物理帧

### 目标
- 理解物理内存布局
- 实现帧分配器
- 学习 Rust 中的 RAII 模式

### 练习 2.1：帧分配器初始化

**文件**：`src/mm/frame_allocator.rs`

完成初始化函数：

```rust
pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR.exclusive_access().init(
        PhysAddr::from(________ as usize).ceil(),  // 起始：内核结束
        PhysAddr::from(________).floor(),          // 结束：MEMORY_END 常量
    );
}
```

**问题**：
1. 为什么我们从 `ekernel` 之后开始？
2. 为什么起始用 `ceil()`，结束用 `floor()`？

---

## 实验 3：虚拟内存 - 页表

### 目标
- 理解 SV39 分页
- 实现页表操作
- 学习地址转换

### 练习 3.1：页表项标志

**文件**：`src/mm/page_table.rs`

```rust
impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ________ << 10 | flags.bits() as usize,
        }
    }
    
    pub fn is_valid(&self) -> bool {
        (self.flags() & ________) != PTEFlags::empty()
    }
}
```

---

## 实验 4：陷阱处理和系统调用

### 目标
- 理解 RISC-V 陷阱机制
- 实现陷阱处理器
- 创建系统调用接口

### 练习 4.1：陷阱上下文

**文件**：`src/trap/context.rs`

```rust
#[repr(C)]
pub struct TrapContext {
    pub x: [usize; ________],  // 有多少通用寄存器？
    pub sstatus: Sstatus,
    pub sepc: usize,
}
```

---

## 实验 5：进程管理（高级）

### 目标
- 实现任务控制块
- 创建进程调度器
- 理解上下文切换

### 练习 5.1：任务控制块

```rust
pub struct TaskControlBlock {
    pub pid: usize,
    pub parent: Option<usize>,
    pub status: TaskStatus,
    pub cx: TaskContext,
    // 填写缺失字段：
    // - 页表？
    // - 内核栈？
    // - 文件描述符？
}

pub enum TaskStatus {
    Ready,
    ________,  // 任务当前正在执行
    ________,  // 任务正在等待
    Zombie,
}
```

---

## 挑战练习

### 挑战 1：实现写时复制

修改 `fork()` 使用 COW 而不是深拷贝：
1. 父进程和子进程之间共享物理页
2. 将所有页标记为只读
3. 写错误时，复制页

### 挑战 2：ELF 加载器

实现 ELF64 加载器：
1. 解析 ELF 头
2. 加载程序段
3. 设置入口点
4. 创建用户栈

---

## 评分标准

| 实验 | 分数 | 标准 |
|-----|------|------|
| 实验 1 | 10 | 启动和控制台输出 |
| 实验 2 | 15 | 帧分配器工作 |
| 实验 3 | 20 | 页表和转换 |
| 实验 4 | 20 | 陷阱处理和系统调用 |
| 实验 5 | 20 | 进程管理 |
| 挑战 | 15 | 挑战练习 |
| **总计** | **100** | |

---

## 提交指南

1. 代码必须无警告编译
2. 所有测试必须通过
3. 代码应格式化（`cargo fmt`）
4. 包括实验报告，记录：
   - 您学到了什么
   - 面临的挑战
   - 实现的解决方案

---

## 额外资源

- [Rust 书籍](https://doc.rust-lang.org/book/)
- [RISC-V 规范](https://riscv.org/technical/specifications/)
- [RPOS 文档](../README.md)

---

## 结论

通过完成这些实验，学生获得了实践经验：
- Rust 低级系统编程
- 内存管理技术
- 操作系统概念
- RISC-V 架构

这些知识可转移到其他 OS 开发项目和嵌入式系统编程。
