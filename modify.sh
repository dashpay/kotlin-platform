#!/bin/bash

# Input file (your header file)
HEADER_FILE="dash_sdk_bindings/target/dash_sdk_bindings.h"

# Temporary file for processing
TEMP_FILE="temp_header_file_modify.h"

# Add pragmas before and after struct platform_value_Value
awk '
    BEGIN { struct_found = 0; }

    # Detect the start of the platform_value_Value struct
    /struct platform_value_Value/ {
        print "#pragma pack(push, 16)";  # Insert pragma before the struct
        struct_found = 1;
    }

    # Print the current line
    { print }

    # Detect the end of the struct and insert pragma after
    struct_found && /};/ {
        print "#pragma pack(pop)";  # Insert pragma after the struct
        struct_found = 0;
    }
' "$HEADER_FILE" > "$TEMP_FILE"

# Replace original header file with modified one
mv "$TEMP_FILE" "$HEADER_FILE"

echo "Pragmas added to $HEADER_FILE"