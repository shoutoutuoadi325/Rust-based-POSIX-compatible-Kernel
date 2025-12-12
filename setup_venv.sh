#!/bin/bash
#
# Setup Python Virtual Environment for RPOS Dashboard
# Creates and activates a venv with required dependencies
#

set -e

VENV_DIR="venv"

echo "Setting up Python virtual environment for RPOS dashboard..."

# Check if Python 3 is installed
if ! command -v python3 &> /dev/null; then
    echo "Error: python3 is not installed. Please install Python 3 first."
    exit 1
fi

# Create virtual environment if it doesn't exist
if [ ! -d "$VENV_DIR" ]; then
    echo "Creating virtual environment in $VENV_DIR/"
    python3 -m venv "$VENV_DIR"
    echo "Virtual environment created successfully."
else
    echo "Virtual environment already exists in $VENV_DIR/"
fi

# Activate virtual environment
echo "Activating virtual environment..."
source "$VENV_DIR/bin/activate"

# Upgrade pip
echo "Upgrading pip..."
pip install --upgrade pip

# Install dependencies
echo "Installing matplotlib..."
pip install matplotlib

echo ""
echo "Setup complete!"
echo ""
echo "To activate the virtual environment in the future, run:"
echo "  source venv/bin/activate"
echo ""
echo "To run the dashboard:"
echo "  source venv/bin/activate"
echo "  python dashboard.py"
echo ""
echo "To deactivate when done:"
echo "  deactivate"
