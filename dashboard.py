#!/usr/bin/env python3
"""
RPOS Kernel Visualization Dashboard
Real-time monitoring and visualization of kernel metrics

For OS Competition - Demonstrates educational value through visualization
"""

import sys
import re
import subprocess
import threading
import time
from collections import deque
from datetime import datetime

# Configuration constants
LOG_TRUNCATE_LENGTH = 60  # Maximum length for log lines in display
DASHBOARD_UPDATE_INTERVAL_MS = 500  # Update interval in milliseconds

# Try to import visualization libraries, fallback to text mode
try:
    import matplotlib
    # Check if display is available before setting backend
    import os
    if os.environ.get('DISPLAY') or os.environ.get('WAYLAND_DISPLAY'):
        matplotlib.use('TkAgg')  # Use TkAgg backend for interactive display
    else:
        # No display available - will use text mode (expected for headless systems)
        raise ImportError("Text mode")
    import matplotlib.pyplot as plt
    from matplotlib.animation import FuncAnimation
    GRAPHICS_AVAILABLE = True
except (ImportError, RuntimeError):
    GRAPHICS_AVAILABLE = False
    print("[WARNING] matplotlib not available or no display found. Running in text-only mode.")
    print("[INFO] Install with: pip3 install --user matplotlib")


class KernelMonitor:
    """Monitor kernel output and extract metrics"""
    
    def __init__(self):
        self.memory_total = 0
        self.memory_used = 0
        self.memory_free = 0
        self.process_count = 0
        self.syscall_count = 0
        self.boot_time = None
        self.logs = deque(maxlen=100)  # Keep last 100 log lines
        
        # Time series data for charts
        self.timestamps = deque(maxlen=50)
        self.memory_used_history = deque(maxlen=50)
        self.memory_free_history = deque(maxlen=50)
        
    def parse_line(self, line):
        """Parse a log line and extract metrics"""
        self.logs.append(line)
        
        # Check for boot
        if "[KERNEL]" in line and "initialization..." in line:
            self.boot_time = datetime.now()
        
        # Parse METRICS lines
        if "[METRICS]" in line:
            # Extract key=value pairs
            match = re.search(r'memory_total_mb=(\d+)', line)
            if match:
                self.memory_total = int(match.group(1))
            
            match = re.search(r'memory_used_mb=(\d+)', line)
            if match:
                self.memory_used = int(match.group(1))
                
            match = re.search(r'memory_free_mb=(\d+)', line)
            if match:
                self.memory_free = int(match.group(1))
            
            match = re.search(r'process_count=(\d+)', line)
            if match:
                self.process_count = int(match.group(1))
            
            match = re.search(r'syscall_count=(\d+)', line)
            if match:
                self.syscall_count = int(match.group(1))
    
    def update_history(self):
        """Add current metrics to history"""
        self.timestamps.append(time.time())
        self.memory_used_history.append(self.memory_used)
        self.memory_free_history.append(self.memory_free)


class TextDashboard:
    """Text-based dashboard for when matplotlib is not available"""
    
    def __init__(self, monitor):
        self.monitor = monitor
        
    def display(self):
        """Display dashboard in text format"""
        print("\n" + "="*60)
        print("  RPOS KERNEL DASHBOARD - Text Mode")
        print("="*60)
        
        if self.monitor.boot_time:
            uptime = datetime.now() - self.monitor.boot_time
            print(f"Uptime: {uptime.total_seconds():.1f}s")
        
        print(f"\nMemory Status:")
        print(f"  Total: {self.monitor.memory_total} MB")
        print(f"  Used:  {self.monitor.memory_used} MB")
        print(f"  Free:  {self.monitor.memory_free} MB")
        
        if self.monitor.memory_total > 0:
            usage_pct = (self.monitor.memory_used / self.monitor.memory_total) * 100
            bar_width = 40
            filled = int(bar_width * usage_pct / 100)
            bar = "█" * filled + "░" * (bar_width - filled)
            print(f"  Usage: [{bar}] {usage_pct:.1f}%")
        
        print(f"\nProcess Management:")
        print(f"  Active Processes: {self.monitor.process_count}")
        print(f"  System Calls:     {self.monitor.syscall_count}")
        
        print(f"\nRecent Kernel Logs:")
        for log in list(self.monitor.logs)[-10:]:
            print(f"  {log}")
        
        print("="*60 + "\n")


class GraphicalDashboard:
    """Graphical dashboard using matplotlib"""
    
    def __init__(self, monitor):
        self.monitor = monitor
        self.fig, self.axes = plt.subplots(2, 2, figsize=(12, 8))
        self.fig.suptitle('RPOS Kernel Real-time Dashboard', fontsize=16)
        
    def update(self, frame):
        """Update all plots"""
        # Clear all axes
        for ax in self.axes.flat:
            ax.clear()
        
        # Plot 1: Memory Usage Pie Chart
        ax1 = self.axes[0, 0]
        if self.monitor.memory_total > 0:
            sizes = [self.monitor.memory_used, self.monitor.memory_free]
            labels = [f'Used ({self.monitor.memory_used} MB)', 
                     f'Free ({self.monitor.memory_free} MB)']
            colors = ['#ff6b6b', '#51cf66']
            ax1.pie(sizes, labels=labels, colors=colors, autopct='%1.1f%%', startangle=90)
            ax1.set_title('Memory Usage')
        else:
            ax1.text(0.5, 0.5, 'Waiting for data...', ha='center', va='center')
            ax1.set_title('Memory Usage')
        
        # Plot 2: Memory Usage Over Time
        ax2 = self.axes[0, 1]
        if len(self.monitor.memory_used_history) > 0:
            x = list(range(len(self.monitor.memory_used_history)))
            ax2.plot(x, list(self.monitor.memory_used_history), 'r-', label='Used', linewidth=2)
            ax2.plot(x, list(self.monitor.memory_free_history), 'g-', label='Free', linewidth=2)
            ax2.set_xlabel('Time (samples)')
            ax2.set_ylabel('Memory (MB)')
            ax2.set_title('Memory Trend')
            ax2.legend()
            ax2.grid(True, alpha=0.3)
        else:
            ax2.text(0.5, 0.5, 'Collecting data...', ha='center', va='center')
            ax2.set_title('Memory Trend')
        
        # Plot 3: System Statistics
        ax3 = self.axes[1, 0]
        stats = [
            f'Total Memory: {self.monitor.memory_total} MB',
            f'Processes: {self.monitor.process_count}',
            f'System Calls: {self.monitor.syscall_count}',
        ]
        if self.monitor.boot_time:
            uptime = datetime.now() - self.monitor.boot_time
            stats.insert(0, f'Uptime: {uptime.total_seconds():.1f}s')
        
        ax3.axis('off')
        ax3.text(0.1, 0.9, 'System Statistics:', fontsize=12, fontweight='bold')
        for i, stat in enumerate(stats):
            ax3.text(0.1, 0.7 - i*0.15, stat, fontsize=10)
        ax3.set_title('Kernel Status')
        
        # Plot 4: Recent Logs
        ax4 = self.axes[1, 1]
        ax4.axis('off')
        ax4.text(0.05, 0.95, 'Recent Kernel Logs:', fontsize=10, fontweight='bold')
        recent_logs = list(self.monitor.logs)[-8:]
        for i, log in enumerate(recent_logs):
            # Truncate long logs to fit in display
            log_text = log[:LOG_TRUNCATE_LENGTH] + '...' if len(log) > LOG_TRUNCATE_LENGTH else log
            y_pos = 0.85 - i * 0.1
            ax4.text(0.05, y_pos, log_text, fontsize=8, family='monospace')
        ax4.set_title('Console Output')
        
        plt.tight_layout()
    
    def show(self):
        """Start the dashboard"""
        ani = FuncAnimation(
            self.fig, 
            self.update, 
            interval=DASHBOARD_UPDATE_INTERVAL_MS,
            cache_frame_data=False
        )
        plt.show()


def run_kernel_and_monitor(monitor, text_mode=False):
    """Run the kernel and monitor its output"""
    kernel_path = "target/riscv64gc-unknown-none-elf/release/rpos-kernel"
    
    cmd = [
        "qemu-system-riscv64",
        "-machine", "virt",
        "-nographic",
        "-bios", "default",
        "-kernel", kernel_path
    ]
    
    print(f"[DASHBOARD] Starting RPOS kernel...")
    print(f"[DASHBOARD] Command: {' '.join(cmd)}")
    print(f"[DASHBOARD] Mode: {'Text' if text_mode else 'Graphical'}")
    print(f"[DASHBOARD] Starting monitoring...")
    print("-" * 60)
    
    try:
        process = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            stdin=subprocess.DEVNULL,
            universal_newlines=True,
            bufsize=1
        )
        
        # Read output line by line
        for line in process.stdout:
            line = line.rstrip()
            if line:
                print(line)  # Echo to console
                monitor.parse_line(line)
                monitor.update_history()
                
                # In text mode, periodically update display
                if text_mode and "[METRICS]" in line:
                    dashboard = TextDashboard(monitor)
                    dashboard.display()
        
        process.wait()
        print(f"\n[DASHBOARD] Kernel exited with code {process.returncode}")
        
    except FileNotFoundError:
        print(f"[ERROR] QEMU or kernel not found. Please build the kernel first:")
        print(f"        make build")
        sys.exit(1)
    except KeyboardInterrupt:
        print(f"\n[DASHBOARD] Interrupted by user")
        process.terminate()


def main():
    """Main entry point"""
    print("╔════════════════════════════════════════════════════════════╗")
    print("║         RPOS Kernel Visualization Dashboard               ║")
    print("║     Real-time Monitoring for OS Competition Demo          ║")
    print("╚════════════════════════════════════════════════════════════╝")
    
    monitor = KernelMonitor()
    
    # Check if we can use graphical mode
    use_text_mode = not GRAPHICS_AVAILABLE or '--text' in sys.argv
    
    if use_text_mode:
        print("\n[INFO] Running in text-only mode")
        print("[INFO] For graphical dashboard, install: pip3 install matplotlib\n")
        run_kernel_and_monitor(monitor, text_mode=True)
    else:
        print("\n[INFO] Starting graphical dashboard...")
        print("[INFO] Kernel output will appear in console")
        print("[INFO] Dashboard will update in real-time in a new window\n")
        
        # Run kernel monitoring in a separate thread
        monitor_thread = threading.Thread(
            target=run_kernel_and_monitor,
            args=(monitor, False),
            daemon=True
        )
        monitor_thread.start()
        
        # Give kernel a moment to start
        time.sleep(1)
        
        # Show graphical dashboard (blocking)
        dashboard = GraphicalDashboard(monitor)
        dashboard.show()
        
        # Wait for monitor thread
        monitor_thread.join(timeout=5)


if __name__ == "__main__":
    main()
