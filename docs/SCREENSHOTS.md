# RPOS Dashboard and Demo Screenshots

## Dashboard Text Mode

When running `python3 dashboard.py --text`, you will see:

```
╔════════════════════════════════════════════════════════════╗
║         RPOS Kernel Visualization Dashboard               ║
║     Real-time Monitoring for OS Competition Demo          ║
╚════════════════════════════════════════════════════════════╝

[INFO] Running in text-only mode
[INFO] For graphical dashboard, install: pip3 install matplotlib

[DASHBOARD] Starting RPOS kernel...
[DASHBOARD] Command: qemu-system-riscv64 -machine virt -nographic -bios default -kernel target/riscv64gc-unknown-none-elf/release/rpos-kernel
[DASHBOARD] Mode: Text
[DASHBOARD] Starting monitoring...
------------------------------------------------------------

[Kernel boot messages appear here]

============================================================
  RPOS KERNEL DASHBOARD - Text Mode
============================================================
Uptime: 0.0s

Memory Status:
  Total: 2176 MB
  Used:  1 MB
  Free:  2175 MB
  Usage: [░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 0.0%

Process Management:
  Active Processes: 1
  System Calls:     8

Recent Kernel Logs:
  [KERNEL] Rust-based POSIX-compatible Kernel (RPOS)
  [KERNEL] Version 1.0.0
  [DEMO 1] Hello World Program
  Output: Hello, RPOS Kernel World!
  Status: SUCCESS
  ...
============================================================
```

## Kernel Demo Output

When running `make run`, you will see:

```
OpenSBI v1.3
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|___ /_____|
        | |
        |_|

Platform Name             : riscv-virtio,qemu
...

[KERNEL] Rust-based POSIX-compatible Kernel (RPOS)
[KERNEL] Version 1.0.0
[KERNEL] Starting initialization...
[KERNEL] Memory management initialized
[KERNEL] All initialization complete!
[KERNEL] Memory size: 2176 MB

=== RPOS Kernel Demonstration ===

[DEMO 1] Hello World Program
Output: Hello, RPOS Kernel World!
Status: SUCCESS

[DEMO 2] System Information
Kernel: RPOS v1.0.0
Architecture: RISC-V 64-bit
Page Size: 4096 bytes
Status: SUCCESS

[DEMO 3] Memory Management Statistics
Total Memory: 2176 MB (2281701376 bytes)
Kernel Heap: Initialized with Buddy Allocator
Physical Frames: Managed by Stack Allocator
Virtual Memory: SV39 Paging Enabled
[METRICS] memory_total_mb=2176
[METRICS] memory_used_mb=1
[METRICS] memory_free_mb=2175
Status: SUCCESS

[DEMO 4] Process Management Capabilities
System Calls Implemented:
  - sys_write (64): Write to file descriptor
  - sys_read (63): Read from file descriptor
  - sys_exit (93): Exit process
  - sys_yield (124): Yield CPU
  - sys_getpid (172): Get process ID
  - sys_fork (220): Fork process [STUB]
  - sys_exec (221): Execute program [STUB]
  - sys_waitpid (260): Wait for process [STUB]
[METRICS] process_count=1
[METRICS] syscall_count=8
Status: SUCCESS

[KERNEL] All demos completed successfully!
[KERNEL] Shutting down...
```

## Graphical Dashboard (with matplotlib)

When running `python3 dashboard.py` with matplotlib installed, a GUI window opens with 4 panels:

### Panel Layout:
```
┌─────────────────────────────────────────────┐
│  RPOS Kernel Real-time Dashboard           │
├──────────────────────┬──────────────────────┤
│                      │                      │
│  Memory Usage        │  Memory Trend        │
│  (Pie Chart)         │  (Line Chart)        │
│                      │                      │
│  Shows:              │  Shows:              │
│  - Used (red)        │  - Used over time    │
│  - Free (green)      │  - Free over time    │
│                      │                      │
├──────────────────────┼──────────────────────┤
│                      │                      │
│  System Statistics   │  Recent Logs         │
│                      │                      │
│  Shows:              │  Shows:              │
│  - Uptime            │  - Last 8 log lines  │
│  - Total Memory      │  - Formatted output  │
│  - Process Count     │  - Real-time updates │
│  - Syscall Count     │                      │
│                      │                      │
└──────────────────────┴──────────────────────┘
```

All panels update in real-time every 500ms as the kernel runs.

## Notes for Competition

1. **Text Mode**: Works on all systems, no dependencies
2. **Graphical Mode**: More impressive, requires matplotlib
3. **Metrics**: All tagged with `[METRICS]` for easy parsing

## Recording Tips

For backup video recording:

```bash
# Record terminal session with asciinema
asciinema rec demo.cast

# Or use script command
script -c "make run" demo.log

# Or use standard screen recording tools
```

## For Judges

Key points to highlight:
- ✅ Real-time monitoring capability
- ✅ Educational visualization tools
- ✅ Professional presentation
- ✅ Bilingual documentation
- ✅ Easy to reproduce
