#!/bin/bash

# Define the release information
RELEASE_URL="https://api.github.com/repos/tsirysndr/crosup/releases/latest"
ASSET_NAME="_x86_64-unknown-linux-gnu.tar.gz"

# Retrieve the download URL for the desired asset
DOWNLOAD_URL=$(curl -sSL $RELEASE_URL | grep -o "browser_download_url.*$ASSET_NAME\"" | cut -d '"' -f 4)

ASSET_NAME=$(basename $DOWNLOAD_URL)

# Define the installation directory
INSTALL_DIR="/usr/local/bin"

# Download the asset
curl -sSL $DOWNLOAD_URL -o /tmp/$ASSET_NAME

# Extract the asset
tar -xzf /tmp/$ASSET_NAME -C /tmp

# Move the extracted binary to the installation directory
mv /tmp/cros $INSTALL_DIR

# Set the correct permissions for the binary
chmod +x $INSTALL_DIR/crosup

# Clean up temporary files
rm /tmp/$ASSET_NAME

echo "Installation completed!"