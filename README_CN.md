# 基于 Rust 的 POSIX 兼容内核 (RPOS)

一个从零开始使用 Rust 构建的单内核操作系统，面向 RISC-V 64 位架构，旨在运行简单的基于 C 的用户应用程序，并兼容 POSIX 标准。

[English Version](README.md) | **[快速开始指南](QUICKSTART.md)** ⚡

## 特性

- **内存安全**: 利用 Rust 的安全保证
- **POSIX 兼容**: 实现标准 POSIX 系统调用
- **SV39 分页**: RISC-V 虚拟内存管理
- **伙伴系统分配器**: 高效的内核堆管理
- **进程管理**: 任务调度和上下文切换
- **系统调用**: 标准 POSIX 系统调用 (fork, exec, wait 等)
- **RISC-V 架构**: 面向 RISC-V 64 位 (RV64GC)
- **🎨 可视化仪表盘**: 实时内核指标监控

## 系统要求

### 构建环境
- Rust nightly 工具链 (nightly-2023-11-01)
- RISC-V 目标: \`riscv64gc-unknown-none-elf\`
- cargo, rustc, rustfmt, clippy

### 运行环境
- QEMU RISC-V 系统模拟器 (qemu-system-riscv64)
- 或兼容的 RISC-V 硬件

### WSL/Ubuntu 安装
\`\`\`bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 QEMU
sudo apt-get update
sudo apt-get install qemu-system-misc

# 添加 RISC-V 目标
rustup target add riscv64gc-unknown-none-elf
\`\`\`

## 构建

\`\`\`bash
# 构建内核
make build

# 或直接使用 cargo
cargo build --release
\`\`\`

## 运行

\`\`\`bash
# 在 QEMU 中运行
make run

# 清理构建产物
make clean
\`\`\`

## 演示

为操作系统竞赛准备了可视化仪表盘：

### 可视化仪表盘

**设置（推荐）：**
\`\`\`bash
# 设置 Python 虚拟环境及依赖
./setup_venv.sh

# 然后运行仪表盘
source venv/bin/activate
python dashboard.py
\`\`\`

**备选方案（全局安装）：**
\`\`\`bash
# 全局安装 matplotlib
pip3 install --user matplotlib

# 运行仪表盘
python3 dashboard.py

# 纯文本模式（无需 matplotlib）
python3 dashboard.py --text
\`\`\`

仪表盘功能：
- 实时内存使用图表
- 进程状态跟踪
- 系统调用统计
- 内核日志实时监控

## 项目结构

\`\`\`
.
├── src/
│   ├── main.rs           # 内核入口点
│   ├── console.rs        # 控制台输出
│   ├── config.rs         # 配置常量
│   ├── lang_items.rs     # Panic 处理
│   ├── sbi/              # SBI 接口
│   ├── mm/               # 内存管理
│   │   ├── address.rs    # 地址类型
│   │   ├── frame_allocator.rs  # 物理帧分配器
│   │   ├── heap_allocator.rs   # 内核堆
│   │   └── page_table.rs       # SV39 页表
│   ├── trap/             # 陷阱处理
│   │   ├── context.rs    # 陷阱上下文
│   │   └── trap.S        # 陷阱入口/出口
│   └── syscall/          # 系统调用
│       ├── fs.rs         # 文件系统系统调用
│       └── process.rs    # 进程系统调用
├── docs/                 # 文档
├── Makefile             # 构建脚本
└── Cargo.toml           # Rust 依赖
\`\`\`

## 架构

内核采用单内核架构，包含以下关键组件：

1. **启动与初始化**: 基于 SBI 的启动、栈设置、BSS 清零
2. **内存管理**: 伙伴分配器、SV39 分页、帧管理
3. **陷阱处理**: 异常和中断处理
4. **系统调用**: POSIX 兼容的系统调用接口
5. **进程管理**: 任务调度和上下文切换
6. **文件系统**: 虚拟文件系统接口

## 文档

- [架构设计（中文）](docs/architecture_cn.md)
- [内存管理（中文）](docs/memory_cn.md)
- [进程调度（中文）](docs/scheduling_cn.md)
- [测试报告（中文）](docs/testing_cn.md)
- [调试指南（中文）](docs/debugging_cn.md)

## 许可证

- 源代码：遵循 GNU GPL-3.0-or-later，详见 [LICENSE](LICENSE)。
- 文档：遵循 CC BY-SA 4.0，详见 [LICENSE-DOCS](LICENSE-DOCS)。


## 致谢

本项目为操作系统竞赛而构建，遵循 POSIX 标准和 Rust 最佳实践。

## 讲解视频链接

通过网盘分享的文件：操作系统竞赛讲解.mp4
链接: https://pan.baidu.com/s/10uo1LRNRCtAUmW7szgje9Q?pwd=kxse 提取码: kxse