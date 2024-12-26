#!/bin/bash

# Usage: ./check_debug_symbols.sh path/to/your_library.aar

AAR_FILE="$1"

# Check if the .aar file is provided
if [ -z "$AAR_FILE" ]; then
    echo "Usage: $0 path/to/your_library.aar"
    exit 1
fi

# Create a temporary directory for extraction
TEMP_DIR=$(mktemp -d)
echo "Extracting $AAR_FILE to $TEMP_DIR"
unzip -q "$AAR_FILE" -d "$TEMP_DIR"

# Find all .so files in the extracted jni directories
SO_FILES=$(find "$TEMP_DIR/jni" -name "*.so")

# Check each .so file for debug symbols
DEBUG_SYMBOLS_FOUND=false
for SO_FILE in $SO_FILES; do
    echo "Checking $SO_FILE for debug symbols..."
    if readelf --debug-dump "$SO_FILE" 2>/dev/null | grep -q '.debug_info'; then
        echo "Debug symbols found in $SO_FILE"
        DEBUG_SYMBOLS_FOUND=true
    else
        echo "No debug symbols in $SO_FILE"
    fi
done

# Clean up the temporary directory
rm -rf "$TEMP_DIR"

if [ "$DEBUG_SYMBOLS_FOUND" = true ]; then
    echo "Debug symbols found in one or more .so files."
else
    echo "No debug symbols found in any .so files."
fi

