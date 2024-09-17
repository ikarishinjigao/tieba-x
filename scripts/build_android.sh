cargo build --release --package=network
cargo uniffi-bindgen generate \
  --library .build/release/libnetwork.so \
  --config components/network/uniffi.toml \
  --language kotlin \
  --out-dir platforms/android/app/core/network/src/main/kotlin

rm -rf platforms/android/app/core/network/src/main/jniLibs
cargo ndk \
  -o platforms/android/app/core/network/src/main/jniLibs \
  --manifest-path components/network/Cargo.toml \
  -t armeabi-v7a \
  -t arm64-v8a \
  -t x86 \
  -t x86_64 \
  build --release
