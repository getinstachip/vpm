#!/bin/bash

set -e

# Set the version
VPM_VERSION="${1:-__VERSION__}"

# Download the tarball
echo "Downloading VPM v${VPM_VERSION}..."
curl -LO "https://github.com/getinstachip/vpm/archive/v${VPM_VERSION}.tar.gz"

# Extract the tarball
echo "Extracting VPM v${VPM_VERSION}..."
tar -xzf "v${VPM_VERSION}.tar.gz"

# Navigate to the extracted directory
cd "vpm-${VPM_VERSION}"

# Build the binary using Cargo
echo "Building VPM..."
cargo build --release

# Move the binary to a directory in PATH
sudo mv target/release/vpm /usr/local/bin/

# Clean up
cd ..
rm -rf "vpm-${VPM_VERSION}"
rm "v${VPM_VERSION}.tar.gz"

echo "VPM v${VPM_VERSION} has been installed successfully!"
