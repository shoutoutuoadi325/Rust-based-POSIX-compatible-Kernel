# Bug Fixes and Improvements

## Date: 2025-12-12

### Issues Fixed

#### 1. Demo Script Auto Mode Read Error

**Problem:** When running `./demo.sh --auto`, the script failed with error:
```
read: read error: 0: Resource temporarily unavailable
```

**Root Cause:** The script used `read -p` commands to pause between demo levels, which doesn't work in non-interactive/automated mode.

**Solution:** 
- Added `AUTO_MODE` flag that is set to `true` when `--auto` is passed
- Modified all `read -p` prompts to check `AUTO_MODE` before executing:
  ```bash
  if [ "$AUTO_MODE" = false ]; then
      read -p "Press Enter to continue to Level 2..."
  fi
  ```
- This allows the script to run continuously in auto mode while still prompting in interactive mode

**Files Modified:**
- `demo.sh`: Lines 11-12 (added AUTO_MODE flag), lines 158-161, 189-192, 307-315

#### 2. Python Virtual Environment Support

**Problem:** User requested Python venv usage instead of global pip installation to avoid package conflicts.

**Solution:**
- Created `setup_venv.sh` script for one-command venv setup with matplotlib
- Updated `demo.sh` to automatically detect and use venv if available:
  ```bash
  PYTHON_CMD="python3"
  if [ -f "venv/bin/activate" ]; then
      source venv/bin/activate
      PYTHON_CMD="python"
  fi
  ```
- Updated `.gitignore` to exclude venv directory and Python cache files
- Updated all documentation to recommend venv usage

**Files Created:**
- `setup_venv.sh`: New script for venv setup

**Files Modified:**
- `demo.sh`: Lines 205-217 (venv detection and usage)
- `.gitignore`: Added venv/, __pycache__/, *.pyc, *.pyo
- `README.md`: Updated Visualization Dashboard section
- `README_CN.md`: Updated 可视化仪表盘 section
- `QUICKSTART.md`: Updated prerequisites section
- `docs/dashboard_demo_guide_en.md`: Added venv method
- `docs/dashboard_demo_guide_cn.md`: Added venv method

### Testing

Both fixes have been tested:
- ✅ `./demo.sh --auto` runs without errors
- ✅ `setup_venv.sh` syntax validated
- ✅ Demo script correctly detects and uses venv
- ✅ Kernel builds successfully (0 errors, 0 warnings)

### Usage

**Recommended workflow:**
```bash
# One-time setup
./setup_venv.sh

# Run demo (venv automatically activated)
./demo.sh --auto

# Or interactive mode
./demo.sh
```

**Alternative (without venv):**
```bash
pip3 install --user matplotlib
./demo.sh --auto
```

### Commit

Commit hash: a91bba7
Message: "Fix demo.sh auto mode and add Python venv support"
