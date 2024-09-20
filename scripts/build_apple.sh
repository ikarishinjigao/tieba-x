#!/bin/bash
set -euo pipefail

# Configuration
export IPHONEOS_DEPLOYMENT_TARGET=12
FRAMEWORK_NAME="TiebaxFFI"
RUST_PROJECT_ROOT="."
APPLE_PLATFORM_DIR="platforms/apple"
STAGING_DIR="$APPLE_PLATFORM_DIR/staging"
FRAMEWORKS_PATH="$APPLE_PLATFORM_DIR/app/Frameworks"
SOURCES_PATH="$APPLE_PLATFORM_DIR/app/Sources"
COMMON_DIR="$APPLE_PLATFORM_DIR/common"
BUILD_DIR=".build"

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
for tool in cargo xcodebuild; do
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
rm -rf $STAGING_DIR
mkdir -p $STAGING_DIR

# Build Rust project and generate bindings
print_colored $YELLOW "Building Rust project and generating bindings..."
run_command cargo build
run_command cargo uniffi-bindgen generate --library $BUILD_DIR/debug/libcrypto.a -c components/crypto/uniffi.toml -l swift -o $STAGING_DIR
run_command cargo uniffi-bindgen generate --library $BUILD_DIR/debug/libnetwork.a -c components/network/uniffi.toml -l swift -o $STAGING_DIR

# Build for iOS and iOS Simulator
print_colored $YELLOW "Building for iOS and iOS Simulator..."
run_command cargo build --release --target=aarch64-apple-ios -p=core
run_command cargo build --release --target=aarch64-apple-ios-sim -p=core

# Copy generated Swift files
print_colored $YELLOW "Copying generated Swift files..."
cp $STAGING_DIR/Crypto.swift $SOURCES_PATH/Crypto/
cp $STAGING_DIR/Network.swift $SOURCES_PATH/Network/

# Function to create framework
create_framework() {
  local platform=$1
  local framework_path="$FRAMEWORKS_PATH/$platform/$FRAMEWORK_NAME.framework"
  print_colored $YELLOW "Creating framework for $platform..."
  mkdir -p "$framework_path/Modules" "$framework_path/Headers"
  cp $BUILD_DIR/$platform/release/libtiebax_core.a "$framework_path/$FRAMEWORK_NAME"
  cp $COMMON_DIR/Info.plist "$framework_path"
  cp $COMMON_DIR/module.modulemap "$framework_path/Modules"
  cp $COMMON_DIR/$FRAMEWORK_NAME.h "$framework_path/Headers"
  cp $STAGING_DIR/*FFI.h "$framework_path/Headers"
}

# Create frameworks
create_framework "aarch64-apple-ios"
create_framework "aarch64-apple-ios-sim"

# Create XCFramework
print_colored $YELLOW "Creating XCFramework..."
rm -rf "$FRAMEWORKS_PATH/$FRAMEWORK_NAME.xcframework"
run_command xcodebuild -create-xcframework \
  -framework "$FRAMEWORKS_PATH/aarch64-apple-ios/$FRAMEWORK_NAME.framework" \
  -framework "$FRAMEWORKS_PATH/aarch64-apple-ios-sim/$FRAMEWORK_NAME.framework" \
  -output "$FRAMEWORKS_PATH/$FRAMEWORK_NAME.xcframework"

# Clean up
print_colored $YELLOW "Cleaning up..."
rm -rf "$FRAMEWORKS_PATH/aarch64-apple-ios"
rm -rf "$FRAMEWORKS_PATH/aarch64-apple-ios-sim"
rm -rf "$STAGING_DIR"

print_colored $GREEN "Build completed successfully!"
