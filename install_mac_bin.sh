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

# Download the binary
echo "Downloading VPM v${VPM_VERSION}..."
curl -L -o vpm "https://github.com/getinstachip/vpm/releases/download/v${VPM_VERSION}/vpm-x86_64-apple-darwin"

# Make the binary executable
chmod +x vpm

# Move the binary to a directory in PATH
sudo mv vpm /usr/local/bin/

echo "VPM v${VPM_VERSION} has been installed successfully!"
