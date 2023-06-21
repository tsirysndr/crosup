#!/bin/bash

readonly MAGENTA="$(tput setaf 5 2>/dev/null || echo '')"
readonly GREEN="$(tput setaf 2 2>/dev/null || echo '')"
readonly CYAN="$(tput setaf 6 2>/dev/null || echo '')"
readonly NO_COLOR="$(tput sgr0 2>/dev/null || echo '')"

# Define the release information
RELEASE_URL="https://api.github.com/repos/tsirysndr/crosup/releases/latest"

# Determine the operating system
OS=$(uname -s)
if [ "$OS" = "Darwin" ]; then
    # Determine the CPU architecture
    ARCH=$(uname -m)
    if [ "$ARCH" = "arm64" ]; then
        ASSET_NAME="_aarch64-apple-darwin.tar.gz"
    else
        ASSET_NAME="_x86_64-apple-darwin.tar.gz"
    fi
elif [ "$OS" = "Linux" ]; then
    ASSET_NAME="_x86_64-unknown-linux-gnu.tar.gz"
else
    echo "Unsupported operating system: $OS"
    exit 1
fi

# Retrieve the download URL for the desired asset
DOWNLOAD_URL=$(curl -sSL $RELEASE_URL | grep -o "browser_download_url.*$ASSET_NAME\"" | cut -d ' ' -f 2)

ASSET_NAME=$(basename $DOWNLOAD_URL)

# Define the installation directory
INSTALL_DIR="/usr/local/bin"

DOWNLOAD_URL=`echo $DOWNLOAD_URL | tr -d '\"'`

# Download the asset
curl -SL $DOWNLOAD_URL -o /tmp/$ASSET_NAME

# Extract the asset
tar -xzf /tmp/$ASSET_NAME -C /tmp

# Set the correct permissions for the binary
chmod +x /tmp/crosup

# Move the extracted binary to the installation directory
# use sudo if OS is Linux
if [ "$OS" = "Linux" ]; then
    if command -v sudo >/dev/null 2>&1; then
        sudo mv /tmp/crosup $INSTALL_DIR
    else
        mv /tmp/crosup $INSTALL_DIR
    fi
else
    mv /tmp/crosup $INSTALL_DIR
fi

# Clean up temporary files
rm /tmp/$ASSET_NAME

echo "Installation completed! 🎉"

cat << EOF
${CYAN}
             ______                __  __    
            / ____/________  _____/ / / /___ 
           / /   / ___/ __ \/ ___/ / / / __ \\
          / /___/ /  / /_/ (__  ) /_/ / /_/ /
          \____/_/   \____/____/\____/ .___/ 
                                    /_/      
${NO_COLOR}
Quickly setup your development environment on your new Chromebook/ChromeOS 🚀 ✨

${GREEN}https://github.com/tsirysndr/crosup${NO_COLOR}

Please file an issue if you encounter any problems!

===============================================================================

EOF

# Check if the "--skip" argument was provided
if [[ "$@" != *--skip* ]]; then
    crosup install --ask
else
    printf "%s\n" "Run ${GREEN}'crosup install'${NO_COLOR} to install your development environment"
fi

