#!/bin/bash

set -e

# Set the version
VPM_VERSION="0.0.4"

# Determine OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Set the filename based on OS and architecture
if [ "$OS" = "linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        FILENAME="vpm-v${VPM_VERSION}-x86_64-unknown-linux-gnu"
    else
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
elif [ "$OS" = "darwin" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        FILENAME="vpm-v${VPM_VERSION}-x86_64-apple-darwin"
    elif [ "$ARCH" = "arm64" ]; then
        FILENAME="vpm-v${VPM_VERSION}-aarch64-apple-darwin"
    else
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
else
    echo "Unsupported operating system: $OS"
    exit 1
fi

# Download the binary
echo "Downloading VPM v${VPM_VERSION} for $OS $ARCH..."
curl -LO "https://github.com/getinstachip/vpm/releases/download/v${VPM_VERSION}/${FILENAME}"

# Make it executable
chmod +x "$FILENAME"

# Move to a directory in PATH
sudo mv "$FILENAME" /usr/local/bin/vpm

echo "VPM v${VPM_VERSION} has been installed successfully!"
