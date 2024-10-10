#!/bin/bash

# Input file (your header file)
HEADER_FILE="dash-sdk-bindings/target/dash_sdk_bindings.h"

# Temporary file for processing
TEMP_FILE="temp_header_file_modify.h"

# Check if the pragmas are already present
if grep -q "#pragma pack(push, 16)" "$HEADER_FILE"; then
    echo "Pragmas already present in $HEADER_FILE. No changes made."
    exit 0
fi

# Add pragmas before and after struct platform_value_Value
awk '
    BEGIN { inside_struct = 0; brace_count = 0; }

    # Detect the start of the platform_value_Value struct
    /struct platform_value_Value/ {
        print "#pragma pack(push, 16)";  # Insert pragma before the struct
        inside_struct = 1;  # Flag to indicate we are inside the struct
    }

    # Track opening braces when inside the struct
    inside_struct && /\{/ {
        brace_count++;
    }

    # Track closing braces when inside the struct
    inside_struct && /\}/ {
        brace_count--;
        if (brace_count == 0) {
            inside_struct = 0;  # We reached the end of the struct
            print;
            print "#pragma pack(pop)";  # Insert pragma after the struct
            next;
        }
    }

    # Print the current line
    { print }
' "$HEADER_FILE" > "$TEMP_FILE"

# Replace original header file with modified one
mv "$TEMP_FILE" "$HEADER_FILE"

echo "Pragmas added to $HEADER_FILE"