#!/bin/bash

# Define the release information
RELEASE_URL="https://api.github.com/repos/tsirysndr/crosup/releases/latest"
ASSET_NAME="_x86_64-unknown-linux-gnu.tar.gz"

# Retrieve the download URL for the desired asset
DOWNLOAD_URL=$(curl -sSL $RELEASE_URL | grep -o "browser_download_url.*$ASSET_NAME\"" | cut -d ' ' -f 2)

ASSET_NAME=$(basename $DOWNLOAD_URL)

# Define the installation directory
INSTALL_DIR="/usr/local/bin"

DOWNLOAD_URL=`echo $DOWNLOAD_URL | tr -d '\"'`

# Download the asset
wget $DOWNLOAD_URL -O /tmp/$ASSET_NAME

# Extract the asset
tar -xzf /tmp/$ASSET_NAME -C /tmp

# Move the extracted binary to the installation directory
sudo mv /tmp/crosup $INSTALL_DIR

# Set the correct permissions for the binary
chmod +x $INSTALL_DIR/crosup

# Clean up temporary files
rm /tmp/$ASSET_NAME

echo "Installation completed! 🎉"

cat << EOF
             ______                __  __    
            / ____/________  _____/ / / /___ 
           / /   / ___/ __ \/ ___/ / / / __ \\
          / /___/ /  / /_/ (__  ) /_/ / /_/ /
          \____/_/   \____/____/\____/ .___/ 
                                    /_/      

Quickly setup your development environment on your new Chromebook/ChromeOS 🚀 ✨

https://github.com/tsirysndr/crosup

Please file an issue if you encounter any problems!

===============================================================================

EOF

crosup install