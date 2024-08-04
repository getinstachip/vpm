#!/bin/bash

set -e

# Set the version
VPM_VERSION="0.0.4"

# Download the binary
curl -LO "https://github.com/getinstachip/vpm/releases/download/v${VPM_VERSION}/vpm-v${VPM_VERSION}-x86_64-unknown-linux-gnu.tar.gz"

# Extract the binary
tar -xzf "vpm-v${VPM_VERSION}-x86_64-unknown-linux-gnu.tar.gz"

# Make it executable
chmod +x vpm

# Move to a directory in PATH
sudo mv vpm /usr/local/bin/

# Clean up
rm "vpm-v${VPM_VERSION}-x86_64-unknown-linux-gnu.tar.gz"

echo "VPM v${VPM_VERSION} has been installed successfully!"
