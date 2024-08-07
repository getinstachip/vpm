#!/bin/bash

set -e

# Set the version
# Get the latest version from GitHub tags
VPM_VERSION=$(curl -s https://api.github.com/repos/getinstachip/vpm/tags | sed -n 's/.*"name": "\([^"]*\)".*/\1/p' | head -n 1)
# Remove the 'v' prefix if present
VPM_VERSION=${VPM_VERSION#v}

if [ -z "$VPM_VERSION" ]; then
    echo "Failed to fetch the latest version. Exiting."
    exit 1
fi

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
