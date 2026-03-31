#!/bin/bash
# Setup script for COSMIC Calculator
# This clones the required version of cosmic-text for building.

set -e

VENDOR_DIR="vendor/cosmic-text"
COSMIC_TEXT_REPO="https://github.com/pop-os/cosmic-text.git"
# Pin to the commit before the breaking "lazy setter" API change
COSMIC_TEXT_REV="e5926ae"

if [ -d "$VENDOR_DIR" ]; then
    echo "vendor/cosmic-text already exists, skipping clone."
else
    echo "Cloning cosmic-text at compatible revision $COSMIC_TEXT_REV..."
    mkdir -p vendor
    git clone "$COSMIC_TEXT_REPO" "$VENDOR_DIR"
    cd "$VENDOR_DIR"
    git checkout "$COSMIC_TEXT_REV"
    cd - > /dev/null
    echo "Done."
fi

echo ""
echo "Setup complete! You can now build with:"
echo "  cargo build --release"
echo "  # or: just run"
