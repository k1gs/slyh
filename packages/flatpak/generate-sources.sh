#!/bin/bash
set -e

cd "$(dirname "$(realpath "$0")")" || exit 1

if ! command -v flatpak-cargo-generator.py &> /dev/null
then
    echo "Downloading flatpak-cargo-generator.py..."
    wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
    chmod +x flatpak-cargo-generator.py
    export PATH="$PWD:$PATH"
fi

flatpak-cargo-generator.py ../../Cargo.lock -o cargo-sources.json
rm flatpak-cargo-generator.py