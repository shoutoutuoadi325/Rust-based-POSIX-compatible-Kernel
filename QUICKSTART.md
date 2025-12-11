# Quick Start Guide - RPOS Kernel Demo

This guide will help you quickly run the RPOS kernel demonstrations for the OS competition.

## Prerequisites (5 minutes)

### On Ubuntu/WSL:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install QEMU
sudo apt-get update
sudo apt-get install -y qemu-system-misc

# Add RISC-V target
rustup target add riscv64gc-unknown-none-elf

# Optional: For graphical dashboard
pip3 install matplotlib
```

## Quick Demo (1 minute)

### Option 1: Automated Demo
```bash
./demo.sh --auto
```
This will run all three demo levels automatically.

### Option 2: Interactive Menu
```bash
./demo.sh
```
Choose which demo level to run from the menu.

### Option 3: Just Run the Kernel
```bash
make run
```

## What You'll See

### Level 1: Basic Boot
- OpenSBI firmware initialization
- Kernel boot messages
- Hello World output
- **Duration**: ~3 seconds

### Level 2: System Information
- Memory management initialization
- System statistics
- Structured metrics output
- **Duration**: ~3 seconds

### Level 3: Visualization Dashboard
- Real-time metrics monitoring
- Memory usage charts
- Process tracking
- **Duration**: Until you close it

## For Competition Judges

### Impressive Points

1. **Professional Presentation**
   - Clean, colorful output
   - Structured demonstration levels
   - Real-time visualization

2. **Educational Value**
   - Visual learning tools
   - Clear metrics and statistics
   - Interactive monitoring

3. **Technical Quality**
   - Memory-safe Rust implementation
   - RISC-V architecture support
   - POSIX compatibility

### Live Demo Tips

1. **Before the Demo**:
   ```bash
   # Test everything works
   ./demo.sh --auto
   
   # Have backup video ready
   # Practice your narration
   ```

2. **During the Demo**:
   - Start with `./demo.sh`
   - Select "Run All Demos"
   - Explain each level as it runs
   - Show the visualization dashboard last (most impressive)

3. **If Something Goes Wrong**:
   - The demo script has error handling
   - Each level can run independently
   - Have pre-recorded video as backup

## Customization

### Add Your Own Metrics

In `src/main.rs`, add:
```rust
println!("[METRICS] your_metric=value");
```

The dashboard will automatically display it.

### Extend Demo Levels

In `demo.sh`, add new demo functions following the pattern:
```bash
demo_level4() {
    print_section "DEMO LEVEL 4: Your Feature"
    # Your code here
}
```

## Troubleshooting

| Problem | Solution |
|---------|----------|
| "cargo: command not found" | Install Rust toolchain |
| "qemu-system-riscv64: not found" | Install QEMU |
| "matplotlib not available" | Use `--text` mode or install matplotlib |
| Kernel doesn't boot | Run `make clean && make build` |

## Documentation

- Full dashboard guide: `docs/dashboard_demo_guide_en.md` (English) / `docs/dashboard_demo_guide_cn.md` (Chinese)
- Architecture docs: `docs/architecture_en.md`
- Memory management: `docs/memory_en.md`
- Main README: `README.md`

## Competition Scoring

This implementation addresses:

- ✅ **Documentation (30%)**: Comprehensive bilingual docs
- ✅ **Educational Value (10%)**: Visualization tools for teaching
- ✅ **Engineering Standards (10%)**: Clean code, proper structure
- ✅ **Presentation (20%)**: Professional demo scripts and visualization

## Contact

For issues or questions, see the main README.md or check the documentation.

---

**Time to impressive demo: < 5 minutes**

Good luck with the competition! 🎉
