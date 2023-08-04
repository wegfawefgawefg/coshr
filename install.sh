#!/bin/bash

# Path where the Rust binary will be installed
BIN_PATH="/usr/local/bin/coshr"

# Path for the wrapper shell script to be installed
SCRIPT_INSTALL_PATH="/usr/local/bin/csc"

# Build the Rust project
cargo build --release

# Install the binary to /usr/local/bin
sudo cp target/release/coshr $BIN_PATH

# Check if binary was successfully copied
if [ -f "$BIN_PATH" ]; then
    echo "coshr binary installed successfully."
else
    echo "Failed to install coshr binary."
    exit 1
fi

# Install the existing shell script
sudo cp csc $SCRIPT_INSTALL_PATH
sudo chmod +x $SCRIPT_INSTALL_PATH

# Check if shell script was successfully copied
if [ -f "$SCRIPT_INSTALL_PATH" ]; then
    echo "Shell script installed successfully."
else
    echo "Failed to install shell script."
    exit 1
fi
