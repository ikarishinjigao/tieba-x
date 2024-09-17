cargo build --release --package=network
cargo uniffi-bindgen generate \
  --library .build/release/libnetwork.a \
  --config components/network/uniffi.toml \
  --language swift \
  --out-dir ./platforms/apple/gen

cargo build --release --target=aarch64-apple-ios --package=network
cargo build --release --target=aarch64-apple-ios-sim --package=network
mv platforms/apple/gen/TiebaxNetworkFFI.modulemap platforms/apple/gen/module.modulemap
mv platforms/apple/gen/TiebaxNetwork.swift platforms/apple/package/Sources/TiebaxNetwork/TiebaxNetwork.swift

rm -rf platforms/apple/package/TiebaxNetworkFFI.xcframework
xcodebuild -create-xcframework \
  -library .build/aarch64-apple-ios-sim/release/libnetwork.a -headers ./platforms/apple/gen \
  -library .build/aarch64-apple-ios/release/libnetwork.a -headers ./platforms/apple/gen \
  -output "platforms/apple/package/TiebaxNetworkFFI.xcframework"
