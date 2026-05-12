#!/bin/bash
set -e

# Run this script to generate or update cargo-sources.json for Flatpak.
# Requires flatpak-cargo-generator.py in your PATH or downloaded.

if ! command -v flatpak-cargo-generator.py &> /dev/null
then
    echo "Downloading flatpak-cargo-generator.py..."
    wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
    chmod +x flatpak-cargo-generator.py
    export PATH="$PWD:$PATH"
fi

flatpak-cargo-generator.py ../../Cargo.lock -o cargo-sources.json
