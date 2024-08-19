#!/bin/bash

# Define the repository URL
REPO_URL="https://github.com/getinstachip/vpm.git"

# Define the directory where the tool will be installed
INSTALL_DIR="$HOME/vpmpro"

# Clone the repository
echo "Cloning the repository..."
git clone $REPO_URL $INSTALL_DIR

# Navigate to the installation directory
cd $INSTALL_DIR

# Check if there is a specific build or installation script
if [ -f "setup.sh" ]; then
    echo "Running setup script..."
    ./setup.sh
elif [ -f "Makefile" ]; then
    echo "Building the project..."
    make
    echo "Installing the project..."
    sudo make install
else
    echo "No setup script or Makefile found. Please check the repository for installation instructions."
fi

echo "Installation complete."
