#!/bin/bash
set -euo pipefail

# Configuration
BUILD_DIR=".build"
ANDROID_APP_DIR="platforms/android/app"
CORE_DIR="$ANDROID_APP_DIR/core"
JNI_LIBS_DIR="$CORE_DIR/ffi/src/main/jniLibs"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_colored() {
  local color=$1
  local message=$2
  echo -e "${color}${message}${NC}"
}

# Function to check if a command exists
command_exists() {
  command -v "$1" >/dev/null 2>&1
}

# Check for required tools
for tool in cargo cargo-ndk; do
  if ! command_exists $tool; then
    print_colored $RED "Error: $tool is not installed. Please install it and try again."
    exit 1
  fi
done

# Function to run a command with error handling
run_command() {
  if ! "$@"; then
    print_colored $RED "Error: Command failed: $*"
    exit 1
  fi
}

# Clean build directories
print_colored $YELLOW "Cleaning build directories..."
rm -rf $JNI_LIBS_DIR

# Build Rust project and generate bindings
print_colored $YELLOW "Building Rust project and generating bindings..."
run_command cargo build
run_command cargo uniffi-bindgen generate --library $BUILD_DIR/debug/libtiebax_core.so --crate crypto -c components/crypto/uniffi.toml -l kotlin -o $CORE_DIR/crypto/src/main/kotlin
run_command cargo uniffi-bindgen generate --library $BUILD_DIR/debug/libtiebax_core.so --crate network -c components/network/uniffi.toml -l kotlin -o $CORE_DIR/network/src/main/kotlin

# Build for Android
print_colored $YELLOW "Building for Android..."
run_command cargo ndk \
  --manifest-path components/core/Cargo.toml \
  -o $JNI_LIBS_DIR \
  -t armeabi-v7a \
  -t arm64-v8a \
  -t x86 \
  -t x86_64 \
  build --release

print_colored $GREEN "Build completed successfully!"
