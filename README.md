# Rust-based POSIX-compatible Kernel (RPOS)

A monolithic operating system kernel built from scratch in Rust for RISC-V 64-bit architecture, designed to run simple C-based user applications with POSIX compatibility.

[中文版本](README_CN.md) | **[Quick Start Guide](QUICKSTART.md)** ⚡

## Features

- **Memory Safety**: Built with Rust's safety guarantees
- **POSIX Compatible**: Implements standard POSIX system calls
- **SV39 Paging**: Virtual memory management for RISC-V
- **Buddy System Allocator**: Efficient kernel heap management
- **Process Management**: Task scheduling and context switching
- **System Calls**: Standard POSIX syscalls (fork, exec, wait, etc.)
- **RISC-V Architecture**: Targets RISC-V 64-bit (RV64GC)
- **🎨 Visualization Dashboard**: Real-time kernel metrics monitoring

## Requirements

### For Building
- Rust nightly toolchain (nightly-2023-11-01)
- RISC-V target: \`riscv64gc-unknown-none-elf\`
- cargo, rustc, rustfmt, clippy

### For Running
- QEMU RISC-V system emulator (qemu-system-riscv64)
- Or compatible RISC-V hardware

### For WSL/Ubuntu
\`\`\`bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install QEMU
sudo apt-get update
sudo apt-get install qemu-system-misc

# Add RISC-V target
rustup target add riscv64gc-unknown-none-elf
\`\`\`

## Building

\`\`\`bash
# Build the kernel
make build

# Or use cargo directly
cargo build --release
\`\`\`

## Running

\`\`\`bash
# Run in QEMU
make run

# Clean build artifacts
make clean
\`\`\`

## Demonstration

For OS Competition, we provide a visualization dashboard:

### Visualization Dashboard

**Setup (Recommended):**
\`\`\`bash
# Setup Python virtual environment with dependencies
./setup_venv.sh

# Then run the dashboard
source venv/bin/activate
python dashboard.py
\`\`\`

**Alternative (System-wide installation):**
\`\`\`bash
# Install matplotlib globally
pip3 install --user matplotlib

# Run dashboard
python3 dashboard.py

# Text-only mode (no matplotlib required)
python3 dashboard.py --text
\`\`\`

The dashboard displays:
- Real-time memory usage charts
- Process state tracking
- System call statistics
- Live kernel log monitoring

## Project Structure

\`\`\`
.
├── src/
│   ├── main.rs           # Kernel entry point
│   ├── console.rs        # Console output
│   ├── config.rs         # Configuration constants
│   ├── lang_items.rs     # Panic handler
│   ├── sbi/              # SBI interface
│   ├── mm/               # Memory management
│   │   ├── address.rs    # Address types
│   │   ├── frame_allocator.rs  # Physical frame allocator
│   │   ├── heap_allocator.rs   # Kernel heap
│   │   └── page_table.rs       # SV39 page tables
│   ├── trap/             # Trap handling
│   │   ├── context.rs    # Trap context
│   │   └── trap.S        # Trap entry/exit
│   └── syscall/          # System calls
│       ├── fs.rs         # File system syscalls
│       └── process.rs    # Process syscalls
├── docs/                 # Documentation
├── Makefile             # Build script
└── Cargo.toml           # Rust dependencies
\`\`\`

## Architecture

The kernel follows a monolithic architecture with these key components:

1. **Boot & Initialization**: SBI-based boot, stack setup, BSS clearing
2. **Memory Management**: Buddy allocator, SV39 paging, frame management
3. **Trap Handling**: Exception and interrupt handling
4. **System Calls**: POSIX-compatible syscall interface
5. **Process Management**: Task scheduling and context switching
6. **File System**: Virtual file system interface

## Documentation

- [Architecture Design (English)](docs/architecture_en.md)
- [Memory Management (English)](docs/memory_en.md)
- [Process Scheduling (English)](docs/scheduling_en.md)
- [Testing Report (English)](docs/testing_en.md)
- [Debugging Guide (English)](docs/debugging_en.md)

## License

- Source code: GNU GPL-3.0-or-later. See [LICENSE](LICENSE).
- Documentation: CC BY-SA 4.0. See [LICENSE-DOCS](LICENSE-DOCS).

## Contributors

RPOS Team

## Acknowledgments

Built for the Operating System Competition, following POSIX standards and Rust best practices.
