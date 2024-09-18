export IPHONEOS_DEPLOYMENT_TARGET=12

cargo build
rm -rf platforms/apple/gen
mkdir -p platforms/apple/gen
cargo uniffi-bindgen generate \
  --library .build/debug/libcrypto.a \
  -c components/crypto/uniffi.toml \
  -l swift \
  -o platforms/apple/gen
cargo uniffi-bindgen generate \
  --library .build/debug/libnetwork.a \
  -c components/network/uniffi.toml \
  -l swift \
  -o platforms/apple/gen

cargo build --release --target=aarch64-apple-ios -p=core
cargo build --release --target=aarch64-apple-ios-sim -p=core

cp platforms/apple/gen/Crypto.swift platforms/apple/package/Sources/Crypto
cp platforms/apple/gen/Network.swift platforms/apple/package/Sources/Network

rm -rf platforms/apple/package/Frameworks
# ios
mkdir -p platforms/apple/package/Frameworks/ios/TiebaxCore.framework/Modules
mkdir -p platforms/apple/package/Frameworks/ios/TiebaxCore.framework/Headers
cp .build/aarch64-apple-ios/release/libtiebax_core.a platforms/apple/package/Frameworks/ios/TiebaxCore.framework/TiebaxCore
cp platforms/apple/common/module.modulemap platforms/apple/package/Frameworks/ios/TiebaxCore.framework/Modules
cp platforms/apple/common/TiebaxCore.h platforms/apple/package/Frameworks/ios/TiebaxCore.framework/Headers
cp platforms/apple/common/Info.plist platforms/apple/package/Frameworks/ios/TiebaxCore.framework
cp platforms/apple/gen/CryptoFFI.h platforms/apple/package/Frameworks/ios/TiebaxCore.framework/Headers
cp platforms/apple/gen/NetworkFFI.h platforms/apple/package/Frameworks/ios/TiebaxCore.framework/Headers
# ios-sim
mkdir -p platforms/apple/package/Frameworks/ios-sim/TiebaxCore.framework/Modules
mkdir -p platforms/apple/package/Frameworks/ios-sim/TiebaxCore.framework/Headers
cp .build/aarch64-apple-ios-sim/release/libtiebax_core.a platforms/apple/package/Frameworks/ios-sim/TiebaxCore.framework/TiebaxCore
cp platforms/apple/common/module.modulemap platforms/apple/package/Frameworks/ios-sim/TiebaxCore.framework/Modules
cp platforms/apple/common/TiebaxCore.h platforms/apple/package/Frameworks/ios-sim/TiebaxCore.framework/Headers
cp platforms/apple/common/Info.plist platforms/apple/package/Frameworks/ios-sim/TiebaxCore.framework
cp platforms/apple/gen/CryptoFFI.h platforms/apple/package/Frameworks/ios-sim/TiebaxCore.framework/Headers
cp platforms/apple/gen/NetworkFFI.h platforms/apple/package/Frameworks/ios-sim/TiebaxCore.framework/Headers
# xc-framework
xcodebuild -create-xcframework \
  -framework platforms/apple/package/Frameworks/ios/TiebaxCore.framework \
  -framework platforms/apple/package/Frameworks/ios-sim/TiebaxCore.framework \
  -output platforms/apple/package/Frameworks/TiebaxCore.xcframework
rm -rf platforms/apple/package/Frameworks/ios
rm -rf platforms/apple/package/Frameworks/ios-sim
