#!/bin/bash

# 16KB Page Size Alignment Verification Script
# This script verifies that native libraries are properly aligned for 16KB page size compatibility

set -e

echo "🔍 Checking 16KB page size alignment of 64-bit native libraries..."

# Function to check alignment of a shared library
check_alignment() {
  local lib_path="$1"
  local lib_name=$(basename "$lib_path")

  echo "Checking $lib_name..."

  # Use llvm-readelf from NDK to check alignment
  local ndk_path="${ANDROID_NDK_ROOT:-$ANDROID_HOME/ndk/29.0.14206865}"
  local readelf="$ndk_path/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-readelf"

  # For macOS, use the darwin prebuilt tools
  if [[ "$OSTYPE" == "darwin"* ]]; then
    readelf="$ndk_path/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-readelf"
  fi

  if [ ! -f "$readelf" ]; then
    echo "❌ Error: llvm-readelf not found at $readelf"
    echo "💡 Make sure ANDROID_NDK_ROOT is set or NDK is installed at $ANDROID_HOME/ndk/29.0.14206865"
    return 1
  fi

  # Check LOAD segments alignment
  local alignments=$($readelf -l "$lib_path" | grep "LOAD" | awk '{print $NF}')

  local all_aligned=true
  for alignment in $alignments; do
    # Convert hex to decimal
    local decimal_alignment=$((alignment))
    local kb_alignment=$((decimal_alignment / 1024))

    echo "  Segment alignment: $alignment ($decimal_alignment bytes = ${kb_alignment}KB)"

    if [ $decimal_alignment -ne 16384 ]; then
      echo "  ❌ Not 16KB aligned (expected 0x4000 = 16384 bytes)"
      all_aligned=false
    else
      echo "  ✅ 16KB aligned"
    fi
  done

  if [ "$all_aligned" = true ]; then
    echo "✅ $lib_name is properly 16KB aligned"
    return 0
  else
    echo "❌ $lib_name has incorrect alignment"
    return 1
  fi
}

# Parse command line arguments
SEARCH_PATH="."
VERBOSE=false
MAX_FILES=20

while [[ $# -gt 0 ]]; do
  case $1 in
    -p|--path)
      SEARCH_PATH="$2"
      shift 2
      ;;
    -v|--verbose)
      VERBOSE=true
      shift
      ;;
    -m|--max-files)
      MAX_FILES="$2"
      shift 2
      ;;
    -h|--help)
      echo "Usage: $0 [OPTIONS]"
      echo "Options:"
      echo "  -p, --path PATH       Search path for native libraries (default: current directory)"
      echo "  -v, --verbose         Enable verbose output"
      echo "  -m, --max-files NUM   Maximum number of files to check (default: 20)"
      echo "  -h, --help           Show this help message"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      echo "Use -h or --help for usage information"
      exit 1
      ;;
  esac
done

# Find all .so files in build outputs (64-bit architectures only)
echo "Finding 64-bit native libraries in build outputs..."
if [ "$VERBOSE" = true ]; then
  echo "Search path: $SEARCH_PATH"
  echo "Max files: $MAX_FILES"
fi

so_files=$(find "$SEARCH_PATH" -path "*/build/intermediates/cxx/*/obj/*/*.so" -type f | grep -E "(Release|RelWithDebInfo)" | grep -E "(arm64-v8a|x86_64)" | head -$MAX_FILES)

if [ -z "$so_files" ]; then
  echo "❌ No native libraries found in build outputs"
  echo "💡 Make sure you've built the project first with: ./gradlew build"
  exit 1
fi

echo "Found native libraries:"
echo "$so_files"
echo

# Check alignment for each library
all_libs_aligned=true
for so_file in $so_files; do
  if ! check_alignment "$so_file"; then
    all_libs_aligned=false
  fi
  echo
done

# Summary
echo "📊 Alignment Verification Summary:"
if [ "$all_libs_aligned" = true ]; then
  echo "🎉 All native libraries are properly 16KB aligned!"
  echo "✅ This build is compatible with 16KB page size devices"
else
  echo "❌ Some libraries are not properly aligned"
  echo "💡 Ensure you're using Android Gradle Plugin 8.5.1+ and target SDK 35+"
  exit 1
fi