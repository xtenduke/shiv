#!/bin/bash

INSTALL_PATH="/usr/local/bin"
BINARY_NAME="shivr"

set -e

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    FILENAME="shivr-linux-x86.zip"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    FILENAME="shivr-macos-x86.zip"
fi

if test -f "$FILENAME"; then
    echo "deleting existing $FILENAME"
    rm "$FILENAME"
fi

# Find and download
curl -s https://api.github.com/repos/xtenduke/shivr/releases/latest | grep "browser_download_url.*$FILENAME" \
| head -1 \
| cut -d : -f 2,3 \
| tr -d \" \
| wget --show-progress -qi - \
|| echo "-> Download failed" \

# If not root just extract and set ex
if [[ "$EUID" -ne 0 ]]
then
    unzip -o "$FILENAME"
    chmod +x "$BINARY_NAME"
    echo "didn't install because you are not root, binary at $PWD/$BINARY_NAME"
else
    # Extract to bin dir.
    # Overwriting older versions.
    unzip -o "$FILENAME" -d "$INSTALL_PATH"

    # Make executable
    chmod +x "$INSTALL_PATH/$BINARY_NAME"
    echo "installed $BINARY_NAME to $INSTALL_PATH"
fi

# Clean up
rm "$FILENAME"

