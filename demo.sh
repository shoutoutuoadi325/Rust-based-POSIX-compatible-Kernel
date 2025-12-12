#!/bin/bash
#
# RPOS Kernel Demonstration Script
# For OS Competition - Showcases kernel capabilities at multiple levels
#

set -e

# Configuration
# DEMO_TIMEOUT: Time in seconds to wait for each kernel demo to complete
# Default is 5 seconds which is sufficient for the current demos.
# Increase this value if:
# - Running on slower systems
# - Using different QEMU configurations (e.g., with debugging)
# - Adding more complex demo programs
DEMO_TIMEOUT=5  # Timeout in seconds for kernel demo runs

# AUTO_MODE: Set to true when running in automatic mode (--auto flag)
AUTO_MODE=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Banner
print_banner() {
    echo -e "${CYAN}"
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║                                                                ║"
    echo "║         RPOS Kernel Demonstration Script                      ║"
    echo "║         Rust-based POSIX-compatible Kernel                    ║"
    echo "║                                                                ║"
    echo "║         For Operating System Competition                      ║"
    echo "║                                                                ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# Print section header
print_section() {
    echo ""
    echo -e "${MAGENTA}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${MAGENTA}  $1${NC}"
    echo -e "${MAGENTA}═══════════════════════════════════════════════════════════${NC}"
    echo ""
}

# Print step
print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

# Print success
print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Print info
print_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

# Print error
print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    print_section "Prerequisites Check"
    
    local all_ok=true
    
    # Check for cargo
    print_step "Checking for Rust toolchain..."
    if command -v cargo &> /dev/null; then
        print_success "Rust toolchain found: $(cargo --version)"
    else
        print_error "Rust toolchain not found. Please install from https://rustup.rs"
        all_ok=false
    fi
    
    # Check for QEMU
    print_step "Checking for QEMU RISC-V emulator..."
    if command -v qemu-system-riscv64 &> /dev/null; then
        print_success "QEMU found: $(qemu-system-riscv64 --version | head -1)"
    else
        print_error "QEMU not found. Please install qemu-system-misc"
        all_ok=false
    fi
    
    # Check for RISC-V target
    print_step "Checking for RISC-V target..."
    if rustup target list | grep -q "riscv64gc-unknown-none-elf (installed)"; then
        print_success "RISC-V target installed"
    else
        print_info "Installing RISC-V target..."
        rustup target add riscv64gc-unknown-none-elf
        print_success "RISC-V target installed"
    fi
    
    if [ "$all_ok" = false ]; then
        print_error "Please fix the issues above before running the demo"
        exit 1
    fi
    
    echo ""
}

# Build the kernel
build_kernel() {
    print_section "Building RPOS Kernel"
    
    print_step "Compiling kernel with Rust..."
    print_info "This may take a moment on first build..."
    
    if cargo build --release 2>&1 | grep -E "(Compiling|Finished)"; then
        print_success "Kernel built successfully!"
        print_info "Binary: target/riscv64gc-unknown-none-elf/release/rpos-kernel"
        
        # Show file info
        if [ -f "target/riscv64gc-unknown-none-elf/release/rpos-kernel" ]; then
            local size=$(ls -lh target/riscv64gc-unknown-none-elf/release/rpos-kernel | awk '{print $5}')
            print_info "Kernel size: $size"
        fi
    else
        print_error "Build failed"
        exit 1
    fi
    
    echo ""
}

# Demo Level 1: Basic Boot and Hello World
demo_level1() {
    print_section "DEMO LEVEL 1: Basic Boot + Hello World"
    
    print_info "Demonstrates:"
    echo "  • Successful kernel boot with OpenSBI"
    echo "  • SBI console output working"
    echo "  • Basic initialization (BSS clearing, stack setup)"
    echo "  • Hello World output"
    echo ""
    
    print_step "Running kernel..."
    print_info "Press Ctrl+C to exit QEMU"
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━ KERNEL OUTPUT ━━━━━━━━━━━━━━━━━━${NC}"
    
    timeout $DEMO_TIMEOUT make run 2>&1 || true
    
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    print_success "Level 1 Demo Complete!"
    echo ""
    
    if [ "$AUTO_MODE" = false ]; then
        read -p "Press Enter to continue to Level 2..."
    fi
}

# Demo Level 2: System Information and Metrics
demo_level2() {
    print_section "DEMO LEVEL 2: System Information & Memory Management"
    
    print_info "Demonstrates:"
    echo "  • Memory management initialization (Buddy Allocator)"
    echo "  • Physical frame allocation"
    echo "  • SV39 virtual memory paging"
    echo "  • System information reporting"
    echo "  • Structured metrics output"
    echo ""
    
    print_step "Running kernel with detailed output..."
    print_info "Watch for [METRICS] tags showing memory statistics"
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━ KERNEL OUTPUT ━━━━━━━━━━━━━━━━━━${NC}"
    
    timeout $DEMO_TIMEOUT make run 2>&1 || true
    
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    print_success "Level 2 Demo Complete!"
    echo ""
    
    if [ "$AUTO_MODE" = false ]; then
        read -p "Press Enter to continue to Level 3..."
    fi
}

# Demo Level 3: Visualization Dashboard
demo_level3() {
    print_section "DEMO LEVEL 3: Real-time Visualization Dashboard"
    
    print_info "Demonstrates:"
    echo "  • Real-time kernel metrics monitoring"
    echo "  • Memory usage visualization"
    echo "  • Process state tracking"
    echo "  • Educational dashboard for teaching"
    echo ""
    
    # Setup Python command - use venv if available
    PYTHON_CMD="python3"
    if [ -f "venv/bin/activate" ]; then
        print_info "Using Python virtual environment"
        source venv/bin/activate
        PYTHON_CMD="python"
    fi
    
    # Check if matplotlib is available
    if $PYTHON_CMD -c "import matplotlib" 2>/dev/null; then
        print_step "Starting graphical dashboard..."
        print_info "A new window will open with live charts"
        print_info "The kernel will run and metrics will update in real-time"
        echo ""
        
        $PYTHON_CMD dashboard.py || true
    else
        print_step "Starting text-mode dashboard..."
        print_info "For graphical mode, run: ./setup_venv.sh"
        echo ""
        
        $PYTHON_CMD dashboard.py --text || true
    fi
    
    print_success "Level 3 Demo Complete!"
    echo ""
}

# Show demo summary
show_summary() {
    print_section "Demo Summary"
    
    echo -e "${GREEN}Demonstration completed successfully!${NC}"
    echo ""
    echo "What we showed:"
    echo ""
    echo "  ✓ Level 1: Basic kernel boot and Hello World"
    echo "  ✓ Level 2: Memory management and system information"
    echo "  ✓ Level 3: Real-time visualization dashboard"
    echo ""
    echo "Key Features Demonstrated:"
    echo ""
    echo "  • Rust-based kernel with memory safety"
    echo "  • RISC-V 64-bit architecture support"
    echo "  • SV39 virtual memory management"
    echo "  • Buddy system heap allocator"
    echo "  • POSIX-compatible system calls (stubs)"
    echo "  • Educational visualization tools"
    echo ""
    print_info "For more information, see:"
    echo "  • README.md - Project overview"
    echo "  • docs/ - Detailed documentation (English & Chinese)"
    echo "  • PRD.md - Architecture and design"
    echo ""
}

# Main menu
show_menu() {
    while true; do
        echo ""
        echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
        echo -e "${CYAN}  RPOS Demo Menu${NC}"
        echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
        echo ""
        echo "  1) Run All Demos (Levels 1-3)"
        echo "  2) Demo Level 1: Basic Boot + Hello World"
        echo "  3) Demo Level 2: System Information & Memory"
        echo "  4) Demo Level 3: Visualization Dashboard"
        echo "  5) Just Build Kernel"
        echo "  6) Exit"
        echo ""
        read -p "Select option [1-6]: " choice
        
        case $choice in
            1)
                demo_level1
                demo_level2
                demo_level3
                show_summary
                break
                ;;
            2)
                demo_level1
                ;;
            3)
                demo_level2
                ;;
            4)
                demo_level3
                ;;
            5)
                build_kernel
                ;;
            6)
                echo ""
                print_info "Thank you for watching the RPOS demo!"
                echo ""
                exit 0
                ;;
            *)
                print_error "Invalid option. Please select 1-6."
                ;;
        esac
    done
}

# Main script execution
main() {
    print_banner
    
    # Parse command line arguments
    if [ "$1" = "--auto" ]; then
        # Run all demos automatically
        AUTO_MODE=true
        check_prerequisites
        build_kernel
        demo_level1
        demo_level2
        demo_level3
        show_summary
    elif [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        echo "Usage: $0 [OPTIONS]"
        echo ""
        echo "Options:"
        echo "  --auto      Run all demos automatically"
        echo "  --help, -h  Show this help message"
        echo ""
        echo "Without options, shows interactive menu"
        exit 0
    else
        # Interactive menu mode
        check_prerequisites
        build_kernel
        show_menu
    fi
}

# Run main
main "$@"
